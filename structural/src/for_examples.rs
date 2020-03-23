//! Structural-deriving types used in examples,
//!
//! These are in the docs purely so that documentation examples only use
//! types that are documented.

use crate::Structural;

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct Tuple1<A>(pub Option<A>);

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct Tuple2<A, B>(pub Option<A>, pub B);

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct Tuple3<A, B, C>(pub Option<A>, pub B, pub C);

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct Struct2<A, B> {
    pub foo: Option<A>,
    pub bar: B,
}

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
pub struct Struct3<A, B, C> {
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

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct Delegating<T> {
    #[struc(delegate_to)]
    bar: T,
}

///////////////////////////////////////////////////////////////////////////////

use std_::cmp::Ordering;

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
#[struc(no_trait)]
#[struc(variant_count_alias)]
pub enum Variants {
    Foo(u32, u64),
    Bar(&'static str),
    Baz(Option<Ordering>),
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
        legs: Option<usize>,
        hands: Option<usize>,
    },
}

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
pub enum EnumOptFlying {
    Limbs {
        legs: Option<usize>,
        hands: Option<usize>,
        noodles: usize,
    },
}

#[derive(Structural)]
pub enum Vegetable {
    #[struc(rename = "Ziemniak")]
    Potato {
        #[struc(rename = "centymetry objętości")]
        volume_cm: u32,
    },
    #[struc(rename = "生菜")]
    Letuce {
        #[struc(rename = "树叶")]
        leaves: u32,
    },
}

#[derive(Structural)]
// #[struc(debug_print)]
pub enum EnumWithNewtype<'a> {
    #[struc(newtype(bounds = "RefWrapper_VSI<'a,u32,@variant>"))]
    U32(RefWrapper<'a, u32>),
    #[struc(newtype(bounds = "RefWrapper_VSI<'a,u64,@variant>"))]
    U64(RefWrapper<'a, u64>),
}

#[derive(Structural)]
#[struc(public)]
#[struc(bound = "T:'a")]
pub struct RefWrapper<'a, T>(T, &'a T);

///////////////////////////////////////////////////////////////////////////////

structural_alias! {
    pub trait AStructuralAlias{
        a:u32,
        b:i32,
        Foo{
            c:(),
            d:i8,
        },
        Bar(i16,u16),
    }
}
