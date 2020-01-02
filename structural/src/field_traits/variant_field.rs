/*!
Traits used to get a field from an enum variant.

# Safety

All of the functions in the traits traits from this module are
unsafe to call because the enum must be the variant specified by
the `V` generic parameter.
*/

use crate::field_traits::FieldErr;

use std_::marker::PhantomData;

/// Gets a shared reference to the `F` field  from the `V` variant
pub trait GetVariantField<V, F> {
    type Ty;
    type Err: FieldErr;

    /// Gets a shared reference to the `F` field  from the `V` variant
    ///
    /// # Safety
    ///
    /// The variant of this enum must be the one specified by the `V` generic parameter
    unsafe fn get_variant_field(&self) -> Result<&Self::Ty, Self::Err>;
}

/// Gets a mutable reference to the `F` field  from the `V` variant
pub unsafe trait GetVariantFieldMut<V, F>: GetVariantField<V, F> {
    /// Gets a mutable reference to the `F` field  from the `V` variant
    ///
    /// # Safety
    ///
    /// The variant of this enum must be the one specified by the `V` generic parameter
    unsafe fn get_variant_field_mut(&mut self) -> Result<&mut Self::Ty, Self::Err>;

    /// Gets a mutable pointer to the `F` field  from the `V` variant
    ///
    /// # Safety
    ///
    /// The variant of this enum must be the one specified by the `V` generic parameter
    unsafe fn get_variant_field_raw_mut(
        ptr: *mut Self,
        _: PhantomData<(V, F)>,
    ) -> Result<*mut Self::Ty, Self::Err>;
}

/// Converts this into the `F` field  from the `V` variant
pub trait IntoVariantField<V, F>: GetVariantField<V, F> {
    /// Converts this into the `F` field  from the `V` variant
    ///
    /// # Safety
    ///
    /// The variant of this enum must be the one specified by the `V` generic parameter
    unsafe fn into_variant_field(self) -> Result<Self::Ty, Self::Err>;
}
