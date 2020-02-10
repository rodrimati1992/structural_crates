/*!
Type-level representatins of a chain of field accesses (`FieldPath`),
and multiple field accesses (`FieldPathSet`).
*/

#![allow(non_snake_case, non_camel_case_types)]

use core_extensions::MarkerType;

use std_::{
    fmt::{self, Debug},
    marker::PhantomData,
};

use crate::type_level::_private::TStr_;

use crate::type_level::collection_traits::{
    Append, Append_, PushBack, PushBack_, ToTList, ToTList_, ToTString_,
};

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests;

mod to_usize;

////////////////////////////////////////////////////////////////////////////////

pub mod aliases {
    field_path_aliases! {
        index_0=0,
        index_1=1,
        index_2=2,
        index_3=3,
        index_4=4,
        index_5=5,
        index_6=6,
        index_7=7,
        index_8=8,
    }
}
////////////////////////////////////////////////////////////////////////////////

mod sealed {
    pub trait Sealed {}
}
use self::sealed::Sealed;

impl<T> Sealed for TStr_<T> {}

/// A marker trait for field paths that only refer to one field.
///
/// # Expectations
///
/// This type is expected to implement `RevGetField`,`RevGetFieldMut`,and `RevIntoField`.
pub trait IsSingleFieldPath: Debug + Copy + MarkerType {}

/// A marker trait for field paths that refer to multiple fields
///
/// # Expectations
///
/// This type is expected to implement `RevGetMultiField`,
/// and to only implement `RevGetMultiFieldMut` if and only if `PathUniqueness == UniquePaths`.
pub trait IsMultiFieldPath: Debug + Copy {
    /// Whether the pats in the set can contain duplicate paths.
    type PathUniqueness;
}

////////////////////////////////////////////////////////////////////////////////

/// A marker trait for type-level string.
///
/// This is only implemented on `TStr_<_>`,
/// which is not in the documentation so that its type parameter
/// can be turned into a `const NAME:&'static str` const parameter.
///
/// # TStr construction
///
/// `TStr_<_>` can be constructed with the `NEW` inherent associated constant,
/// or the `<TStr_<_> as MarkerType>::MTVAL` associated constant.
///
/// Examples of constructing a `TStr_<_>`:
///
/// - `<TStr!("hello")>::NEW`
///
/// - `<TStr!(ẁorld)>::NEW`
///
/// - `<TStr!(0)>::NEW`
///
/// - `<TStr!(0)>::MTVAL`(requires importing the `MarkerType` trait)
///
/// # TStr methods
///
/// These are the methods on `TStr_`:
///
/// - `const fn to_path(self) -> FieldPath<(Self,)>`
///
/// - `const fn to_set(self) -> FieldPathSet<(FieldPath<(Self,)>,), UniquePaths>`
///
pub trait IsTStr: Sealed + Debug + Copy + MarkerType {}

impl<T> IsTStr for TStr_<T> {}

impl<T> Debug for TStr_<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TStr_").finish()
    }
}

impl<T> TStr_<T> {
    /// Constructs the TStr_.
    pub const NEW: Self = TStr_(PhantomData);

    #[inline(always)]
    pub const fn to_path(self) -> FieldPath<(TStr_<T>,)> {
        FieldPath::NEW
    }

    #[inline(always)]
    pub const fn to_set(self) -> FieldPathSet<(FieldPath<(TStr_<T>,)>,), UniquePaths> {
        FieldPath::NEW.to_set()
    }
}

impl<T> IsSingleFieldPath for TStr_<T> {}

impl<T> Copy for TStr_<T> {}
impl<T> Clone for TStr_<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl<T> MarkerType for TStr_<T> {}

impl<T> ToTString_ for TStr_<T> {
    type Output = Self;
}

impl_cmp_traits! {
    impl[T] TStr_<T>
    where[]
}

////////////////////////////////////////////////////////////////////////////////

/// A pair of identifiers for the `F` field inside the `V` variant.
pub struct VariantField<V, F>(PhantomData<(V, F)>);

impl<V, F> VariantField<V, F> {
    pub const NEW: Self = VariantField(PhantomData);

    #[inline(always)]
    pub const fn new() -> Self {
        VariantField(PhantomData)
    }
}

impl<V, F> IsSingleFieldPath for VariantField<V, F> {}

impl<V, F> Copy for VariantField<V, F> {}

impl<V, F> Clone for VariantField<V, F> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, U> Debug for VariantField<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariantField").finish()
    }
}

unsafe impl<V, F> MarkerType for VariantField<V, F> {}

/// A FieldPath for the `F` field inside the `V` variant.
pub type VariantFieldPath<V, F> = FieldPath<(VariantField<V, F>,)>;

impl_cmp_traits! {
    impl[V,F] VariantField<V,F>
    where[]
}

////////////////////////////////////////////////////////////////////////////////

/// The identifier for the `V` variant.
pub struct VariantName<V>(PhantomData<V>);

impl<V> VariantName<V> {
    pub const NEW: Self = VariantName(PhantomData);

    #[inline(always)]
    pub const fn new() -> Self {
        VariantName(PhantomData)
    }
}

impl<V> IsSingleFieldPath for VariantName<V> {}

impl<V> Copy for VariantName<V> {}

impl<V> Clone for VariantName<V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Debug for VariantName<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariantName").finish()
    }
}

unsafe impl<V> MarkerType for VariantName<V> {}

impl_cmp_traits! {
    impl[T] VariantName<T>
    where[]
}

////////////////////////////////////////////////////////////////////////////////

/// A marker type passed to accessor trait methods called on an enum,
/// which guarantees that the enum is the variant that `V` represents.
///
/// This is only constructible by VariantProxy.
pub struct UncheckedVariantField<V, F>(PhantomData<(V, F)>);

// MarkerType is intentionally not implemented
// unsafe impl<V, F> !MarkerType for VariantField<V, F>{}

impl<V, F> UncheckedVariantField<V, F> {
    /// Constructs an UncheckedVariantField.
    ///
    /// # Safety
    ///
    /// This must only be passed to an accessor method of an enum with the `V` variant,
    /// eg:you can only soundly pass `UncheckedVariantField::<TStr!(A),TStr!(b)>::new()`
    /// for an enum whose current variant is `A`.
    ///
    /// One example correspondance:
    /// `GetFieldImpl< FP!(::Foo.bar), UncheckedVariantField<TStr!(Foo),TStr!(bar)> >`
    /// corresponds to the
    /// `GetVariantFieldImpl<TStr!(Foo),TStr!(bar)>` unsafe marker trait.
    ///
    /// A `GetVariantFieldImpl` impl guarantees
    /// that the corresponding impl of the `GetFieldImpl` trait does what's expected.
    ///
    pub const unsafe fn new() -> Self {
        UncheckedVariantField(PhantomData)
    }
}

// No UncheckedVariantFieldPath because UncheckedVariantField is not
// going to be part of any `FieldPath`.
// pub type VariantFieldPath<V, F> = FieldPath<(VariantField<V, F>,)>;

impl_cmp_traits! {
    impl[V,F] UncheckedVariantField<V,F>
    where[]
}

////////////////////////////////////////////////////////////////////////////////

/// A type-level representation of a chain of field accesses,like `.a.b.c.d`.
///
pub struct FieldPath<T>(PhantomData<T>);

/// A FieldPath for accesing a single field.
pub type FieldPath1<Str> = FieldPath<(Str,)>;

impl<T> IsSingleFieldPath for FieldPath<T> {}

impl<T> Copy for FieldPath<T> {}
impl<T> Clone for FieldPath<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Debug for FieldPath<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldPath").finish()
    }
}

impl<T> FieldPath<T> {
    pub const NEW: Self = FieldPath(PhantomData);

    #[inline(always)]
    pub const fn new() -> FieldPath<T> {
        FieldPath(PhantomData)
    }
}

unsafe impl<T> MarkerType for FieldPath<T> {}

#[doc(hidden)]
impl<S> ToTString_ for FieldPath<(TStr_<S>,)> {
    type Output = TStr_<S>;
}

impl<T> ToTList_ for FieldPath<T>
where
    T: ToTList_,
{
    type Output = ToTList<T>;
}

#[doc(hidden)]
impl<T, S> PushBack_<TStr_<S>> for FieldPath<T>
where
    T: PushBack_<TStr_<S>>,
{
    type Output = FieldPath<PushBack<T, TStr_<S>>>;
}

impl<T, S> PushBack_<FieldPath<(S,)>> for FieldPath<T>
where
    T: PushBack_<S>,
{
    type Output = FieldPath<PushBack<T, S>>;
}

impl<T, U> Append_<FieldPath<U>> for FieldPath<T>
where
    T: Append_<U>,
{
    type Output = FieldPath<Append<T, U>>;
}

impl<T> FieldPath<T> {
    /// Constructs a new FieldPath with `_other` appended at the end.
    ///
    /// Currently this can only be a single element FieldPath
    /// (ie:`fp!(a)`/`fp!(foo)`/`fp!(bar)`)
    #[inline(always)]
    pub fn push<U, V>(self, _other: U) -> FieldPath<V>
    where
        Self: PushBack_<U, Output = FieldPath<V>>,
    {
        MarkerType::MTVAL
    }

    /// Constructs a new FieldPath with `_other` appended at the end.
    #[inline(always)]
    pub fn append<U>(self, _other: FieldPath<U>) -> FieldPath<Append<T, U>>
    where
        T: Append_<U>,
    {
        MarkerType::MTVAL
    }

    /// Converts this `FieldPath` to a `FieldPathSet`.
    #[inline(always)]
    pub const fn to_set(self) -> FieldPathSet<(Self,), UniquePaths> {
        unsafe { FieldPathSet::new_unchecked() }
    }
}

impl<S> FieldPath<(TStr_<S>,)> {
    #[doc(hidden)]
    pub const fn to_tstr(self) -> TStr_<S> {
        MarkerType::MTVAL
    }
}

impl_cmp_traits! {
    impl[T] FieldPath<T>
    where[]
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
pub struct FieldPathSet<T, U>(PhantomData<(T, U)>);

/// A merker type indicating that FieldPathSet contains unique paths,
/// in which no path is a prefix of any other path in the set,
/// this is required to call `GetFieldExt::fields_mut`.
#[derive(Debug, Copy, Clone)]
pub struct UniquePaths;

/// A merker type indicating that FieldPathSet may not contain unique `FielsPath`s,
/// which means that its not safe to pass the FieldPathSet to `GetFieldExt::fields_mut`
/// (this is why it requires `FieldPathSet<_,UniquePaths>`).
#[derive(Debug, Copy, Clone)]
pub struct AliasedPaths;

impl<T, U> IsMultiFieldPath for FieldPathSet<T, U> {
    type PathUniqueness = U;
}

impl<T, U> Copy for FieldPathSet<T, U> {}

impl<T, U> Clone for FieldPathSet<T, U> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, U> Debug for FieldPathSet<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldPathSet").finish()
    }
}

// `MarkerType` is not implemented for `FieldPathSet<T.UniquePaths>`
// because `FieldPathSet<T.UniquePaths>` ought only be constructible
// by satisfying the safety requirements of `FieldPathSet::<T.UniquePaths>::new`,
// which aren't cheaply enforceable on the type level.
//
// impl<T> !MarkerType for FieldPathSet<T.UniquePaths>{}

unsafe impl<T> MarkerType for FieldPathSet<T, AliasedPaths> {}

impl<T, U> FieldPathSet<T, U> {
    // The constructor function used by proc macros,
    #[doc(hidden)]
    #[inline(always)]
    pub const unsafe fn new_unchecked() -> Self {
        FieldPathSet(PhantomData)
    }
}
impl<T> FieldPathSet<T, UniquePaths> {
    /// Constructs a `FieldPathSet`.
    ///
    /// # Safety
    ///
    /// `T` must be a tuple of `FieldPaths<_>`s,
    /// where none of them is a subset of each other.
    #[inline(always)]
    pub const unsafe fn new() -> Self {
        FieldPathSet(PhantomData)
    }

    /// Converts a `FieldPathSet<T,UniquePaths>` to a `FieldPathSet<T,AliasedPaths>`
    #[inline(always)]
    pub const fn downgrade(self) -> FieldPathSet<T, AliasedPaths> {
        FieldPathSet(PhantomData)
    }
}

impl<T> FieldPathSet<T, AliasedPaths> {
    /// Constructs a `FieldPathSet`.
    #[inline(always)]
    pub const fn new() -> Self {
        FieldPathSet(PhantomData)
    }

    /// Constructs a `FieldPathSet`.
    pub const NEW: Self = FieldPathSet(PhantomData);

    /// Converts a `FieldPathSet<T,AliasedPaths>` to a `FieldPathSet<T,UniquePaths>`
    ///
    /// # Safety
    ///
    /// You must ensure that all the `FieldPath`s are unique,
    /// there must be no `FieldPath` that is a prefix of any other `FieldPath`.
    #[inline(always)]
    pub const unsafe fn upgrade_unchecked(self) -> FieldPathSet<T, UniquePaths> {
        FieldPathSet(PhantomData)
    }
}

impl<T, U> FieldPathSet<(FieldPath<T>,), U> {
    /// Converts a `FieldPathSet` containing a single `FieldPath`
    /// into that `FieldPath`.
    #[inline(always)]
    pub const fn to_path(self) -> FieldPath<T> {
        MarkerType::MTVAL
    }
}

impl<T, U> FieldPathSet<T, U> {
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
    pub fn push<O, Out>(self, _other: O) -> FieldPathSet<Out, AliasedPaths>
    where
        Self: PushBack_<O, Output = FieldPathSet<Out, AliasedPaths>>,
    {
        MarkerType::MTVAL
    }

    /// Constructs a new FieldPathSet with the `_other` FieldPathSet
    /// appended at the end.
    #[inline(always)]
    pub fn append<T2, U2>(
        self,
        _other: FieldPathSet<T2, U2>,
    ) -> FieldPathSet<Append<T, T2>, AliasedPaths>
    where
        T: Append_<T2>,
    {
        MarkerType::MTVAL
    }
}

impl<T, U> ToTList_ for FieldPathSet<T, U>
where
    T: ToTList_,
{
    type Output = ToTList<T>;
}

impl<T, U, P> PushBack_<FieldPath<P>> for FieldPathSet<T, U>
where
    T: PushBack_<FieldPath<P>>,
{
    type Output = FieldPathSet<PushBack<T, FieldPath<P>>, AliasedPaths>;
}

impl<T, U, P, U2> PushBack_<FieldPathSet<(P,), U2>> for FieldPathSet<T, U>
where
    T: PushBack_<P>,
{
    type Output = FieldPathSet<PushBack<T, P>, AliasedPaths>;
}

impl<T, T2, U, U2> Append_<FieldPathSet<T2, U2>> for FieldPathSet<T, U>
where
    T: Append_<T2>,
{
    type Output = FieldPathSet<Append<T, T2>, AliasedPaths>;
}

impl_cmp_traits! {
    impl[T,U] FieldPathSet<T,U>
    where[]
}

////////////////////////////////////////////////////////////////////////////////

/// Allows accessing multiple fields inside of some nested field.
///
/// This is useful for accessing multiple fields inside of an optional one,
/// including accessing the fields in an enum variant.
pub struct NestedFieldPathSet<F, S, U> {
    pub path: FieldPath<F>,
    pub path_set: FieldPathSet<S, U>,
}

impl<F, S> NestedFieldPathSet<F, S, AliasedPaths> {
    pub const NEW: Self = Self {
        path: FieldPath::NEW,
        path_set: FieldPathSet::NEW,
    };

    /// Constructs a `NestedFieldPathSet`.
    #[inline(always)]
    pub const fn new() -> Self {
        Self::NEW
    }
}

impl<F, S, U> NestedFieldPathSet<F, S, U> {
    /// Constructs a `NestedFieldPathSet`.
    #[inline(always)]
    #[doc(hidden)]
    pub const unsafe fn new_unchecked() -> Self {
        Self {
            path: FieldPath::NEW,
            path_set: FieldPathSet::new_unchecked(),
        }
    }
}

impl<F, S> NestedFieldPathSet<F, S, UniquePaths> {
    /// Constructs a `NestedFieldPathSet`.
    ///
    /// # Safety
    ///
    /// `S` must be a tuple of `FieldPaths<_>`s,
    /// where none of them is a subset of each other.
    #[inline(always)]
    pub const unsafe fn new() -> Self {
        Self::new_unchecked()
    }
}

impl<F, S, U> IsMultiFieldPath for NestedFieldPathSet<F, S, U> {
    type PathUniqueness = U;
}

unsafe impl<F, S> MarkerType for NestedFieldPathSet<F, S, AliasedPaths> {}

impl<F, S, U> Debug for NestedFieldPathSet<F, S, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NestedFieldPathSet").finish()
    }
}

impl<F, S, U> Copy for NestedFieldPathSet<F, S, U> {}

impl<F, S, U> Clone for NestedFieldPathSet<F, S, U> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
impl<S> From<FieldPath<(TStr_<S>,)>> for TStr_<S> {
    #[inline(always)]
    fn from(_this: FieldPath<(TStr_<S>,)>) -> Self {
        MarkerType::MTVAL
    }
}

#[doc(hidden)]
impl<S> From<TStr_<S>> for FieldPath<(TStr_<S>,)> {
    #[inline(always)]
    fn from(_this: TStr_<S>) -> Self {
        MarkerType::MTVAL
    }
}
impl<T, U> From<FieldPathSet<(FieldPath<T>,), U>> for FieldPath<T> {
    #[inline(always)]
    fn from(_this: FieldPathSet<(FieldPath<T>,), U>) -> Self {
        MarkerType::MTVAL
    }
}

impl<P> From<FieldPath<P>> for FieldPathSet<(FieldPath<P>,), UniquePaths> {
    #[inline(always)]
    fn from(this: FieldPath<P>) -> Self {
        this.to_set()
    }
}

////////////////////////////////////////////////////////////////////////////////
