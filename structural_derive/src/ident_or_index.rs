use crate::{
    arenas::Arenas,
    ignored_wrapper::Ignored,
    utils::{remove_raw_prefix, DisplayWith},
};

use std::fmt::Display;

use as_derive_utils::datastructure::FieldIdent;

use proc_macro2::{Literal, Span, TokenStream as TokenStream2};

use quote::ToTokens;

use syn::{
    parse::{Parse, ParseStream},
    Ident, Index as SynIndex,
};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum IdentOrIndex {
    Ident(Ident),
    Index(SynIndex),
    Str { str: String, span: Ignored<Span> },
}

impl IdentOrIndex {
    pub(crate) fn borrowed(&self) -> IdentOrIndexRef<'_> {
        match self {
            IdentOrIndex::Ident(x) => IdentOrIndexRef::Ident(x),
            IdentOrIndex::Index(x) => x.into(),
            IdentOrIndex::Str { str, span } => IdentOrIndexRef::Str { str, span: *span },
        }
    }
}

impl Parse for IdentOrIndex {
    fn parse(input: ParseStream<'_>) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident) {
            Ok(IdentOrIndex::Ident(input.parse()?))
        } else if lookahead.peek(syn::LitInt) {
            Ok(IdentOrIndex::Index(input.parse()?))
        } else if lookahead.peek(syn::LitStr) {
            let lit = input.parse::<syn::LitStr>()?;
            Ok(IdentOrIndex::Str {
                str: lit.value(),
                span: Ignored::new(lit.span()),
            })
        } else {
            Err(lookahead.error())
        }
    }
}

impl IdentOrIndex {
    #[allow(dead_code)]
    pub(crate) fn peek(input: ParseStream<'_>) -> bool {
        input.peek(syn::Ident) || input.peek(syn::LitInt) || input.peek(syn::LitStr)
    }

    #[allow(dead_code)]
    pub(crate) fn peek_parse(input: ParseStream<'_>) -> Result<Option<Self>, syn::Error> {
        let lookahead = input.lookahead1();
        let ret = if lookahead.peek(syn::Ident) {
            IdentOrIndex::Ident(input.parse()?)
        } else if lookahead.peek(syn::LitInt) {
            IdentOrIndex::Index(input.parse()?)
        } else if lookahead.peek(syn::LitStr) {
            let lit = input.parse::<syn::LitStr>()?;
            IdentOrIndex::Str {
                str: lit.value(),
                span: Ignored::new(lit.span()),
            }
        } else {
            return Ok(None);
        };
        Ok(Some(ret))
    }

    pub(crate) fn span(&self) -> Span {
        match self {
            IdentOrIndex::Ident(x) => x.span(),
            IdentOrIndex::Index(x) => x.span,
            IdentOrIndex::Str { span, .. } => span.value,
        }
    }

    pub(crate) fn tstr_tokens(&self) -> TokenStream2 {
        use crate::tokenizers::tstr_tokens;
        let (string, span) = self.string_and_span();
        tstr_tokens(string, span)
    }

    pub(crate) fn string_and_span(&self) -> (String, Span) {
        match self {
            IdentOrIndex::Ident(x) => (remove_raw_prefix(x.to_string()), x.span()),
            IdentOrIndex::Index(x) => (x.index.to_string(), x.span),
            IdentOrIndex::Str { str, span } => (str.into(), span.value),
        }
    }

    #[allow(dead_code)]
    pub fn display(&self) -> impl Display + '_ {
        DisplayWith::new(move |f| match self {
            IdentOrIndex::Ident(x) => Display::fmt(&remove_raw_prefix(x.to_string()), f),
            IdentOrIndex::Index(x) => Display::fmt(&x.index, f),
            IdentOrIndex::Str { str, .. } => f.write_str(str),
        })
    }
}

impl ToString for IdentOrIndex {
    fn to_string(&self) -> String {
        self.string_and_span().0
    }
}

impl ToTokens for IdentOrIndex {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.borrowed().to_tokens(tokens)
    }
}

impl From<syn::LitStr> for IdentOrIndex {
    fn from(lit: syn::LitStr) -> Self {
        IdentOrIndex::Str {
            str: lit.value(),
            span: Ignored::new(lit.span()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum IdentOrIndexRef<'a> {
    Ident(&'a Ident),
    Index { index: u32, span: Ignored<Span> },
    Str { str: &'a str, span: Ignored<Span> },
}

impl<'a> From<&'a Ident> for IdentOrIndexRef<'a> {
    fn from(ident: &'a Ident) -> Self {
        IdentOrIndexRef::Ident(ident)
    }
}

impl<'a> From<SynIndex> for IdentOrIndexRef<'a> {
    fn from(SynIndex { index, span }: SynIndex) -> Self {
        IdentOrIndexRef::Index {
            index,
            span: Ignored::new(span),
        }
    }
}

impl<'a> From<&'a SynIndex> for IdentOrIndexRef<'a> {
    fn from(&SynIndex { index, span }: &'a SynIndex) -> Self {
        IdentOrIndexRef::Index {
            index,
            span: Ignored::new(span),
        }
    }
}

impl<'a> From<&'_ FieldIdent<'a>> for IdentOrIndexRef<'a> {
    fn from(x: &'_ FieldIdent<'a>) -> Self {
        match x {
            FieldIdent::Index(index, ident) => IdentOrIndexRef::Index {
                index: (*index) as u32,
                span: Ignored::new(ident.span()),
            },
            FieldIdent::Named(index) => IdentOrIndexRef::from(*index),
        }
    }
}

impl<'a> IdentOrIndexRef<'a> {
    pub(crate) fn parse(arenas: &'a Arenas, input: ParseStream<'_>) -> Result<Self, syn::Error> {
        IdentOrIndex::parse(input).map(|ioi| match ioi {
            IdentOrIndex::Ident(x) => IdentOrIndexRef::Ident(arenas.alloc(x)),
            IdentOrIndex::Index(x) => x.into(),
            IdentOrIndex::Str { str, span } => IdentOrIndexRef::Str {
                str: arenas.alloc(str),
                span,
            },
        })
    }

    pub(crate) fn span(&self) -> Span {
        match self {
            IdentOrIndexRef::Ident(x) => x.span(),
            IdentOrIndexRef::Index { span, .. } => span.value,
            IdentOrIndexRef::Str { span, .. } => span.value,
        }
    }

    pub(crate) fn tstr_tokens(self) -> TokenStream2 {
        use crate::tokenizers::tstr_tokens;
        let (borrowed, span) = self.string_and_span();
        tstr_tokens(borrowed, span)
    }

    pub(crate) fn string_and_span(self) -> (String, Span) {
        match self {
            IdentOrIndexRef::Ident(x) => (remove_raw_prefix(x.to_string()), x.span()),
            IdentOrIndexRef::Index { index, span } => (index.to_string(), span.value),
            IdentOrIndexRef::Str { str, span } => (str.into(), span.value),
        }
    }

    pub fn display(&self) -> impl Display + '_ {
        DisplayWith::new(move |f| match self {
            IdentOrIndexRef::Ident(x) => Display::fmt(&remove_raw_prefix(x.to_string()), f),
            IdentOrIndexRef::Index { index, .. } => Display::fmt(index, f),
            IdentOrIndexRef::Str { str, .. } => f.write_str(str),
        })
    }
}

impl<'a> ToString for IdentOrIndexRef<'a> {
    fn to_string(&self) -> String {
        self.string_and_span().0
    }
}

impl ToTokens for IdentOrIndexRef<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            IdentOrIndexRef::Ident(x) => x.to_tokens(tokens),
            IdentOrIndexRef::Index { index, span } => {
                let mut lit = Literal::u32_unsuffixed(*index);
                lit.set_span(span.value);
                lit.to_tokens(tokens);
            }
            IdentOrIndexRef::Str { str, span } => {
                let mut lit = Literal::string(*str);
                lit.set_span(span.value);
                lit.to_tokens(tokens);
            }
        }
    }
}
