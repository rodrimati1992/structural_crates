use crate::{datastructure::StructOrEnum, parse_utils::ParseBufferExt};

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

    pub(crate) fn compute_trait(
        self,
        optionality: IsOptional,
        struct_or_enum: StructOrEnum,
    ) -> ComputeTrait {
        ComputeTrait {
            access: self,
            optionality,
            struct_or_enum,
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
pub(crate) struct ComputeTrait {
    pub(crate) access: Access,
    pub(crate) optionality: IsOptional,
    pub(crate) struct_or_enum: StructOrEnum,
}

/*

Using this to generate the match branches below.

Using code generation for this seems like the best approach,
since manually writing the 16 branches is error prone.

fn main(){
    let kind_list=[("Struct","Field"),("Enum","VariantField")];

    let access_list=[
        ("Shared","Get",""),
        ("Value","Into",""),
        ("Mutable","Get","Mut"),
        ("MutValue","Into","Mut"),
    ];

    let optionality_list=[("Yes","Opt"), ("No","")];

    for (kind,field_pc) in &kind_list {
        for (access,pre_pc,post_pc) in &access_list {
            for (opt,opt_pc) in &optionality_list {
                println!(
                    "(SOE::{0}, {1}, IO::{2})=> \
                        AAIO_match!(inner; $kind {opt}{pre}{field}{post} ),\
                    ",
                    kind,
                    access,
                    opt,
                    opt=opt_pc,
                    pre=pre_pc,
                    field=field_pc,
                    post=post_pc,
                );
            }
        }
    }
}
*/

#[allow(non_upper_case_globals)]
mod access_consts {
    use super::Access;
    pub(super) const Shared: Access = Access::Shared;
    pub(super) const Mutable: Access = Access::Mutable;
    pub(super) const Value: Access = Access::Value;
    pub(super) const MutValue: Access = Access::MutValue;
}

macro_rules! AAIO_match {
    ( self=$this:ident kind=$kind:ident ) => ({
        use access_consts::{Shared,Mutable,Value,MutValue};
        use self::IsOptional as IO;
        use self::StructOrEnum as SOE;


        let ComputeTrait{access,optionality,struct_or_enum}=$this;
        #[allow(non_upper_case_globals)]
        match (struct_or_enum,access,optionality) {
            (SOE::Struct, Shared, IO::Yes)=> AAIO_match!(inner; $kind OptGetField ),
            (SOE::Struct, Shared, IO::No)=> AAIO_match!(inner; $kind GetField ),
            (SOE::Struct, Value, IO::Yes)=> AAIO_match!(inner; $kind OptIntoField ),
            (SOE::Struct, Value, IO::No)=> AAIO_match!(inner; $kind IntoField ),
            (SOE::Struct, Mutable, IO::Yes)=> AAIO_match!(inner; $kind OptGetFieldMut ),
            (SOE::Struct, Mutable, IO::No)=> AAIO_match!(inner; $kind GetFieldMut ),
            (SOE::Struct, MutValue, IO::Yes)=> AAIO_match!(inner; $kind OptIntoFieldMut ),
            (SOE::Struct, MutValue, IO::No)=> AAIO_match!(inner; $kind IntoFieldMut ),
            (SOE::Enum, Shared, IO::Yes)=> AAIO_match!(inner; $kind OptGetVariantField ),
            (SOE::Enum, Shared, IO::No)=> AAIO_match!(inner; $kind GetVariantField ),
            (SOE::Enum, Value, IO::Yes)=> AAIO_match!(inner; $kind OptIntoVariantField ),
            (SOE::Enum, Value, IO::No)=> AAIO_match!(inner; $kind IntoVariantField ),
            (SOE::Enum, Mutable, IO::Yes)=> AAIO_match!(inner; $kind OptGetVariantFieldMut ),
            (SOE::Enum, Mutable, IO::No)=> AAIO_match!(inner; $kind GetVariantFieldMut ),
            (SOE::Enum, MutValue, IO::Yes)=> AAIO_match!(inner; $kind OptIntoVariantFieldMut ),
            (SOE::Enum, MutValue, IO::No)=> AAIO_match!(inner; $kind IntoVariantFieldMut ),
        }
    });
    (inner; quote $trait_:ident )=>{
        quote!($trait_)
    };
    (inner; stringify $trait_:ident )=>{
        stringify!($trait_)
    };
}

impl ComputeTrait {
    pub(crate) fn trait_name(self) -> &'static str {
        let this = self;

        AAIO_match!( self=this kind=stringify )
    }

    pub(crate) fn trait_tokens(self) -> TokenStream2 {
        let this = self;

        AAIO_match!( self=this kind=quote )
    }
}

impl ToTokens for ComputeTrait {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        ts.append_all(self.trait_tokens());
    }
}
