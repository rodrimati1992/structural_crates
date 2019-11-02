use std::fmt::{self,Display};

use proc_macro2::{
    TokenStream as TokenStream2,
    Span,
};

use quote::ToTokens;

use syn::{
    parse::{self,ParseStream,Parse},
    spanned::Spanned,
    Ident,
};


pub(crate) enum StrOrIdent{
    Str(syn::LitStr),
    Ident(IdentOrIndex)
}


impl parse::Parse for StrOrIdent{
    fn parse(input: parse::ParseStream) -> parse::Result<Self>{
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            input.parse().map(StrOrIdent::Str)
        } else {
            input.parse().map(StrOrIdent::Ident)
        }
    }
}


impl ToTokens for StrOrIdent{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            StrOrIdent::Str(x)=>x.to_tokens(tokens),
            StrOrIdent::Ident(x)=>x.to_tokens(tokens),
        }
    }
}

impl StrOrIdent {
    pub(crate) fn value(&self)->String{
        match self {
            StrOrIdent::Str(x)=>x.value(),
            StrOrIdent::Ident(x)=>x.to_string(),
        }
    }

    pub(crate) fn span(&self)->Span{
        Spanned::span(self)
    }
}


////////////////////////////////////////////////////////////////////////////////


#[derive(Debug)]
pub(crate) enum IdentOrIndex{
    Ident(Ident),
    Index(syn::LitInt),
}

impl IdentOrIndex{
    pub(crate) fn borrowed(&self)->IdentOrIndexRef<'_>{
        match self {
            IdentOrIndex::Ident(x) => IdentOrIndexRef::Ident(x),
            IdentOrIndex::Index(x) => IdentOrIndexRef::Index(x),
        }
    }
}

impl Parse for IdentOrIndex{
    fn parse(input: ParseStream) -> Result<Self,syn::Error> {
        let lookahead=input.lookahead1();
        if lookahead.peek(syn::Ident) {
            Ok(IdentOrIndex::Ident(input.parse()?))
        } else if lookahead.peek(syn::LitInt) {
            Ok(IdentOrIndex::Index(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for IdentOrIndex{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            IdentOrIndex::Ident(x) => x.to_tokens(tokens),
            IdentOrIndex::Index(x) => x.to_tokens(tokens),
        }
    }
}

impl Display for IdentOrIndex{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        match self {
            IdentOrIndex::Ident(x) => Display::fmt(x,f),
            IdentOrIndex::Index(x) => Display::fmt(x,f),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////


pub(crate) enum IdentOrIndexRef<'a>{
    Ident(&'a Ident),
    Index(&'a syn::LitInt),
}

impl ToTokens for IdentOrIndexRef<'_>{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            IdentOrIndexRef::Ident(x) => x.to_tokens(tokens),
            IdentOrIndexRef::Index(x) => x.to_tokens(tokens),
        }
    }
}

impl Display for IdentOrIndexRef<'_>{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        match self {
            IdentOrIndexRef::Ident(x) => Display::fmt(x,f),
            IdentOrIndexRef::Index(x) => Display::fmt(x,f),
        }
    }
}
