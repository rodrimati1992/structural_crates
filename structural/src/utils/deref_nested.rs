#![allow(non_snake_case)]

/// For dereferencing tuples of pointers into tuples of mutable references.
///
/// # Safety
///
/// The mutable raw pointers in `Self` must be dereferenced into mutable references.
///
/// The `Dereffed` associated type must have the same structure as `Self`,
/// in which the mutable pointers are replaced with mutable references with the `'a` lifetime.
///
/// # Example
///
/// ```rust
/// use structural::field::InfallibleAccess;
/// use structural::utils::DerefNested;
///
/// let mut left  = 100_u32;
/// let mut middle = 200_u32;
/// let mut right = 300_u32;
///
/// type ResRawPtr = Result<*mut u32, InfallibleAccess>;
/// type ResMutRef<'a> = Result<&'a mut u32, InfallibleAccess>;
///
/// let tuple: ((ResRawPtr, ResRawPtr),ResRawPtr) =
///     ((Ok(&mut left as *mut _), Ok(&mut middle as *mut _)), Ok(&mut right as *mut _));
/// unsafe{
///     let mutref_tuple: ((ResMutRef<'_>, ResMutRef<'_>), ResMutRef<'_>) =
///         tuple.deref_nested();
///     assert_eq!(mutref_tuple, ((Ok(&mut 100), Ok(&mut 200)), Ok(&mut 300)));
/// }
/// ```
pub unsafe trait DerefNested<'a> {
    /// A type with the same structure as `Self`,
    /// in which the mutable pointers are replaced with mutable references
    /// with the `'a` lifetime.
    type Dereffed: 'a;

    /// Dereferences the mutable pointers in this into mutable references.
    ///
    /// # Safety
    ///
    /// The raw pointers in `Self` must point to non-dangling, initialized values,
    /// which are valid for the `'a` lifetime.
    unsafe fn deref_nested(self) -> Self::Dereffed;
}

/// The return type of the `DerefNested::deref_nested` method for `This`
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
