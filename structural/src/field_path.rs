/*!
Type-level representatins of access to one or multiple fields.
*/

#![allow(non_snake_case, non_camel_case_types)]

use core_extensions::MarkerType;

use std_::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem::ManuallyDrop,
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
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_to_path_to_set {
    (
        impl[ $($impl_params:tt)* ] $self:ty
        $(where[ $($where_clause:tt)* ])?
    ) => (
        impl< $($impl_params)* > $self
        where
            $($($where_clause)*)?
        {
            /// Constructs a FieldPath from this.
            #[inline(always)]
            pub const fn to_path(self) -> FieldPath<(Self,)> {
                FieldPath::one(self)
            }

            /// Constructs a FieldPathSet from this.
            #[inline(always)]
            pub const fn to_set(self) -> FieldPathSet<(FieldPath<(Self,)>,), UniquePaths> {
                FieldPathSet::one(
                    FieldPath::one(self)
                )
            }
        }
    )
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

/// A marker trait for type-level string.
///
/// This is only implemented on `TStr_<_>`,
/// which is not in the documentation so that its type parameter
/// can be turned into a `const NAME:&'static str` const parameter.
///
/// # TStr construction
///
/// `TStr_<_>` can be constructed with:
///
/// - the `tstr` macro,which takes a string literal/ident/integer as an input.
///
/// - the `NEW` inherent associated constant,
///
/// - The `<TStr_<_> as MarkerType>::MTVAL` associated constant.
///
/// Examples of constructing a `TStr_<_>`:
///
/// - `tstr!(foo)` (in every Rust version)
///
/// - `tstr!(f o o)` (in every Rust version)
///
/// - `tstr!("bar")` (in every Rust version)
///
/// - `tstr!(1)` (in every Rust version)
///
/// - `tstr!(100)` (in every Rust version)
///
/// - `tstr!(1 0 0)` (in every Rust version)
///
/// - `<TStr!("hello")>::NEW` (from Rust 1.40 onwards)
///
/// - `<TStr!(world)>::NEW` (from Rust 1.40 onwards)
///
/// - `<TStr!(100)>::NEW` (from Rust 1.40 onwards)
///
/// - `<TStr!(w o r l d)>::NEW` (in every Rust version)
///
/// - `<TStr!(0)>::NEW` (in every Rust version)
///
/// - `<TStr!(1 0 0)>::NEW` (in every Rust version)
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
}

impl_to_path_to_set! {
    impl[T] TStr_<T>
}

impl<T> IsSingleFieldPath for TStr_<T> {}

impl<T> IsMultiFieldPath for TStr_<T> {
    type PathUniqueness = UniquePaths;
}

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
///
/// This is the type parameter of the `FieldPath<_>` in `fp!(::Foo.bar)`.
///
/// Both the V and F type parameters are `TStr_`s,
/// the docs for `TStr_` are in [::structural::field_path::IsTStr]
#[derive(Copy, Clone)]
pub struct VariantField<V, F> {
    pub variant: V,
    pub field: F,
}

impl<V, F> VariantField<V, F>
where
    V: MarkerType,
    F: MarkerType,
{
    pub const NEW: Self = MarkerType::MTVAL;
}

impl_to_path_to_set! {
    impl[V,F] VariantField<V,F>
}

impl<V, F> VariantField<V, F> {
    /// Constructs a VariantField from the name of the variant,and field.
    ///
    /// Both `name` and `field` is expected to be a `TStr_`
    /// the docs for `TStr_` are in [::structural::field_path::IsTStr]
    pub const fn new(variant: V, field: F) -> Self {
        Self { variant, field }
    }
}

impl<V, F> IsSingleFieldPath for VariantField<V, F> {}

impl<T, U> Debug for VariantField<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariantField").finish()
    }
}

unsafe impl<V, F> MarkerType for VariantField<V, F>
where
    V: MarkerType,
    F: MarkerType,
{
}

/// A FieldPath for the `F` field inside the `V` variant.
pub type VariantFieldPath<V, F> = FieldPath<(VariantField<V, F>,)>;

impl_cmp_traits! {
    impl[V,F] VariantField<V,F>
    where[]
}

////////////////////////////////////////////////////////////////////////////////

/// The identifier for the `V` variant.
///
/// This is the type parameter of the `FieldPath<_>` in `fp!(::Foo)`.
/// Note that `fp!(::Foo.bar)` constructs a `FieldPath<(VariantField<_,_>,)>` instead.
///
/// The V type parameters is a `TStr_`,
/// the docs for `TStr_` are in [::structural::field_path::IsTStr]
#[derive(Default, Copy, Clone)]
pub struct VariantName<V> {
    pub name: V,
}

impl<V> VariantName<V>
where
    V: MarkerType,
{
    /// Constructs a VariantName.
    const NEW: Self = Self {
        name: MarkerType::MTVAL,
    };
}

impl_to_path_to_set! {
    impl[V] VariantName<V>
}

impl<V> VariantName<V> {
    /// Constructs a VariantName from `name`.
    ///
    /// `name` is expected to be a `TStr_`
    /// the docs for `TStr_` are in [::structural::field_path::IsTStr]
    pub fn new(name: V) -> Self {
        Self { name }
    }
}

impl<V> IsSingleFieldPath for VariantName<V> {}

impl<T> Debug for VariantName<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariantName").finish()
    }
}

unsafe impl<V> MarkerType for VariantName<V> where V: MarkerType {}

impl_cmp_traits! {
    impl[T] VariantName<T>
    where[]
}

////////////////////////////////////////////////////////////////////////////////

/// A marker type passed to accessor trait methods called on an enum,
/// which guarantees that the enum is the variant that `V` represents.
pub struct UncheckedVariantField<V, F>(PhantomData<(V, F)>);

// MarkerType is intentionally not implemented
// // // unsafe impl<V, F> !MarkerType for VariantField<V, F>{}

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
#[repr(transparent)]
#[derive(Default, Copy, Clone)]
pub struct FieldPath<T> {
    pub list: T,
}

/// A FieldPath for accesing a single non-nested field.
pub type FieldPath1<Str> = FieldPath<(Str,)>;

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
    T: MarkerType,
{
    /// Constructs a `FieldPath<T>`
    pub const NEW: Self = FieldPath { list: T::MTVAL };
}

impl<T> FieldPath<(T,)> {
    /// Construcst a field path from a single path component.
    ///
    /// Example: `FieldPath::one(tstr!(a))` is equivalent to `fp!(a)`
    ///
    /// Example:
    /// `FieldPath::one(VariantField::new(tstr!(a),tstr!(b)))`
    /// is equivalent to `fp!(::a.b)`
    ///
    /// Example:
    /// `FieldPath::one(VariantName::new(tstr!(Left)))`
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
    /// `FieldPath::many(( tstr!(a), tstr!(b) ))`
    /// is equivalent to `fp!(a.b)`
    ///
    /// Example:
    /// `FieldPath::many(( VariantField::new(tstr!(A), tstr!(b)), tstr!(c) ))`
    /// is equivalent to `fp!(::A.b.c)`
    #[inline(always)]
    pub const fn many(list: T) -> Self {
        Self { list }
    }
}

unsafe impl<T> MarkerType for FieldPath<T> where T: MarkerType {}

impl<S> ToTString_ for FieldPath<(TStr_<S>,)> {
    type Output = TStr_<S>;
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
        FieldPath<V>: MarkerType,
    {
        MarkerType::MTVAL
    }

    /// Constructs a new FieldPath with `_other` appended at the end.
    #[inline(always)]
    pub fn append<U>(self, _other: FieldPath<U>) -> FieldPath<Append<T, U>>
    where
        T: Append_<U>,
        FieldPath<Append<T, U>>: MarkerType,
    {
        MarkerType::MTVAL
    }

    /// Converts this `FieldPath` to a `FieldPathSet`.
    #[inline(always)]
    pub const fn to_set(self) -> FieldPathSet<(Self,), UniquePaths> {
        FieldPathSet::one(self)
    }
}

impl<S> FieldPath<(TStr_<S>,)> {
    /// Converts this single field path to a `TStr_`.
    ///
    /// The docs for `TStr_` are in [::structural::field_path::IsTStr]
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
/// If `U` is a `UniquePaths` then all the `FieldPath`s are unique,
/// and this can be passed to `GetFieldExt::fields_mut`,
/// since you can't have aliasing mutable references to the same field.
///
/// If `U` is a `AliasedPaths` then there might be repeated `FieldPath`s,
/// and this cannot be passed to `GetFieldExt::fields_mut`,
/// because it might borrow the same field mutably twice.
///
/// # Drop Types
///
/// To make all the inherent methods in this type `const fn`
/// this type wraps the `T` inside a `ManuallyDrop`,
/// which means that `T` won't be dropped inside.
/// If that is a problem don't construct a FieldPathSet with a `T` that owns some resource.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FieldPathSet<T, U> {
    // The ManuallyDrop allows every const fn to be defined as that.
    paths: ManuallyDrop<T>,
    uniqueness: PhantomData<U>,
}

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

// `MarkerType` is not implemented for `FieldPathSet<T.UniquePaths>`
// because `FieldPathSet<T.UniquePaths>` ought only be constructible
// by satisfying the safety requirements of `FieldPathSet::<T.UniquePaths>::new`,
// which aren't cheaply enforceable on the type level.
//
// impl<T> !MarkerType for FieldPathSet<T.UniquePaths>{}

unsafe impl<T> MarkerType for FieldPathSet<T, AliasedPaths> where T: MarkerType {}

impl<T> Default for FieldPathSet<T, AliasedPaths>
where
    T: Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::many(T::default())
    }
}

impl<T> FieldPathSet<(FieldPath<T>,), UniquePaths> {
    /// Constructs a FieldPathSet from a single field path.
    pub const fn one(val: FieldPath<T>) -> Self {
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
    T: MarkerType,
{
    /// Constructs a `FieldPathSet`.
    ///
    /// This can also be used to construct a `FieldPathSet<T, UniquePaths>`
    /// in a const context where `T` can be inferred,
    /// by doing `unsafe{ FieldPathSet::NEW.upgrade_unchecked() }`
    /// (read the docs for [upgrade_unchecked] first).
    pub const NEW: Self = FieldPathSet {
        paths: ManuallyDrop::new(T::MTVAL),
        uniqueness: PhantomData,
    };
}

impl<T, U> FieldPathSet<T, U>
where
    T: MarkerType,
{
    /// This can be used to construct a `FieldPathSet<T, UniquePaths>`
    /// from a type alias in a const context,
    /// by doing `unsafe{ FOO::NEW_ALIASED.upgrade_unchecked() }`
    /// (read the docs for [upgrade_unchecked] first).
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
    /// there must be no `FieldPath` that is a prefix of any other `FieldPath`.
    #[inline(always)]
    pub const unsafe fn upgrade_unchecked(self) -> FieldPathSet<T, UniquePaths> {
        self.set_uniqueness()
    }

    /// Converts a `FieldPathSet<T,AliasedPaths>` to a `FieldPathSet<T,U>`
    ///
    /// # Safety
    ///
    /// You must ensure that if `U==UniquePaths`,then all the `FieldPath`s are unique,
    /// there must be no `FieldPath` that is a prefix of any other `FieldPath`.
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

impl<T, U> FieldPathSet<(FieldPath<T>,), U> {
    /// Converts a `FieldPathSet` containing a single `FieldPath`
    /// into that `FieldPath`.
    #[inline(always)]
    pub fn to_path(self) -> FieldPath<T> {
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
        FieldPathSet<Out, AliasedPaths>: MarkerType,
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
        FieldPathSet<Append<T, T2>, AliasedPaths>: MarkerType,
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
///
/// # Construction
///
/// NestedFieldPathSet can be constructed in these ways:
///
/// - Using `fp!`.Examples:`fp!(::Foo=>a,b)`,`fp!(a.b=>a,b)`
///
/// - Constructing it from a FieldPath and a FieldPathSet using the struct literal.
/// Example:
/// `NestedFieldPathSet::new( fp!(a.b.c), fp!(foo,bar,baz) )`,
/// this gets the `foo`,`bar`,and `baz` fields from inside the `a.b.c` field.
///
/// - Constructing it from a FieldPath and a FieldPathSet using a constructor.
/// Example:
/// `NestedFieldPathSet::new( fp!(::Foo), fp!(a,b) ),
/// this gets the `a`,and `b` fields from inside the `Foo` variant.
///
/// # Drop Types
///
/// To make all the inherent methods in this type `const fn`
/// this type wraps the `FieldPath<F>` inside a `ManuallyDrop`,
/// which means that `F` won't be dropped inside.
/// If that is a problem don't construct a NestedFieldPathSet with an `F`
/// that owns some resource.
#[derive(Debug, Clone, Copy)]
pub struct NestedFieldPathSet<F, S, U> {
    /// The path to a nested field.
    nested: ManuallyDrop<FieldPath<F>>,
    /// The field path for fields accessed inside of the nested field.
    set: FieldPathSet<S, U>,
}

impl<F, S> NestedFieldPathSet<F, S, AliasedPaths>
where
    F: MarkerType,
    S: MarkerType,
{
    /// Constructs a `NestedFieldPathSet`.
    pub const NEW: Self = Self::MTVAL;
}

impl<F, S, U> NestedFieldPathSet<F, S, U>
where
    F: MarkerType,
    S: MarkerType,
{
    /// This can be used to construct a `NestedFieldPathSet<T, UniquePaths>`
    /// from a type alias in a const context,
    /// by doing `unsafe{ FOO::NEW_ALIASED.upgrade_unchecked() }`
    /// (read the docs for [upgrade_unchecked] first).
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
    /// Constructs a `NestedFieldPathSet` from a `FieldPath` and a `FieldPathSet`
    pub const fn new(nested: FieldPath<F>, set: FieldPathSet<S, U>) -> Self {
        Self {
            nested: ManuallyDrop::new(nested),
            set,
        }
    }

    /// Unwraps a `NestedFieldPathSet` into a `FieldPath` and a `FieldPathSet`
    pub const fn into_inner(self) -> (FieldPath<F>, FieldPathSet<S, U>) {
        (ManuallyDrop::into_inner(self.nested), self.set)
    }

    /// Unwraps a `NestedFieldPathSet` into the `FieldPath` for the nested field.
    pub const fn into_nested(self) -> FieldPath<F> {
        ManuallyDrop::into_inner(self.nested)
    }

    /// Unwraps a `NestedFieldPathSet` into the `FieldPathSet` used to
    /// access the multiple fields inside a nested field.
    pub const fn into_set(self) -> FieldPathSet<S, U> {
        self.set
    }
}

impl<F, S> NestedFieldPathSet<F, S, AliasedPaths> {
    /// Converts a `NestedFieldPathSet<F, S, AliasedPaths>` to a
    /// `NestedFieldPathSet<F, S, UniquePaths>`
    ///
    /// # Safety
    ///
    /// You must ensure that all the `FieldPath`s in `S` are unique,
    /// there must be no `FieldPath` that is a prefix of any other `FieldPath`.
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
    /// there must be no `FieldPath` that is a prefix of any other `FieldPath`.
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

unsafe impl<F, S> MarkerType for NestedFieldPathSet<F, S, AliasedPaths>
where
    F: MarkerType,
    S: MarkerType,
{
}

////////////////////////////////////////////////////////////////////////////////

impl<S> From<FieldPath<(TStr_<S>,)>> for TStr_<S> {
    #[inline(always)]
    fn from(this: FieldPath<(TStr_<S>,)>) -> Self {
        this.list.0
    }
}

impl<S> From<TStr_<S>> for FieldPath<(TStr_<S>,)> {
    #[inline(always)]
    fn from(this: TStr_<S>) -> Self {
        FieldPath::one(this)
    }
}
impl<T, U> From<FieldPathSet<(FieldPath<T>,), U>> for FieldPath<T> {
    #[inline(always)]
    fn from(this: FieldPathSet<(FieldPath<T>,), U>) -> Self {
        this.into_paths().0
    }
}

impl<P> From<FieldPath<P>> for FieldPathSet<(FieldPath<P>,), UniquePaths> {
    #[inline(always)]
    fn from(this: FieldPath<P>) -> Self {
        this.to_set()
    }
}

////////////////////////////////////////////////////////////////////////////////
