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

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
pub enum Enum2 {
    Foo(u8, u16),
    Bar(Ordering, Option<u64>),
}

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
pub enum Enum3 {
    Foo(u8, u16),
    Bar(Ordering, Option<u64>),
    Baz { foom: &'static str },
}

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
pub enum Enum4 {
    Foo(u8, u16),
    Bar(Ordering, Option<u64>),
    Baz { foom: &'static str },
    Qux { uh: [u8; 4], what: (bool, bool) },
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Structural, Copy, Clone)]
// #[struc(debug_print)]
pub enum EnumWithNewtype<'a> {
    #[struc(newtype(bounds = "RefWrapper_VSI<'a,u32,@variant>"))]
    U32(RefWrapper<'a, u32>),
    #[struc(newtype(bounds = "RefWrapper_VSI<'a,u64,@variant>"))]
    U64(RefWrapper<'a, u64>),
}

#[derive(Structural, Copy, Clone)]
#[struc(bound = "T:'a")]
pub struct RefWrapper<'a, T>(pub T, pub &'a T);

#[derive(Structural, Copy, Clone)]
pub enum EnumWithoutNewtype<'a> {
    U32(u32, &'a u32),
    U64(u64, &'a u64),
}

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

tstr_aliases! {
    Ooh_TStr = Ooh,
    Qux_TStr = Qux,
}

structural_alias! {
    pub trait WithGenericNames<B,C,D>{
        <TS!(a)>:u32,
        <B>:i32,
        Foo{
            c:(),
            ref <FP!("what the")>:i8,
            mut move <C>:&'static str,
            <Ooh_TStr>:&'static str,
        },
        Bar(i16,u16),
        ref <TS!(Baz)>,
        mut <Qux_TStr>,
        mut move <D>,
    }
}
