use std_::{
    cell::{Cell, RefCell},
    mem::ManuallyDrop,
    ops::{Deref, Index},
};

///////////////////////////////////////////////////////////////////////////////

pub struct OrOnDrop<'a, T> {
    value: T,
    bits: &'a Cell<u64>,
    bits_to_set: u64,
}

impl<'a, T> OrOnDrop<'a, T> {
    pub fn new(value: T, bits: &'a Cell<u64>, bits_to_set: u64) -> Self {
        Self {
            value,
            bits,
            bits_to_set,
        }
    }
    pub fn into_inner(self) -> T {
        self.on_drop();
        let mut this = ManuallyDrop::new(self);
        unsafe { std_::ptr::read(&mut this.value) }
    }
    pub fn into_inner_and_bits(self) -> (T, u64) {
        let bits = self.bits_to_set();
        (self.into_inner(), bits)
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
pub struct DecOnDrop<'a> {
    counter: &'a Cell<usize>,
}

impl<'a> DecOnDrop<'a> {
    pub fn new(counter: &'a Cell<usize>) -> Self {
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

const FIXED_ARR_LEN: usize = 72;

/// Data structure used to test the order in which fields are dropped.
pub struct FixedArray {
    len: usize,
    arr: [u8; FIXED_ARR_LEN],
}

impl FixedArray {
    pub const fn new() -> Self {
        Self {
            len: 0,
            arr: [0; FIXED_ARR_LEN],
        }
    }
    pub fn push(&mut self, val: u8) {
        assert!(self.len < FIXED_ARR_LEN);
        self.arr[self.len] = val;
        self.len += 1;
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.arr[..self.len]
    }
}

impl Default for FixedArray {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for FixedArray {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.arr[..self.len]
    }
}

impl<I> Index<I> for FixedArray
where
    [u8]: Index<I>,
{
    type Output = <[u8] as Index<I>>::Output;

    fn index(&self, index: I) -> &<[u8] as Index<I>>::Output {
        &self.as_slice()[index]
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct PushOnDrop<'a, T> {
    value: T,
    arr: &'a RefCell<FixedArray>,
    to_push: u8,
}

impl<'a, T> PushOnDrop<'a, T> {
    pub fn new(value: T, arr: &'a RefCell<FixedArray>, to_push: u8) -> Self {
        Self {
            value,
            arr,
            to_push,
        }
    }
    pub fn into_inner(self) -> T {
        self.on_drop();
        let mut this = ManuallyDrop::new(self);
        unsafe { std_::ptr::read(&mut this.value) }
    }
    fn on_drop(&self) {
        self.arr.borrow_mut().push(self.to_push);
    }
}

impl<'a, T> Drop for PushOnDrop<'a, T> {
    fn drop(&mut self) {
        self.on_drop();
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
pub enum RefKind {
    Shared,
    Mutable,
}

pub trait GetRefKind {
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
