use crate::{
    parse_utils::ParseBufferExt,
    tokenizers::{tident_tokens, FullPathForChars},
};

use as_derive_utils::return_spanned_err;

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, quote_spanned, TokenStreamExt};

use syn::{
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitStr, Token,
};

use std::collections::HashSet;

pub(crate) fn impl_(parsed: StrAliases) -> Result<TokenStream2, syn::Error> {
    let mut doc_fp_inner = String::new();

    let mut ident_set = HashSet::new();

    let alias_count = parsed.aliases.len();

    let include_count = parsed.include_count;

    let mut tokens = TokenStream2::new();

    if include_count {
        let alias_count_str = tident_tokens(alias_count.to_string(), FullPathForChars::Yes);
        tokens.append_all(quote!(
            /// The amount of strings aliased in this module.
            pub type __TString_Aliases_Count=#alias_count_str;
        ));
    }

    tokens.append_all(
        parsed
            .aliases
            .iter()
            .map(move |StrAlias { alias_name, string }| {
                use std::fmt::Write;

                doc_fp_inner.clear();

                let span = alias_name.span();

                if ident_set.replace(alias_name).is_some() {
                    return_spanned_err! {
                        alias_name,
                        "Cannot have multiple aliases named {}",
                        alias_name,
                    }
                }

                let _ = writeln!(doc_fp_inner, "The type-level equivalent of {:?}", string,);

                let string = tident_tokens(string, FullPathForChars::Yes);

                Ok(quote_spanned!(span=>
                    #[doc=#doc_fp_inner]
                    #[allow(non_camel_case_types,dead_code)]
                    pub type #alias_name=#string;
                ))
            })
            .collect::<Result<TokenStream2, syn::Error>>()?,
    );

    Ok(tokens)
}

pub struct StrAliases {
    include_count: bool,
    aliases: Punctuated<StrAlias, Token![,]>,
}

pub struct StrAlias {
    alias_name: Ident,
    string: String,
}

mod keywords {
    syn::custom_keyword!(count);
}

impl Parse for StrAliases {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let include_count = {
            input.peek_parse(Token!(@))?.is_some() && input.peek_parse(keywords::count)?.is_some()
        };
        Ok(StrAliases {
            include_count,
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
