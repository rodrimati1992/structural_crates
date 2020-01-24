use std_::cell::Cell;

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

#[derive(Debug,PartialEq,Eq)]
pub(crate) enum RefKind{
    Shared,
    Mutable,
}

pub(crate) trait GetRefKind{
    fn get_ref_kind(self)->RefKind;
}

impl<T:?Sized> GetRefKind for &T {
    fn get_ref_kind(self)->RefKind{
        RefKind::Shared
    }
}

impl<T:?Sized> GetRefKind for &mut T {
    fn get_ref_kind(self)->RefKind{
        RefKind::Mutable
    }
}
