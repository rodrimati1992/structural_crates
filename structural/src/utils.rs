/*!
Some helper functions.
*/

use crate::field_traits::OptionalField;

/// Used to coerce `&[T;N]` to `&[T]`.
pub const fn coerce_slice<'a, T>(slic: &'a [T]) -> &'a [T] {
    slic
}

/////////////////////////////////////////////////////////

#[doc(hidden)]
pub trait MakeUnit {
    const UNIT: Self;
}

impl<'a> MakeUnit for () {
    const UNIT: Self = ();
}

impl<'a> MakeUnit for &'a () {
    const UNIT: Self = &();
}

impl<'a> MakeUnit for *mut () {
    const UNIT: Self = 1024 as *mut ();
}

#[inline(always)]
pub fn unit_mut_ref<'a>() -> &'a mut () {
    unsafe { &mut *(1024 as *mut ()) }
}

/////////////////////////////////////////////////////////

#[inline(always)]
pub unsafe fn option_as_mut_result<T>(ptr: *mut Option<T>) -> Result<*mut T, OptionalField> {
    match *ptr {
        Some(ref mut x) => Ok(x as *mut T),
        None => Err(OptionalField),
    }
}
