use crate::parse_utils::ParseBufferExt;

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::{
    TokenStream as TokenStream2,
    Span,
};

use quote::ToTokens;

use syn::{
    parse::{Parse,ParseStream},
    Ident,
    Token,
};



/// Whether a field can be accessed by reference/mutable-reference/value.
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub(crate) struct Access{
    mutable:bool,
    value:bool,
}

#[allow(non_upper_case_globals)]
impl Access{
    /// A field gets a GetField impl.
    pub(crate) const Shared:Self=Self{ mutable:false, value:false };

    /// A field gets GetField,and GetFieldMut impls.
    pub(crate) const Mutable:Self=Self{ mutable:true, value:false };

    /// A field gets GetField,and IntoField impls.
    pub(crate) const Value:Self=Self{ mutable:false, value:true };

    /// A field gets GetField,GetFieldMut,and IntoField impls.
    pub(crate) const MutValue:Self=Self{ mutable:true, value:true };
}


impl Default for Access{
    fn default()->Self{
        Access::MutValue
    }
}

impl Parse for Access {
    fn parse(input: ParseStream) -> Result<Self,syn::Error> {
        if input.peek_parse(Token![ref]).is_some() {
            if input.peek_parse(Token![move]).is_some() {
                Ok(Access::Value)
            }else if input.peek(Token![mut]) {
                Err(input.error("Expected `move` or nothing."))
            }else{
                Ok(Access::Shared)
            }
        }else if input.peek_parse(Token![mut]).is_some() {
            if input.peek_parse(Token![move]).is_some() {
                Ok(Access::MutValue)
            }else if input.peek(Token![ref]) {
                Err(input.error("Expected `move` or nothing."))
            }else{
                Ok(Access::Mutable)
            }
        }else if input.peek_parse(Token![move]).is_some() {
            Ok(Access::Value)
        }else{
            Ok(Access::MutValue)
        }
    }
}

impl ToTokens for Access{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match *self {
            Access::Shared=>Ident::new("GetField",Span::call_site()),
            Access::Mutable=>Ident::new("GetFieldMut",Span::call_site()),
            Access::Value=>Ident::new("IntoField",Span::call_site()),
            Access::MutValue=>Ident::new("IntoFieldMut",Span::call_site()),
        }.to_tokens(tokens);
    }
}