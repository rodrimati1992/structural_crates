use crate::{field_paths::FieldPaths, tokenizers::FullPathForChars};

use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use syn::{
    parse::{self, ParseStream},
    Ident,
};

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

pub(crate) fn low_fp_impl(LowFpParams { paths }: LowFpParams) -> Result<TokenStream2, syn::Error> {
    let const_name = Ident::new("VALUE", proc_macro2::Span::call_site());

    Ok(paths.constant_named(&const_name, FullPathForChars::Yes))
}

pub(crate) struct LowFpParams {
    paths: FieldPaths,
}

impl parse::Parse for LowFpParams {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        input
            .parse::<FieldPaths>()
            .map(|paths| LowFpParams { paths })
    }
}
