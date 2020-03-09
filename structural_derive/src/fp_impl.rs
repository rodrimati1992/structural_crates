use crate::{
    field_paths::{parse_field, FieldPaths},
    ident_or_index::IdentOrIndex,
};

use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use syn::{
    parse::{self, ParseStream},
    Ident,
};

#[allow(non_snake_case)]
pub(crate) fn FP_impl(parsed: FieldPaths) -> Result<TokenStream2, syn::Error> {
    parsed.type_tokens().piped(Ok)
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

    Ok(paths.constant_named(&const_name))
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

///////////////////////////////////////////////////////////////////////////////

#[allow(non_snake_case)]
pub(crate) fn FP_literal_impl(params: FpLitParams) -> Result<TokenStream2, syn::Error> {
    let FpLitParams { first, second } = params;

    let first = first.tstr_tokens();
    let ret = match second {
        Some(second) => {
            let second = second.tstr_tokens();
            quote::quote!( ::structural::pmr::FieldPath<(#first,#second)> )
        }
        None => first,
    };
    Ok(ret)
}

pub(crate) struct FpLitParams {
    first: IdentOrIndex,
    second: Option<IdentOrIndex>,
}

impl parse::Parse for FpLitParams {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let (first, second) = parse_field(input)?;
        Ok(FpLitParams { first, second })
    }
}
