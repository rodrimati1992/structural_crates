#![allow(non_snake_case, non_camel_case_types)]

use core_extensions::MarkerType;

use std_::marker::PhantomData;


/// A type-level string,represented as a tuple of type-level bytes.
///
/// This cannot be converted to a constant.
pub struct TString<T>(PhantomData<T>);

impl<T> TString<T>{
    pub const NEW:Self=TString(PhantomData);
}

impl<T> Copy for TString<T>{}
impl<T> Clone for TString<T>{
    fn clone(&self)->Self{
        *self
    }
}
unsafe impl<T> MarkerType for TString<T>{}


macro_rules! create_unit_struct {
    (inner; ($struct_:ident ,$alias:ident ) )=>{
        #[derive(Debug)]
        pub struct $struct_;
        
        pub type $alias=$struct_;
    };
    (inner; ($struct_:ident) )=>{
        #[derive(Debug)]
        pub struct $struct_;
    };
    ($( $param:tt ),* $(,)*) => {
        $(
            create_unit_struct!(inner; $param );
        )*
    }
}


/// A tuple of multiple unique `TString`s
pub struct MultiTString<T>(PhantomData<T>);

impl<T> MultiTString<T>{
    /// Constructs a `MultiTString`.
    ///
    /// # Safety
    ///
    /// `T` must be a tuple of `TString<_>`s,
    /// where no `TString<_>` type is repeated within the tuple.
    pub const unsafe fn new()->Self{
        MultiTString(PhantomData)
    }
}

impl<T> Copy for MultiTString<T>{}
impl<T> Clone for MultiTString<T>{
    fn clone(&self)->Self{
        *self
    }
}

// `MarkerType` is not implemented for `MultiTString` 
// because `MultiTString` ought only be constructible
// by satisfying the safety requirements of `MultiTString::new`,
// which aren't cheaply enforceable on the type level.
//
// impl<T> !MarkerType for MultiTString<T>{}


/*

This is code used to generate the macro invocation.

fn main() {
    let mut list=(0..=255u8) 
        .map(|b|{
            let c=b as char;
            if (c.is_alphanumeric() || c=='_') && b<128 {
                format!("(_{1},B{0}),",b,b as char)
            }else{
                format!("(B{0}),",b)
            }        
        })
        .collect::<Vec<_>>();
    for chunk in list.chunks(8) {
        for param in chunk {
            print!("{}",param);
        }
        println!();
    }
}


*/

create_unit_struct! {
    (B0),(B1),(B2),(B3),(B4),(B5),(B6),(B7),
    (B8),(B9),(B10),(B11),(B12),(B13),(B14),(B15),
    (B16),(B17),(B18),(B19),(B20),(B21),(B22),(B23),
    (B24),(B25),(B26),(B27),(B28),(B29),(B30),(B31),
    (B32),(B33),(B34),(B35),(B36),(B37),(B38),(B39),
    (B40),(B41),(B42),(B43),(B44),(B45),(B46),(B47),
    (_0,B48),(_1,B49),(_2,B50),(_3,B51),(_4,B52),(_5,B53),(_6,B54),(_7,B55),
    (_8,B56),(_9,B57),(B58),(B59),(B60),(B61),(B62),(B63),
    (B64),(_A,B65),(_B,B66),(_C,B67),(_D,B68),(_E,B69),(_F,B70),(_G,B71),
    (_H,B72),(_I,B73),(_J,B74),(_K,B75),(_L,B76),(_M,B77),(_N,B78),(_O,B79),
    (_P,B80),(_Q,B81),(_R,B82),(_S,B83),(_T,B84),(_U,B85),(_V,B86),(_W,B87),
    (_X,B88),(_Y,B89),(_Z,B90),(B91),(B92),(B93),(B94),(__,B95),
    (B96),(_a,B97),(_b,B98),(_c,B99),(_d,B100),(_e,B101),(_f,B102),(_g,B103),
    (_h,B104),(_i,B105),(_j,B106),(_k,B107),(_l,B108),(_m,B109),(_n,B110),(_o,B111),
    (_p,B112),(_q,B113),(_r,B114),(_s,B115),(_t,B116),(_u,B117),(_v,B118),(_w,B119),
    (_x,B120),(_y,B121),(_z,B122),(B123),(B124),(B125),(B126),(B127),
    (B128),(B129),(B130),(B131),(B132),(B133),(B134),(B135),
    (B136),(B137),(B138),(B139),(B140),(B141),(B142),(B143),
    (B144),(B145),(B146),(B147),(B148),(B149),(B150),(B151),
    (B152),(B153),(B154),(B155),(B156),(B157),(B158),(B159),
    (B160),(B161),(B162),(B163),(B164),(B165),(B166),(B167),
    (B168),(B169),(B170),(B171),(B172),(B173),(B174),(B175),
    (B176),(B177),(B178),(B179),(B180),(B181),(B182),(B183),
    (B184),(B185),(B186),(B187),(B188),(B189),(B190),(B191),
    (B192),(B193),(B194),(B195),(B196),(B197),(B198),(B199),
    (B200),(B201),(B202),(B203),(B204),(B205),(B206),(B207),
    (B208),(B209),(B210),(B211),(B212),(B213),(B214),(B215),
    (B216),(B217),(B218),(B219),(B220),(B221),(B222),(B223),
    (B224),(B225),(B226),(B227),(B228),(B229),(B230),(B231),
    (B232),(B233),(B234),(B235),(B236),(B237),(B238),(B239),
    (B240),(B241),(B242),(B243),(B244),(B245),(B246),(B247),
    (B248),(B249),(B250),(B251),(B252),(B253),(B254),(B255),
}
