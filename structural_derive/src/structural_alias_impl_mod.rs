use crate::{
    arenas::Arenas, field_access::Access, ident_or_index::IdentOrIndexRef,
    tokenizers::NamedModuleAndTokens, tokenizers::NamesModuleIndex,
};

use as_derive_utils::gen_params_in::{GenParamsIn, InWhat};

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, Generics, Ident, Token, Visibility,
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

        let datatype = StructuralDataType::parse(&mut names_mod, arenas, &content)?;

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
            datatype,
        })
    }
}

impl<'a> StructuralDataType<'a> {
    fn parse(
        names_mod: &mut NamedModuleAndTokens,
        arenas: &'a Arenas,
        input: ParseStream,
    ) -> Result<Self, syn::Error> {
        let mut variants = Vec::new();
        let mut fields = Vec::new();
        loop {
            if input.is_empty() {
                break;
            }

            if input.peek(syn::Ident) && input.peek(token::Brace) {
                let ident = arenas.alloc(input.parse::<Ident>()?);
                let ident_index = names_mod.push_path(ident.into());

                let content;
                let _ = syn::braced!(content in input);
                input.parse::<Token![,]>()?;

                variants.push(StructuralVariant::parse(
                    names_mod,
                    ident,
                    ident_index,
                    arenas,
                    &content,
                )?);
            } else {
                fields.push(StructuralField::parse(names_mod, None, arenas, input)?);
            }
        }
        Ok(Self { variants, fields })
    }
}

impl<'a> StructuralVariant<'a> {
    fn parse(
        names_mod: &mut NamedModuleAndTokens,
        name: &'a Ident,
        alias_index: NamesModuleIndex,
        arenas: &'a Arenas,
        input: ParseStream,
    ) -> Result<Self, syn::Error> {
        let mut fields = Vec::<StructuralField<'a>>::new();
        while !input.is_empty() {
            fields.push(StructuralField::parse(
                names_mod,
                Some(name),
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
        arenas: &'a Arenas,
        input: ParseStream,
    ) -> Result<Self, syn::Error> {
        let access = input.parse::<Access>()?;
        let ident = IdentOrIndexRef::parse(arenas, input)?;
        let alias_index = match enum_variant {
            Some(variant) => names_mod.push_variant_field(variant, ident),
            None => names_mod.push_path(ident),
        };
        let _: Token![:] = input.parse()?;
        let ty = FieldType::parse(arenas, input)?;
        input.parse::<Token![,]>()?;

        Ok(Self {
            access,
            ident,
            alias_index,
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
    syn::parse_str(saf).and_then(macro_impl)
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
                "A trait alias generated by `structural::structural_alias`,\
                 for the following traits:\n"
            ),
            &saf.vis,
            &saf.trait_token,
            &saf.ident,
            &saf.generics,
            &saf.supertraits,
            &saf.names_mod,
            &saf.datatype,
        )?
        .piped(|x| out.append_all(x));
    }
    // panic!("{}", out);
    Ok(out)
}

/// This allows both `structural_alias` and `#[derive(Structural)]` to generate
/// the trait alias and its impl.
pub(crate) fn for_delegation<'a, A>(
    span: Span,
    attrs: A,
    mut docs: String,
    vis: &syn::Visibility,
    trait_token: &Token!(trait),
    ident: &Ident,
    generics: &syn::Generics,
    supertraits: &Punctuated<syn::TypeParamBound, token::Add>,
    names_mod: &NamedModuleAndTokens,
    datatype: &StructuralDataType<'_>,
) -> Result<TokenStream2, syn::Error>
where
    A: IntoIterator,
    A::Item: ToTokens,
{
    use self::FieldType as FT;

    let attrs = attrs.into_iter();

    let names_mod_path = &names_mod.names_module;

    let mut field_bounds = TokenStream2::new();
    let mut assoc_ty_bounds = TokenStream2::new();

    let mut process_field = |field: &StructuralField<'_>| {
        let alias_name = names_mod.alias_name(field.alias_index);
        if let FT::Impl(bounds) = &field.ty {
            quote!(
                ::structural::GetFieldType<Self,#names_mod_path::#alias_name>:
                    #bounds,
            )
            .to_tokens(&mut assoc_ty_bounds);
        }

        let trait_ = field.access;
        let assoc_ty = match &field.ty {
            FT::Ty(ty) => quote!(Ty=#ty),
            FT::Impl(bounds) => quote!(Ty:#bounds),
        };

        quote!(
            structural::#trait_<
                #names_mod_path::#alias_name,
                Err=structural::pmr::NonOptField,
                #assoc_ty
            >+
        )
        .to_tokens(&mut field_bounds);
    };

    for variant in &datatype.variants {
        for field in &variant.fields {
            process_field(field);
        }
    }

    for field in datatype.fields.iter() {
        process_field(field);
    }

    use std::fmt::Write;

    let _ = writeln!(docs,);

    for field in datatype.fields.iter() {
        let (the_trait, access_desc) = match field.access {
            Access::Shared => ("GetFieldImpl", "shared"),
            Access::Mutable => ("GetFieldMutImpl", "shared and mutable"),
            Access::Value => ("IntoFieldImpl", "shared, and by value"),
            Access::MutValue => ("IntoFieldMut", "shared,mutable and by value"),
        };
        let assoc_ty = match field.ty {
            FT::Ty(ty) => format!("Ty= {}", ty.to_token_stream()),
            FT::Impl(bounds) => format!("Ty: {}", bounds.to_token_stream()),
        };
        let field_ty = match field.ty {
            FT::Ty(ty) => format!("{}", ty.to_token_stream()),
            FT::Impl(bounds) => format!("impl {}", bounds.to_token_stream()),
        };
        let _ = writeln!(
            docs,
            "`{0}<FP!( {1} ),{2}>`\n<br>\
             The &nbsp; `{1}: {3}` &nbsp; field,with {4} access.
            \n",
            the_trait, field.ident, assoc_ty, field_ty, access_desc,
        );
    }

    if !supertraits.is_empty() {
        for supertrait in supertraits {
            let _ = writeln!(docs, "`{}`", supertrait.to_token_stream(),);
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

        #[doc=#docs]
        #(#attrs)*
        #vis
        #trait_token #ident #generics :
            #( #supertraits_a+ )*
            #field_bounds
        where
            #(#where_preds_a,)*
            #assoc_ty_bounds
        {}


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
