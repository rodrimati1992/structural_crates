use crate::field_paths::{FieldPaths, NestedFieldPath};

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use quote::quote_spanned;

use syn::{
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token,
};

pub(crate) fn impl_(parsed: NameAliases) -> Result<TokenStream2, syn::Error> {
    // This uses the full path to each character to allow aliases
    // with the same name as the characters themselves.
    let mut doc_fp_inner = String::new();

    parsed
        .aliases
        .iter()
        .map(move |alias| {
            doc_fp_inner.clear();
            alias.value.write_fp_inside(&mut doc_fp_inner);

            let alias_name = &alias.name;
            let field_name = alias.value.type_tokens();
            let value = alias.value.inferred_expression_tokens();

            Ok(quote_spanned!(alias_name.span()=>
                #[allow(non_camel_case_types,dead_code)]
                #[doc="An alias for `structural::FP!("]
                #[doc=#doc_fp_inner]
                #[doc=")`"]
                pub type #alias_name=#field_name;

                #[allow(non_upper_case_globals,dead_code)]
                #[doc="An alias for `structural::fp!("]
                #[doc=#doc_fp_inner]
                #[doc=")`"]
                pub const #alias_name:#alias_name=#value;
            ))
        })
        .collect()
}

#[derive(Debug)]
pub(crate) struct NameAliases {
    aliases: Punctuated<NameAlias, Token![,]>,
}

#[derive(Debug)]
pub(crate) struct NameAlias {
    name: Ident,
    value: FieldPaths,
}

impl Parse for NameAliases {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        Ok(NameAliases {
            aliases: input.parse_terminated(Parse::parse)?,
        })
    }
}

impl Parse for NameAlias {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let name = input.parse::<Ident>()?;
        let value = if input.peek(Token!(=)) {
            input.parse::<Token![=]>()?;
            if input.peek(syn::token::Paren) {
                let content;
                let _ = syn::parenthesized!(content in input);
                content.parse::<FieldPaths>()?
            } else {
                input
                    .parse::<NestedFieldPath>()?
                    .piped(FieldPaths::from_path)
            }
        } else {
            FieldPaths::from_ident(name.clone())
        };
        Ok(Self { name, value })
    }
}
