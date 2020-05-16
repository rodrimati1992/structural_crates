use crate::ident_or_index::IdentOrIndexRef;

use proc_macro2::{Span, TokenStream as TokenStream2};

#[allow(unused_imports)]
use quote::{quote_spanned, ToTokens};

////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(all(feature = "use_const_str", not(feature = "disable_const_str")))]
pub(crate) fn tstr_tokens<S>(string: S, span: Span) -> TokenStream2
where
    S: AsRef<str>,
{
    let string = string.as_ref();

    quote_spanned!(span=> ::structural::TStr<::structural::__TS<#string>> )
}

#[cfg(any(not(feature = "use_const_str"), feature = "disable_const_str"))]
/// Tokenizes a `TStr<>` in which each character is written as a type.
pub(crate) fn tstr_tokens<S>(string: S, span: Span) -> TokenStream2
where
    S: AsRef<str>,
{
    let mut buffer = String::with_capacity(6);
    let bytes = string.as_ref().bytes().map(move |b| {
        buffer.clear();
        let c = b as char;
        if (c.is_alphanumeric() || c == '_') && b < 128 {
            buffer.push_str("__");
            buffer.push(c);
        } else {
            buffer.push_str("__0x");
            write_hex(b / 16, &mut buffer);
            write_hex(b % 16, &mut buffer);
        }
        syn::Ident::new(&buffer, Span::call_site())
    });
    quote_spanned!(span=>
        ::structural::TStr<::structural::__TS<( #( ::structural::#bytes,)* )>>
    )
}

#[inline]
#[allow(dead_code)]
fn write_hex(mut n: u8, buffer: &mut String) {
    n &= 0xF;
    const HEX_OFFSET: u8 = b'A' - 10;
    let offset = if n < 10 { b'0' } else { HEX_OFFSET };
    buffer.push((n + offset) as char);
}

pub(crate) fn variant_field_tokens(
    variant: IdentOrIndexRef<'_>,
    field: IdentOrIndexRef<'_>,
) -> TokenStream2 {
    let variant_tokens = variant.tstr_tokens();
    let field_tokens = field.tstr_tokens();
    let span = field.span();
    quote_spanned!(span=>
        ::structural::pmr::VariantField<
            #variant_tokens,
            #field_tokens,
        >
    )
}

pub(crate) fn variant_name_tokens(variant: IdentOrIndexRef<'_>) -> TokenStream2 {
    let variant_tokens = variant.tstr_tokens();
    let span = variant.span();
    quote_spanned!(span=>
        ::structural::pmr::VariantName< #variant_tokens >
    )
}
