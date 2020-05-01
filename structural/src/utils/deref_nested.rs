#![allow(non_snake_case)]

pub unsafe trait DerefNested<'a> {
    type Dereffed: 'a;

    /// # Safety
    ///
    /// When `Self` contains any raw pointers,those must point to non-dangling,
    /// initialized values.
    unsafe fn deref_nested(self) -> Self::Dereffed;
}

pub type DerefNestedOut<'a, This> = <This as DerefNested<'a>>::Dereffed;

macro_rules! deref_nested_impl {
    ( $($ident:ident),* $(,)? ) => (
        unsafe impl<'a,$($ident,)*> DerefNested<'a> for ($($ident,)*)
        where
            $($ident: 'a + DerefNested<'a>,)*
        {
            type Dereffed=($($ident::Dereffed,)*);

            #[inline(always)]
            unsafe fn deref_nested(self)->Self::Dereffed {
                let ($($ident,)*)=self;
                ($(
                    <$ident as DerefNested>::deref_nested($ident),
                )*)
            }
        }
    )
}

deref_nested_impl! {}
deref_nested_impl! { F0 }
deref_nested_impl! { F0,F1 }
deref_nested_impl! { F0,F1,F2 }
deref_nested_impl! { F0,F1,F2,F3 }
deref_nested_impl! { F0,F1,F2,F3,F4 }
deref_nested_impl! { F0,F1,F2,F3,F4,F5 }
deref_nested_impl! { F0,F1,F2,F3,F4,F5,F6 }
deref_nested_impl! { F0,F1,F2,F3,F4,F5,F6,F7 }

unsafe impl<'a, T: 'a, E: 'a> DerefNested<'a> for Result<*mut T, E> {
    type Dereffed = Result<&'a mut T, E>;

    #[inline(always)]
    unsafe fn deref_nested(self) -> Self::Dereffed {
        match self {
            Ok(x) => Ok(&mut *x),
            Err(e) => Err(e),
        }
    }
}
