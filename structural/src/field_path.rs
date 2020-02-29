/*!
Type-level representatins of access to one or multiple fields.
*/

#![allow(non_snake_case, non_camel_case_types)]

use crate::type_level::collection_traits::{
    Append, Append_, PushBack, PushBack_, ToTList, ToTList_, ToTString_,
};

use core_extensions::ConstDefault;

use std_::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem::ManuallyDrop,
};

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests;

mod to_usize;

mod path_components;

pub use self::path_components::*;

#[doc(inline)]
pub use crate::*;

////////////////////////////////////////////////////////////////////////////////

/// Aliases for field paths.
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

/// Aliases for TStr.
pub mod string_aliases {
    tstr_aliases! {
        str_0=0,
        str_1=1,
        str_2=2,
        str_3=3,
        str_4=4,
        str_5=5,
        str_6=6,
        str_7=7,
        str_8=8,
        str_9=9,
        str_underscore="_",
    }
}

////////////////////////////////////////////////////////////////////////////////

mod sealed {
    pub trait Sealed {}
}
use self::sealed::Sealed;

impl<T> Sealed for TStr<T> {}

/// A marker trait for field paths that only refer to one field.
///
/// # Expectations
///
/// This type is expected to implement `RevGetFieldImpl`,`RevGetFieldMutImpl`,and `RevIntoFieldImpl`.
pub trait IsSingleFieldPath: Sized {}

/// A marker trait for field paths that refer to multiple fields
///
/// # Expectations
///
/// This type is expected to implement `RevGetMultiField`,
/// and to only implement `RevGetMultiFieldMut` if and only if `PathUniqueness == UniquePaths`.
pub trait IsMultiFieldPath: Sized {
    /// Whether the paths in the set can contain duplicate paths.
    ///
    /// This is expected to be either:
    ///
    /// - `structural::field_path::AliasedPaths`:
    /// for a field path that might refer to the same field multiple times.
    ///
    /// - `structural::field_path::UniquePaths`:
    /// for a field path that only refers to a field once.
    ///
    type PathUniqueness;
}

////////////////////////////////////////////////////////////////////////////////

impl<T> IsSingleFieldPath for FieldPath<T> {}

impl<T> IsMultiFieldPath for FieldPath<T> {
    type PathUniqueness = UniquePaths;
}

impl<T> Debug for FieldPath<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldPath").finish()
    }
}

impl<T> FieldPath<T>
where
    T: ConstDefault,
{
    /// Constructs a `FieldPath<T>`
    pub const NEW: Self = Self::DEFAULT;
}

impl<T> FieldPath<(T,)> {
    /// Construcst a field path from a single path component.
    ///
    /// Example: `FieldPath::one(ts!(a))` is equivalent to `fp!(a)`
    ///
    /// Example:
    /// `FieldPath::one(VariantField::new(ts!(a),ts!(b)))`
    /// is equivalent to `fp!(::a.b)`
    ///
    /// Example:
    /// `FieldPath::one(VariantName::new(ts!(Left)))`
    /// is equivalent to `fp!(::Left)`
    #[inline(always)]
    pub const fn one(value: T) -> Self {
        Self { list: (value,) }
    }
}

impl<T> FieldPath<T> {
    /// Constructs a field path for a nested field.
    ///
    /// Example:
    /// `FieldPath::many(( ts!(a), ts!(b) ))`
    /// is equivalent to `fp!(a.b)`
    ///
    /// Example:
    /// `FieldPath::many(( VariantField::new(ts!(A), ts!(b)), ts!(c) ))`
    /// is equivalent to `fp!(::A.b.c)`
    #[inline(always)]
    pub const fn many(list: T) -> Self {
        Self { list }
    }
}

impl<T> ConstDefault for FieldPath<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = FieldPath {
        list: ConstDefault::DEFAULT,
    };
}

impl<S> ToTString_ for FieldPath<(TStr<S>,)> {
    type Output = TStr<S>;
}

impl<T> ToTList_ for FieldPath<T>
where
    T: ToTList_,
{
    type Output = ToTList<T>;
}

impl<T, S> PushBack_<S> for FieldPath<T>
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
    /// Example arguments:`fp!(a)`/`fp!(foo)`/`fp!(bar)`
    #[inline(always)]
    pub fn push<U, V>(self, _other: U) -> FieldPath<V>
    where
        Self: PushBack_<U, Output = FieldPath<V>>,
        FieldPath<V>: ConstDefault,
    {
        ConstDefault::DEFAULT
    }

    /// Constructs a new FieldPath with `_other` appended at the end.
    #[inline(always)]
    pub fn append<U>(self, _other: FieldPath<U>) -> FieldPath<Append<T, U>>
    where
        T: Append_<U>,
        FieldPath<Append<T, U>>: ConstDefault,
    {
        ConstDefault::DEFAULT
    }

    /// Converts this `FieldPath` to a `FieldPathSet`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use structural::{GetFieldExt, fp};
    ///
    /// let tup=(3,(5,8),(13,21));
    ///
    /// assert_eq!( tup.fields(fp!(2.0).into_set()), (&13,) );
    ///
    /// ```
    #[inline(always)]
    pub const fn into_set(self) -> FieldPathSet<(Self,), UniquePaths> {
        FieldPathSet::one(self)
    }
}

impl<C> FieldPath<(C,)> {
    /// Unwraps this non-nested field path into `C`.
    ///
    /// This can also be done with `path.list.0`.
    pub fn into_component(self) -> C {
        self.list.0
    }
}

impl_cmp_traits! {
    impl[T] FieldPath<T>
    where[]
}

////////////////////////////////////////////////////////////////////////////////

/// A merker type indicating that FieldPathSet contains unique paths,
/// in which no path is a prefix of any other path in the set,
/// this is required to call `GetFieldExt::fields_mut`.
#[derive(Debug, Copy, Clone)]
pub struct UniquePaths;

/// A merker type indicating that FieldPathSet may not contain unique `FielsPath`s,
/// which means that its possible to pass a `FieldPathSet<__,AliasedPaths>` to
/// `GetFieldExt::fields_mut`.
#[derive(Debug, Copy, Clone)]
pub struct AliasedPaths;

impl<T, U> IsMultiFieldPath for FieldPathSet<T, U> {
    type PathUniqueness = U;
}

// `ConstDefault` is not implemented for `FieldPathSet<T.UniquePaths>`
// because `FieldPathSet<T.UniquePaths>` ought only be constructible
// by satisfying the safety requirements of `FieldPathSet::<T.UniquePaths>::new`,
// which aren't cheaply enforceable on the type level.
//
// impl<T> !ConstDefault for FieldPathSet<T.UniquePaths>{}

impl<T> ConstDefault for FieldPathSet<T, AliasedPaths>
where
    T: ConstDefault,
{
    const DEFAULT: Self = FieldPathSet {
        paths: ManuallyDrop::new(ConstDefault::DEFAULT),
        uniqueness: PhantomData,
    };
}

impl<T> Default for FieldPathSet<T, AliasedPaths>
where
    T: Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::many(T::default())
    }
}

impl<T> FieldPathSet<(T,), UniquePaths> {
    /// Constructs a FieldPathSet from a single field path.
    pub const fn one(val: T) -> Self {
        FieldPathSet {
            paths: ManuallyDrop::new((val,)),
            uniqueness: PhantomData,
        }
    }
}

impl<T> FieldPathSet<T, AliasedPaths> {
    /// Constructs a FieldPathSet from a tuple of field paths.
    ///
    /// Note that this doesn't enforce that its input is in fact a tuple of FieldPath,
    /// so you can use type inference for the arguments to this function.
    ///
    /// To be able to access multiple fields mutably at the same time,
    /// must call the unsafe `.upgrade()` method.
    pub const fn many(paths: T) -> Self {
        FieldPathSet {
            paths: ManuallyDrop::new(paths),
            uniqueness: PhantomData,
        }
    }
}

impl<T> FieldPathSet<T, AliasedPaths>
where
    T: ConstDefault,
{
    /// Constructs a `FieldPathSet`.
    ///
    /// This can also be used to construct a `FieldPathSet<T, UniquePaths>`
    /// in a const context where `T` can be inferred,
    /// by doing `unsafe{ FieldPathSet::NEW.upgrade_unchecked() }`
    /// (read the docs for `upgrade_unchecked` first).
    pub const NEW: Self = Self::DEFAULT;
}

impl<T, U> FieldPathSet<T, U>
where
    T: ConstDefault,
{
    /// This can be used to construct a `FieldPathSet<T, UniquePaths>`
    /// from a type alias in a const context,
    /// by doing `unsafe{ FOO::NEW_ALIASED.upgrade_unchecked() }`
    /// (read the docs for `upgrade_unchecked` first).
    pub const NEW_ALIASED: FieldPathSet<T, AliasedPaths> = FieldPathSet::NEW;
}

impl<T> FieldPathSet<T, UniquePaths> {
    /// Converts a `FieldPathSet<T,UniquePaths>` to a `FieldPathSet<T,AliasedPaths>`
    #[inline(always)]
    pub const fn downgrade(self) -> FieldPathSet<T, AliasedPaths> {
        FieldPathSet {
            paths: self.paths,
            uniqueness: PhantomData,
        }
    }
}

impl<T> FieldPathSet<T, AliasedPaths> {
    /// Converts a `FieldPathSet<T,AliasedPaths>` to a `FieldPathSet<T,UniquePaths>`
    ///
    /// # Safety
    ///
    /// You must ensure that all the `FieldPath`s are unique,
    /// there must be no field path that is a prefix of any other `FieldPath`.
    #[inline(always)]
    pub const unsafe fn upgrade_unchecked(self) -> FieldPathSet<T, UniquePaths> {
        self.set_uniqueness()
    }

    /// Converts a `FieldPathSet<T,AliasedPaths>` to a `FieldPathSet<T,U>`
    ///
    /// # Safety
    ///
    /// You must ensure that if `U==UniquePaths`,then all the `FieldPath`s are unique,
    /// there must be no field path that is a prefix of any other `FieldPath`.
    #[inline(always)]
    pub const unsafe fn set_uniqueness<U>(self) -> FieldPathSet<T, U> {
        FieldPathSet {
            paths: self.paths,
            uniqueness: PhantomData,
        }
    }
}
impl<T, U> FieldPathSet<T, U> {
    /// Gets the tuple of field paths out of this FieldPathSet.
    #[inline(always)]
    pub const fn into_paths(self) -> T {
        ManuallyDrop::into_inner(self.paths)
    }
}

impl<T, U> FieldPathSet<(T,), U> {
    /// Converts a `FieldPathSet` containing a single field path into that field path.
    #[inline(always)]
    pub fn into_path(self) -> T {
        ManuallyDrop::into_inner(self.paths).0
    }
}

impl<T, U> FieldPathSet<T, U> {
    /// Constructs a new FieldPathSet with `_other` appended at the end.
    ///
    /// Example arguments`fp!(a)`/`fp!(a.b.c)`/`fp!(::foo)`/`fp!(::bar.baz.bam)`
    #[inline(always)]
    pub fn push<O, Out>(self, _other: O) -> FieldPathSet<Out, AliasedPaths>
    where
        Self: PushBack_<O, Output = FieldPathSet<Out, AliasedPaths>>,
        FieldPathSet<Out, AliasedPaths>: ConstDefault,
    {
        ConstDefault::DEFAULT
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
        FieldPathSet<Append<T, T2>, AliasedPaths>: ConstDefault,
    {
        ConstDefault::DEFAULT
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

impl<F, S> NestedFieldPathSet<F, S, AliasedPaths>
where
    F: ConstDefault,
    S: ConstDefault,
{
    /// Constructs a `NestedFieldPathSet`.
    pub const NEW: Self = Self::DEFAULT;
}

impl<F, S, U> NestedFieldPathSet<F, S, U>
where
    F: ConstDefault,
    S: ConstDefault,
{
    /// This can be used to construct a `NestedFieldPathSet<T, UniquePaths>`
    /// from a type alias in a const context,
    /// by doing `unsafe{ FOO::NEW_ALIASED.upgrade_unchecked() }`
    /// (read the docs for `upgrade_unchecked` first).
    pub const NEW_ALIASED: NestedFieldPathSet<F, S, AliasedPaths> = NestedFieldPathSet::NEW;
}

impl<F, S> Default for NestedFieldPathSet<F, S, AliasedPaths>
where
    F: Default,
    S: Default,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<F, S, U> NestedFieldPathSet<F, S, U> {
    /// Constructs a `NestedFieldPathSet` from an `F` and a `FieldPathSet`
    pub const fn new(nested: F, set: FieldPathSet<S, U>) -> Self {
        Self {
            nested: ManuallyDrop::new(nested),
            set,
        }
    }

    /// Unwraps a `NestedFieldPathSet` into a `FieldPath` and a `FieldPathSet`
    pub const fn into_inner(self) -> (F, FieldPathSet<S, U>) {
        (ManuallyDrop::into_inner(self.nested), self.set)
    }

    /// Unwraps a `NestedFieldPathSet` into the `FieldPath` for the nested field.
    pub const fn into_nested(self) -> F {
        ManuallyDrop::into_inner(self.nested)
    }

    /// Unwraps a `NestedFieldPathSet` into the `FieldPathSet` used to
    /// access the multiple fields inside a nested field.
    pub const fn into_set(self) -> FieldPathSet<S, U> {
        self.set
    }
}

impl<F, S> NestedFieldPathSet<F, S, UniquePaths> {
    /// Converts a `NestedFieldPathSet<F, S, UniquePaths>` to a
    /// `NestedFieldPathSet<F, S, AliasedPaths>`
    #[inline(always)]
    pub const fn downgrade(self) -> NestedFieldPathSet<F, S, AliasedPaths> {
        NestedFieldPathSet {
            nested: self.nested,
            set: self.set.downgrade(),
        }
    }
}

impl<F, S> NestedFieldPathSet<F, S, AliasedPaths> {
    /// Converts a `NestedFieldPathSet<F, S, AliasedPaths>` to a
    /// `NestedFieldPathSet<F, S, UniquePaths>`
    ///
    /// # Safety
    ///
    /// You must ensure that all the `FieldPath`s in `S` are unique,
    /// there must be no field path that is a prefix of any other `FieldPath`.
    #[inline(always)]
    pub const unsafe fn upgrade_unchecked(self) -> NestedFieldPathSet<F, S, UniquePaths> {
        self.set_uniqueness()
    }

    /// Converts a `NestedFieldPathSet<F, S, AliasedPaths>` to a
    /// `NestedFieldPathSet<F, S, UniquePaths>`
    ///
    /// # Safety
    ///
    /// If `U == UniquePaths`,
    /// you must ensure that all the `FieldPath`s in `S` are unique,
    /// there must be no field path that is a prefix of any other `FieldPath`.
    #[inline(always)]
    pub const unsafe fn set_uniqueness<U>(self) -> NestedFieldPathSet<F, S, U> {
        NestedFieldPathSet {
            nested: self.nested,
            set: self.set.set_uniqueness(),
        }
    }
}

impl<F, S, U> IsMultiFieldPath for NestedFieldPathSet<F, S, U> {
    type PathUniqueness = U;
}

impl<F, S> ConstDefault for NestedFieldPathSet<F, S, AliasedPaths>
where
    F: ConstDefault,
    S: ConstDefault,
{
    const DEFAULT: Self = NestedFieldPathSet {
        nested: ConstDefault::DEFAULT,
        set: ConstDefault::DEFAULT,
    };
}

////////////////////////////////////////////////////////////////////////////////
