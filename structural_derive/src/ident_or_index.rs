use crate::arenas::Arenas;

use std::fmt::{self, Display};

use as_derive_utils::datastructure::FieldIdent;

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::ToTokens;

use syn::{
    parse::{Parse, ParseStream},
    Ident, Index as SynIndex,
};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) enum IdentOrIndex {
    Ident(Ident),
    Index(SynIndex),
}

impl IdentOrIndex {
    pub(crate) fn borrowed(&self) -> IdentOrIndexRef<'_> {
        match self {
            IdentOrIndex::Ident(x) => IdentOrIndexRef::Ident(x),
            IdentOrIndex::Index(x) => x.into(),
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
            IdentOrIndex::Index(x) => Display::fmt(&x.index, f),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
pub(crate) enum IdentOrIndexRef<'a> {
    Ident(&'a Ident),
    Index { index: u32, span: Span },
}

impl<'a> From<&'a Ident> for IdentOrIndexRef<'a> {
    fn from(ident: &'a Ident) -> Self {
        IdentOrIndexRef::Ident(ident)
    }
}

impl<'a> From<SynIndex> for IdentOrIndexRef<'a> {
    fn from(SynIndex { index, span }: SynIndex) -> Self {
        IdentOrIndexRef::Index { index, span }
    }
}

impl<'a> From<&'a SynIndex> for IdentOrIndexRef<'a> {
    fn from(&SynIndex { index, span }: &'a SynIndex) -> Self {
        IdentOrIndexRef::Index { index, span }
    }
}

impl<'a> From<&'_ FieldIdent<'a>> for IdentOrIndexRef<'a> {
    fn from(x: &'_ FieldIdent<'a>) -> Self {
        match x {
            FieldIdent::Index(index, ident) => IdentOrIndexRef::Index {
                index: (*index) as u32,
                span: ident.span(),
            },
            FieldIdent::Named(index) => IdentOrIndexRef::from(*index),
        }
    }
}

impl<'a> IdentOrIndexRef<'a> {
    pub(crate) fn parse(arenas: &'a Arenas, input: ParseStream) -> Result<Self, syn::Error> {
        IdentOrIndex::parse(input).map(|ioi| match ioi {
            IdentOrIndex::Ident(x) => IdentOrIndexRef::Ident(arenas.alloc(x)),
            IdentOrIndex::Index(x) => x.into(),
        })
    }

    pub(crate) fn span(&self) -> Span {
        match self {
            IdentOrIndexRef::Ident(x) => x.span(),
            IdentOrIndexRef::Index { span, .. } => *span,
        }
    }
}

impl ToTokens for IdentOrIndexRef<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            IdentOrIndexRef::Ident(x) => x.to_tokens(tokens),
            IdentOrIndexRef::Index { index, .. } => index.to_tokens(tokens),
        }
    }
}

impl Display for IdentOrIndexRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentOrIndexRef::Ident(x) => Display::fmt(x, f),
            IdentOrIndexRef::Index { index, .. } => Display::fmt(index, f),
        }
    }
}
