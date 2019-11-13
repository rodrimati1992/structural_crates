/*!
Contains type-level strings,and multi-strings.
*/

#![allow(non_snake_case, non_camel_case_types)]

use core_extensions::MarkerType;

use std_::marker::PhantomData;

pub use crate::chars::*;
pub use crate::type_level::collection_traits::{
    Append,Append_,
    PushBack,PushBack_,
};


////////////////////////////////////////////////////////////////////////////////

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
    #[inline(always)]
    fn clone(&self)->Self{
        *self
    }
}
unsafe impl<T> MarkerType for TString<T>{}


////////////////////////////////////////////////////////////////////////////////


/// A type-level representation of a chain of field accesses,like `.a.b.c.d`.
///
pub struct FieldPath<T>(PhantomData<T>);

impl<T> Copy for FieldPath<T>{}
impl<T> Clone for FieldPath<T>{
    fn clone(&self)->Self{
        *self
    }
}

impl<T> FieldPath<T>{
    pub const NEW:Self=FieldPath(PhantomData);

    pub const fn new()->FieldPath<T>{
        FieldPath(PhantomData)
    }
}

unsafe impl<T> MarkerType for FieldPath<T>{}


impl<T,S> PushBack_<TString<S>> for FieldPath<T>
where
    T:PushBack_<TString<S>>
{
    type Output=FieldPath<PushBack<T,TString<S>>>;
}

impl<T,S> PushBack_<FieldPath<(S,)>> for FieldPath<T>
where
    T:PushBack_<S>
{
    type Output=FieldPath<PushBack<T,S>>;
}


impl<T,U> Append_<FieldPath<U>> for FieldPath<T>
where
    T:Append_<U>
{
    type Output=FieldPath<Append<T,U>>;
}


impl<T> FieldPath<T>{
    #[inline(always)]
    pub fn push<U,O>(self,_other:U)->FieldPath<O>
    where
        Self:PushBack_<U,Output=FieldPath<O>>
    {
        MarkerType::MTVAL
    }

    #[inline(always)]
    pub fn append<U>(self,_other:FieldPath<U>)->FieldPath<Append<T,U>>
    where
        T:Append_<U>
    {
        MarkerType::MTVAL
    }
}


////////////////////////////////////////////////////////////////////////////////

pub struct FieldPaths<T,A>(PhantomData<(T,A)>);

pub struct MutableAccess;
pub struct SharedAccess;


impl<T> FieldPaths<T,MutableAccess>{
    /// Constructs a `FieldPaths`.
    ///
    /// # Safety
    ///
    /// `T` must be a tuple of `TString<_>`s,
    /// where no `TString<_>` type is repeated within the tuple.
    #[inline(always)]
    pub const unsafe fn new()->Self{
        FieldPaths(PhantomData)
    }
}

impl<T> FieldPaths<T,SharedAccess>{
    /// Constructs a `FieldPaths`.
    #[inline(always)]
    pub const fn new()->Self{
        FieldPaths(PhantomData)
    }
}

impl<T,A> From<FieldPaths<(FieldPath<T>,),A>> for FieldPath<T>{
    #[inline(always)]
    fn from(_this:FieldPaths<(FieldPath<T>,),A>)->Self{
        MarkerType::MTVAL
    }
}

impl<T,A> FieldPaths<(FieldPath<T>,),A> {
    #[inline(always)]
    pub fn to_path(self)->FieldPath<T>{
        MarkerType::MTVAL
    }
}


impl<T,A> FieldPaths<T,A>{
    #[inline(always)]
    pub fn push<U,O>(self,_other:U)->FieldPaths<O,SharedAccess>
    where
        Self:PushBack_<U,Output=FieldPaths<O,SharedAccess>>
    {
        MarkerType::MTVAL
    }

    #[inline(always)]
    pub fn append<U,A2>(self,_other:FieldPaths<U,A2>)->FieldPaths<Append<T,U>,SharedAccess>
    where
        T:Append_<U>
    {
        MarkerType::MTVAL
    }
}


impl<T,Access> Copy for FieldPaths<T,Access>{}

impl<T,Access> Clone for FieldPaths<T,Access>{
    #[inline(always)]
    fn clone(&self)->Self{
        *self
    }
}

// `MarkerType` is not implemented for `FieldPaths<T.MutableAccess>` 
// because `FieldPaths<T.MutableAccess>` ought only be constructible
// by satisfying the safety requirements of `FieldPaths::<T.MutableAccess>::new`,
// which aren't cheaply enforceable on the type level.
//
// impl<T> !MarkerType for FieldPaths<T.MutableAccess>{}

unsafe impl<T> MarkerType for FieldPaths<T,SharedAccess>{}


impl<T,A,P> PushBack_<FieldPath<P>> for FieldPaths<T,A>
where
    T:PushBack_<FieldPath<P>>
{
    type Output=FieldPaths<PushBack<T,FieldPath<P>>,SharedAccess>;
}


impl<T,A,P,A2> PushBack_<FieldPaths<(P,),A2>> for FieldPaths<T,A>
where
    T:PushBack_<P>
{
    type Output=FieldPaths<PushBack<T,P>,SharedAccess>;
}

impl<T,T2,A,A2> Append_<FieldPaths<T2,A2>> for FieldPaths<T,A>
where
    T:Append_<T2>
{
    type Output=FieldPaths<Append<T,T2>,SharedAccess>;
}



////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests;