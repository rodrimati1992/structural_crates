#[cfg(feature = "alloc")]
use crate::alloc::boxed::Box;

use crate::{
    enum_traits::{IsVariant, VariantProxy},
    type_level::FieldPath1,
};

pub trait EnumExt {
    #[inline(always)]
    fn as_variant<V>(
        &self,
        vari: FieldPath1<V>,
    ) -> Result<&VariantProxy<Self, FieldPath1<V>>, &Self>
    where
        Self: IsVariant<FieldPath1<V>>,
    {
        if IsVariant::is_variant_(self, vari) {
            unsafe { Ok(VariantProxy::from_ref(self)) }
        } else {
            Err(self)
        }
    }

    #[inline(always)]
    fn as_mut_variant<V>(
        &mut self,
        vari: FieldPath1<V>,
    ) -> Result<&mut VariantProxy<Self, FieldPath1<V>>, &mut Self>
    where
        Self: IsVariant<FieldPath1<V>>,
    {
        if IsVariant::is_variant_(&*self, vari) {
            unsafe { Ok(VariantProxy::from_mut(self)) }
        } else {
            Err(self)
        }
    }

    #[inline(always)]
    unsafe fn as_raw_mut_variant<V>(
        this: *mut Self,
        vari: FieldPath1<V>,
    ) -> Result<*mut VariantProxy<Self, FieldPath1<V>>, *mut Self>
    where
        Self: IsVariant<FieldPath1<V>>,
    {
        if IsVariant::is_variant_(&*this, vari) {
            Ok(VariantProxy::from_raw_mut(this))
        } else {
            Err(this)
        }
    }

    #[inline(always)]
    fn into_variant<V>(self, vari: FieldPath1<V>) -> Result<VariantProxy<Self, FieldPath1<V>>, Self>
    where
        Self: IsVariant<FieldPath1<V>> + Sized,
    {
        if IsVariant::is_variant_(&self, vari) {
            unsafe { Ok(VariantProxy::new(self)) }
        } else {
            Err(self)
        }
    }

    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn box_into_variant<V>(
        self: Box<Self>,
        vari: FieldPath1<V>,
    ) -> Result<VariantProxy<Box<Self>, FieldPath1<V>>, Box<Self>>
    where
        Self: IsVariant<FieldPath1<V>>,
    {
        if IsVariant::is_variant_(&*self, vari) {
            unsafe { Ok(VariantProxy::from_box(self)) }
        } else {
            Err(self)
        }
    }
}

impl<This: ?Sized> EnumExt for This {}
