use std::fmt::{self, Display};

use proc_macro2::TokenStream as TokenStream2;

use quote::ToTokens;

use syn::{
    parse::{Parse, ParseStream},
    Ident,
};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum IdentOrIndex {
    Ident(Ident),
    Index(syn::LitInt),
}

impl IdentOrIndex {
    pub(crate) fn borrowed(&self) -> IdentOrIndexRef<'_> {
        match self {
            IdentOrIndex::Ident(x) => IdentOrIndexRef::Ident(x),
            IdentOrIndex::Index(x) => IdentOrIndexRef::Index(x),
        }
    }
}

impl Parse for IdentOrIndex {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident) {
            Ok(IdentOrIndex::Ident(input.parse()?))
        } else if lookahead.peek(syn::LitInt) {
            Ok(IdentOrIndex::Index(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for IdentOrIndex {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            IdentOrIndex::Ident(x) => x.to_tokens(tokens),
            IdentOrIndex::Index(x) => x.to_tokens(tokens),
        }
    }
}

impl Display for IdentOrIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentOrIndex::Ident(x) => Display::fmt(x, f),
            IdentOrIndex::Index(x) => Display::fmt(x, f),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) enum IdentOrIndexRef<'a> {
    Ident(&'a Ident),
    Index(&'a syn::LitInt),
}

impl ToTokens for IdentOrIndexRef<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            IdentOrIndexRef::Ident(x) => x.to_tokens(tokens),
            IdentOrIndexRef::Index(x) => x.to_tokens(tokens),
        }
    }
}

impl Display for IdentOrIndexRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentOrIndexRef::Ident(x) => Display::fmt(x, f),
            IdentOrIndexRef::Index(x) => Display::fmt(x, f),
        }
    }
}
