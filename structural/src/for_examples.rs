//! Structural-deriving types used in examples

use crate::Structural;

/// Struct used to demonstrate optional accessors.
#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct Tuple1<A>(#[struc(optional)] pub Option<A>);

/// Struct used to demonstrate optional accessors.
#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct Tuple2<A, B>(#[struc(optional)] pub Option<A>, pub B);

/// Struct used to demonstrate optional accessors.
#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct Tuple3<A, B, C>(#[struc(optional)] pub Option<A>, pub B, pub C);

/// Struct used to demonstrate optional accessors.
#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct Struct2<A, B> {
    #[struc(optional)]
    pub foo: Option<A>,
    pub bar: B,
}

/// Struct used to demonstrate optional accessors.
#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct Struct3<A, B, C> {
    #[struc(optional)]
    pub foo: Option<A>,
    pub bar: B,
    pub baz: C,
}

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct StructFoo<T> {
    pub foo: T,
}

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct StructBar<T> {
    pub bar: T,
}

///////////////////////////////////////////////////////////////////////////////

use std_::cmp::Ordering;

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
#[struc(no_trait)]
pub enum Variants {
    Foo(u32, u64),
    Bar(&'static str),
    Baz(#[struc(optional)] Option<Ordering>),
    Boom {
        a: Option<&'static [u8]>,
        b: &'static [u16],
    },
}

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
#[struc(no_trait)]
pub enum WithBar {
    Nope,
    Bar(&'static str),
}

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
#[struc(no_trait)]
pub enum WithBoom {
    Nope,
    Boom { a: &'static str, b: &'static [u16] },
}

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
#[struc(no_trait)]
pub enum Bomb {
    Nope,
    Boom { a: &'static str, b: &'static [u16] },
    Exploded,
}

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
#[struc(no_trait)]
pub enum EnumOptA {
    Limbs {
        #[struc(optional)]
        legs: Option<usize>,
        hands: Option<usize>,
    },
}

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
#[struc(no_trait)]
pub enum EnumOptFlying {
    Limbs {
        #[struc(optional)]
        legs: Option<usize>,
        hands: Option<usize>,
        noodles: usize,
    },
}
