/*!
An implementation detail of structural.
*/

#![recursion_limit = "192"]
// #![deny(unused_variables)]
// #![deny(unused_imports)]
// #![deny(unused_parens)]
// #![deny(unused_assignments)]
// #![deny(unused_mut)]
#![deny(unreachable_patterns)]
#![deny(unused_doc_comments)]
#![deny(unconditional_recursion)]

extern crate proc_macro;

mod field_access;
mod field_path_aliases_macro;
mod field_paths;
mod fp_impl;
mod ident_or_index;
mod parse_utils;
mod structural_alias_impl;
mod structural_derive;
mod tokenizers;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;

/**


This macro is documented in structural::docs::structural_macro

*/

#[proc_macro_derive(Structural, attributes(struc))]
pub fn derive_structural(input: TokenStream1) -> TokenStream1 {
    parse_or_compile_err(input, structural_derive::derive).into()
}

#[proc_macro]
#[doc(hidden)]
pub fn structural_alias_impl(input: TokenStream1) -> TokenStream1 {
    parse_or_compile_err(input, structural_alias_impl::macro_impl).into()
}

#[proc_macro]
#[allow(non_snake_case)]
#[doc(hidden)]
pub fn _FP_impl_(input: TokenStream1) -> TokenStream1 {
    parse_or_compile_err(input, fp_impl::FP_impl).into()
}

/**
The implementation of the fp macro without enabling proc macros in expression position.
*/
#[proc_macro]
#[doc(hidden)]
pub fn old_fp_impl_(input: TokenStream1) -> TokenStream1 {
    parse_or_compile_err(input, fp_impl::old_fp_impl).into()
}

/*
/// This is for referencing generic parameters within `fp!()`,
uncomment this if you add a cargo feature to enable proc macros in expression position.
#[proc_macro]
#[doc(hidden)]
pub fn new_fp_impl_(input: TokenStream1) -> TokenStream1 {
    parse_or_compile_err(input,fp_impl::new_fp_impl).into()
}
*/

#[proc_macro]
#[doc(hidden)]
pub fn _field_path_aliases_impl(input: TokenStream1) -> TokenStream1 {
    parse_or_compile_err(input, field_path_aliases_macro::impl_).into()
}

////////////////////////////////////////////////////////////////////////////////

fn parse_or_compile_err<P, F>(input: TokenStream1, f: F) -> TokenStream2
where
    P: syn::parse::Parse,
    F: FnOnce(P) -> Result<TokenStream2, syn::Error>,
{
    syn::parse::<P>(input)
        .and_then(f)
        .unwrap_or_else(|e| e.to_compile_error())
}
