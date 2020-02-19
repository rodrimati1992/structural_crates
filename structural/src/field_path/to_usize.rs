use crate::field_path::FieldPath1;
use crate::p::TStrPriv;
use crate::type_level::to_value_traits::ToUsize;

// macros can contain arbitrary syntax,
// which allows this to be defined in this file even if Rust stops parsing `const IDENT:Foo`
#[cfg(feature = "use_const_str")]
macro_rules! declare_const_impls {
    () => {
        use crate::const_generic_utils::str_to_usize;

        impl<const S: &'static str> ToUsize for TStrPriv<S> {
            const USIZE: usize = str_to_usize(S);
        }
    };
}

#[cfg(feature = "use_const_str")]
declare_const_impls! {}

#[cfg(not(feature = "use_const_str"))]
mod tstr_type_param {
    use super::*;

    use crate::type_level::to_value_traits::ToDigit;

    macro_rules! impl_to_usize {
        ( $($typ:ident)* ) => (

            impl<$($typ,)*> ToUsize for TStrPriv<($($typ,)*)>
            where
                $($typ:ToDigit,)*
            {
                const USIZE:usize={
                    #[allow(unused_mut)]
                    let mut num:usize=0;
                    $(
                        num*=10;
                        num+=$typ::DIGIT as usize;
                    )*
                    num
                };
            }
        )
    }

    /*
    fn main(){
        for i in 1..=20 {
            print!("impl_to_usize!{{ ");
            for j in 0..i {
                print!("P{} ",j);
            }
            println!("}}");
        }
    }
    */

    impl_to_usize! {}
    impl_to_usize! { P0 }
    impl_to_usize! { P0 P1 }
    impl_to_usize! { P0 P1 P2 }
    impl_to_usize! { P0 P1 P2 P3 }
    impl_to_usize! { P0 P1 P2 P3 P4 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15 P16 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15 P16 P17 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15 P16 P17 P18 }
    impl_to_usize! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15 P16 P17 P18 P19 }
}

/// Single index `FieldPath`s up to 19 digits can be converted to usize
/// (assuming that usize can store that number)
impl<S> ToUsize for FieldPath1<S>
where
    S: ToUsize,
{
    const USIZE: usize = S::USIZE;
}
