/*!
Traits and types for comparison.
*/

/// Compares the ordering of Self and Right
pub trait Compare_<Right> {
    type Output;
}

/// Compares the ordering of Left and Right
pub type Compare<Left, Right> = <Left as Compare_<Right>>::Output;

/// A type-level equivalent of `Ordering::Less`
pub struct TLess;

/// A type-level equivalent of `Ordering::Equal`
pub struct TEqual;

/// A type-level equivalent of `Ordering::Greater`
pub struct TGreater;

////////////////////////////////////////////////////////////////////////////////

/// Reverses a type-level `Ordering`.
pub trait ReverseOrdering_ {
    type Output;
}

/// Reverses a type-level `Ordering`.
pub type ReverseOrdering<This> = <This as ReverseOrdering_>::Output;

impl ReverseOrdering_ for TLess {
    type Output = TGreater;
}
impl ReverseOrdering_ for TEqual {
    type Output = TEqual;
}
impl ReverseOrdering_ for TGreater {
    type Output = TLess;
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use core_extensions::type_asserts::AssertEq;

    #[test]
    fn reverse() {
        let _: AssertEq<ReverseOrdering<TLess>, TGreater>;
        let _: AssertEq<ReverseOrdering<TEqual>, TEqual>;
        let _: AssertEq<ReverseOrdering<TGreater>, TLess>;
    }
}
