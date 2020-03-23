use crate::arenas::Arenas;

use super::Exhaustiveness;

use as_derive_utils::{
    attribute_parsing::with_nested_meta,
    spanned_err,
    utils::{LinearResult, SynResultExt},
};

use proc_macro2::Span;

use quote::ToTokens;

use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, Ident, Lit, Meta, MetaList, MetaNameValue,
    NestedMeta,
};

use std::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub(crate) struct StructuralAliasOptions<'a> {
    pub(crate) debug_print: bool,
    pub(crate) generate_docs: bool,
    pub(crate) enum_exhaustiveness: Exhaustiveness<'a>,
    errors: LinearResult<()>,

    _marker: PhantomData<&'a ()>,
}

////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub(crate) enum ParseContext<'a> {
    Trait { ident: &'a Ident },
}

/// Parses the attributes for a structural alias.
pub(crate) fn parse_attrs<'a>(
    attrs: &[Attribute],
    trait_ident: &Ident,
    arenas: &'a Arenas,
) -> Result<StructuralAliasOptions<'a>, syn::Error> {
    let mut this = StructuralAliasOptions {
        debug_print: false,
        generate_docs: true,
        enum_exhaustiveness: Exhaustiveness::Nonexhaustive,
        errors: LinearResult::ok(()),
        _marker: PhantomData,
    };

    let ctx = ParseContext::Trait { ident: trait_ident };
    parse_inner(&mut this, attrs, ctx, arenas)?;

    this.errors.take()?;

    Ok(this)
}

/// Parses an individual attribute
fn parse_inner<'a>(
    this: &mut StructuralAliasOptions<'a>,
    attrs: &[Attribute],
    pctx: ParseContext<'_>,
    arenas: &'a Arenas,
) -> Result<(), syn::Error> {
    for attr in attrs {
        match attr.parse_meta() {
            Ok(Meta::List(list)) => {
                parse_attr_list(this, pctx, list, arenas).combine_into_err(&mut this.errors);
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
    this: &mut StructuralAliasOptions<'a>,
    pctx: ParseContext<'_>,
    list: MetaList,
    arenas: &'a Arenas,
) -> Result<(), syn::Error> {
    if list.path.is_ident("struc") {
        with_nested_meta("struc", list.nested, |attr| {
            parse_sabi_attr(this, pctx, attr, arenas).combine_into_err(&mut this.errors);
            Ok(())
        })?;
    }
    Ok(())
}

/// Parses the contents of a `#[sabi( .. )]` attribute.
fn parse_sabi_attr<'a>(
    this: &mut StructuralAliasOptions<'a>,
    pctx: ParseContext<'_>,
    attr: Meta,
    arenas: &'a Arenas,
) -> Result<(), syn::Error> {
    fn make_err(tokens: &dyn ToTokens) -> syn::Error {
        spanned_err!(tokens, "unrecognized attribute")
    }

    match (pctx, attr) {
        (ParseContext::Trait { ident }, Meta::Path(ref path)) => {
            if path.is_ident("debug_print") {
                this.debug_print = true;
            } else if path.is_ident("no_docs") {
                this.generate_docs = false;
            } else if path.is_ident("exhaustive_enum") {
                this.enum_exhaustiveness = Exhaustiveness::Exhaustive;
            } else if path.is_ident("and_exhaustive_enum") {
                let name = NEIdent::Exhaustive.apply(ident, arenas);
                this.enum_exhaustiveness = Exhaustiveness::AndExhaustive { name };
            } else {
                return Err(make_err(&path));
            }
        }
        (ParseContext::Trait { ident }, Meta::List(MetaList { path, nested, .. })) => {
            if path.is_ident("and_exhaustive_enum") {
                let name = parse_neident(path.span(), nested)?.apply(ident, arenas);
                this.enum_exhaustiveness = Exhaustiveness::AndExhaustive { name };
            } else {
                return Err(make_err(&path));
            }
        }
        (_, x) => return Err(make_err(&x)),
    }
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////

enum NEIdent {
    Exhaustive,
    Suffix(Ident),
    Ident(Ident),
}

impl NEIdent {
    fn apply<'a>(self, trait_ident: &Ident, arenas: &'a Arenas) -> &'a Ident {
        let (new_ident, span) = match self {
            NEIdent::Exhaustive => (format!("{}_Exhaustive", trait_ident), None),
            NEIdent::Suffix(suff) => (format!("{}{}", trait_ident, suff), Some(suff.span())),
            NEIdent::Ident(ident) => return arenas.alloc(ident),
        };

        let trait_ident_span = trait_ident.span();
        let ident = Ident::new(&new_ident, span.unwrap_or(trait_ident_span));
        arenas.alloc(ident)
    }
}

fn parse_neident(
    _path_span: Span,
    list: Punctuated<NestedMeta, syn::Token![,]>,
) -> Result<NEIdent, syn::Error> {
    let mut ret_ident = NEIdent::Exhaustive;

    fn make_err(tokens: &dyn ToTokens) -> syn::Error {
        spanned_err!(
            tokens,
            "unexpected `#[struc(and_exhaustive_enum())]` subattribute"
        )
    }

    with_nested_meta("and_exhaustive_enum", list, |attr| {
        match attr {
            Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(lit),
                ..
            }) => {
                if path.is_ident("suffix") {
                    ret_ident = NEIdent::Suffix(lit.parse()?);
                } else if path.is_ident("name") {
                    ret_ident = NEIdent::Ident(lit.parse()?);
                } else {
                    return Err(make_err(&path));
                }
            }
            _ => return Err(make_err(&attr)),
        }
        Ok(())
    })?;

    Ok(ret_ident)
}
