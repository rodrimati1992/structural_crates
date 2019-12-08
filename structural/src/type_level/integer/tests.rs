use super::*;

use crate::type_level::cmp::{Compare, ReverseOrdering};

use core_extensions::type_asserts::AssertEq;

#[test]
fn compare() {
    type AssertCmp<L, R, Expected> = (
        AssertEq<Compare<L, R>, Expected>,
        AssertEq<Compare<R, L>, ReverseOrdering<Expected>>,
    );

    /*
    use std::cmp::{Ord, Ordering};
    use std::collections::HashSet;

    fn main() {
        let len = 33;
        let mut set = HashSet::new();
        for i in 0..=len {
            for j in 0..=len {
                if set.contains(&(j, i)) {
                    continue;
                }
                println!(
                    "(U{0},U{1},T{2:?})",
                    i,
                    j,
                    i.cmp(&j)
                );
                set.insert((i, j));
            }
        }
    }
    */

    macro_rules! cmp_assertions {
        ( $(( $left:ident,$right:ident,$expected:ident ))* ) => {
            $( let _:AssertCmp<$left,$right,$expected>; )*
        };
    }

    cmp_assertions! {
        (U0,U0,TEqual)
        (U0,U1,TLess)
        (U0,U2,TLess)
        (U0,U3,TLess)
        (U0,U4,TLess)
        (U0,U5,TLess)
        (U0,U6,TLess)
        (U0,U7,TLess)
        (U0,U8,TLess)
        (U0,U9,TLess)
        (U0,U10,TLess)
        (U0,U11,TLess)
        (U0,U12,TLess)
        (U0,U13,TLess)
        (U0,U14,TLess)
        (U0,U15,TLess)
        (U0,U16,TLess)
        (U0,U17,TLess)
        (U0,U18,TLess)
        (U0,U19,TLess)
        (U0,U20,TLess)
        (U0,U21,TLess)
        (U0,U22,TLess)
        (U0,U23,TLess)
        (U0,U24,TLess)
        (U0,U25,TLess)
        (U0,U26,TLess)
        (U0,U27,TLess)
        (U0,U28,TLess)
        (U0,U29,TLess)
        (U0,U30,TLess)
        (U0,U31,TLess)
        (U0,U32,TLess)
        (U0,U33,TLess)
        (U1,U1,TEqual)
        (U1,U2,TLess)
        (U1,U3,TLess)
        (U1,U4,TLess)
        (U1,U5,TLess)
        (U1,U6,TLess)
        (U1,U7,TLess)
        (U1,U8,TLess)
        (U1,U9,TLess)
        (U1,U10,TLess)
        (U1,U11,TLess)
        (U1,U12,TLess)
        (U1,U13,TLess)
        (U1,U14,TLess)
        (U1,U15,TLess)
        (U1,U16,TLess)
        (U1,U17,TLess)
        (U1,U18,TLess)
        (U1,U19,TLess)
        (U1,U20,TLess)
        (U1,U21,TLess)
        (U1,U22,TLess)
        (U1,U23,TLess)
        (U1,U24,TLess)
        (U1,U25,TLess)
        (U1,U26,TLess)
        (U1,U27,TLess)
        (U1,U28,TLess)
        (U1,U29,TLess)
        (U1,U30,TLess)
        (U1,U31,TLess)
        (U1,U32,TLess)
        (U1,U33,TLess)
        (U2,U2,TEqual)
        (U2,U3,TLess)
        (U2,U4,TLess)
        (U2,U5,TLess)
        (U2,U6,TLess)
        (U2,U7,TLess)
        (U2,U8,TLess)
        (U2,U9,TLess)
        (U2,U10,TLess)
        (U2,U11,TLess)
        (U2,U12,TLess)
        (U2,U13,TLess)
        (U2,U14,TLess)
        (U2,U15,TLess)
        (U2,U16,TLess)
        (U2,U17,TLess)
        (U2,U18,TLess)
        (U2,U19,TLess)
        (U2,U20,TLess)
        (U2,U21,TLess)
        (U2,U22,TLess)
        (U2,U23,TLess)
        (U2,U24,TLess)
        (U2,U25,TLess)
        (U2,U26,TLess)
        (U2,U27,TLess)
        (U2,U28,TLess)
        (U2,U29,TLess)
        (U2,U30,TLess)
        (U2,U31,TLess)
        (U2,U32,TLess)
        (U2,U33,TLess)
        (U3,U3,TEqual)
        (U3,U4,TLess)
        (U3,U5,TLess)
        (U3,U6,TLess)
        (U3,U7,TLess)
        (U3,U8,TLess)
        (U3,U9,TLess)
        (U3,U10,TLess)
        (U3,U11,TLess)
        (U3,U12,TLess)
        (U3,U13,TLess)
        (U3,U14,TLess)
        (U3,U15,TLess)
        (U3,U16,TLess)
        (U3,U17,TLess)
        (U3,U18,TLess)
        (U3,U19,TLess)
        (U3,U20,TLess)
        (U3,U21,TLess)
        (U3,U22,TLess)
        (U3,U23,TLess)
        (U3,U24,TLess)
        (U3,U25,TLess)
        (U3,U26,TLess)
        (U3,U27,TLess)
        (U3,U28,TLess)
        (U3,U29,TLess)
        (U3,U30,TLess)
        (U3,U31,TLess)
        (U3,U32,TLess)
        (U3,U33,TLess)
        (U4,U4,TEqual)
        (U4,U5,TLess)
        (U4,U6,TLess)
        (U4,U7,TLess)
        (U4,U8,TLess)
        (U4,U9,TLess)
        (U4,U10,TLess)
        (U4,U11,TLess)
        (U4,U12,TLess)
        (U4,U13,TLess)
        (U4,U14,TLess)
        (U4,U15,TLess)
        (U4,U16,TLess)
        (U4,U17,TLess)
        (U4,U18,TLess)
        (U4,U19,TLess)
        (U4,U20,TLess)
        (U4,U21,TLess)
        (U4,U22,TLess)
        (U4,U23,TLess)
        (U4,U24,TLess)
        (U4,U25,TLess)
        (U4,U26,TLess)
        (U4,U27,TLess)
        (U4,U28,TLess)
        (U4,U29,TLess)
        (U4,U30,TLess)
        (U4,U31,TLess)
        (U4,U32,TLess)
        (U4,U33,TLess)
        (U5,U5,TEqual)
        (U5,U6,TLess)
        (U5,U7,TLess)
        (U5,U8,TLess)
        (U5,U9,TLess)
        (U5,U10,TLess)
        (U5,U11,TLess)
        (U5,U12,TLess)
        (U5,U13,TLess)
        (U5,U14,TLess)
        (U5,U15,TLess)
        (U5,U16,TLess)
        (U5,U17,TLess)
        (U5,U18,TLess)
        (U5,U19,TLess)
        (U5,U20,TLess)
        (U5,U21,TLess)
        (U5,U22,TLess)
        (U5,U23,TLess)
        (U5,U24,TLess)
        (U5,U25,TLess)
        (U5,U26,TLess)
        (U5,U27,TLess)
        (U5,U28,TLess)
        (U5,U29,TLess)
        (U5,U30,TLess)
        (U5,U31,TLess)
        (U5,U32,TLess)
        (U5,U33,TLess)
        (U6,U6,TEqual)
        (U6,U7,TLess)
        (U6,U8,TLess)
        (U6,U9,TLess)
        (U6,U10,TLess)
        (U6,U11,TLess)
        (U6,U12,TLess)
        (U6,U13,TLess)
        (U6,U14,TLess)
        (U6,U15,TLess)
        (U6,U16,TLess)
        (U6,U17,TLess)
        (U6,U18,TLess)
        (U6,U19,TLess)
        (U6,U20,TLess)
        (U6,U21,TLess)
        (U6,U22,TLess)
        (U6,U23,TLess)
        (U6,U24,TLess)
        (U6,U25,TLess)
        (U6,U26,TLess)
        (U6,U27,TLess)
        (U6,U28,TLess)
        (U6,U29,TLess)
        (U6,U30,TLess)
        (U6,U31,TLess)
        (U6,U32,TLess)
        (U6,U33,TLess)
        (U7,U7,TEqual)
        (U7,U8,TLess)
        (U7,U9,TLess)
        (U7,U10,TLess)
        (U7,U11,TLess)
        (U7,U12,TLess)
        (U7,U13,TLess)
        (U7,U14,TLess)
        (U7,U15,TLess)
        (U7,U16,TLess)
        (U7,U17,TLess)
        (U7,U18,TLess)
        (U7,U19,TLess)
        (U7,U20,TLess)
        (U7,U21,TLess)
        (U7,U22,TLess)
        (U7,U23,TLess)
        (U7,U24,TLess)
        (U7,U25,TLess)
        (U7,U26,TLess)
        (U7,U27,TLess)
        (U7,U28,TLess)
        (U7,U29,TLess)
        (U7,U30,TLess)
        (U7,U31,TLess)
        (U7,U32,TLess)
        (U7,U33,TLess)
        (U8,U8,TEqual)
        (U8,U9,TLess)
        (U8,U10,TLess)
        (U8,U11,TLess)
        (U8,U12,TLess)
        (U8,U13,TLess)
        (U8,U14,TLess)
        (U8,U15,TLess)
        (U8,U16,TLess)
        (U8,U17,TLess)
        (U8,U18,TLess)
        (U8,U19,TLess)
        (U8,U20,TLess)
        (U8,U21,TLess)
        (U8,U22,TLess)
        (U8,U23,TLess)
        (U8,U24,TLess)
        (U8,U25,TLess)
        (U8,U26,TLess)
        (U8,U27,TLess)
        (U8,U28,TLess)
        (U8,U29,TLess)
        (U8,U30,TLess)
        (U8,U31,TLess)
        (U8,U32,TLess)
        (U8,U33,TLess)
        (U9,U9,TEqual)
        (U9,U10,TLess)
        (U9,U11,TLess)
        (U9,U12,TLess)
        (U9,U13,TLess)
        (U9,U14,TLess)
        (U9,U15,TLess)
        (U9,U16,TLess)
        (U9,U17,TLess)
        (U9,U18,TLess)
        (U9,U19,TLess)
        (U9,U20,TLess)
        (U9,U21,TLess)
        (U9,U22,TLess)
        (U9,U23,TLess)
        (U9,U24,TLess)
        (U9,U25,TLess)
        (U9,U26,TLess)
        (U9,U27,TLess)
        (U9,U28,TLess)
        (U9,U29,TLess)
        (U9,U30,TLess)
        (U9,U31,TLess)
        (U9,U32,TLess)
        (U9,U33,TLess)
        (U10,U10,TEqual)
        (U10,U11,TLess)
        (U10,U12,TLess)
        (U10,U13,TLess)
        (U10,U14,TLess)
        (U10,U15,TLess)
        (U10,U16,TLess)
        (U10,U17,TLess)
        (U10,U18,TLess)
        (U10,U19,TLess)
        (U10,U20,TLess)
        (U10,U21,TLess)
        (U10,U22,TLess)
        (U10,U23,TLess)
        (U10,U24,TLess)
        (U10,U25,TLess)
        (U10,U26,TLess)
        (U10,U27,TLess)
        (U10,U28,TLess)
        (U10,U29,TLess)
        (U10,U30,TLess)
        (U10,U31,TLess)
        (U10,U32,TLess)
        (U10,U33,TLess)
        (U11,U11,TEqual)
        (U11,U12,TLess)
        (U11,U13,TLess)
        (U11,U14,TLess)
        (U11,U15,TLess)
        (U11,U16,TLess)
        (U11,U17,TLess)
        (U11,U18,TLess)
        (U11,U19,TLess)
        (U11,U20,TLess)
        (U11,U21,TLess)
        (U11,U22,TLess)
        (U11,U23,TLess)
        (U11,U24,TLess)
        (U11,U25,TLess)
        (U11,U26,TLess)
        (U11,U27,TLess)
        (U11,U28,TLess)
        (U11,U29,TLess)
        (U11,U30,TLess)
        (U11,U31,TLess)
        (U11,U32,TLess)
        (U11,U33,TLess)
        (U12,U12,TEqual)
        (U12,U13,TLess)
        (U12,U14,TLess)
        (U12,U15,TLess)
        (U12,U16,TLess)
        (U12,U17,TLess)
        (U12,U18,TLess)
        (U12,U19,TLess)
        (U12,U20,TLess)
        (U12,U21,TLess)
        (U12,U22,TLess)
        (U12,U23,TLess)
        (U12,U24,TLess)
        (U12,U25,TLess)
        (U12,U26,TLess)
        (U12,U27,TLess)
        (U12,U28,TLess)
        (U12,U29,TLess)
        (U12,U30,TLess)
        (U12,U31,TLess)
        (U12,U32,TLess)
        (U12,U33,TLess)
        (U13,U13,TEqual)
        (U13,U14,TLess)
        (U13,U15,TLess)
        (U13,U16,TLess)
        (U13,U17,TLess)
        (U13,U18,TLess)
        (U13,U19,TLess)
        (U13,U20,TLess)
        (U13,U21,TLess)
        (U13,U22,TLess)
        (U13,U23,TLess)
        (U13,U24,TLess)
        (U13,U25,TLess)
        (U13,U26,TLess)
        (U13,U27,TLess)
        (U13,U28,TLess)
        (U13,U29,TLess)
        (U13,U30,TLess)
        (U13,U31,TLess)
        (U13,U32,TLess)
        (U13,U33,TLess)
        (U14,U14,TEqual)
        (U14,U15,TLess)
        (U14,U16,TLess)
        (U14,U17,TLess)
        (U14,U18,TLess)
        (U14,U19,TLess)
        (U14,U20,TLess)
        (U14,U21,TLess)
        (U14,U22,TLess)
        (U14,U23,TLess)
        (U14,U24,TLess)
        (U14,U25,TLess)
        (U14,U26,TLess)
        (U14,U27,TLess)
        (U14,U28,TLess)
        (U14,U29,TLess)
        (U14,U30,TLess)
        (U14,U31,TLess)
        (U14,U32,TLess)
        (U14,U33,TLess)
        (U15,U15,TEqual)
        (U15,U16,TLess)
        (U15,U17,TLess)
        (U15,U18,TLess)
        (U15,U19,TLess)
        (U15,U20,TLess)
        (U15,U21,TLess)
        (U15,U22,TLess)
        (U15,U23,TLess)
        (U15,U24,TLess)
        (U15,U25,TLess)
        (U15,U26,TLess)
        (U15,U27,TLess)
        (U15,U28,TLess)
        (U15,U29,TLess)
        (U15,U30,TLess)
        (U15,U31,TLess)
        (U15,U32,TLess)
        (U15,U33,TLess)
        (U16,U16,TEqual)
        (U16,U17,TLess)
        (U16,U18,TLess)
        (U16,U19,TLess)
        (U16,U20,TLess)
        (U16,U21,TLess)
        (U16,U22,TLess)
        (U16,U23,TLess)
        (U16,U24,TLess)
        (U16,U25,TLess)
        (U16,U26,TLess)
        (U16,U27,TLess)
        (U16,U28,TLess)
        (U16,U29,TLess)
        (U16,U30,TLess)
        (U16,U31,TLess)
        (U16,U32,TLess)
        (U16,U33,TLess)
        (U17,U17,TEqual)
        (U17,U18,TLess)
        (U17,U19,TLess)
        (U17,U20,TLess)
        (U17,U21,TLess)
        (U17,U22,TLess)
        (U17,U23,TLess)
        (U17,U24,TLess)
        (U17,U25,TLess)
        (U17,U26,TLess)
        (U17,U27,TLess)
        (U17,U28,TLess)
        (U17,U29,TLess)
        (U17,U30,TLess)
        (U17,U31,TLess)
        (U17,U32,TLess)
        (U17,U33,TLess)
        (U18,U18,TEqual)
        (U18,U19,TLess)
        (U18,U20,TLess)
        (U18,U21,TLess)
        (U18,U22,TLess)
        (U18,U23,TLess)
        (U18,U24,TLess)
        (U18,U25,TLess)
        (U18,U26,TLess)
        (U18,U27,TLess)
        (U18,U28,TLess)
        (U18,U29,TLess)
        (U18,U30,TLess)
        (U18,U31,TLess)
        (U18,U32,TLess)
        (U18,U33,TLess)
        (U19,U19,TEqual)
        (U19,U20,TLess)
        (U19,U21,TLess)
        (U19,U22,TLess)
        (U19,U23,TLess)
        (U19,U24,TLess)
        (U19,U25,TLess)
        (U19,U26,TLess)
        (U19,U27,TLess)
        (U19,U28,TLess)
        (U19,U29,TLess)
        (U19,U30,TLess)
        (U19,U31,TLess)
        (U19,U32,TLess)
        (U19,U33,TLess)
        (U20,U20,TEqual)
        (U20,U21,TLess)
        (U20,U22,TLess)
        (U20,U23,TLess)
        (U20,U24,TLess)
        (U20,U25,TLess)
        (U20,U26,TLess)
        (U20,U27,TLess)
        (U20,U28,TLess)
        (U20,U29,TLess)
        (U20,U30,TLess)
        (U20,U31,TLess)
        (U20,U32,TLess)
        (U20,U33,TLess)
        (U21,U21,TEqual)
        (U21,U22,TLess)
        (U21,U23,TLess)
        (U21,U24,TLess)
        (U21,U25,TLess)
        (U21,U26,TLess)
        (U21,U27,TLess)
        (U21,U28,TLess)
        (U21,U29,TLess)
        (U21,U30,TLess)
        (U21,U31,TLess)
        (U21,U32,TLess)
        (U21,U33,TLess)
        (U22,U22,TEqual)
        (U22,U23,TLess)
        (U22,U24,TLess)
        (U22,U25,TLess)
        (U22,U26,TLess)
        (U22,U27,TLess)
        (U22,U28,TLess)
        (U22,U29,TLess)
        (U22,U30,TLess)
        (U22,U31,TLess)
        (U22,U32,TLess)
        (U22,U33,TLess)
        (U23,U23,TEqual)
        (U23,U24,TLess)
        (U23,U25,TLess)
        (U23,U26,TLess)
        (U23,U27,TLess)
        (U23,U28,TLess)
        (U23,U29,TLess)
        (U23,U30,TLess)
        (U23,U31,TLess)
        (U23,U32,TLess)
        (U23,U33,TLess)
        (U24,U24,TEqual)
        (U24,U25,TLess)
        (U24,U26,TLess)
        (U24,U27,TLess)
        (U24,U28,TLess)
        (U24,U29,TLess)
        (U24,U30,TLess)
        (U24,U31,TLess)
        (U24,U32,TLess)
        (U24,U33,TLess)
        (U25,U25,TEqual)
        (U25,U26,TLess)
        (U25,U27,TLess)
        (U25,U28,TLess)
        (U25,U29,TLess)
        (U25,U30,TLess)
        (U25,U31,TLess)
        (U25,U32,TLess)
        (U25,U33,TLess)
        (U26,U26,TEqual)
        (U26,U27,TLess)
        (U26,U28,TLess)
        (U26,U29,TLess)
        (U26,U30,TLess)
        (U26,U31,TLess)
        (U26,U32,TLess)
        (U26,U33,TLess)
        (U27,U27,TEqual)
        (U27,U28,TLess)
        (U27,U29,TLess)
        (U27,U30,TLess)
        (U27,U31,TLess)
        (U27,U32,TLess)
        (U27,U33,TLess)
        (U28,U28,TEqual)
        (U28,U29,TLess)
        (U28,U30,TLess)
        (U28,U31,TLess)
        (U28,U32,TLess)
        (U28,U33,TLess)
        (U29,U29,TEqual)
        (U29,U30,TLess)
        (U29,U31,TLess)
        (U29,U32,TLess)
        (U29,U33,TLess)
        (U30,U30,TEqual)
        (U30,U31,TLess)
        (U30,U32,TLess)
        (U30,U33,TLess)
        (U31,U31,TEqual)
        (U31,U32,TLess)
        (U31,U33,TLess)
        (U32,U32,TEqual)
        (U32,U33,TLess)
        (U33,U33,TEqual)
    }
}
