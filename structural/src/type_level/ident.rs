/*!
Contains type-level strings,and multi-strings.
*/

#![allow(non_snake_case, non_camel_case_types)]

use core_extensions::MarkerType;

use std_::marker::PhantomData;



pub use crate::chars::*;


/// A type-level string,represented as a tuple of type-level bytes.
///
/// This cannot be converted to a `&'static str` constant.
pub struct TString<T>(PhantomData<T>);

impl<T> TString<T>{
    /// Constructs the TString.
    pub const NEW:Self=TString(PhantomData);
}

impl<T> Copy for TString<T>{}
impl<T> Clone for TString<T>{
    fn clone(&self)->Self{
        *self
    }
}
unsafe impl<T> MarkerType for TString<T>{}


/// A type-level set of strings (`TString`s).
///
pub struct TStringSet<T>(PhantomData<T>);

impl<T> TStringSet<T>{
    /// Constructs a `TStringSet`.
    ///
    /// # Safety
    ///
    /// `T` must be a tuple of `TString<_>`s,
    /// where no `TString<_>` type is repeated within the tuple.
    pub const unsafe fn new()->Self{
        TStringSet(PhantomData)
    }
}

impl<T> Copy for TStringSet<T>{}
impl<T> Clone for TStringSet<T>{
    fn clone(&self)->Self{
        *self
    }
}

// `MarkerType` is not implemented for `TStringSet` 
// because `TStringSet` ought only be constructible
// by satisfying the safety requirements of `TStringSet::new`,
// which aren't cheaply enforceable on the type level.
//
// impl<T> !MarkerType for TStringSet<T>{}


