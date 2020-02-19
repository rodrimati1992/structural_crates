use super::Sealed;
use crate::field_path::{FieldPath, FieldPathSet, IsSingleFieldPath, UniquePaths};
use crate::type_level::collection_traits::ToTString_;
use crate::{TStr_, VariantField, VariantName};

use core_extensions::MarkerType;

use std_::{
    fmt::{self, Debug},
    marker::PhantomData,
};

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

/// A marker trait for type-level string.
///
/// This is only implemented on `TStr_<_>`,
/// which is not in the documentation so that its type parameter
/// can be turned into a `const NAME:&'static str` const parameter.
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
    /// Both `name` and `field` is expected to be a [::structural::field_path::TStr_].
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

impl<V> VariantName<V>
where
    V: MarkerType,
{
    /// Constructs a VariantName.
    pub const NEW: Self = Self {
        name: MarkerType::MTVAL,
    };
}

impl_to_path_to_set! {
    impl[V] VariantName<V>
}

impl<V> VariantName<V> {
    /// Constructs a VariantName from `name`.
    ///
    /// `name` is expected to be a [::structural::field_path::TStr_].
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
