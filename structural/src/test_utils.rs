use std_::{cell::Cell, mem::ManuallyDrop};

///////////////////////////////////////////////////////////////////////////////

pub(crate) struct OrOnDrop<'a, T> {
    value: T,
    bits: &'a Cell<u64>,
    bits_to_set: u64,
}

impl<'a, T> OrOnDrop<'a, T> {
    pub(crate) fn new(value: T, bits: &'a Cell<u64>, bits_to_set: u64) -> Self {
        Self {
            value,
            bits,
            bits_to_set,
        }
    }
    pub fn into_inner(self) -> T {
        self.on_drop();
        let mut this = ManuallyDrop::new(self);
        unsafe { std::ptr::read(&mut this.value) }
    }
    pub fn bits_to_set(&self) -> u64 {
        self.bits_to_set
    }
    fn on_drop(&self) {
        let prev = self.bits.get();
        let next = prev | self.bits_to_set;
        assert_ne!(
            prev, next,
            "Expected a different prev and next,found both the the same value:\n{:b}",
            prev,
        );
        self.bits.set(next);
    }
}

impl<'a, T> Drop for OrOnDrop<'a, T> {
    fn drop(&mut self) {
        self.on_drop();
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Used to test that types that manually drop stuff do it correctly.
pub(crate) struct DecOnDrop<'a> {
    counter: &'a Cell<usize>,
}

impl<'a> DecOnDrop<'a> {
    pub(crate) fn new(counter: &'a Cell<usize>) -> Self {
        counter.set(counter.get() + 1);

        Self { counter }
    }
}

impl<'a> Clone for DecOnDrop<'a> {
    fn clone(&self) -> Self {
        self.counter.set(self.counter.get() + 1);
        Self {
            counter: self.counter,
        }
    }
}

impl<'a> Drop for DecOnDrop<'a> {
    fn drop(&mut self) {
        self.counter.set(self.counter.get() - 1);
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RefKind {
    Shared,
    Mutable,
}

pub(crate) trait GetRefKind {
    fn get_ref_kind(self) -> RefKind;
}

impl<T: ?Sized> GetRefKind for &T {
    fn get_ref_kind(self) -> RefKind {
        RefKind::Shared
    }
}

impl<T: ?Sized> GetRefKind for &mut T {
    fn get_ref_kind(self) -> RefKind {
        RefKind::Mutable
    }
}
