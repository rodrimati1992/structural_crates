use crate::{
    parse_utils::ParseBufferExt,
    tokenizers::{tident_tokens, FullPathForChars},
};

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use quote::quote_spanned;

use syn::{
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitStr, Token,
};

pub(crate) fn impl_(parsed: StrAliases) -> Result<TokenStream2, syn::Error> {
    let mut doc_fp_inner = String::new();

    parsed
        .aliases
        .iter()
        .map(move |StrAlias { alias_name, string }| {
            use std::fmt::Write;

            doc_fp_inner.clear();

            let _ = writeln!(doc_fp_inner, "The type-level equivalent of {:?}", string,);

            let string = tident_tokens(string, FullPathForChars::Yes);

            let span = alias_name.span();

            Ok(quote_spanned!(span=>
                #[doc=#doc_fp_inner]
                #[allow(non_camel_case_types,dead_code)]
                pub type #alias_name=#string;
            ))
        })
        .collect()
}

pub struct StrAliases {
    aliases: Punctuated<StrAlias, Token![,]>,
}

pub struct StrAlias {
    alias_name: Ident,
    string: String,
}

impl Parse for StrAliases {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        Ok(StrAliases {
            aliases: input.parse_terminated(Parse::parse)?,
        })
    }
}

impl Parse for StrAlias {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let alias_name = input.parse::<Ident>()?;
        let string = if let Some(_) = input.peek_parse(Token!(=))? {
            input.parse::<LitStr>()?.value()
        } else {
            alias_name.to_string()
        };
        Ok(Self { alias_name, string })
    }
}
