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

    let mut buffer = String::with_capacity(6);
    let bytes = string.as_ref().bytes().map(move |b| {
        buffer.clear();
        let c = b as char;
        let _ = if (c.is_alphanumeric() || c == '_') && b < 128 {
            buffer.push_str("__");
            buffer.push(c);
        } else {
            buffer.push_str("__0x");
            write_hex(b / 16, &mut buffer);
            write_hex(b % 16, &mut buffer);
        };
        syn::Ident::new(&buffer, Span::call_site())
    });
    quote!( ::structural::TStr<::structural::__TS<( #( ::structural::#bytes,)* )>> )
}

#[inline]
#[allow(dead_code)]
fn write_hex(mut n: u8, buffer: &mut String) {
    n = n & 0xF;
    const HEX_OFFSET: u8 = b'A' - 10;
    let offset = if n < 10 { b'0' } else { HEX_OFFSET };
    buffer.push((n + offset) as char);
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
