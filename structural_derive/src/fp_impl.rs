use crate::{
    field_paths::{parse_field, FieldPaths},
    ident_or_index::IdentOrIndex,
};

use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use syn::parse::{self, ParseStream};

#[allow(non_snake_case)]
pub(crate) fn FP_impl(parsed: FieldPaths) -> Result<TokenStream2, syn::Error> {
    parsed.type_tokens().piped(Ok)
}

#[test]
#[allow(non_snake_case)]
fn test_FP_macro() {
    use as_derive_utils::test_framework::Tests;

    Tests::load("field_paths").run_test(|input: &str| syn::parse_str(input).and_then(FP_impl));
}

///////////////////////////////////////////////////////////////////////////////

/// This is what the `fp` and `FP` macros call when literals are passed in.
#[allow(non_snake_case)]
pub(crate) fn FP_literal_impl(params: FpLitParams) -> Result<TokenStream2, syn::Error> {
    let FpLitParams { first, second } = params;

    let first = first.tstr_tokens();
    let ret = match second {
        Some(second) => {
            let second = second.tstr_tokens();
            quote::quote!( ::structural::pmr::NestedFieldPath<(#first,#second)> )
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
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        let (first, second) = parse_field(input)?;
        Ok(FpLitParams { first, second })
    }
}
