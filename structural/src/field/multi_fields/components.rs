use crate::{
    field::{
        NormalizeFields, RevGetFieldErr as RGFE, RevGetFieldImpl, RevGetFieldMutImpl,
        RevGetFieldType as RGFT, RevGetMultiFieldImpl, RevGetMultiFieldMutImpl,
    },
    NestedFieldPath, TStr, VariantField, VariantName,
};

macro_rules! delegate_multi_field_traits {
    ( impl[$($impl_params:tt)*] $type:ty ) => {
        impl<'a,This, $($impl_params)*> RevGetMultiFieldImpl<'a, This> for $type
        where
            Self: RevGetFieldImpl<'a, This>,
            This: 'a + ?Sized,
            RGFT<Self,This>:'a,
            Result<&'a RGFT<Self,This>,RGFE<'a,Self,This>>:
                'a + NormalizeFields,
        {
            type UnnormFields = (Result<&'a RGFT<Self,This>,RGFE<'a,Self,This>>,);

            #[inline(always)]
            fn rev_get_multi_field_impl(
                self,
                this: &'a This,
            ) -> (Result<&'a RGFT<Self,This>,RGFE<'a,Self,This>>,){
                (self.rev_get_field(this),)
            }
        }

        unsafe impl<'a, This, $($impl_params)*> RevGetMultiFieldMutImpl<'a, This> for $type
        where
            Self: RevGetFieldMutImpl<'a, This>,
            This: 'a + ?Sized,
            RGFT<Self,This>:'a,
            Result<&'a mut RGFT<Self,This>,RGFE<'a,Self,This>>: NormalizeFields,
            Result<*mut RGFT<Self,This>,RGFE<'a,Self,This>>: NormalizeFields,
        {
            type UnnormFieldsMut = (Result<&'a mut RGFT<Self,This>,RGFE<'a,Self,This>>,);
            type UnnormFieldsRawMut = (Result<*mut RGFT<Self,This>,RGFE<'a,Self,This>>,);

            #[inline(always)]
            fn rev_get_multi_field_mut_impl(
                self,
                this: &'a mut This,
            ) -> (Result<&'a mut RGFT<Self,This>,RGFE<'a,Self,This>>,) {
                (self.rev_get_field_mut(this),)
            }

            #[inline(always)]
            unsafe fn rev_get_multi_field_raw_mut_impl(
                self,
                this: *mut This,
            ) -> (Result<*mut RGFT<Self,This>,RGFE<'a,Self,This>>,) {
                (self.rev_get_field_raw_mut(this),)
            }
        }

    };
}

delegate_multi_field_traits! {
    impl[T] TStr<T>
}

delegate_multi_field_traits! {
    impl[T] NestedFieldPath<T>
}

delegate_multi_field_traits! {
    impl[V,F] VariantField<V,F>
}

delegate_multi_field_traits! {
    impl[T] VariantName<T>
}
