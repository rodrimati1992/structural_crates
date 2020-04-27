/*!
Some helper functions.
*/

use std_::{marker::PhantomData, mem::ManuallyDrop};

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

/// Takes the contents out of a `ManuallyDrop<T>`.
///
/// # Safety
///
/// After this function is called `slot` becomes uninitialized and
/// must not be used again.
pub unsafe fn take_manuallydrop<T>(slot: &mut ManuallyDrop<T>) -> T {
    #[cfg(feature = "rust_1_42")]
    {
        ManuallyDrop::take(slot)
    }
    #[cfg(not(feature = "rust_1_42"))]
    {
        ManuallyDrop::into_inner(std_::ptr::read(slot))
    }
}

/////////////////////////////////////////////////////////

/// A wrapper type to run a closure(`F` type parameter) with a value(`T` type parameter).
///
/// This type allows accessing the value before it's passed by value to the closure.
pub struct RunOnDrop<T, F>
where
    F: FnOnce(T),
{
    value: ManuallyDrop<T>,
    function: ManuallyDrop<F>,
}

impl<T, F> RunOnDrop<T, F>
where
    F: FnOnce(T),
{
    /// Constructs this RunOnDrop.
    #[inline(always)]
    pub fn new(value: T, function: F) -> Self {
        Self {
            value: ManuallyDrop::new(value),
            function: ManuallyDrop::new(function),
        }
    }
}

impl<T, F> RunOnDrop<T, F>
where
    F: FnOnce(T),
{
    /// Reborrows the wrapped value mutably.
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        &mut *self.value
    }
}

impl<'a, T, F> RunOnDrop<&'a mut T, F>
where
    F: FnOnce(&'a mut T),
{
    /// Reborrows the wrapped reference.
    #[inline(always)]
    pub fn reborrow(&self) -> &T {
        &*self.value
    }
    /// Reborrows the wrapped reference mutably.
    #[inline(always)]
    pub fn reborrow_mut(&mut self) -> &mut T {
        &mut *self.value
    }
}

impl<'a, T, F> Drop for RunOnDrop<T, F>
where
    F: FnOnce(T),
{
    fn drop(&mut self) {
        unsafe {
            let value = take_manuallydrop(&mut self.value);
            let function = take_manuallydrop(&mut self.function);
            function(value);
        }
    }
}
