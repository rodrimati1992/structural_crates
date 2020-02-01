use crate::{
    field_paths::FieldPaths, ident_or_index::IdentOrIndex, parse_utils::ParseBufferExt,
    tokenizers::FullPathForChars,
};

use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use quote::quote;

use syn::{
    parse::{self, ParseStream},
    Ident,
};

/// This is the implementation of the FP macro when
/// the input isn't space separated characters.
#[allow(non_snake_case)]
pub(crate) fn FP_impl(parsed: FieldPaths) -> Result<TokenStream2, syn::Error> {
    parsed.type_tokens(FullPathForChars::Yes).piped(Ok)
}

#[cfg(test)]
#[allow(non_snake_case)]
pub(crate) fn FP_from_str(input: &str) -> Result<TokenStream2, syn::Error> {
    syn::parse_str(input).and_then(FP_impl)
}

#[test]
#[allow(non_snake_case)]
fn test_FP_macro() {
    use as_derive_utils::test_framework::Tests;

    Tests::load("field_paths").run_test(FP_from_str);
}

///////////////////////////////////////////////////////////////////////////////

pub(crate) fn low_fp_impl(params: LowFpParams) -> Result<TokenStream2, syn::Error> {
    let const_name = Ident::new("VALUE", proc_macro2::Span::call_site());
    let constant = match params {
        LowFpParams::Ident(ident) => {
            FieldPaths::constant_from_single(&const_name, &ident, FullPathForChars::StructPmr)
        }
        LowFpParams::FieldPaths(fps) => {
            fps.constant_named(&const_name, FullPathForChars::StructPmr)
        }
    };

    Ok(quote!(
        #constant
    ))
}

mod low_fp_kw {
    use syn::custom_keyword;
    custom_keyword! {normal}
    custom_keyword! {ident}
}

pub(crate) enum LowFpParams {
    Ident(IdentOrIndex),
    FieldPaths(FieldPaths),
}

impl parse::Parse for LowFpParams {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let s_tokens;
        let _ = syn::bracketed!(s_tokens in input);
        if s_tokens.peek_parse(low_fp_kw::ident)?.is_some() {
            input.parse::<IdentOrIndex>().map(LowFpParams::Ident)
        } else {
            s_tokens.peek_parse(low_fp_kw::normal)?;
            input.parse::<FieldPaths>().map(LowFpParams::FieldPaths)
        }
    }
}
