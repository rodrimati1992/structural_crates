use crate::{ident_or_index::IdentOrIndex, parse_utils::ParseBufferExt, tokenizers::tstr_tokens};

use as_derive_utils::return_spanned_err;

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned, TokenStreamExt};

use syn::{
    parse::{self, Parse, ParseStream},
    Ident, Token,
};

use std::collections::HashSet;

pub(crate) fn impl_(parsed: StrAliases) -> Result<TokenStream2, syn::Error> {
    let mut doc_fp_inner = String::new();
    let mut doc_fpc_inner = String::new();

    let mut ident_set = HashSet::new();

    let alias_count = parsed.aliases.len();

    let config = parsed.config;

    let mut tokens = TokenStream2::new();

    if config.inner_module {
        tokens.append_all(quote!(
            //! Type aliases for `TStr` (type-level string)
            //! (from the structural crate).
            //!
            //! `TStr` values can be constructed with the NEW associated constant.
            //!
            //! The source code for this module can only be accessed from
            //! the type aliases.<br>
            //! As of writing this documentation,`cargo doc` links
            //! to the inplementation of the `field_path_aliases` macro
            //! instead of where this module is declared.
        ));
    }

    if config.include_count {
        let alias_count_str = tstr_tokens(alias_count.to_string(), Span::call_site());
        tokens.append_all(quote!(
            #[allow(non_camel_case_types,dead_code)]
            /// The amount of strings aliased in this module.
            pub type __TString_Aliases_Count=#alias_count_str;
        ));
    }

    tokens.append_all(
        parsed
            .aliases
            .iter()
            .map(
                move |StrAlias {
                          alias_name,
                          string,
                          string_span,
                      }| {
                    use std::fmt::Write;

                    doc_fp_inner.clear();
                    doc_fpc_inner.clear();

                    let span = alias_name.span();

                    if ident_set.replace(alias_name).is_some() {
                        return_spanned_err! {
                            alias_name,
                            "Cannot have multiple aliases named {}",
                            alias_name,
                        }
                    }

                    let _ = writeln!(
                        doc_fp_inner,
                        "The `structural::TStr` equivalent of {:?}",
                        string
                    );
                    let _ = writeln!(
                        doc_fpc_inner,
                        "An instance of the `structural::TStr` equivalent of {:?}.",
                        string,
                    );

                    let string = tstr_tokens(string, *string_span);

                    Ok(quote_spanned!(span=>
                        #[doc=#doc_fp_inner]
                        #[allow(non_camel_case_types,dead_code)]
                        pub type #alias_name=#string;

                        #[doc=#doc_fpc_inner]
                        #[allow(non_upper_case_globals,dead_code)]
                        pub const #alias_name:#alias_name=#alias_name::NEW;
                    ))
                },
            )
            .collect::<Result<TokenStream2, syn::Error>>()?,
    );

    tokens.append_all(
        parsed
            .modules
            .into_iter()
            .map(move |StrModule { name, aliases }| {
                let span = name.span();
                let tokens = self::impl_(aliases)?;
                Ok(quote_spanned!(span=>
                    pub mod #name {
                        #tokens
                    }
                ))
            })
            .collect::<Result<TokenStream2, syn::Error>>()?,
    );

    Ok(tokens)
}

pub struct StrAliases {
    config: StrAliasCfg,
    aliases: Vec<StrAlias>,
    modules: Vec<StrModule>,
}

#[derive(Copy, Clone, Default)]
pub struct StrAliasCfg {
    inner_module: bool,
    include_count: bool,
}

pub struct StrModule {
    name: Ident,
    aliases: StrAliases,
}

pub struct StrAlias {
    alias_name: Ident,
    string: String,
    string_span: Span,
}

pub(crate) struct TString {
    pub(crate) str: String,
    pub(crate) span: Span,
}

mod keywords {
    syn::custom_keyword!(count);
}

impl Parse for StrAliases {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        Self::parse_(input, StrAliasCfg::default())
    }
}

impl StrAliases {
    fn parse_(input: ParseStream<'_>, mut config: StrAliasCfg) -> parse::Result<Self> {
        while let Some(_) = input.peek_parse(Token!(@))? {
            if input.peek_parse(keywords::count)?.is_some() {
                config.include_count = true;
            }
        }

        let mut aliases = Vec::<StrAlias>::new();
        let mut modules = Vec::<StrModule>::new();

        while !input.is_empty() {
            if input.peek(Token![mod]) {
                let mut config = config;
                config.inner_module = true;
                modules.push(StrModule::parse_(input, config)?);
            } else {
                aliases.push(StrAlias::parse(input)?);
            }
        }

        Ok(Self {
            config,
            aliases,
            modules,
        })
    }
}

impl StrModule {
    fn parse_(input: ParseStream<'_>, config: StrAliasCfg) -> parse::Result<Self> {
        let _: Token!(mod) = input.parse()?;
        let name = input.parse::<Ident>()?;

        let aliases = {
            let content;
            let _ = syn::braced!(content in input);
            StrAliases::parse_(&content, config)?
        };

        Ok(Self { name, aliases })
    }
}

impl Parse for StrAlias {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        let alias_name = input.parse::<Ident>()?;
        let (string, string_span) = if let Some(_) = input.peek_parse(Token!(=))? {
            let tstr = input.parse::<TString>()?;
            (tstr.str, tstr.span)
        } else {
            (alias_name.to_string(), alias_name.span())
        };
        if !input.is_empty() {
            let _: Token!(,) = input.parse()?;
        }
        Ok(Self {
            alias_name,
            string,
            string_span,
        })
    }
}

impl Parse for TString {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        let (str, span) = input.parse::<IdentOrIndex>()?.string_and_span();
        Ok(TString { str, span })
    }
}
