#[macro_export]
macro_rules! declare_variant_proxy {
    ( $proxy:ident ) => {
        #[derive(Debug, Copy, Clone)]
        struct $proxy<T: ?Sized, V> {
            _marker: $crate::pmr::PhantomData<V>,
            value: T,
        }

        impl<T: ?Sized, V> $proxy<T, V> {
            #[inline(always)]
            const unsafe fn new(value: T) -> Self
            where
                T: Sized,
            {
                Self {
                    value,
                    _marker: $crate::pmr::PhantomData,
                }
            }

            #[inline(always)]
            unsafe fn from_ref(reference: &T) -> &Self {
                &*(reference as *const T as *const $proxy<T, V>)
            }

            #[inline(always)]
            unsafe fn from_mut(reference: &mut T) -> &mut Self {
                &mut *Self::from_raw_mut(reference)
            }

            #[inline(always)]
            const unsafe fn from_raw_mut(ptr: *mut T) -> *mut Self {
                ptr as *mut $proxy<T, V>
            }
        }
    };
}
