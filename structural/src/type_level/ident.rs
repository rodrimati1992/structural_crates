/*!
Contains type-level strings,and multi-strings.
*/

#![allow(non_snake_case, non_camel_case_types)]

use core_extensions::MarkerType;

use std_::marker::PhantomData;

use crate::type_level::collection_traits::{
    Append,Append_,
    PushBack,PushBack_,
    ToTList,ToTList_,
    ToTString_,
};


////////////////////////////////////////////////////////////////////////////////

mod sealed{
    pub trait Sealed{}
}
use self::sealed::Sealed;

impl<T> Sealed for TString<T>{}
impl<T> Sealed for FieldPath<T>{}
impl<T,U> Sealed for FieldPathSet<T,U>{}


/// A marker trait only implemented by FieldPath
pub trait IsFieldPath:Sealed+Copy{}

impl<T> IsFieldPath for FieldPath<T>{}


/// A marker trait only implemented by FieldPathSet
pub trait IsFieldPathSet:Sealed+Copy{
    /// Whether the pats in the set can contain duplicate paths.
    type PathUniqueness;
}

impl<T,U> IsFieldPathSet for FieldPathSet<T,U>{
    type PathUniqueness=U;
}


////////////////////////////////////////////////////////////////////////////////

/// A type-level string,represented as a tuple of type-level bytes.
///
/// This is an implementation detail of structural,
/// so that it's possible to replace it with `pub struct TString<const NAME:&'static str>`
///
/// This cannot be converted to a `&'static str` constant
/// (if you can figure out a cheap way to do that please create an issue/pull request).
///
#[doc(hidden)]
pub struct TString<T>(PhantomData<T>);

impl<T> TString<T>{
    /// Constructs the TString.
    pub const NEW:Self=TString(PhantomData);

    #[inline(always)]
    pub const fn to_path(self)->FieldPath<(TString<T>,)>{
        FieldPath::NEW
    }

    #[inline(always)]
    pub const fn to_set(self)->FieldPathSet<(FieldPath<(TString<T>,)>,),UniquePaths>{
        FieldPath::NEW.to_set()
    }
}

impl<T> Copy for TString<T>{}
impl<T> Clone for TString<T>{
    #[inline(always)]
    fn clone(&self)->Self{
        *self
    }
}
unsafe impl<T> MarkerType for TString<T>{}


impl<T> ToTString_ for TString<T>{
    type Output=Self;
}




////////////////////////////////////////////////////////////////////////////////


/// U type-level representation of a chain of field accesses,like `.a.b.c.d`.
///
pub struct FieldPath<T>(PhantomData<T>);

/// A FieldPath for accesing a single `Str` field.
#[doc(hidden)]
pub type FieldPath1<Str>=FieldPath<(Str,)>;


/// A FieldPath constructed in the same way that TString is.
#[doc(hidden)]
pub type FieldPathString<Str>=FieldPath<(TString<Str>,)>;

impl<T> Copy for FieldPath<T>{}
impl<T> Clone for FieldPath<T>{
    #[inline(always)]
    fn clone(&self)->Self{
        *self
    }
}

impl<T> FieldPath<T>{
    pub const NEW:Self=FieldPath(PhantomData);

    #[inline(always)]
    pub const fn new()->FieldPath<T>{
        FieldPath(PhantomData)
    }
}

unsafe impl<T> MarkerType for FieldPath<T>{}

#[doc(hidden)]
impl<S> ToTString_ for FieldPath<(TString<S>,)>{
    type Output=TString<S>;
}

impl<T> ToTList_ for FieldPath<T>
where
    T:ToTList_
{
    type Output=ToTList<T>;
}

#[doc(hidden)]
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
    /// Constructs a new FieldPath with `_other` appended at the end.
    ///
    /// Currently this can only be a single element FieldPath
    /// (ie:`fp!(a)`/`fp!(foo)`/`fp!(bar)`)
    #[inline(always)]
    pub fn push<U,V>(self,_other:U)->FieldPath<V>
    where
        Self:PushBack_<U,Output=FieldPath<V>>
    {
        MarkerType::MTVAL
    }

    /// Constructs a new FieldPath with `_other` appended at the end.
    #[inline(always)]
    pub fn append<U>(self,_other:FieldPath<U>)->FieldPath<Append<T,U>>
    where
        T:Append_<U>
    {
        MarkerType::MTVAL
    }

    /// Converts this `FieldPath` to a `FieldPathSet`.
    #[inline(always)]
    pub const fn to_set(self)->FieldPathSet<(Self,),UniquePaths>{
        unsafe{
            FieldPathSet::new_unchecked()
        }
    }
}

impl<S> FieldPath<(TString<S>,)>{
    #[doc(hidden)]
    pub const fn to_tstr(self)->TString<S>{
        MarkerType::MTVAL
    }
}


////////////////////////////////////////////////////////////////////////////////

/// A list of `FieldPath`s whose uniqueness is determined by `U`.
///
/// If `U=UniquePaths` then all the `FieldPath`s are unique,
/// and this can be passed to `GetFieldExt::fields_mut`,
/// since you can't have aliasing mutable references to the same field.
///
/// If `U=AliasedPaths` then there might be repeated `FieldPath`s,
/// and this cannot be passed to `GetFieldExt::fields_mut`,
/// because it might borrow the same field mutably twice.
///
pub struct FieldPathSet<T,U>(PhantomData<(T,U)>);


/// A merker type indicating that FieldPathSet contains unique paths,
/// in which no path is a prefix of any other path in the set,
/// this is required to call `GetFieldExt::fields_mut`.
pub struct UniquePaths;

/// A merker type indicating that FieldPathSet may not contain unique `FielsPath`s,
/// which means that its not safe to pass the FieldPathSet to `GetFieldExt::fields_mut`
/// (this is why it requires `FieldPathSet<_,UniquePaths>`).
pub struct AliasedPaths;


impl<T,U> Copy for FieldPathSet<T,U>{}

impl<T,U> Clone for FieldPathSet<T,U>{
    #[inline(always)]
    fn clone(&self)->Self{
        *self
    }
}

// `MarkerType` is not implemented for `FieldPathSet<T.UniquePaths>` 
// because `FieldPathSet<T.UniquePaths>` ought only be constructible
// by satisfying the safety requirements of `FieldPathSet::<T.UniquePaths>::new`,
// which aren't cheaply enforceable on the type level.
//
// impl<T> !MarkerType for FieldPathSet<T.UniquePaths>{}

unsafe impl<T> MarkerType for FieldPathSet<T,AliasedPaths>{}

impl<T,U> FieldPathSet<T,U>{
    // The constructor function used by proc macros,
    #[doc(hidden)]
    #[inline(always)]
    pub const unsafe fn new_unchecked()->Self{
        FieldPathSet(PhantomData)
    }
}
impl<T> FieldPathSet<T,UniquePaths>{
    /// Constructs a `FieldPathSet`.
    ///
    /// # Safety
    ///
    /// `T` must be a tuple of `FieldPaths<_>`s,
    /// where none of them is a subset of each other.
    #[inline(always)]
    pub const unsafe fn new()->Self{
        FieldPathSet(PhantomData)
    }

    /// Converts a `FieldPathSet<T,UniquePaths>` to a `FieldPathSet<T,AliasedPaths>`
    #[inline(always)]
    pub const fn downgrade(self)->FieldPathSet<T,AliasedPaths>{
        FieldPathSet(PhantomData)
    }
}

impl<T> FieldPathSet<T,AliasedPaths>{
    /// Constructs a `FieldPathSet`.
    #[inline(always)]
    pub const fn new()->Self{
        FieldPathSet(PhantomData)
    }


    /// Converts a `FieldPathSet<T,AliasedPaths>` to a `FieldPathSet<T,UniquePaths>`
    ///
    /// # Safety
    ///
    /// You must ensure that all the `FieldPath`s are unique,
    /// there must be no `FieldPath` that is a prefix of any other `FieldPath`.
    #[inline(always)]
    pub const unsafe fn upgrade_unchecked(self)->FieldPathSet<T,UniquePaths>{
        FieldPathSet(PhantomData)
    }
}

impl<T,U> FieldPathSet<(FieldPath<T>,),U> {
    /// Converts a `FieldPathSet` containing a single `FieldPath` 
    /// into that `FieldPath`.
    #[inline(always)]
    pub const fn to_path(self)->FieldPath<T>{
        MarkerType::MTVAL
    }
}


impl<T,U> FieldPathSet<T,U>{
    /// Constructs a new FieldPathSet with `_other` appended at the end.
    ///
    /// Currently this accepts:
    ///
    /// - A FieldPath
    /// (ie:`fp!(a)`/`fp!(foo)`/`fp!(bar)`)
    ///
    /// - A FieldPathSet containing a single FieldPath
    /// (ie:`fp!(a).to_set()`/`fp!(foo).to_set()`/`fp!(bar).to_set()`)
    #[inline(always)]
    pub fn push<O,Out>(self,_other:O)->FieldPathSet<Out,AliasedPaths>
    where
        Self:PushBack_<O,Output=FieldPathSet<Out,AliasedPaths>>
    {
        MarkerType::MTVAL
    }

    /// Constructs a new FieldPathSet with the `_other` FieldPathSet
    /// appended at the end.
    #[inline(always)]
    pub fn append<T2,U2>(
        self,
        _other:FieldPathSet<T2,U2>
    )->FieldPathSet<Append<T,T2>,AliasedPaths>
    where
        T:Append_<T2>
    {
        MarkerType::MTVAL
    }
}


impl<T,U> ToTList_ for FieldPathSet<T,U>
where
    T:ToTList_
{
    type Output=ToTList<T>;
}

impl<T,U,P> PushBack_<FieldPath<P>> for FieldPathSet<T,U>
where
    T:PushBack_<FieldPath<P>>
{
    type Output=FieldPathSet<PushBack<T,FieldPath<P>>,AliasedPaths>;
}


impl<T,U,P,U2> PushBack_<FieldPathSet<(P,),U2>> for FieldPathSet<T,U>
where
    T:PushBack_<P>
{
    type Output=FieldPathSet<PushBack<T,P>,AliasedPaths>;
}

impl<T,T2,U,U2> Append_<FieldPathSet<T2,U2>> for FieldPathSet<T,U>
where
    T:Append_<T2>
{
    type Output=FieldPathSet<Append<T,T2>,AliasedPaths>;
}



////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
impl<S> From<FieldPath<(TString<S>,)>> for TString<S>{
    #[inline(always)]
    fn from(_this:FieldPath<(TString<S>,)>)->Self{
        MarkerType::MTVAL
    }
}

#[doc(hidden)]
impl<S> From<TString<S>> for FieldPath<(TString<S>,)>{
    #[inline(always)]
    fn from(_this:TString<S>)->Self{
        MarkerType::MTVAL
    }
}
impl<T,U> From<FieldPathSet<(FieldPath<T>,),U>> for FieldPath<T>{
    #[inline(always)]
    fn from(_this:FieldPathSet<(FieldPath<T>,),U>)->Self{
        MarkerType::MTVAL
    }
}

impl<P> From<FieldPath<P>> for FieldPathSet<(FieldPath<P>,),UniquePaths>{
    #[inline(always)]
    fn from(this:FieldPath<P>)->Self{
        this.to_set()
    }
}



////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests;