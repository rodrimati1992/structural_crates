//! Structural-deriving types used in examples

use crate::Structural;

/// Struct used to demonstrate optional accessors.
#[derive(Structural, Debug, Copy, Clone, PartialEq)]
pub struct Tuple1<A>(#[struc(optional)] pub Option<A>);

/// Struct used to demonstrate optional accessors.
#[derive(Structural, Debug, Copy, Clone, PartialEq)]
pub struct Tuple2<A, B>(#[struc(optional)] pub Option<A>, pub B);

/// Struct used to demonstrate optional accessors.
#[derive(Structural, Debug, Copy, Clone, PartialEq)]
pub struct Tuple3<A, B, C>(#[struc(optional)] pub Option<A>, pub B, pub C);

/// Struct used to demonstrate optional accessors.
#[derive(Structural, Debug, Copy, Clone, PartialEq)]
pub struct Struct2<A, B> {
    #[struc(optional)]
    pub foo: Option<A>,
    pub bar: B,
}

/// Struct used to demonstrate optional accessors.
#[derive(Structural, Debug, Copy, Clone, PartialEq)]
pub struct Struct3<A, B, C> {
    #[struc(optional)]
    pub foo: Option<A>,
    pub bar: B,
    pub baz: C,
}
