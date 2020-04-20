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

//////////////////////////////////

/// Information about a panic,used in `ffi_panic_message`.
#[derive(Debug, Copy, Clone)]
pub struct PanicInfo {
    pub file: &'static str,
    pub line: u32,
    pub context: &'static str,
}

/// Prints an error message for attempting to panic out of the `abort_on_return` macro.
#[inline(never)]
#[cold]
pub fn ffi_panic_message(info: &'static PanicInfo) -> ! {
    panic!(
        "\n\
        Attempted to panic.\n\
        file:{}\n\
        line:{}\n\
        {}\n\
        Aborting to handle the panic...\n\
        ",
        info.file, info.line, info.context,
    )
}

#[doc(hidden)]
pub struct AbortBomb {
    pub fuse: &'static PanicInfo,
}

impl Drop for AbortBomb {
    fn drop(&mut self) {
        ffi_panic_message(self.fuse);
    }
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
