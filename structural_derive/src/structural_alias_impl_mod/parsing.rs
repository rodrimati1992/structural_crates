use crate::{
    arenas::Arenas, field_access::Access, ident_or_index::IdentOrIndexRef,
    ignored_wrapper::Ignored, parse_utils::ParseBufferExt,
};

use super::{
    attribute_parsing, FieldType, IdentType, StructuralAlias, StructuralAliases,
    StructuralDataType, StructuralField, StructuralVariant, TinyStructuralField,
};

use as_derive_utils::{datastructure::StructKind, return_syn_err, utils::type_from_ident};

#[allow(unused_imports)]
use core_extensions::{matches, SelfOps};

use syn::{
    parse::{discouraged::Speculative, ParseStream, Parser},
    punctuated::Punctuated,
    token, Attribute, Generics, Ident, Token, TraitItem, Visibility,
};

use std::collections::HashSet;

////////////////////////////////////////////////////////////////////////////////

impl<'a> StructuralAliases<'a> {
    pub(super) fn parse(arenas: &'a Arenas, input: ParseStream<'_>) -> Result<Self, syn::Error> {
        let mut list = Vec::<StructuralAlias<'_>>::new();
        while !input.is_empty() {
            list.push(StructuralAlias::parse(arenas, input)?);
        }
        Ok(Self { list })
    }
}

impl<'a> StructuralAlias<'a> {
    pub(super) fn parse(arenas: &'a Arenas, input: ParseStream<'_>) -> Result<Self, syn::Error> {
        let mut extra_items = Vec::<TraitItem>::new();
        let mut attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;

        let trait_token: Token![trait] = input.parse()?;

        let ident = arenas.alloc(input.parse::<Ident>()?);

        let options = attribute_parsing::parse_attrs(&attrs, ident, arenas)?;

        attrs.retain(|attr| !attr.path.is_ident("struc"));

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

        let type_params = generics
            .type_params()
            .map(|tp| &tp.ident)
            .collect::<Vec<&Ident>>();

        let datatype = StructuralDataType::parse(&mut extra_items, &type_params, arenas, &content)?;

        let span = trait_token
            .span
            .join(braces.span)
            .unwrap_or(trait_token.span);

        Ok(Self {
            span,
            attrs,
            vis,
            ident,
            generics,
            supertraits,
            extra_items,
            datatype,
            options,
        })
    }
}

impl<'a> StructuralDataType<'a> {
    pub(super) fn parse(
        extra_items: &mut Vec<TraitItem>,
        type_params: &[&syn::Ident],
        arenas: &'a Arenas,
        input: ParseStream<'_>,
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
            let ident = IdentType::parse(type_params, arenas, input)?;
            if let Some(enum_token) = VariantToken::peek_from(input) {
                let variant_kind: StructKind = enum_token.into();
                let mut push_variant = |content: ParseStream<'_>| -> Result<(), syn::Error> {
                    variants.push(StructuralVariant::parse(
                        access,
                        ident,
                        variant_kind,
                        type_params,
                        arenas,
                        content,
                    )?);
                    Ok(())
                };

                let content;
                match enum_token {
                    VariantToken::Brace => {
                        let _ = syn::braced!(content in input);
                        push_variant(&content)?;
                    }
                    VariantToken::Paren => {
                        let _ = syn::parenthesized!(content in input);
                        push_variant(&content)?;
                    }
                    VariantToken::NoToken => {
                        Parser::parse_str(push_variant, "")?;
                    }
                }
                if !input.is_empty() {
                    input.parse::<Token![,]>()?;
                }
            } else {
                fields.push(StructuralField::parse_braced_field(
                    access, ident, arenas, input,
                )?);
            }
        }
        {
            let mut set = HashSet::new();
            for variant in &variants {
                if set.replace(variant.name).is_some() {
                    return_syn_err!(
                        variant.name.span(),
                        "Cannot repeat variant name in the same trait declaration"
                    )
                }
            }
        }
        check_no_repeated_field(&fields)?;
        Ok(Self {
            type_name: None,
            variants,
            fields,
        })
    }
}

impl<'a> StructuralVariant<'a> {
    pub(super) fn parse(
        access: Access,
        name: IdentType<'a>,
        variant_kind: StructKind,
        type_params: &[&syn::Ident],
        arenas: &'a Arenas,
        input: ParseStream<'_>,
    ) -> Result<Self, syn::Error> {
        let mut fields = Vec::<StructuralField<'a>>::new();

        match variant_kind {
            StructKind::Braced => {
                while !input.is_empty() {
                    let nested_access = Access::parse_optional(input)?;
                    let ident = IdentType::parse(type_params, arenas, input)?;
                    fields.push(StructuralField::parse_braced_field(
                        nested_access.unwrap_or(access),
                        ident,
                        arenas,
                        input,
                    )?);
                    check_no_repeated_field(&fields)?;
                }
            }
            StructKind::Tuple => {
                let mut index = 0;
                while !input.is_empty() {
                    let nested_access = Access::parse_optional(input)?;
                    fields.push(StructuralField::parse_tuple_field(
                        nested_access.unwrap_or(access),
                        index,
                        arenas,
                        input,
                    )?);
                    index += 1;
                }
            }
        }
        Ok(Self {
            name,
            pub_vari_rename: None,
            fields,
            is_newtype: false,
            replace_bounds: None,
        })
    }
}

impl<'a> TinyStructuralField<'a> {
    pub(crate) fn parse(
        access: Access,
        type_params: &[&syn::Ident],
        arenas: &'a Arenas,
        input: ParseStream<'_>,
    ) -> Result<Self, syn::Error> {
        let ident = IdentType::parse(type_params, arenas, input)?;
        let _: Token![:] = input.parse()?;
        let ty = FieldType::parse(arenas, input)?;

        Ok(Self { access, ident, ty })
    }
}

impl<'a> StructuralField<'a> {
    pub(super) fn parse_braced_field(
        access: Access,
        ident: IdentType<'a>,
        arenas: &'a Arenas,
        input: ParseStream<'_>,
    ) -> Result<Self, syn::Error> {
        let _: Token![:] = input.parse()?;
        let ty = FieldType::parse(arenas, input)?;

        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }

        Ok(Self {
            access,
            ident,
            pub_field_rename: None,
            ty,
        })
    }

    pub(super) fn parse_tuple_field(
        access: Access,
        index: u32,
        arenas: &'a Arenas,
        input: ParseStream<'_>,
    ) -> Result<Self, syn::Error> {
        let span = input.cursor().span();
        let ty = FieldType::parse(arenas, input)?;
        let ident = IdentOrIndexRef::Index {
            index,
            span: Ignored::new(span),
        }
        .piped(IdentType::Ident);

        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }

        Ok(Self {
            access,
            ident,
            pub_field_rename: None,
            ty,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> IdentType<'a> {
    pub(super) fn parse(
        type_params: &[&syn::Ident],
        arenas: &'a Arenas,
        input: ParseStream<'_>,
    ) -> Result<Self, syn::Error> {
        if let Some(_) = input.peek_parse(Token!(<))? {
            let ret = if input.peek(syn::Ident) && input.peek2(Token!(>)) {
                let ident = input.parse::<syn::Ident>()?;
                if type_params.contains(&&ident) {
                    IdentType::Generic(arenas.alloc(ident))
                } else {
                    let ty = type_from_ident(ident);
                    IdentType::SomeType(arenas.alloc(ty))
                }
            } else {
                let ty = input.parse::<syn::Type>()?;
                IdentType::SomeType(arenas.alloc(ty))
            };
            input.parse::<Token!(>)>()?;
            Ok(ret)
        } else {
            IdentOrIndexRef::parse(arenas, input).map(IdentType::Ident)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
enum VariantToken {
    Brace,
    Paren,
    NoToken,
}

impl VariantToken {
    pub(super) fn peek_from(input: ParseStream<'_>) -> Option<Self> {
        if input.peek(token::Brace) {
            Some(VariantToken::Brace)
        } else if input.peek(token::Paren) {
            Some(VariantToken::Paren)
        } else if input.peek(token::Comma) || input.is_empty() {
            Some(VariantToken::NoToken)
        } else {
            None
        }
    }
}

impl From<VariantToken> for StructKind {
    fn from(v: VariantToken) -> Self {
        match v {
            VariantToken::Brace => StructKind::Braced,
            _ => StructKind::Tuple,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn check_no_repeated_field(fields: &[StructuralField<'_>]) -> Result<(), syn::Error> {
    let mut set = HashSet::with_capacity(fields.len());
    for field in fields {
        if set.replace(field.ident).is_some() {
            return_syn_err!(field.ident.span(), "Cannot redefine field")
        }
    }
    Ok(())
}

impl<'a> FieldType<'a> {
    pub(super) fn parse(arenas: &'a Arenas, input: ParseStream<'_>) -> Result<Self, syn::Error> {
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
