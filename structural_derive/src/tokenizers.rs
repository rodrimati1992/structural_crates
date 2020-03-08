use proc_macro2::TokenStream as TokenStream2;

#[allow(unused_imports)]
use quote::{quote, ToTokens};

////////////////////////////////////////////////////////////////////////////////////////////

/// Whether to use the full path to an item when refering to it.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum FullPathForChars {
    Yes,
}

////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "use_const_str")]
pub(crate) fn tident_tokens<S>(string: S, _: FullPathForChars) -> TokenStream2
where
    S: AsRef<str>,
{
    let string = string.as_ref();

    quote!( ::structural::TStr<::structural::p::TS<#string>> )
}

#[cfg(not(feature = "use_const_str"))]
/// Tokenizes a `TStr<>` in which each character is written as a type.
pub(crate) fn tident_tokens<S>(string: S, char_verbosity: FullPathForChars) -> TokenStream2
where
    S: AsRef<str>,
{
    use proc_macro2::Span;

    let path_prefix = match char_verbosity {
        FullPathForChars::Yes => quote!(::structural::p::),
    };

    use std::fmt::Write;
    let mut buffer = String::new();
    let bytes = string.as_ref().bytes().map(move |b| {
        buffer.clear();
        let c = b as char;
        let _ = if (c.is_alphanumeric() || c == '_') && b < 128 {
            write!(buffer, "_{}", c)
        } else {
            write!(buffer, "B{}", b)
        };
        syn::Ident::new(&buffer, Span::call_site())
    });
    quote!( ::structural::TStr<::structural::p::TS<( #( #path_prefix #bytes,)* )>> )
}

pub(crate) fn variant_field_tokens<S0, S1>(
    variant: S0,
    field: S1,
    char_verbosity: FullPathForChars,
) -> TokenStream2
where
    S0: AsRef<str>,
    S1: AsRef<str>,
{
    let variant_tokens = tident_tokens(variant.as_ref(), char_verbosity);
    let field_tokens = tident_tokens(field.as_ref(), char_verbosity);
    quote!(
        ::structural::pmr::VariantField<
            #variant_tokens,
            #field_tokens,
        >
    )
}

pub(crate) fn variant_name_tokens<S0>(variant: S0, char_verbosity: FullPathForChars) -> TokenStream2
where
    S0: AsRef<str>,
{
    let variant_tokens = tident_tokens(variant.as_ref(), char_verbosity);
    quote!(
        ::structural::pmr::VariantName< #variant_tokens >
    )
}
