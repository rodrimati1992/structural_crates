use crate::parse_utils::ParseBufferExt;

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, ToTokens, TokenStreamExt};

use syn::{
    parse::{Parse, ParseStream},
    Ident, Token,
};

/// Whether a field can be accessed by reference/mutable-reference/value.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Access {
    mutable: bool,
    value: bool,
}

#[allow(non_upper_case_globals)]
impl Access {
    /// A field gets a GetFieldImpl impl.
    pub(crate) const Shared: Self = Self {
        mutable: false,
        value: false,
    };

    /// A field gets GetFieldImpl,and GetFieldMutImpl impls.
    pub(crate) const Mutable: Self = Self {
        mutable: true,
        value: false,
    };

    /// A field gets GetFieldImpl,and IntoFieldImpl impls.
    pub(crate) const Value: Self = Self {
        mutable: false,
        value: true,
    };

    /// A field gets GetFieldImpl,GetFieldMutImpl,and IntoFieldImpl impls.
    pub(crate) const MutValue: Self = Self {
        mutable: true,
        value: true,
    };

    pub(crate) fn and_optionality(self, optionality: IsOptional) -> AccessAndIsOptional {
        AccessAndIsOptional {
            access: self,
            optionality,
        }
    }

    pub(crate) fn parse_optional(input: ParseStream) -> Result<Option<Self>, syn::Error> {
        if input.peek_parse(Token![ref])?.is_some() {
            if input.peek_parse(Token![move])?.is_some() {
                Ok(Some(Access::Value))
            } else if input.peek(Token![mut]) {
                Err(input.error("Expected `move` or nothing."))
            } else {
                Ok(Some(Access::Shared))
            }
        } else if input.peek_parse(Token![mut])?.is_some() {
            if input.peek_parse(Token![move])?.is_some() {
                Ok(Some(Access::MutValue))
            } else if input.peek(Token![ref]) {
                Err(input.error("Expected `move` or nothing."))
            } else {
                Ok(Some(Access::Mutable))
            }
        } else if input.peek_parse(Token![move])?.is_some() {
            Ok(Some(Access::Value))
        } else {
            Ok(None)
        }
    }
}

impl Default for Access {
    fn default() -> Self {
        Access::MutValue
    }
}

impl Parse for Access {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::parse_optional(input).map(|x| x.unwrap_or(Access::MutValue))
    }
}

impl ToTokens for Access {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match *self {
            Access::Shared => Ident::new("GetFieldImpl", Span::call_site()),
            Access::Mutable => Ident::new("GetFieldMutImpl", Span::call_site()),
            Access::Value => Ident::new("IntoFieldImpl", Span::call_site()),
            Access::MutValue => Ident::new("IntoFieldMut", Span::call_site()),
        }
        .to_tokens(tokens);
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum IsOptional {
    Yes,
    No,
}

impl IsOptional {
    pub(crate) fn derive_arg(self) -> IsOptionalDeriveArg {
        IsOptionalDeriveArg { value: self }
    }
}

impl ToTokens for IsOptional {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(match *self {
            IsOptional::Yes => quote!(OptionalField),
            IsOptional::No => quote!(NonOptField),
        });
    }
}

impl Parse for IsOptional {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.peek_parse(syn::Token![?]).map(|x| match x {
            Some(_) => IsOptional::Yes,
            None => IsOptional::No,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
pub(crate) struct IsOptionalDeriveArg {
    value: IsOptional,
}

impl ToTokens for IsOptionalDeriveArg {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(match self.value {
            IsOptional::Yes => quote!(opt),
            IsOptional::No => quote!(nonopt),
        });
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct AccessAndIsOptional {
    pub(crate) access: Access,
    pub(crate) optionality: IsOptional,
}

macro_rules! AAIO_match {
    ( self=$this:ident kind=$kind:ident ) => ({
        let AccessAndIsOptional{access,optionality}=$this;
        match (access,optionality) {
            (Access::Shared  ,IsOptional::No )=>AAIO_match!(inner; $kind GetField ),
            (Access::Shared  ,IsOptional::Yes)=>AAIO_match!(inner; $kind OptGetField ),
            (Access::Value   ,IsOptional::No )=>AAIO_match!(inner; $kind IntoField ),
            (Access::Value   ,IsOptional::Yes)=>AAIO_match!(inner; $kind OptIntoField ),
            (Access::Mutable ,IsOptional::No )=>AAIO_match!(inner; $kind GetFieldMut ),
            (Access::Mutable ,IsOptional::Yes)=>AAIO_match!(inner; $kind OptGetFieldMut ),
            (Access::MutValue,IsOptional::No )=>AAIO_match!(inner; $kind IntoFieldMut ),
            (Access::MutValue,IsOptional::Yes)=>AAIO_match!(inner; $kind OptIntoFieldMut ),
        }
    });
    (inner; quote $trait_:ident )=>{
        quote!($trait_)
    };
    (inner; stringify $trait_:ident )=>{
        stringify!($trait_)
    };
}

impl AccessAndIsOptional {
    pub(crate) fn trait_name(self) -> &'static str {
        let this = self;

        AAIO_match!( self=this kind=stringify )
    }
}

impl ToTokens for AccessAndIsOptional {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let this = *self;

        AAIO_match!( self=this kind=quote ).to_tokens(tokens);
    }
}
