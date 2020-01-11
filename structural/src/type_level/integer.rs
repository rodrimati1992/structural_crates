/*!
Small,type-level integers.
*/

use crate::type_level::{
    cmp::{Compare_, TEqual, TGreater, TLess},
    to_value_traits::ToUsize,
};

use std_::marker::PhantomData;

#[cfg(test)]
mod tests;

/////////////////////////////////////////////////////////////////////

mod sealed {
    pub trait Sealed {}
}

use self::sealed::Sealed;

/// A marker trait for type-level unsigned integers.
pub trait IsUnsigned: Sealed {}

impl<T> Sealed for Unsigned<T> {}
impl<T> IsUnsigned for Unsigned<T> {}

/////////////////////////////////////////////////////////////////////

pub trait IsBit: Sealed {
    const VALUE: bool;
}

impl Sealed for Bit0 {}
impl Sealed for Bit1 {}

impl IsBit for Bit0 {
    const VALUE: bool = false;
}
impl IsBit for Bit1 {
    const VALUE: bool = true;
}

/////////////////////////////////////////////////////////////////////

/// Represents a small,type-level unsigned integer
pub struct Unsigned<T>(PhantomData<T>);

/// Represents a `0` bit inside of `Unsigned`
pub struct Bit0;

/// Represents a `1` bit inside of `Unsigned`
pub struct Bit1;

/////////////////////////////////////////////////////////////////////

/*
fn main() {
    for i in 0..=33{
        println!("An unsigned {}",i);
        print!("pub type U{}=Unsigned<(",i);
        for bit_ind in (0..6).rev() {
            let bit=1 << bit_ind;
            print!("Bit{},",if i&bit==0 { 0 }else{ 1 } );
        }
        println!(")>;");
    }
}
*/

/// A type-level 0 (unsigned)
pub type U0 = Unsigned<(Bit0, Bit0, Bit0, Bit0, Bit0, Bit0)>;

/// A type-level 1 (unsigned)
pub type U1 = Unsigned<(Bit0, Bit0, Bit0, Bit0, Bit0, Bit1)>;

/// A type-level 2 (unsigned)
pub type U2 = Unsigned<(Bit0, Bit0, Bit0, Bit0, Bit1, Bit0)>;

/// A type-level 3 (unsigned)
pub type U3 = Unsigned<(Bit0, Bit0, Bit0, Bit0, Bit1, Bit1)>;

/// A type-level 4 (unsigned)
pub type U4 = Unsigned<(Bit0, Bit0, Bit0, Bit1, Bit0, Bit0)>;

/// A type-level 5 (unsigned)
pub type U5 = Unsigned<(Bit0, Bit0, Bit0, Bit1, Bit0, Bit1)>;

/// A type-level 6 (unsigned)
pub type U6 = Unsigned<(Bit0, Bit0, Bit0, Bit1, Bit1, Bit0)>;

/// A type-level 7 (unsigned)
pub type U7 = Unsigned<(Bit0, Bit0, Bit0, Bit1, Bit1, Bit1)>;

/// A type-level 8 (unsigned)
pub type U8 = Unsigned<(Bit0, Bit0, Bit1, Bit0, Bit0, Bit0)>;

/// A type-level 9 (unsigned)
pub type U9 = Unsigned<(Bit0, Bit0, Bit1, Bit0, Bit0, Bit1)>;

/// A type-level 10 (unsigned)
pub type U10 = Unsigned<(Bit0, Bit0, Bit1, Bit0, Bit1, Bit0)>;

/// A type-level 11 (unsigned)
pub type U11 = Unsigned<(Bit0, Bit0, Bit1, Bit0, Bit1, Bit1)>;

/// A type-level 12 (unsigned)
pub type U12 = Unsigned<(Bit0, Bit0, Bit1, Bit1, Bit0, Bit0)>;

/// A type-level 13 (unsigned)
pub type U13 = Unsigned<(Bit0, Bit0, Bit1, Bit1, Bit0, Bit1)>;

/// A type-level 14 (unsigned)
pub type U14 = Unsigned<(Bit0, Bit0, Bit1, Bit1, Bit1, Bit0)>;

/// A type-level 15 (unsigned)
pub type U15 = Unsigned<(Bit0, Bit0, Bit1, Bit1, Bit1, Bit1)>;

/// A type-level 16 (unsigned)
pub type U16 = Unsigned<(Bit0, Bit1, Bit0, Bit0, Bit0, Bit0)>;

/// A type-level 17 (unsigned)
pub type U17 = Unsigned<(Bit0, Bit1, Bit0, Bit0, Bit0, Bit1)>;

/// A type-level 18 (unsigned)
pub type U18 = Unsigned<(Bit0, Bit1, Bit0, Bit0, Bit1, Bit0)>;

/// A type-level 19 (unsigned)
pub type U19 = Unsigned<(Bit0, Bit1, Bit0, Bit0, Bit1, Bit1)>;

/// A type-level 20 (unsigned)
pub type U20 = Unsigned<(Bit0, Bit1, Bit0, Bit1, Bit0, Bit0)>;

/// A type-level 21 (unsigned)
pub type U21 = Unsigned<(Bit0, Bit1, Bit0, Bit1, Bit0, Bit1)>;

/// A type-level 22 (unsigned)
pub type U22 = Unsigned<(Bit0, Bit1, Bit0, Bit1, Bit1, Bit0)>;

/// A type-level 23 (unsigned)
pub type U23 = Unsigned<(Bit0, Bit1, Bit0, Bit1, Bit1, Bit1)>;

/// A type-level 24 (unsigned)
pub type U24 = Unsigned<(Bit0, Bit1, Bit1, Bit0, Bit0, Bit0)>;

/// A type-level 25 (unsigned)
pub type U25 = Unsigned<(Bit0, Bit1, Bit1, Bit0, Bit0, Bit1)>;

/// A type-level 26 (unsigned)
pub type U26 = Unsigned<(Bit0, Bit1, Bit1, Bit0, Bit1, Bit0)>;

/// A type-level 27 (unsigned)
pub type U27 = Unsigned<(Bit0, Bit1, Bit1, Bit0, Bit1, Bit1)>;

/// A type-level 28 (unsigned)
pub type U28 = Unsigned<(Bit0, Bit1, Bit1, Bit1, Bit0, Bit0)>;

/// A type-level 29 (unsigned)
pub type U29 = Unsigned<(Bit0, Bit1, Bit1, Bit1, Bit0, Bit1)>;

/// A type-level 30 (unsigned)
pub type U30 = Unsigned<(Bit0, Bit1, Bit1, Bit1, Bit1, Bit0)>;

/// A type-level 31 (unsigned)
pub type U31 = Unsigned<(Bit0, Bit1, Bit1, Bit1, Bit1, Bit1)>;

/// A type-level 32 (unsigned)
pub type U32 = Unsigned<(Bit1, Bit0, Bit0, Bit0, Bit0, Bit0)>;

/// A type-level 33 (unsigned)
pub type U33 = Unsigned<(Bit1, Bit0, Bit0, Bit0, Bit0, Bit1)>;

/////////////////////////////////////////////////////////////////////

/// Compares bits,taking into account the ordering of higher bits.
#[doc(hidden)]
pub trait CompareBit<HigherBitOrd, Right> {
    type Output;
}

impl CompareBit<TEqual, Bit1> for Bit0 {
    type Output = TLess;
}
impl<This> CompareBit<TEqual, This> for This {
    type Output = TEqual;
}
impl CompareBit<TEqual, Bit0> for Bit1 {
    type Output = TGreater;
}

impl<This, Other> CompareBit<TLess, Other> for This {
    type Output = TLess;
}
impl<This, Other> CompareBit<TGreater, Other> for This {
    type Output = TGreater;
}

/////////////////////////////////////////////////////////////////////

/*
fn main() {
    let len=6;

    print!("impl<");
    for i in 0..len{
        print!("L{0},R{0},T{0},",i);
    }
    print!(">\n    Compare_<Unsigned<(");
    for i in 0..len{
        print!("R{0},",i);
    }
    print!(")>>\nfor Unsigned<(");
    for i in 0..len{
        print!("L{0},",i);
    }
    print!(")>\nwhere\n");
    println!("    L0:CompareBit<TEqual,R0,Output=T0>,");
    for i in 1..len{
        println!("    L{0}:CompareBit<T{1},R{0},Output=T{0}>,",i,i-1);
    }
    println!("{{\n    type Output=T{};\n}}",len-1);
}
*/

impl<L0, R0, T0, L1, R1, T1, L2, R2, T2, L3, R3, T3, L4, R4, T4, L5, R5, T5>
    Compare_<Unsigned<(R0, R1, R2, R3, R4, R5)>> for Unsigned<(L0, L1, L2, L3, L4, L5)>
where
    L0: CompareBit<TEqual, R0, Output = T0>,
    L1: CompareBit<T0, R1, Output = T1>,
    L2: CompareBit<T1, R2, Output = T2>,
    L3: CompareBit<T2, R3, Output = T3>,
    L4: CompareBit<T3, R4, Output = T4>,
    L5: CompareBit<T4, R5, Output = T5>,
{
    type Output = T5;
}

/////////////////////////////////////////////////////////////////////

impl<B5, B4, B3, B2, B1, B0> ToUsize for Unsigned<(B5, B4, B3, B2, B1, B0)>
where
    B5: IsBit,
    B4: IsBit,
    B3: IsBit,
    B2: IsBit,
    B1: IsBit,
    B0: IsBit,
{
    const USIZE: usize = {
        ((B5::VALUE as usize) << 5)
            | ((B4::VALUE as usize) << 4)
            | ((B3::VALUE as usize) << 3)
            | ((B2::VALUE as usize) << 2)
            | ((B1::VALUE as usize) << 1)
            | (B0::VALUE as usize)
    };
}
