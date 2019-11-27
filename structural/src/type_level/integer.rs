/*!
Small,type-level integers.
*/

use crate::type_level::cmp::{Compare_,TLess,TEqual,TGreater};

use std_::marker::PhantomData;


#[cfg(test)]
mod tests;

/////////////////////////////////////////////////////////////////////

mod sealed{
    pub trait Sealed{}
}

use self::sealed::Sealed;

/// A marker trait for type-level unsigned integers.
pub trait IsUnsigned:Sealed{}

impl<T> Sealed for Unsigned<T>{}
impl<T> IsUnsigned for Unsigned<T>{}


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
        print!("pub type U{}=Unsigned<(",i);
        for bit_ind in (0..6).rev() {
            let bit=1 << bit_ind;
            print!("Bit{},",if i&bit==0 { 0 }else{ 1 } );
        }
        println!(")>;");
    }
}
*/

pub type U0=Unsigned<(Bit0,Bit0,Bit0,Bit0,Bit0,Bit0,)>;
pub type U1=Unsigned<(Bit0,Bit0,Bit0,Bit0,Bit0,Bit1,)>;
pub type U2=Unsigned<(Bit0,Bit0,Bit0,Bit0,Bit1,Bit0,)>;
pub type U3=Unsigned<(Bit0,Bit0,Bit0,Bit0,Bit1,Bit1,)>;
pub type U4=Unsigned<(Bit0,Bit0,Bit0,Bit1,Bit0,Bit0,)>;
pub type U5=Unsigned<(Bit0,Bit0,Bit0,Bit1,Bit0,Bit1,)>;
pub type U6=Unsigned<(Bit0,Bit0,Bit0,Bit1,Bit1,Bit0,)>;
pub type U7=Unsigned<(Bit0,Bit0,Bit0,Bit1,Bit1,Bit1,)>;
pub type U8=Unsigned<(Bit0,Bit0,Bit1,Bit0,Bit0,Bit0,)>;
pub type U9=Unsigned<(Bit0,Bit0,Bit1,Bit0,Bit0,Bit1,)>;
pub type U10=Unsigned<(Bit0,Bit0,Bit1,Bit0,Bit1,Bit0,)>;
pub type U11=Unsigned<(Bit0,Bit0,Bit1,Bit0,Bit1,Bit1,)>;
pub type U12=Unsigned<(Bit0,Bit0,Bit1,Bit1,Bit0,Bit0,)>;
pub type U13=Unsigned<(Bit0,Bit0,Bit1,Bit1,Bit0,Bit1,)>;
pub type U14=Unsigned<(Bit0,Bit0,Bit1,Bit1,Bit1,Bit0,)>;
pub type U15=Unsigned<(Bit0,Bit0,Bit1,Bit1,Bit1,Bit1,)>;
pub type U16=Unsigned<(Bit0,Bit1,Bit0,Bit0,Bit0,Bit0,)>;
pub type U17=Unsigned<(Bit0,Bit1,Bit0,Bit0,Bit0,Bit1,)>;
pub type U18=Unsigned<(Bit0,Bit1,Bit0,Bit0,Bit1,Bit0,)>;
pub type U19=Unsigned<(Bit0,Bit1,Bit0,Bit0,Bit1,Bit1,)>;
pub type U20=Unsigned<(Bit0,Bit1,Bit0,Bit1,Bit0,Bit0,)>;
pub type U21=Unsigned<(Bit0,Bit1,Bit0,Bit1,Bit0,Bit1,)>;
pub type U22=Unsigned<(Bit0,Bit1,Bit0,Bit1,Bit1,Bit0,)>;
pub type U23=Unsigned<(Bit0,Bit1,Bit0,Bit1,Bit1,Bit1,)>;
pub type U24=Unsigned<(Bit0,Bit1,Bit1,Bit0,Bit0,Bit0,)>;
pub type U25=Unsigned<(Bit0,Bit1,Bit1,Bit0,Bit0,Bit1,)>;
pub type U26=Unsigned<(Bit0,Bit1,Bit1,Bit0,Bit1,Bit0,)>;
pub type U27=Unsigned<(Bit0,Bit1,Bit1,Bit0,Bit1,Bit1,)>;
pub type U28=Unsigned<(Bit0,Bit1,Bit1,Bit1,Bit0,Bit0,)>;
pub type U29=Unsigned<(Bit0,Bit1,Bit1,Bit1,Bit0,Bit1,)>;
pub type U30=Unsigned<(Bit0,Bit1,Bit1,Bit1,Bit1,Bit0,)>;
pub type U31=Unsigned<(Bit0,Bit1,Bit1,Bit1,Bit1,Bit1,)>;
pub type U32=Unsigned<(Bit1,Bit0,Bit0,Bit0,Bit0,Bit0,)>;
pub type U33=Unsigned<(Bit1,Bit0,Bit0,Bit0,Bit0,Bit1,)>;


/////////////////////////////////////////////////////////////////////

/// Compares bits,taking into account the ordering of higher bits.
#[doc(hidden)]
pub trait CompareBit<HigherBitOrd,Right>{
    type Output;
}

impl CompareBit<TEqual,Bit1> for Bit0{
    type Output=TLess;
}
impl<This> CompareBit<TEqual,This> for This{
    type Output=TEqual;
}
impl CompareBit<TEqual,Bit0> for Bit1{
    type Output=TGreater;
}

impl<This,Other> CompareBit<TLess,Other> for This{
    type Output=TLess;
}
impl<This,Other> CompareBit<TGreater,Other> for This{
    type Output=TGreater;
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

impl<L0,R0,T0,L1,R1,T1,L2,R2,T2,L3,R3,T3,L4,R4,T4,L5,R5,T5,>
    Compare_<Unsigned<(R0,R1,R2,R3,R4,R5,)>>
for Unsigned<(L0,L1,L2,L3,L4,L5,)>
where
    L0:CompareBit<TEqual,R0,Output=T0>,
    L1:CompareBit<T0,R1,Output=T1>,
    L2:CompareBit<T1,R2,Output=T2>,
    L3:CompareBit<T2,R3,Output=T3>,
    L4:CompareBit<T3,R4,Output=T4>,
    L5:CompareBit<T4,R5,Output=T5>,
{
    type Output=T5;
}


/////////////////////////////////////////////////////////////////////





