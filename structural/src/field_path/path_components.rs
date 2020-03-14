
macro_rules! impl_to_path_to_set {
    (
        impl[ $($impl_params:tt)* ] $self:ty
        $(where[ $($where_clause:tt)* ])?
    ) => (
        impl< $($impl_params)* > $self
        where
            $($($where_clause)*)?
        {
            /// Constructs a NestedFieldPath from this.
            #[inline(always)]
            pub const fn into_path(self) -> NestedFieldPath<(Self,)> {
                NestedFieldPath::one(self)
            }

            /// Constructs a FieldPathSet from this.
            #[inline(always)]
            pub const fn into_set(self) -> FieldPathSet<(Self,), UniquePaths> {
                FieldPathSet::one(self)
            }
        }
    )
}

////////////////////////////////////////////////////////////////////////////////

/// A marker trait for type-level strings.
///
/// This is only implemented on [`TStr`](../struct.TStr.html).
///
pub trait IsTStr: Sealed + Debug + Copy + ConstDefault {}

/// A marker trait to assert that `P` is a [`TStr`](crate::TStr).
#[doc(hidden)]
pub trait AssertTStrParam<P>: AssertTStrParamSealed<P> {}

mod is_tstr_param_sealed {
    pub trait AssertTStrParamSealed<P> {}
}
use is_tstr_param_sealed::AssertTStrParamSealed;

impl<This: ?Sized, P> AssertTStrParamSealed<TStr<P>> for This {}
impl<This: ?Sized, P> AssertTStrParam<TStr<P>> for This {}

impl<T> IsTStr for TStr<T> {}

impl<T> Debug for TStr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TStr").finish()
    }
}

impl<T> TStr<T> {
    /// Constructs the TStr.
    pub const NEW: Self = TStr(PhantomData);
}

impl_to_path_to_set! {
    impl[T] TStr<T>
}

impl<T> IsSingleFieldPath for TStr<T> {}

impl<T> Copy for TStr<T> {}
impl<T> Clone for TStr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> ConstDefault for TStr<T> {
    const DEFAULT: Self = TStr(PhantomData);
}

impl<T> ToTString for TStr<T> {
    type Output = Self;
}

impl_cmp_traits! {
    impl[T] TStr<T>
    where[]
}

////////////////////////////////////////////////////////////////////////////////

impl<V, F> VariantField<V, F>
where
    V: ConstDefault,
    F: ConstDefault,
{
    /// Constructs a `VariantField<V,F>`
    pub const NEW: Self = ConstDefault::DEFAULT;
}

impl_to_path_to_set! {
    impl[V,F] VariantField<V,F>
}

impl<V, F> VariantField<V, F> {
    /// Constructs a VariantField from the name of the variant,and field.
    ///
    /// Both `name` and `field` is expected to be a [::structural::field_path::TStr].
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

impl<V, F> ConstDefault for VariantField<V, F>
where
    V: ConstDefault,
    F: ConstDefault,
{
    const DEFAULT: Self = Self {
        variant: ConstDefault::DEFAULT,
        field: ConstDefault::DEFAULT,
    };
}

/// A NestedFieldPath for the `F` field inside the `V` variant.
pub type VariantFieldPath<V, F> = VariantField<V, F>;

impl_cmp_traits! {
    impl[V,F] VariantField<V,F>
    where[]
}

////////////////////////////////////////////////////////////////////////////////

impl<V> VariantName<V>
where
    V: ConstDefault,
{
    /// Constructs a VariantName.
    pub const NEW: Self = Self::DEFAULT;
}

impl_to_path_to_set! {
    impl[V] VariantName<V>
}

impl<V> VariantName<V> {
    /// Constructs a VariantName from `name`.
    ///
    /// `name` is expected to be a [::structural::field_path::TStr].
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

impl<V> ConstDefault for VariantName<V>
where
    V: ConstDefault,
{
    const DEFAULT: Self = VariantName {
        name: ConstDefault::DEFAULT,
    };
}

impl_cmp_traits! {
    impl[T] VariantName<T>
    where[]
}

////////////////////////////////////////////////////////////////////////////////
