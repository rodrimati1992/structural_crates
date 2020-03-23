use proc_macro2::TokenStream as TokenStream2;

#[allow(unused_imports)]
use quote::{quote, ToTokens};

////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(all(feature = "use_const_str", not(feature = "disable_const_str")))]
pub(crate) fn tstr_tokens<S>(string: S) -> TokenStream2
where
    S: AsRef<str>,
{
    let string = string.as_ref();

    quote!( ::structural::TStr<::structural::__TS<#string>> )
}

#[cfg(any(not(feature = "use_const_str"), feature = "disable_const_str"))]
/// Tokenizes a `TStr<>` in which each character is written as a type.
pub(crate) fn tstr_tokens<S>(string: S) -> TokenStream2
where
    S: AsRef<str>,
{
    use proc_macro2::Span;

    use std::fmt::Write;
    let mut buffer = String::new();
    let bytes = string.as_ref().bytes().map(move |b| {
        buffer.clear();
        let c = b as char;
        let _ = if (c.is_alphanumeric() || c == '_') && b < 128 {
            write!(buffer, "__{}", c)
        } else {
            write!(buffer, "__0x{:02X}", b)
        };
        syn::Ident::new(&buffer, Span::call_site())
    });
    quote!( ::structural::TStr<::structural::__TS<( #( ::structural::#bytes,)* )>> )
}

pub(crate) fn variant_field_tokens<S0, S1>(variant: S0, field: S1) -> TokenStream2
where
    S0: AsRef<str>,
    S1: AsRef<str>,
{
    let variant_tokens = tstr_tokens(variant.as_ref());
    let field_tokens = tstr_tokens(field.as_ref());
    quote!(
        ::structural::pmr::VariantField<
            #variant_tokens,
            #field_tokens,
        >
    )
}

pub(crate) fn variant_name_tokens<S0>(variant: S0) -> TokenStream2
where
    S0: AsRef<str>,
{
    let variant_tokens = tstr_tokens(variant.as_ref());
    quote!(
        ::structural::pmr::VariantName< #variant_tokens >
    )
}
