use crate::{
    arenas::Arenas,
    field_access::{Access, IsOptional},
    ident_or_index::IdentOrIndexRef,
    tokenizers::{NamedModuleAndTokens, NamesModuleIndex},
};

use as_derive_utils::gen_params_in::{GenParamsIn, InWhat};
use as_derive_utils::return_spanned_err;

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, Generics, Ident, Token, TraitItem, Visibility,
};

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests;

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct StructuralAliasesHack {
    pub(crate) tokens: TokenStream2,
}

pub(crate) struct StructuralAliases<'a> {
    pub(crate) list: Vec<StructuralAlias<'a>>,
}

pub(crate) struct StructuralAlias<'a> {
    pub(crate) names_mod: NamedModuleAndTokens,
    pub(crate) span: Span,
    pub(crate) ident: &'a syn::Ident,
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) vis: syn::Visibility,
    pub(crate) trait_token: Token!(trait),
    pub(crate) generics: syn::Generics,
    pub(crate) supertraits: Punctuated<syn::TypeParamBound, token::Add>,
    pub(crate) extra_items: Vec<TraitItem>,
    pub(crate) datatype: StructuralDataType<'a>,
}

#[derive(Debug)]
pub struct StructuralDataType<'a> {
    pub(crate) variants: Vec<StructuralVariant<'a>>,
    pub(crate) fields: Vec<StructuralField<'a>>,
}

#[derive(Debug, Clone)]
pub struct StructuralVariant<'a> {
    pub(crate) name: &'a Ident,
    pub(crate) alias_index: NamesModuleIndex,
    pub(crate) fields: Vec<StructuralField<'a>>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct StructuralField<'a> {
    pub(crate) access: Access,
    pub(crate) ident: IdentOrIndexRef<'a>,
    pub(crate) alias_index: NamesModuleIndex,
    pub(crate) optionality: IsOptional,
    pub(crate) ty: FieldType<'a>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum FieldType<'a> {
    Ty(&'a syn::Type),
    Impl(&'a TypeParamBounds),
}

pub(crate) type TypeParamBounds = Punctuated<syn::TypeParamBound, syn::token::Add>;

// A hack to allow borrowing from the arena inside a parser
impl<'a> Parse for StructuralAliasesHack {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let arenas = crate::arenas::Arenas::default();

        let sa = StructuralAliases::parse(&arenas, input)?;

        Ok(StructuralAliasesHack {
            tokens: self::macro_impl(sa)?,
        })
    }
}

impl<'a> StructuralAliases<'a> {
    fn parse(arenas: &'a Arenas, input: ParseStream) -> Result<Self, syn::Error> {
        let mut list = Vec::<StructuralAlias>::new();
        while !input.is_empty() {
            list.push(StructuralAlias::parse(arenas, input)?);
        }
        Ok(Self { list })
    }
}

impl<'a> StructuralAlias<'a> {
    fn parse(arenas: &'a Arenas, input: ParseStream) -> Result<Self, syn::Error> {
        let mut extra_items = Vec::<TraitItem>::new();
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;

        let trait_token: Token![trait] = input.parse()?;

        let ident = arenas.alloc(input.parse::<Ident>()?);

        let mut names_mod = NamedModuleAndTokens::new(ident);
        let mut generics: Generics = input.parse()?;
        let colon_token: Option<Token![:]> = input.parse()?;

        let mut supertraits = Punctuated::new();
        if colon_token.is_some() {
            loop {
                supertraits.push_value(input.parse()?);
                if input.peek(Token![where]) || input.peek(token::Brace) {
                    break;
                }
                supertraits.push_punct(input.parse()?);
                if input.peek(Token![where]) || input.peek(token::Brace) {
                    break;
                }
            }
        }

        generics.where_clause = input.parse()?;

        // let equal:Token![=]= input.parse()?;

        let content;
        let braces = syn::braced!(content in input);

        let datatype =
            StructuralDataType::parse(&mut names_mod, &mut extra_items, arenas, &content)?;

        let span = trait_token
            .span
            .join(braces.span)
            .unwrap_or(trait_token.span);

        Ok(Self {
            names_mod,
            span,
            attrs,
            vis,
            trait_token,
            ident,
            generics,
            supertraits,
            extra_items,
            datatype,
        })
    }
}

impl<'a> StructuralDataType<'a> {
    fn parse(
        names_mod: &mut NamedModuleAndTokens,
        extra_items: &mut Vec<TraitItem>,
        arenas: &'a Arenas,
        input: ParseStream,
    ) -> Result<Self, syn::Error> {
        let mut variants = Vec::new();
        let mut fields = Vec::new();
        loop {
            if input.is_empty() {
                break;
            }

            let forked = input.fork();
            if let Ok(item) = forked.parse::<TraitItem>() {
                input.advance_to(&forked);
                extra_items.push(item);
                continue;
            }

            let access = input.parse::<Access>()?;
            if input.peek(syn::Ident) && input.peek2(token::Brace) {
                let ident = arenas.alloc(input.parse::<Ident>()?);
                let ident_index = names_mod.push_path(ident.into());

                let content;
                let _ = syn::braced!(content in input);
                input.parse::<Token![,]>()?;

                variants.push(StructuralVariant::parse(
                    names_mod,
                    access,
                    ident,
                    ident_index,
                    arenas,
                    &content,
                )?);
            } else {
                fields.push(StructuralField::parse(
                    names_mod, None, access, arenas, input,
                )?);
            }
        }
        Ok(Self { variants, fields })
    }
}

impl<'a> StructuralVariant<'a> {
    fn parse(
        names_mod: &mut NamedModuleAndTokens,
        access: Access,
        name: &'a Ident,
        alias_index: NamesModuleIndex,
        arenas: &'a Arenas,
        input: ParseStream,
    ) -> Result<Self, syn::Error> {
        let mut fields = Vec::<StructuralField<'a>>::new();
        while !input.is_empty() {
            let nested_access = Access::parse_optional(input)?;

            fields.push(StructuralField::parse(
                names_mod,
                Some(name),
                nested_access.unwrap_or(access),
                arenas,
                input,
            )?);
        }
        Ok(Self {
            name,
            alias_index,
            fields,
        })
    }
}

impl<'a> StructuralField<'a> {
    fn parse(
        names_mod: &mut NamedModuleAndTokens,
        enum_variant: Option<&'a Ident>,
        access: Access,
        arenas: &'a Arenas,
        input: ParseStream,
    ) -> Result<Self, syn::Error> {
        let ident = IdentOrIndexRef::parse(arenas, input)?;
        let alias_index = match enum_variant {
            Some(variant) => names_mod.push_variant_field(variant, ident),
            None => names_mod.push_path(ident),
        };
        let _: Token![:] = input.parse()?;
        let optionality = input.parse::<IsOptional>()?;
        let ty = FieldType::parse(arenas, input)?;
        input.parse::<Token![,]>()?;

        Ok(Self {
            access,
            ident,
            alias_index,
            optionality: match enum_variant {
                Some(_) => IsOptional::Yes,
                None => optionality,
            },
            ty,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> FieldType<'a> {
    fn parse(arenas: &'a Arenas, input: ParseStream) -> Result<Self, syn::Error> {
        const ASSOC_TY_BOUNDS: bool = cfg!(feature = "impl_fields");

        use syn::Type;

        match input.parse::<syn::Type>()? {
            Type::ImplTrait(x) => {
                if ASSOC_TY_BOUNDS {
                    Ok(FieldType::Impl(arenas.alloc(x.bounds)))
                } else {
                    use syn::spanned::Spanned;
                    Err(syn::Error::new(
                        x.span(),
                        "\
                         Cannot use an `impl Trait` field without enabling the \
                         \"nightly_impl_fields\" or \"impl_fields\" feature.\
                         ",
                    ))
                }
            }
            x => Ok(FieldType::Ty(arenas.alloc(x))),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub(crate) fn derive_from_str(saf: &str) -> Result<TokenStream2, syn::Error> {
    syn::parse_str::<StructuralAliasesHack>(saf).map(|x| x.tokens)
}

pub(crate) fn macro_impl<'a>(aliases: StructuralAliases<'a>) -> Result<TokenStream2, syn::Error> {
    let list = aliases.list;

    if list.is_empty() {
        return Ok(quote!());
    }

    let mut out = TokenStream2::new();
    for saf in list {
        for_delegation(
            saf.span,
            &saf.attrs,
            format!(
                "A trait which aliases `structural` accessor traits,\
                 generated by the `structural_alias` macro.\n\n"
            ),
            &saf.vis,
            &saf.trait_token,
            &saf.ident,
            &saf.generics,
            &saf.supertraits,
            &saf.names_mod,
            &saf.extra_items,
            &saf.datatype,
        )?
        .piped(|x| out.append_all(x));
    }
    // panic!("{}", out);
    Ok(out)
}

fn write_field_docs(
    docs: &mut String,
    left_padding: &str,
    variant: Option<&Ident>,
    field: &StructuralField<'_>,
) -> std::fmt::Result {
    use std::fmt::Write;

    use self::FieldType as FT;

    let the_trait = field.access.and_optionality(field.optionality).trait_name();

    let ident = match variant {
        Some(v_ident) => format!("::{}.{}", v_ident, field.ident),
        None => field.ident.to_string(),
    };

    let access_desc = match field.access {
        Access::Shared => "a shared reference accessor",
        Access::Mutable => "shared and mutable reference accessors",
        Access::Value => "shared reference, and by value accessors",
        Access::MutValue => "shared reference,mutable reference, and by value accessors",
    };
    let assoc_ty = match field.ty {
        FT::Ty(ty) => format!("Ty= {}", ty.to_token_stream()),
        FT::Impl(bounds) => format!("Ty: {}", bounds.to_token_stream()),
    };
    let field_ty = match field.ty {
        FT::Ty(ty) => format!("{}", ty.to_token_stream()),
        FT::Impl(bounds) => format!("impl {}", bounds.to_token_stream()),
    };
    writeln!(
        docs,
        "{LP}Bound:`{0}<FP!( {1} ),{2}>`\n<br>",
        the_trait,
        ident,
        assoc_ty,
        LP = left_padding,
    )?;
    writeln!(
        docs,
        "{LP}The &nbsp; `{0}: {1}` &nbsp; ",
        field.ident,
        field_ty,
        LP = left_padding,
    )?;
    match (variant, field.optionality) {
        (Some(vari_name), _) => {
            write!(docs, "field in the `{}` variant", vari_name)?;
        }
        (None, IsOptional::Yes) => docs.push_str("optional field"),
        (None, IsOptional::No) => docs.push_str("field"),
    }
    docs.push_str(", with ");
    docs.push_str(access_desc);
    docs.push_str("\n\n");
    Ok(())
}

fn process_field(
    field: &StructuralField<'_>,
    names_mod: &NamedModuleAndTokens,
    field_bounds: &mut TokenStream2,
    assoc_ty_bounds: &mut TokenStream2,
) {
    use self::FieldType as FT;

    let alias_name = names_mod.alias_name(field.alias_index);
    let names_mod_path = &names_mod.names_module;
    if let FT::Impl(bounds) = &field.ty {
        quote!(
            ::structural::GetFieldType<Self,#names_mod_path::#alias_name>:
                #bounds,
        )
        .to_tokens(assoc_ty_bounds);
    }

    let trait_ = field.access.and_optionality(field.optionality);

    let assoc_ty = match &field.ty {
        FT::Ty(ty) => quote!(Ty=#ty),
        FT::Impl(bounds) => quote!(Ty:#bounds),
    };

    quote!(
        structural::#trait_<
            #names_mod_path::#alias_name,
            #assoc_ty
        >+
    )
    .to_tokens(field_bounds);
}

/// This allows both `structural_alias` and `#[derive(Structural)]` to generate
/// the trait alias and its impl.
pub(crate) fn for_delegation<'a, A, I>(
    span: Span,
    attrs: A,
    mut docs: String,
    vis: &syn::Visibility,
    trait_token: &Token!(trait),
    ident: &Ident,
    generics: &syn::Generics,
    supertraits: &Punctuated<syn::TypeParamBound, token::Add>,
    names_mod: &NamedModuleAndTokens,
    trait_items: I,
    datatype: &StructuralDataType<'_>,
) -> Result<TokenStream2, syn::Error>
where
    A: IntoIterator,
    A::Item: ToTokens,
    I: IntoIterator<Item = &'a TraitItem>,
{
    use std::fmt::Write;

    let attrs = attrs.into_iter();

    let names_mod_path = &names_mod.names_module;

    let mut field_bounds = TokenStream2::new();
    let mut assoc_ty_bounds = TokenStream2::new();

    const SPACES_X8: &'static str = "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;";

    if !datatype.variants.is_empty() {
        docs.push_str("### Variants\n\n");
        docs.push_str(
            "This trait aliases the `IsVariant<FP!( Variant )>` trait for \
             each of the variants below.\n\n\
             The accessor for every enum variant field is optional,\
             because the enum might not be that variant.\n\n",
        );
    }

    let mut out_trait_items = TokenStream2::new();
    for item in trait_items {
        let (is_defaulted, item_name) = match item {
            TraitItem::Const(x) => {
                x.to_tokens(&mut out_trait_items);

                (x.default.is_some(), "associated constant")
            }
            TraitItem::Method(x) => {
                x.to_tokens(&mut out_trait_items);

                (x.default.is_some(), "associated function")
            }
            _ => return_spanned_err!(
                item,
                "Only defaulted associated constant/function are supported.",
            ),
        };
        if !is_defaulted {
            return_spanned_err!(item, "Expected this {} to be defaulted", item_name)
        }
    }

    for variant in &datatype.variants {
        let variant_name_type = names_mod.alias_name(variant.alias_index);

        quote!(
            structural::pmr::IsVariant<#names_mod_path::#variant_name_type>+
        )
        .to_tokens(&mut field_bounds);

        let _ = write!(docs, "Variant `{}` {{", variant.name,);
        docs.push_str(if variant.fields.is_empty() {
            " "
        } else {
            "<br>"
        });
        for field in &variant.fields {
            process_field(field, names_mod, &mut field_bounds, &mut assoc_ty_bounds);
            let _ = write_field_docs(&mut docs, SPACES_X8, Some(variant.name), field);
        }
        let _ = writeln!(docs, "}}\n");
    }

    if !datatype.fields.is_empty() {
        docs.push_str("### Fields\n\n");
    }
    for field in datatype.fields.iter() {
        process_field(field, names_mod, &mut field_bounds, &mut assoc_ty_bounds);
        let _ = write_field_docs(&mut docs, "", None, field);
    }

    if !supertraits.is_empty() {
        docs.push_str("### supertraits\n\n");

        for supertrait in supertraits {
            let _ = writeln!(docs, "- `{}`", supertrait.to_token_stream(),);
        }
    }

    let supertraits_a = supertraits.into_iter();
    let supertraits_b = supertraits.into_iter();

    let impl_generics =
        GenParamsIn::with_after_types(generics, InWhat::ImplHeader, quote!(__This: ?Sized,));

    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let empty_preds = Punctuated::new();
    let where_preds = where_clause
        .as_ref()
        .map_or(&empty_preds, |x| &x.predicates);
    let where_preds_a = where_preds.into_iter();
    let where_preds_b = where_preds.into_iter();

    Ok(quote_spanned!(span=>
        #names_mod

        #(#attrs)*
        #[doc=#docs]
        #vis
        #trait_token #ident #generics :
            #( #supertraits_a+ )*
            #field_bounds
        where
            #(#where_preds_a,)*
            #assoc_ty_bounds
        {
            #out_trait_items
        }


        impl<#impl_generics> #ident #ty_generics
        for __This
        where
            __This:
                #( #supertraits_b+ )*
                #field_bounds,
            #assoc_ty_bounds
            #(#where_preds_b,)*
        {}
    ))
}
