/*!
Wrapper type(s) where their value is ignored in comparisons .
*/

use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::{self, Debug},
    hash::{Hash, Hasher},
};
/// Wrapper type used to ignore its contents in comparisons.
#[repr(transparent)]
#[derive(Default, Copy, Clone)]
pub struct Ignored<T> {
    pub value: T,
}

impl<T> Ignored<T> {
    pub const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Debug for Ignored<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

impl<T> Eq for Ignored<T> {}

impl<T> PartialEq for Ignored<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Ord for Ignored<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<T> PartialOrd for Ignored<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<T> Hash for Ignored<T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        ().hash(state)
    }
}
