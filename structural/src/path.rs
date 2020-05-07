/*!
Types used to refer to the field(s) that one is accessing.

The re-exported items are all field-path related.
*/

#![allow(non_snake_case, non_camel_case_types)]

use crate::type_level::collection_traits::{
    Append, AppendOut, PushBack, PushBackOut, ToTList, ToTListOut, ToTString,
};

pub use crate::{field_path_aliases, fp, FP};

use core_extensions::ConstDefault;

use std_::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem::ManuallyDrop,
};

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests;

pub mod array_paths;

mod to_usize;

include! { "./path/path_components.rs" }

pub use crate::{
    FieldPathSet, NestedFieldPath, NestedFieldPathSet, TStr, VariantField, VariantName,
};

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

/// A marker trait for field paths that only refers to one field.
///
/// # Expectations
///
/// This type is expected to implement `RevGetFieldImpl`,`RevGetFieldMutImpl`, `RevIntoFieldImpl`.
pub trait IsSingleFieldPath: Sized {}

/// A marker trait for field paths of non-nested field(s).
///
/// # Safety
///
/// If this type implements any of the
/// `RevGetFieldImpl`/`RevGetFieldMutImpl`/`RevIntoFieldImpl` traits,
/// it must delegate those impls to non-nested
/// `Get*Field`/`Get*FieldMut`/`Into*Field` impls
/// of the `this` parameter (the `This` type parameter in the `Rev*` traits).
pub unsafe trait ShallowFieldPath: Sized {}

/// A marker trait for field paths that refer to multiple fields
///
/// # Expectations
///
/// This type is expected to implement `RevGetMultiField`.
pub trait IsMultiFieldPath: Sized {
    /// Whether the paths in the set can contain duplicate paths.
    ///
    /// This is expected to be either:
    ///
    /// - `structural::path::AliasedPaths`:
    /// for a field path that might refer to the same field multiple times.
    ///
    /// - `structural::path::UniquePaths`:
    /// for a field path that doesn't refer to a field more than once.
    ///
    type PathUniqueness;
}

////////////////////////////////////////////////////////////////////////////////

impl<T> IsSingleFieldPath for NestedFieldPath<T> {}

unsafe impl<F0> ShallowFieldPath for NestedFieldPath<(F0,)> where F0: ShallowFieldPath {}

impl<T> IsMultiFieldPath for NestedFieldPath<T> {
    type PathUniqueness = UniquePaths;
}

impl<T> Debug for NestedFieldPath<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NestedFieldPath").finish()
    }
}

impl<T> NestedFieldPath<T>
where
    T: ConstDefault,
{
    /// Constructs a `NestedFieldPath<T>`
    pub const NEW: Self = Self::DEFAULT;
}

// Defined for the `fp` macro
impl<T> NestedFieldPath<T>
where
    Self: ConstDefault,
{
    #[doc(hidden)]
    pub const NEW_ALIASED: Self = Self::DEFAULT;

    #[doc(hidden)]
    pub const unsafe fn set_uniqueness(self) -> Self {
        self
    }
}

impl<T> NestedFieldPath<(T,)> {
    /// Construcst a `NestedFieldPath` from a single path component.
    #[inline(always)]
    pub const fn one(value: T) -> Self {
        Self { list: (value,) }
    }
}

impl<T> NestedFieldPath<T> {
    /// Constructs a `NestedFieldPath` for a nested field.
    ///
    /// Example:
    /// `NestedFieldPath::many(( ts!(a), ts!(b) ))`
    /// is equivalent to `fp!(a.b)`
    ///
    /// Example:
    /// `NestedFieldPath::many(( VariantField::new(ts!(A), ts!(b)), ts!(c) ))`
    /// is equivalent to `fp!(::A.b.c)`
    #[inline(always)]
    pub const fn many(list: T) -> Self {
        Self { list }
    }
}

impl<T> ConstDefault for NestedFieldPath<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = NestedFieldPath {
        list: ConstDefault::DEFAULT,
    };
}

impl<S> ToTString for NestedFieldPath<(TStr<S>,)> {
    type Output = TStr<S>;
}

impl<T> ToTList for NestedFieldPath<T>
where
    T: ToTList,
{
    type Output = ToTListOut<T>;
}

impl<T, S> PushBack<S> for NestedFieldPath<T>
where
    T: PushBack<S>,
{
    type Output = NestedFieldPath<PushBackOut<T, S>>;
}

impl<T, U> Append<NestedFieldPath<U>> for NestedFieldPath<T>
where
    T: Append<U>,
{
    type Output = NestedFieldPath<AppendOut<T, U>>;
}

impl<T> NestedFieldPath<T> {
    /// Constructs a new NestedFieldPath with `_other` appended at the end.
    ///
    /// Example arguments:`fp!(a)`/`fp!(::Foo.bar)`/`fp!(::Foo)`
    #[inline(always)]
    pub fn push<U, V>(self, _other: U) -> NestedFieldPath<V>
    where
        Self: PushBack<U, Output = NestedFieldPath<V>>,
        NestedFieldPath<V>: ConstDefault,
    {
        ConstDefault::DEFAULT
    }

    /// Constructs a new NestedFieldPath with `_other` appended at the end.
    ///
    /// Example arguments:`fp!(a,b)`/`fp!(::Foo.bar.baz)`
    #[inline(always)]
    pub fn append<U>(self, _other: NestedFieldPath<U>) -> NestedFieldPath<AppendOut<T, U>>
    where
        T: Append<U>,
        NestedFieldPath<AppendOut<T, U>>: ConstDefault,
    {
        ConstDefault::DEFAULT
    }

    /// Converts this `NestedFieldPath` to a `FieldPathSet`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use structural::{StructuralExt, fp};
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

impl<C> NestedFieldPath<(C,)> {
    /// Unwraps this non-nested field path into `C`.
    ///
    /// This can also be done with `path.list.0`.
    pub fn into_component(self) -> C {
        self.list.0
    }
}

impl_cmp_traits! {
    impl[T] NestedFieldPath<T>
    where[]
}

////////////////////////////////////////////////////////////////////////////////

/// A merker type indicating that a ([`Nested`])[`FieldPathSet`] contains unique field paths,
/// in which no path is a prefix of any other path in the set,
/// this is required to call `StructuralExt::fields_mut`.
///
/// [`FieldPathSet`]: ../struct.FieldPathSet.html
/// [`Nested`]: ../struct.NestedFieldPathSet.html
#[derive(Debug, Copy, Clone)]
pub struct UniquePaths;

/// A merker type indicating that a ([`Nested`])[`FieldPathSet`]
/// might not contain unique field paths.
/// Its not possible to pass a `FieldPathSet<_,AliasedPaths>` to
/// `StructuralExt::fields_mut`.
///
/// [`FieldPathSet`]: ../struct.FieldPathSet.html
/// [`Nested`]: ../struct.NestedFieldPathSet.html
#[derive(Debug, Copy, Clone)]
pub struct AliasedPaths;

impl<T, U> IsSingleFieldPath for FieldPathSet<(T,), U> {}

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
    /// Constructs a FieldPathSet from a tuple of up to 8 field paths.
    ///
    /// Note that this doesn't enforce that its input is in fact a tuple of
    /// up to 8 field paths (because `const fn` can't have bounds yet).
    ///
    /// To be able to access multiple fields mutably at the same time,
    /// you must call the unsafe `.upgrade()` method.
    ///
    /// To access more than 8 fields, you must use the [large constructor](#method.large).
    pub const fn many(paths: T) -> Self {
        FieldPathSet {
            paths: ManuallyDrop::new(paths),
            uniqueness: PhantomData,
        }
    }
}

impl<T> FieldPathSet<LargePathSet<T>, AliasedPaths> {
    /// Constructs a FieldPathSet from a tuple of tuples of field paths,
    /// for accessing up to 64 fields.
    ///
    /// Note that this doesn't enforce that its input is in fact a
    /// tuple of tuples of field paths (because `const fn` can't have bounds yet).
    ///
    /// To be able to access multiple fields mutably at the same time,
    /// you must call the unsafe `.upgrade()` method.
    ///
    /// # Example
    ///
    /// This example demonstrates calling this function with 8 and 9 field paths,
    /// as well as the return value of `StructuralExt` methods for both path sets.
    ///
    /// Accessing over 8 fields returns a tuple of tuples (8 fields each).
    ///
    /// You can also destructure the tuple returned by accessor methods by using
    /// the [`field_pat`] macro, usable from 0 to 64 fields.
    ///
    /// ```rust
    /// use structural::{ FieldPathSet, StructuralExt, path_tuple, ts };
    ///
    /// let array = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    ///
    /// {
    ///     let path8 = FieldPathSet::large(path_tuple!(
    ///         ts!(0), ts!(1), ts!(2), ts!(3), ts!(4), ts!(5), ts!(6), ts!(7),
    ///     ));
    ///     
    ///     assert_eq!(
    ///         array.cloned_fields(path8),
    ///         (10, 11, 12, 13, 14, 15, 16, 17),
    ///     );
    /// }
    /// {
    ///     let path9 = FieldPathSet::large(path_tuple!(
    ///         ts!(0), ts!(1), ts!(2), ts!(3), ts!(4), ts!(5), ts!(6), ts!(7),
    ///         ts!(8),
    ///     ));
    ///     
    ///     assert_eq!(
    ///         array.cloned_fields(path9),
    ///         (
    ///             (10, 11, 12, 13, 14, 15, 16, 17),
    ///             (18,),
    ///         ),
    ///     );
    /// }
    ///
    /// ```
    ///
    /// [`field_pat`]: ./macro.field_pat.html    
    pub const fn large(paths: T) -> Self {
        FieldPathSet {
            paths: ManuallyDrop::new(LargePathSet(paths)),
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
    /// in a context where `T` can be inferred,
    /// by doing `unsafe{ FieldPathSet::NEW.upgrade_unchecked() }`
    /// (read the docs for `upgrade_unchecked` first).
    pub const NEW: Self = Self::DEFAULT;
}

impl<T, U> FieldPathSet<T, U>
where
    T: ConstDefault,
{
    /// This can be used to construct a `FieldPathSet<T, UniquePaths>`
    /// from a type alias,
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
    /// You must ensure that all the field paths are unique,
    /// there must be no field path that is a prefix of any other field path.
    #[inline(always)]
    pub const unsafe fn upgrade_unchecked(self) -> FieldPathSet<T, UniquePaths> {
        self.set_uniqueness()
    }

    /// Converts a `FieldPathSet<T,AliasedPaths>` to a `FieldPathSet<T,U>`
    ///
    /// # Safety
    ///
    /// You must ensure that if `U==UniquePaths`,then all the field paths are unique,
    /// there must be no field path that is a prefix of any other field path.
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
        Self: PushBack<O, Output = FieldPathSet<Out, AliasedPaths>>,
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
    ) -> FieldPathSet<AppendOut<T, T2>, AliasedPaths>
    where
        T: Append<T2>,
        FieldPathSet<AppendOut<T, T2>, AliasedPaths>: ConstDefault,
    {
        ConstDefault::DEFAULT
    }
}

impl<T, U> ToTList for FieldPathSet<T, U>
where
    T: ToTList,
{
    type Output = ToTListOut<T>;
}

impl<T, U, P> PushBack<NestedFieldPath<P>> for FieldPathSet<T, U>
where
    T: PushBack<NestedFieldPath<P>>,
{
    type Output = FieldPathSet<PushBackOut<T, NestedFieldPath<P>>, AliasedPaths>;
}

impl<T, U, P, U2> PushBack<FieldPathSet<(P,), U2>> for FieldPathSet<T, U>
where
    T: PushBack<P>,
{
    type Output = FieldPathSet<PushBackOut<T, P>, AliasedPaths>;
}

impl<T, T2, U, U2> Append<FieldPathSet<T2, U2>> for FieldPathSet<T, U>
where
    T: Append<T2>,
{
    type Output = FieldPathSet<AppendOut<T, T2>, AliasedPaths>;
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
    /// from a type alias,
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

    /// Unwraps a `NestedFieldPathSet` into a `NestedFieldPath` and a `FieldPathSet`
    pub const fn into_inner(self) -> (F, FieldPathSet<S, U>) {
        (ManuallyDrop::into_inner(self.nested), self.set)
    }

    /// Unwraps a `NestedFieldPathSet` into the `NestedFieldPath` for the nested field.
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
    /// You must ensure that all the field paths in `S` are unique,
    /// there must be no field path that is a prefix of any other field path.
    #[inline(always)]
    pub const unsafe fn upgrade_unchecked(self) -> NestedFieldPathSet<F, S, UniquePaths> {
        self.set_uniqueness()
    }

    /// Converts a `NestedFieldPathSet<F, S, AliasedPaths>` to a
    /// `NestedFieldPathSet<F, S, U>`
    ///
    /// # Safety
    ///
    /// If `U == UniquePaths`,
    /// you must ensure that all the field paths in `S` are unique,
    /// there must be no field path that is a prefix of any other field path.
    #[inline(always)]
    pub const unsafe fn set_uniqueness<U>(self) -> NestedFieldPathSet<F, S, U> {
        NestedFieldPathSet {
            nested: self.nested,
            set: self.set.set_uniqueness(),
        }
    }
}

impl<F, S, U> IsSingleFieldPath for NestedFieldPathSet<F, (S,), U> {}

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

/// A newtype wrapper used to allow `FieldPathSet` to access from 9 up to 64 fields.
///
/// For examples of constructing and using `FieldPathSet<LargePathSet<_>,_>` you can look at
/// the [docs for `FieldPathSet::large`](../struct.FieldPathSet.html#method.large)
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct LargePathSet<T>(pub T);

impl<T> ConstDefault for LargePathSet<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = LargePathSet(T::DEFAULT);
}

////////////////////////////////////////////////////////////////////////////////

/// A workaround for long compile-time errors.
///
/// The is used to work compile-time errors that are around 200 kilobytes long,
/// caused by recusive impls.
///
/// This is an example of the code that triggered a long error message:
/// ```ignore
/// ().fields(FieldPathSet::many(panic!()))
/// ```
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[doc(hidden)]
pub struct SmallPathSet<T>(pub(crate) T);

impl<T> ConstDefault for SmallPathSet<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = SmallPathSet(T::DEFAULT);
}

////////////////////////////////////////////////////////////////////////////////

/// Converts a `FieldPathSet<_,UniquePaths>` into a `FieldPathSet<_,AliasedPaths>`
/// on the type level.
pub trait IntoAliasing: IsMultiFieldPath {
    type Output: IsMultiFieldPath<PathUniqueness = AliasedPaths>;
}

/// Converts a `FieldPathSet<_,UniquePaths>` into a `FieldPathSet<_,AliasedPaths>`
/// on the type level.
pub type IntoAliasingOut<This> = <This as IntoAliasing>::Output;

impl<F, U> IntoAliasing for FieldPathSet<F, U> {
    type Output = FieldPathSet<F, AliasedPaths>;
}

impl<F, S, U> IntoAliasing for NestedFieldPathSet<F, S, U> {
    type Output = NestedFieldPathSet<F, S, AliasedPaths>;
}
