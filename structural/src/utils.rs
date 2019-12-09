/*!
Some helper functions.
*/

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
