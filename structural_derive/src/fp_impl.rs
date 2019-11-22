use crate::{
    tokenizers::FullPathForChars,
    field_paths::FieldPaths,
};

use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use quote::quote;

use syn::Ident;


/// This is the implementation of the FP macro when 
/// the input isn't space separated characters.
#[allow(non_snake_case)]
pub(crate) fn FP_impl(parsed: FieldPaths) -> Result<TokenStream2,syn::Error> {
    parsed.type_tokens(FullPathForChars::Yes)
        .piped(Ok)
}


#[cfg(test)]
#[allow(non_snake_case)]
pub(crate) fn FP_from_str(input: &str) -> Result<TokenStream2,syn::Error> {
    syn::parse_str(input).and_then(FP_impl)
}

#[test]
#[allow(non_snake_case)]
fn test_FP_macro(){
    use as_derive_utils::test_framework::Tests;

    Tests::load("field_paths").run_test(FP_from_str);
}




///////////////////////////////////////////////////////////////////////////////

pub(crate) fn old_fp_impl(set: FieldPaths) -> Result<TokenStream2,syn::Error> {
    let const_name=Ident::new("VALUE",proc_macro2::Span::call_site());
    let constant=set.constant_named(&const_name,FullPathForChars::StructPmr);

    Ok(quote!(
        #constant
    ))
}

/*

This is for referencing generic parameters within `fp!()`,
uncomment this if you add a cargo feature to enable proc macros in expression position.

pub(crate) fn new_fp_impl(set: FieldPaths) -> Result<TokenStream2,syn::Error> {
    let const_name=Ident::new("value",proc_macro2::Span::call_site());
    let variable=set.variable_named(&const_name,FullPathForChars::StructPmr);

    let pmr_rename=crate::tokenizers::struct_pmr();

    Ok(quote!({
        use structural::pmr as #pmr_rename;
        #variable
        value
    }))
}
*/