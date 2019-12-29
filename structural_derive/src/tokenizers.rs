use crate::ident_or_index::IdentOrIndexRef;

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, ToTokens};

use syn::Ident;

/// Whether to use the full path to an item when refering to it.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum FullPathForChars {
    Yes,
    No,
    StructPmr,
}

////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn struct_pmr_prefix() -> TokenStream2 {
    quote!( __struct_pmr:: )
}

#[allow(dead_code)]
pub(crate) fn struct_pmr() -> TokenStream2 {
    quote!(__struct_pmr)
}

/// Tokenizes a `TString<>` in which each character is written as a type.
pub(crate) fn tident_tokens<S>(string: S, char_verbosity: FullPathForChars) -> TokenStream2
where
    S: AsRef<str>,
{
    let path_prefix = match char_verbosity {
        FullPathForChars::Yes => quote!(::structural::chars::),
        FullPathForChars::No => quote!(),
        FullPathForChars::StructPmr => struct_pmr_prefix(),
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
    quote!( ::structural::pmr::TString<( #( #path_prefix #bytes,)* )> )
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

////////////////////////////////////////////////////////////////////////////////////////////

/// Represents a crate-visible module with a bunch of type aliases for TStrings.
pub(crate) struct NamedModuleAndTokens {
    pub(crate) names_module: Ident,
    pub(crate) alias_names: Vec<Ident>,
    pub(crate) aliases_definitions: TokenStream2,
}

impl NamedModuleAndTokens {
    pub fn new<'a>(thing_ident: &'a syn::Ident) -> Self {
        Self {
            names_module: Ident::new(&format!("{}_names_module", thing_ident), Span::call_site()),
            alias_names: Vec::new(),
            aliases_definitions: TokenStream2::new(),
        }
    }

    pub fn alias_name(&self, index: NamesModuleIndex) -> &Ident {
        &self.alias_names[index.0]
    }

    fn push_inner<F>(&mut self, f: F) -> NamesModuleIndex
    where
        F: FnOnce(usize, &mut TokenStream2) -> Ident,
    {
        let index = self.alias_names.len();
        let alias_name = f(index, &mut self.aliases_definitions);

        self.alias_names.push(alias_name);

        NamesModuleIndex(index)
    }

    pub fn push_path(&mut self, ident: IdentOrIndexRef<'_>) -> NamesModuleIndex {
        self.push_inner(|index, ts| {
            let string = ident.to_string();

            let alias_name = Ident::new(&format!("STR_{}___{}", string, index), ident.span());

            let field_name = tident_tokens(&string, FullPathForChars::No);

            quote!(
                #[allow(non_camel_case_types)]
                pub type #alias_name=structural::pmr::FieldPath<(#field_name,)>;
            )
            .to_tokens(ts);
            alias_name
        })
    }

    pub fn push_str(&mut self, str: IdentOrIndexRef<'_>) -> NamesModuleIndex {
        self.push_inner(|index, ts| {
            let string = str.to_string();

            let alias_name = Ident::new(&format!("STR_{}___{}", string, index), str.span());

            let string = tident_tokens(&string, FullPathForChars::No);

            quote!(
                #[allow(non_camel_case_types)]
                pub type #alias_name=#string;
            )
            .to_tokens(ts);

            alias_name
        })
    }

    pub fn push_variant_field(
        &mut self,
        variant: &syn::Ident,
        field: IdentOrIndexRef<'_>,
    ) -> NamesModuleIndex {
        self.push_inner(|index, ts| {
            let variant = variant.to_string();
            let field_s = field.to_string();
            let alias_name = Ident::new(
                &format!("STR_{}_{}__{}", variant, field, index),
                field.span(),
            );

            let variant_field = variant_field_tokens(&variant, &field_s, FullPathForChars::No);

            quote!(
                #[allow(non_camel_case_types)]
                pub type #alias_name=::structural::pmr::FieldPath<(#variant_field,)>;
            )
            .to_tokens(ts);

            alias_name
        })
    }
}

impl ToTokens for NamedModuleAndTokens {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let aliases_definitions = &self.aliases_definitions;
        let names_module = &self.names_module;

        quote!(
            pub(crate) mod #names_module{
                use super::*;
                use structural::pmr::*;

                #aliases_definitions
            }
        )
        .to_tokens(tokens);
    }
}

///////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct NamesModuleIndex(usize);
