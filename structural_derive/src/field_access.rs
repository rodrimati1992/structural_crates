use crate::{datastructure::StructOrEnum, parse_utils::ParseBufferExt};

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens, TokenStreamExt};

use syn::{
    parse::{Parse, ParseStream},
    Token,
};

/// Whether a field can be accessed by reference/mutable-reference/value.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Access {
    mutable: bool,
    value: bool,
}

#[allow(non_upper_case_globals)]
impl Access {
    /// A field gets a GetField impl.
    pub(crate) const Shared: Self = Self {
        mutable: false,
        value: false,
    };

    /// A field gets GetField,and GetFieldMut impls.
    pub(crate) const Mutable: Self = Self {
        mutable: true,
        value: false,
    };

    /// A field gets GetField,and IntoField impls.
    pub(crate) const Value: Self = Self {
        mutable: false,
        value: true,
    };

    /// A field gets GetField,GetFieldMut,and IntoField impls.
    pub(crate) const MutValue: Self = Self {
        mutable: true,
        value: true,
    };

    pub(crate) fn has_by_value_access(&self) -> bool {
        self.value
    }

    #[allow(dead_code)]
    pub(crate) fn has_mutable_access(&self) -> bool {
        self.mutable
    }

    pub(crate) fn compute_trait(self, struct_or_enum: StructOrEnum) -> ComputeTrait {
        ComputeTrait {
            access: self,
            struct_or_enum,
        }
    }

    pub(crate) fn parse_optional(input: ParseStream<'_>) -> Result<Option<Self>, syn::Error> {
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
    fn parse(input: ParseStream<'_>) -> Result<Self, syn::Error> {
        Self::parse_optional(input).map(|x| x.unwrap_or(Access::MutValue))
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct ComputeTrait {
    pub(crate) access: Access,
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

    for (kind,field_pc) in &kind_list {
        for (access,pre_pc,post_pc) in &access_list {
            println!(
                "(SOE::{0}, {1})=> \
                    AAIO_match!(inner; $kind {pre}{field}{post} ),\
                ",
                kind,
                access,
                pre=pre_pc,
                field=field_pc,
                post=post_pc,
            );
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
        use self::StructOrEnum as SOE;


        let ComputeTrait{access,struct_or_enum}=$this;
        #[allow(non_upper_case_globals)]
        match (struct_or_enum,access) {
            (SOE::Struct, Shared)=> AAIO_match!(inner; $kind GetField ),
            (SOE::Struct, Value)=> AAIO_match!(inner; $kind IntoField ),
            (SOE::Struct, Mutable)=> AAIO_match!(inner; $kind GetFieldMut ),
            (SOE::Struct, MutValue)=> AAIO_match!(inner; $kind IntoFieldMut ),
            (SOE::Enum, Shared)=> AAIO_match!(inner; $kind GetVariantField ),
            (SOE::Enum, Value)=> AAIO_match!(inner; $kind IntoVariantField ),
            (SOE::Enum, Mutable)=> AAIO_match!(inner; $kind GetVariantFieldMut ),
            (SOE::Enum, MutValue)=> AAIO_match!(inner; $kind IntoVariantFieldMut ),
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
