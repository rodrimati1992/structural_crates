use crate::{
    field_traits::{
        variant_field::{GetVariantFieldImpl, GetVariantFieldMutImpl, IntoVariantFieldImpl},
        FieldType, GetFieldImpl, GetFieldMutImpl, GetFieldRawMutFn, IntoFieldImpl,
    },
    type_level::{FieldPath1, UncheckedVariantField, VariantFieldPath},
};

#[cfg(feature = "alloc")]
use crate::alloc::boxed::Box;

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
pub struct VariantProxy<T: ?Sized, V> {
    _marker: PhantomData<V>,
    value: T,
}

impl<T: ?Sized, V> VariantProxy<T, FieldPath1<V>> {
    /// Constructs this VariantProxy from an enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    #[inline(always)]
    pub const unsafe fn new(value: T) -> Self
    where
        T: Sized,
    {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    /// Constructs this VariantProxy from a boxed enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    #[inline(always)]
    #[cfg(feature = "alloc")]
    pub unsafe fn from_box(value: Box<T>) -> Box<Self> {
        std_::mem::transmute::<Box<T>, Box<VariantProxy<T, FieldPath1<V>>>>(value)
    }

    /// Constructs this VariantProxy from a reference to an enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    #[inline(always)]
    pub unsafe fn from_ref(reference: &T) -> &Self {
        &*(reference as *const T as *const VariantProxy<T, FieldPath1<V>>)
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
        ptr as *mut VariantProxy<T, FieldPath1<V>>
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
    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
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

impl<T, V, F> FieldType<FieldPath1<F>> for VariantProxy<T, FieldPath1<V>>
where
    T: ?Sized + FieldType<VariantFieldPath<V, F>>,
{
    type Ty = T::Ty;
}

impl<T, V, F> GetFieldImpl<FieldPath1<F>> for VariantProxy<T, FieldPath1<V>>
where
    T: ?Sized + GetVariantFieldImpl<V, F>,
{
    type Err = T::Err;

    #[inline(always)]
    fn get_field_(&self, _: FieldPath1<F>, _: ()) -> Result<&T::Ty, T::Err> {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe {
            self.value.get_field_(
                VariantFieldPath::<V, F>::NEW,
                UncheckedVariantField::<V, F>::new(),
            )
        }
    }
}

unsafe impl<T, V, F> GetFieldMutImpl<FieldPath1<F>> for VariantProxy<T, FieldPath1<V>>
where
    T: GetVariantFieldMutImpl<V, F>,
{
    #[inline(always)]
    fn get_field_mut_(&mut self, _: FieldPath1<F>, _: ()) -> Result<&mut T::Ty, T::Err> {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe {
            self.value.get_field_mut_(
                VariantFieldPath::<V, F>::NEW,
                UncheckedVariantField::<V, F>::new(),
            )
        }
    }

    #[inline(always)]
    unsafe fn get_field_raw_mut(
        this: *mut (),
        _: FieldPath1<F>,
        _: (),
    ) -> Result<*mut T::Ty, T::Err> {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        T::get_field_raw_mut(
            this,
            VariantFieldPath::<V, F>::new(),
            UncheckedVariantField::<V, F>::new(),
        )
    }

    #[inline(always)]
    fn get_field_raw_mut_func(&self) -> GetFieldRawMutFn<FieldPath1<F>, (), T::Ty, T::Err> {
        <Self as GetFieldMutImpl<FieldPath1<F>>>::get_field_raw_mut
    }
}

impl<T, V, F> IntoFieldImpl<FieldPath1<F>> for VariantProxy<T, FieldPath1<V>>
where
    T: IntoVariantFieldImpl<V, F>,
{
    #[inline(always)]
    fn into_field_(self, _: FieldPath1<F>, _: ()) -> Result<T::Ty, T::Err> {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe {
            self.value.into_field_(
                VariantFieldPath::<V, F>::NEW,
                UncheckedVariantField::<V, F>::new(),
            )
        }
    }

    z_impl_box_into_field_method! {FieldPath1<F>,T::Ty,T::Err}
}
