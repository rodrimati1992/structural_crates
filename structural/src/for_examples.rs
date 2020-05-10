#![allow(missing_docs)]

use crate::{
    convert::{EmptyTryFromError, TryFromError},
    IntoField, Structural, StructuralExt,
};

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
pub struct Tuple4<A, B, C, D>(pub Option<A>, pub B, pub C, pub D);

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct Tuple16<P01, P02, P03, P04, P05, P06, P07, P08, P09, P10, P11, P12, P13, P14, P15, P16>(
    pub P01,
    pub P02,
    pub P03,
    pub P04,
    pub P05,
    pub P06,
    pub P07,
    pub P08,
    pub P09,
    pub P10,
    pub P11,
    pub P12,
    pub P13,
    pub P14,
    pub P15,
    pub P16,
);

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

z_impl_from_structural! {
    impl[F, T] FromStructural<F> for StructFoo<T>
    where[ F: IntoField<TS!(foo), Ty = T>, ]
    {
        fn from_structural(this){
            Self {
                foo: this.into_field(fp!(foo)),
            }
        }
    }
}

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub struct StructBar<T> {
    pub bar: T,
}

z_impl_from_structural! {
    impl[F, T] FromStructural<F> for StructBar<T>
    where[ F: IntoField<TS!(bar), Ty = T>, ]
    {
        fn from_structural(this){
            Self {
                bar: this.into_field(fp!(bar)),
            }
        }
    }
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
pub enum Enum1 {
    Foo(u8, u16),
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

structural::z_impl_try_from_structural_for_enum! {
    impl[F] TryFromStructural<F> for Enum2
    where[ F: Enum2_SI, ]
    {
        type Error = EmptyTryFromError;

        fn try_from_structural(this){
            switch! {this;
                Foo(a,b) => Ok(Self::Foo(a,b)),
                Bar(a,b) => Ok(Self::Bar(a,b)),
                _ => Err(TryFromError::with_empty_error(this)),
            }
        }
    }

    FromStructural
    where[ F: Enum2_ESI, ]
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Structural)]
// #[struc(debug_print)]
#[derive(Copy, Clone)]
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

#[derive(Debug, Copy, Clone, Structural, PartialEq)]
pub struct MaxFields<T>(
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
    pub T,
);

///////////////////////////////////////////////////////////////////////////////

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub enum OptionLike<T> {
    Some(T),
    None,
}

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub enum ExtraOption<T> {
    Some(T),
    None,
    FileNotFound,
}

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub enum ResultLike<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub enum ExtraResult<T, E> {
    Ok(T),
    Err(E),
    Warn,
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
