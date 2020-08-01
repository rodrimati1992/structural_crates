use crate::{
    field_access::Access,
    ident_or_index::IdentOrIndex,
    parse_utils::ParsePunctuated,
    structural_alias_impl_mod::{ReplaceBounds, TypeParamBounds},
};

use super::{
    attribute_config::{DropParams, FieldConfig, VariantConfig},
    delegation::{DelegateTo, RawMutImplParam},
    from_structural::{FromStructuralConfig, InitWith},
};

use as_derive_utils::{
    attribute_parsing::with_nested_meta,
    datastructure::{DataStructure, DataVariant, Field, FieldMap, Struct},
    return_spanned_err, return_syn_err, spanned_err,
    utils::{LinearResult, SynResultExt},
};

use core_extensions::matches;

use proc_macro2::Span;

use quote::ToTokens;

use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, Ident, Lit, Meta, MetaList, MetaNameValue,
    NestedMeta, WherePredicate,
};

use std::marker::PhantomData;

#[derive(Debug)]
pub(crate) struct StructuralOptions<'a> {
    pub(crate) from_struc: Option<FromStructuralConfig>,
    pub(crate) variants: Vec<VariantConfig>,
    pub(crate) fields: FieldMap<FieldConfig<'a>>,
    pub(crate) make_variant_count_alias: bool,
    pub(crate) bounds: Punctuated<WherePredicate, syn::Token!(,)>,

    pub(crate) drop_params: DropParams,
    pub(crate) debug_print: bool,
    pub(crate) with_trait_alias: bool,
    pub(crate) generate_docs: bool,
    pub(crate) non_exhaustive_attr: bool,
    pub(crate) delegate_to: Option<DelegateTo<'a>>,

    _marker: PhantomData<&'a ()>,
}

impl<'a> StructuralOptions<'a> {
    fn new(_ds: &'a DataStructure<'a>, this: StructuralAttrs<'a>) -> Result<Self, syn::Error> {
        let StructuralAttrs {
            from_struc,
            variants,
            fields,
            make_variant_count_alias,
            bounds,
            drop_params,
            debug_print,
            with_trait_alias,
            generate_docs,
            non_exhaustive_attr,
            delegate_to,
            errors: _,
            _marker,
        } = this;

        let make_variant_count_alias = match (make_variant_count_alias, non_exhaustive_attr) {
            (Some(span), true) => return_syn_err!(
                span,
                "Cannot use the `#[struc(variant_count_alias)]` attribute on a \
                     `#[non_exhaustive]` enum."
            ),
            (x, _) => x.is_some(),
        };

        Ok(Self {
            from_struc,
            variants,
            fields,
            make_variant_count_alias,
            bounds,
            drop_params,
            debug_print,
            with_trait_alias,
            generate_docs,
            non_exhaustive_attr,
            delegate_to,
            _marker,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////

struct StructuralAttrs<'a> {
    from_struc: Option<FromStructuralConfig>,
    variants: Vec<VariantConfig>,
    fields: FieldMap<FieldConfig<'a>>,
    make_variant_count_alias: Option<Span>,
    bounds: Punctuated<WherePredicate, syn::Token!(,)>,

    drop_params: DropParams,

    debug_print: bool,
    with_trait_alias: bool,
    generate_docs: bool,

    /// Whether the built-in `#[non_exhaustive]` attribute was used.
    non_exhaustive_attr: bool,

    delegate_to: Option<DelegateTo<'a>>,

    errors: LinearResult<()>,

    _marker: PhantomData<&'a ()>,
}

#[derive(Debug, Copy, Clone)]
enum ParseContext<'a> {
    TypeAttr {
        name: &'a Ident,
        data_variant: DataVariant,
    },
    Variant {
        name: &'a Ident,
        index: usize,
        variant: &'a Struct<'a>,
    },
    Field {
        field: &'a Field<'a>,
    },
}

/// Parses the attributes for the `Structural` derive macro.
pub(crate) fn parse_attrs_for_structural<'a>(
    ds: &'a DataStructure<'a>,
) -> Result<StructuralOptions<'a>, syn::Error> {
    let mut this = StructuralAttrs {
        from_struc: None,
        variants: vec![VariantConfig::default(); ds.variants.len()],
        fields: FieldMap::with(ds, |field| FieldConfig {
            field,
            access: Default::default(),
            renamed: Default::default(),
            init_with: None,
            is_impl: None,
            is_pub: field.is_public() || ds.data_variant == DataVariant::Enum,
        }),
        make_variant_count_alias: None,
        bounds: Punctuated::new(),
        drop_params: DropParams::default(),
        debug_print: false,
        with_trait_alias: true,
        generate_docs: matches!(syn::Visibility::Public{..} = ds.vis),
        non_exhaustive_attr: false,
        delegate_to: None,
        errors: LinearResult::ok(()),
        _marker: PhantomData,
    };

    let name = ds.name;

    let ty_ctx = ParseContext::TypeAttr {
        name,
        data_variant: ds.data_variant,
    };
    parse_inner(&mut this, ds.attrs, ty_ctx)?;

    for (var_i, variant) in ds.variants.iter().enumerate() {
        let ctx = ParseContext::Variant {
            name: variant.name,
            index: var_i,
            variant,
        };
        parse_inner(&mut this, variant.attrs, ctx)?;

        for field in variant.fields.iter() {
            parse_inner(&mut this, field.attrs, ParseContext::Field { field })?;
        }
    }

    this.errors.take()?;

    StructuralOptions::new(ds, this)
}

/// Parses an individual attribute
fn parse_inner<'a, I>(
    this: &mut StructuralAttrs<'a>,
    attrs: I,
    pctx: ParseContext<'a>,
) -> Result<(), syn::Error>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    for attr in attrs {
        match attr.parse_meta() {
            // Handling this generically just in case that the `#[non_exhaustive]` attribute
            // gets extended to have parameters,like `#[non_exhaustive(pub)]`
            Ok(meta) if meta.path().is_ident("non_exhaustive") => {
                this.non_exhaustive_attr = true;
            }
            Ok(Meta::List(list)) => {
                parse_attr_list(this, pctx, list).combine_into_err(&mut this.errors);
            }
            Err(e) => {
                this.errors.push_err(e);
            }
            _ => {}
        }
    }
    Ok(())
}

/// Parses an individual attribute list (A `#[attribute( .. )] attribute`).
fn parse_attr_list<'a>(
    this: &mut StructuralAttrs<'a>,
    pctx: ParseContext<'a>,
    list: MetaList,
) -> Result<(), syn::Error> {
    if list.path.is_ident("struc") {
        with_nested_meta("struc", list.nested, |attr| {
            parse_sabi_attr(this, pctx, attr).combine_into_err(&mut this.errors);
            Ok(())
        })?;
    }

    Ok(())
}

/// Parses the contents of a `#[sabi( .. )]` attribute.
fn parse_sabi_attr<'a>(
    this: &mut StructuralAttrs<'a>,
    pctx: ParseContext<'a>,
    attr: Meta,
) -> Result<(), syn::Error> {
    fn make_err(tokens: &dyn ToTokens) -> syn::Error {
        spanned_err!(tokens, "unrecognized attribute")
    }
    match (pctx, attr) {
        (
            ParseContext::Field { field, .. },
            Meta::NameValue(MetaNameValue {
                lit: Lit::Str(value),
                path,
                ..
            }),
        ) => {
            if path.is_ident("rename") {
                this.fields[field].renamed = Some(IdentOrIndex::from(value));
            } else if path.is_ident("access") {
                let access = value.parse::<Access>()?;
                let fa = &mut this.fields[field];
                fa.access = access;
                fa.is_pub = true;
            } else if path.is_ident("impl") {
                if !cfg!(feature = "impl_fields") {
                    return_spanned_err! {
                        path,
                        "\
                            Cannot use the `#[struc(impl=\"Trait\")]` \
                            attribute without enabling the \
                            \"nightly_impl_fields\" or \"impl_fields\" feature.\
                        ",
                    }
                } else if !this.with_trait_alias {
                    return Err(trait_alias_err(&path));
                }
                let bounds: TypeParamBounds = value.parse::<ParsePunctuated<_, _>>()?.list;
                this.fields[field].is_impl = Some(bounds)
            } else if path.is_ident("init_with_fn") {
                set_init_with(this, InitWith::Fn(value.parse()?), field, &path)?;
            } else if path.is_ident("init_with_val") {
                set_init_with(this, InitWith::Val(value.parse()?), field, &path)?;
            } else if path.is_ident("init_with_lit") {
                set_init_with(this, InitWith::Lit(Lit::Str(value)), field, &path)?;
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::Field { field, .. }, Meta::NameValue(MetaNameValue { lit, path, .. })) => {
            if path.is_ident("init_with_lit") {
                set_init_with(this, InitWith::Lit(lit), field, &path)?;
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::Field { field, .. }, Meta::Path(path)) => {
            if path.is_ident("public") {
                this.fields[field].is_pub = true;
            } else if path.is_ident("not_public") || path.is_ident("private") {
                this.fields[field].is_pub = false;
            } else if path.is_ident("delegate_to") {
                parse_delegate_to(this, Default::default(), path.span(), field)?;
            } else if path.is_ident("init_with_default") {
                set_init_with(this, InitWith::Default, field, &path)?;
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::Field { field, .. }, Meta::List(MetaList { path, nested, .. })) => {
            if path.is_ident("delegate_to") {
                parse_delegate_to(this, nested, path.span(), field)?;
            } else {
                return Err(make_err(&path));
            }
        }
        (
            ParseContext::Variant { index, variant, .. },
            Meta::NameValue(MetaNameValue {
                lit: Lit::Str(unparsed_lit),
                path,
                ..
            }),
        ) => {
            if path.is_ident("replace_bounds") {
                if !this.with_trait_alias {
                    return Err(trait_alias_err(&path));
                }

                this.variants[index].replace_bounds = Some(ReplaceBounds {
                    bounds: unparsed_lit.value(),
                    span: unparsed_lit.span(),
                });
            } else if path.is_ident("access") {
                let access = unparsed_lit.parse::<Access>()?;
                for field in &variant.fields {
                    this.fields[field.index].access = access;
                }
            } else if path.is_ident("rename") {
                this.variants[index].renamed = Some(IdentOrIndex::from(unparsed_lit));
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::Variant { index, .. }, Meta::Path(ref path)) => {
            if path.is_ident("newtype") {
                this.variants[index].is_newtype = true;
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::Variant { index, .. }, Meta::List(list)) => {
            if list.path.is_ident("newtype") {
                if !this.with_trait_alias {
                    return Err(trait_alias_err(&list.path));
                }
                let variant_c = &mut this.variants[index];
                variant_c.is_newtype = true;
                parse_is_newtype(variant_c, list.nested)?;
            } else {
                return Err(make_err(&list.path));
            }
        }
        (
            ParseContext::TypeAttr {
                name, data_variant, ..
            },
            Meta::Path(ref path),
        ) => {
            if path.is_ident("debug_print") {
                this.debug_print = true;
            } else if path.is_ident("no_trait") {
                this.with_trait_alias = false;
            } else if path.is_ident("no_docs") {
                this.generate_docs = false;
            } else if path.is_ident("public") {
                for (_, field) in this.fields.iter_mut() {
                    field.is_pub = true;
                }
            } else if path.is_ident("pre_post_drop_fields") {
                this.drop_params.pre_post_drop_fields = true;
            } else if path.is_ident("variant_count_alias") {
                if data_variant != DataVariant::Enum {
                    return_spanned_err! {
                        name,
                        "Can only use `#[struc(variant_count_alias)]` on enums"
                    }
                }
                this.make_variant_count_alias = Some(path.span());
            } else if path.is_ident("not_public") || path.is_ident("private") {
                for (_, field) in this.fields.iter_mut() {
                    field.is_pub = false;
                }
            } else if path.is_ident("from_structural") {
                this.from_struc = Some(parse_from_struc(Punctuated::new())?);
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::TypeAttr { .. }, Meta::List(list)) => {
            if list.path.is_ident("from_structural") {
                this.from_struc = Some(parse_from_struc(list.nested)?);
            } else {
                return Err(make_err(&list));
            }
        }
        (
            ParseContext::TypeAttr { .. },
            Meta::NameValue(MetaNameValue {
                lit: Lit::Str(ref unparsed_lit),
                ref path,
                ..
            }),
        ) => {
            let ident = path.get_ident().ok_or_else(|| make_err(path))?;

            if ident == "access" {
                let access = unparsed_lit.parse::<Access>()?;
                for (_, fa) in this.fields.iter_mut() {
                    fa.access = access;
                }
            } else if ident == "bound" {
                this.bounds.push(unparsed_lit.parse::<WherePredicate>()?);
            } else if ident == "pre_move" {
                if this.drop_params.pre_move.is_some() {
                    return_spanned_err!(
                        ident,
                        "Canno use the `#[struc(pre_move = \"...\"` attribute twice"
                    )
                }
                this.drop_params.pre_move = Some(unparsed_lit.parse::<syn::Path>()?);
            } else {
                return Err(make_err(path));
            }
        }
        (_, x) => return Err(make_err(&x)),
    }
    Ok(())
}

fn set_init_with<'a>(
    this: &mut StructuralAttrs<'a>,
    init_with: InitWith,
    field: &'a Field<'a>,
    tokens: &dyn ToTokens,
) -> Result<(), syn::Error> {
    if this.from_struc.is_none() {
        return_spanned_err!(
            tokens,
            "Cannot use this attribute without deriving FromStructural.\n\
             You can use the `#[struc(from_structural)]` attribute to derive FromStructural.\n\
            "
        )
    }
    this.fields[field].init_with = Some(init_with);
    Ok(())
}

fn trait_alias_err(tokens: &dyn ToTokens) -> syn::Error {
    spanned_err!(
        tokens,
        "Cannot use this attribute when no trait alias is being generated"
    )
}

fn parse_delegate_to<'a>(
    this: &mut StructuralAttrs<'a>,
    list: Punctuated<NestedMeta, syn::Token![,]>,
    span: Span,
    field: &'a Field<'a>,
) -> Result<(), syn::Error> {
    if this.delegate_to.is_some() {
        return_syn_err! {
            span,
            "Cannot use the `#[struc(delegate_to)]` attribute on multiple fields."
        };
    }
    this.with_trait_alias = false;

    let mut delegate_to = DelegateTo {
        field,
        delegation_params: RawMutImplParam::Unspecified,
        bounds: Vec::new(),
        mut_bounds: Vec::new(),
        move_bounds: Vec::new(),
    };

    with_nested_meta("delegate_to", list, |attr| {
        match attr {
            Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(lit),
                ..
            }) => {
                if path.is_ident("bound") {
                    delegate_to.bounds.push(lit.parse()?);
                } else if path.is_ident("mut_bound") {
                    delegate_to.mut_bounds.push(lit.parse()?);
                } else if path.is_ident("move_bound") || path.is_ident("into_bound") {
                    delegate_to.move_bounds.push(lit.parse()?);
                } else {
                    return_spanned_err!(path, "unexpected `#[struc(delegate_to())]` subattribute")
                }
            }
            _ => return_spanned_err!(attr, "unexpected `#[struc(delegate_to())]` subattribute"),
        }
        Ok(())
    })?;

    this.delegate_to = Some(delegate_to);

    Ok(())
}

fn parse_is_newtype(
    this: &mut VariantConfig,
    list: Punctuated<NestedMeta, syn::Token![,]>,
) -> Result<(), syn::Error> {
    with_nested_meta("newtype", list, |attr| {
        match attr {
            Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(lit),
                ..
            }) => {
                if path.is_ident("bound") || path.is_ident("bounds") {
                    this.replace_bounds = Some(ReplaceBounds {
                        bounds: lit.value(),
                        span: lit.span(),
                    });
                } else {
                    return_spanned_err!(path, "unexpected `#[struc(newtype())]` subattribute")
                }
            }
            _ => return_spanned_err!(attr, "unexpected `#[struc(newtype())]` subattribute"),
        }
        Ok(())
    })
}

fn parse_from_struc(
    list: Punctuated<NestedMeta, syn::Token![,]>,
) -> Result<FromStructuralConfig, syn::Error> {
    let mut cfg = FromStructuralConfig {
        bounds: Punctuated::new(),
    };

    const ATTR_MSG: &str = "unexpected `#[struc(from_structural())]` subattribute";

    with_nested_meta("from_structural", list, |attr| {
        match attr {
            Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(lit),
                ..
            }) => {
                if path.is_ident("bound") {
                    cfg.bounds.push(lit.parse::<WherePredicate>()?);
                } else {
                    return_spanned_err!(path, "{}", ATTR_MSG)
                }
            }
            _ => return_spanned_err!(attr, "{}", ATTR_MSG),
        }
        Ok(())
    })?;

    Ok(cfg)
}
