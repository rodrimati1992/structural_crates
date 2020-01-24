/*!
Some helper functions.
*/

use crate::field_traits::OptionalField;

use std::marker::PhantomData;

/// Used to coerce `&[T;N]` to `&[T]`.
pub const fn coerce_slice<'a, T>(slic: &'a [T]) -> &'a [T] {
    slic
}

/////////////////////////////////////////////////////////

mod opsealed {
    pub trait Sealed {}
}

impl<T> self::opsealed::Sealed for Option<T> {}

pub trait OptionParam_: self::opsealed::Sealed {
    type Param;
}

pub type OptionParam<This> = <This as OptionParam_>::Param;

impl<T> OptionParam_ for Option<T> {
    type Param = T;
}

/////////////////////////////////////////////////////////

#[inline(always)]
pub fn as_phantomdata<T>(_: &T) -> PhantomData<T> {
    PhantomData
}

/////////////////////////////////////////////////////////

// Used to get a `&T` from both a `T` and a `&T`
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait _Structural_BorrowSelf{
    fn _structural_borrow_self(&self)->&Self;
    fn _structural_borrow_self_mut(&mut self)->&mut Self;
}

impl<T> _Structural_BorrowSelf for T
where
    T:?Sized
{
    #[inline(always)]
    fn _structural_borrow_self(&self)->&Self{
        self
    }
    
    #[inline(always)]
    fn _structural_borrow_self_mut(&mut self)->&mut Self{
        self
    }
}


/////////////////////////////////////////////////////////

#[inline(always)]
pub unsafe fn option_as_mut_result<T>(ptr: *mut Option<T>) -> Result<*mut T, OptionalField> {
    match *ptr {
        Some(ref mut x) => Ok(x as *mut T),
        None => Err(OptionalField),
    }
}
