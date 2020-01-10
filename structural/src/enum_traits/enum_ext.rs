#[cfg(feature = "alloc")]
use crate::alloc::boxed::Box;

use crate::{
    enum_traits::{IsVariant, VariantProxy},
    type_level::FieldPath1,
};

pub trait EnumExt {
    fn as_variant<V>(&self, vari: FieldPath1<V>) -> Option<&VariantProxy<Self, FieldPath1<V>>>
    where
        Self: IsVariant<FieldPath1<V>>,
    {
        if IsVariant::is_variant_(self, vari) {
            unsafe { Some(VariantProxy::from_ref(self)) }
        } else {
            None
        }
    }

    fn as_mut_variant<V>(
        &mut self,
        vari: FieldPath1<V>,
    ) -> Option<&mut VariantProxy<Self, FieldPath1<V>>>
    where
        Self: IsVariant<FieldPath1<V>>,
    {
        if IsVariant::is_variant_(&*self, vari) {
            unsafe { Some(VariantProxy::from_mut(self)) }
        } else {
            None
        }
    }

    unsafe fn as_raw_mut_variant<V>(
        this: *mut Self,
        vari: FieldPath1<V>,
    ) -> Option<*mut VariantProxy<Self, FieldPath1<V>>>
    where
        Self: IsVariant<FieldPath1<V>> + Sized,
    {
        if IsVariant::is_variant_(&*this, vari) {
            Some(VariantProxy::from_raw_mut(this))
        } else {
            None
        }
    }

    fn into_variant<V>(self, vari: FieldPath1<V>) -> Option<VariantProxy<Self, FieldPath1<V>>>
    where
        Self: IsVariant<FieldPath1<V>> + Sized,
    {
        if IsVariant::is_variant_(&self, vari) {
            unsafe { Some(VariantProxy::new(self)) }
        } else {
            None
        }
    }

    #[cfg(feature = "alloc")]
    fn box_into_variant<V>(
        self: Box<Self>,
        vari: FieldPath1<V>,
    ) -> Option<Box<VariantProxy<Self, FieldPath1<V>>>>
    where
        Self: IsVariant<FieldPath1<V>> + Sized,
    {
        if IsVariant::is_variant_(&*self, vari) {
            unsafe { Some(VariantProxy::from_box(self)) }
        } else {
            None
        }
    }
}

impl<This: ?Sized> EnumExt for This {}
