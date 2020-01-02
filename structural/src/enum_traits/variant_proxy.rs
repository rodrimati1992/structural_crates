use crate::{
    field_traits::{
        variant_field::{GetVariantField, GetVariantFieldMut, IntoVariantField},
        GetFieldImpl, GetFieldMutImpl, GetFieldRawMutFn, IntoFieldImpl,
    },
    type_level::FieldPath1,
};

use std_::{
    fmt::{self, Debug},
    marker::PhantomData,
};

/// Wraps an enum,guaranteeing that it's a particular variant.
///
/// # Generic parameters
///
/// `T` is the enum this wraps.
///
/// `V` is the name of the wrapped variant (example type:`FP!(Bar)`).
///
#[derive(Copy, Clone)]
pub struct VariantProxy<T, V> {
    _marker: PhantomData<V>,
    value: T,
}

impl<T, V> VariantProxy<T, V> {
    /// Constructs this VariantProxy from an enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    #[inline(always)]
    pub const unsafe fn new(value: T) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    /// Constructs this VariantProxy from a reference to an enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    #[inline(always)]
    pub unsafe fn from_ref(reference: &T) -> &Self {
        &*(reference as *const T as *const VariantProxy<T, V>)
    }

    /// Constructs this VariantProxy from a mutable reference to the enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    #[inline(always)]
    pub unsafe fn from_mut(reference: &mut T) -> &mut Self {
        &mut *Self::from_raw_mut(reference)
    }

    /// Constructs this VariantProxy from a raw pointer to the enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    #[inline(always)]
    pub const unsafe fn from_raw_mut(ptr: *mut T) -> *mut Self {
        ptr as *mut VariantProxy<T, V>
    }

    /// Gets a reference to the wrapped enum.
    pub fn as_ref(&self) -> &T {
        &self.value
    }

    /// Gets a mutable reference to the wrapped enum.
    ///
    /// # Safety
    ///
    /// You must not change the variant of the wrapped enum,
    /// since VariantProxy relies on it being the one that the `V`
    /// generic parmeter specifies
    pub unsafe fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Unwraps this VariantProxy into the enum it wraps.
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Gets a mutable raw pointer to the wrapped enum.
    ///
    /// # Safety
    ///
    /// You must not change the variant of the wrapped enum,
    /// because VariantProxy relies on it being the one that the `V`
    /// generic parmaetere specifies
    pub unsafe fn as_raw_mut(this: *mut Self) -> *mut T {
        this as *mut T
    }
}

impl<T, V> Debug for VariantProxy<T, V>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariantProxy")
            .field("value", &self.value)
            .finish()
    }
}

impl<T, V, F> GetFieldImpl<FieldPath1<F>> for VariantProxy<T, V>
where
    T: GetVariantField<V, FieldPath1<F>>,
{
    type Ty = T::Ty;
    type Err = T::Err;

    #[inline(always)]
    fn get_field_(&self) -> Result<&T::Ty, T::Err> {
        // unsafe: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe { self.value.get_variant_field() }
    }
}

unsafe impl<T, V, F> GetFieldMutImpl<FieldPath1<F>> for VariantProxy<T, V>
where
    T: GetVariantFieldMut<V, FieldPath1<F>>,
{
    #[inline(always)]
    fn get_field_mut_(&mut self) -> Result<&mut T::Ty, T::Err> {
        // unsafe: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe { self.value.get_variant_field_mut() }
    }

    #[inline(always)]
    unsafe fn get_field_raw_mut(
        this: *mut (),
        name: PhantomData<FieldPath1<F>>,
    ) -> Result<*mut T::Ty, T::Err> {
        // unsafe: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        T::get_variant_field_raw_mut(this as *mut T, PhantomData)
    }

    #[inline(always)]
    fn get_field_raw_mut_func(&self) -> GetFieldRawMutFn<FieldPath1<F>, T::Ty, T::Err> {
        <Self as GetFieldMutImpl<FieldPath1<F>>>::get_field_raw_mut
    }
}

impl<T, V, F> IntoFieldImpl<FieldPath1<F>> for VariantProxy<T, V>
where
    T: IntoVariantField<V, FieldPath1<F>>,
{
    #[inline(always)]
    fn into_field_(self) -> Result<T::Ty, T::Err> {
        // unsafe: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe { self.value.into_variant_field() }
    }

    z_impl_box_into_field_method! {FieldPath1<F>,T::Ty,T::Err}
}
