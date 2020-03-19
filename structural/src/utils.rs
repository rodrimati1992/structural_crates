/*!
Some helper functions.
*/

use std_::marker::PhantomData;

/////////////////////////////////////////////////////////

/// Defined this function just in case that `unreachable_unchecked`
/// doesn't optimize as expected.
#[inline(always)]
#[doc(hidden)]
pub unsafe fn unreachable_unchecked() -> ! {
    std_::hint::unreachable_unchecked()
}

/////////////////////////////////////////////////////////

/// Gets a `PhantomData<T>`.
#[inline(always)]
pub fn as_phantomdata<T>(_: &T) -> PhantomData<T> {
    PhantomData
}

/////////////////////////////////////////////////////////

// Used to get a `&T` from both a `T` and a `&T`
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait _Structural_BorrowSelf {
    fn _structural_borrow_self(&self) -> &Self;
    fn _structural_borrow_self_mut(&mut self) -> &mut Self;
}

impl<T> _Structural_BorrowSelf for T
where
    T: ?Sized,
{
    #[inline(always)]
    fn _structural_borrow_self(&self) -> &Self {
        self
    }

    #[inline(always)]
    fn _structural_borrow_self_mut(&mut self) -> &mut Self {
        self
    }
}

/////////////////////////////////////////////////////////
