use std_::fmt::{self, Display};

mod sealed {
    pub trait Sealed {}
}
use self::sealed::Sealed;

/// Marker trait for the errors that can be returned from the field accessor traits.
///
/// The errors can be:
///
/// - `std::convert::infallible` / `core::convert::infallible`:
///     An error type that cannot be constructed,used when a field always exists.
///
/// - `structural::field_traits::OptionalField`:
///     Used when a field is optional.
///
/// This trait is sealed,and cannot be implemented outside of the `structural` crate.
pub trait FieldErr: Sealed {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OptionalField;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonOptField {}

impl Sealed for OptionalField {}
impl FieldErr for OptionalField {}

impl Sealed for NonOptField {}
impl FieldErr for NonOptField {}

#[cfg(feature = "std")]
mod std_impls {
    use super::*;

    use std::error::Error;

    impl Error for OptionalField {
        #[inline(always)]
        fn description(&self) -> &str {
            "Some field could not be accessed"
        }
    }
    impl Error for NonOptField {
        #[inline(always)]
        fn description(&self) -> &str {
            "The field isn't optional,this function is uncallable"
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Display for OptionalField {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Some field could not be accessed")
    }
}

impl From<NonOptField> for OptionalField {
    #[inline(always)]
    fn from(_: NonOptField) -> Self {
        OptionalField
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Display for NonOptField {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("This field can always be accessed.")
    }
}

impl NonOptField {
    fn to<T>(self) -> T {
        match self {}
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait IntoFieldErr<T>: FieldErr {
    fn into_field_err(self) -> T
    where
        T: FieldErr;
}

impl<T> IntoFieldErr<T> for T
where
    T: FieldErr,
{
    #[inline(always)]
    fn into_field_err(self) -> T {
        self
    }
}

impl IntoFieldErr<OptionalField> for NonOptField {
    #[inline(always)]
    fn into_field_err(self) -> OptionalField {
        match self {}
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait CombineErrs {
    type Combined: FieldErr;
}

pub type CombineErrsOut<This> = <This as CombineErrs>::Combined;

mod impl_combine_errs {
    use super::{NonOptField as IF, OptionalField as OF, *};

    macro_rules! combined_err_impls {
        (small=>
            $( $ty:ty = $output:ty ,)*
        ) => {
            $(
                impl CombineErrs for $ty {
                    type Combined=$output;
                }
            )*
        };
        (large=>
            $((
                $( ($t0:ident,$t1:ident,$t2:ident,$t3:ident,), )*
                $($trailing:ident,)*
            ))*
        )=>{
            $(
                #[allow(non_camel_case_types)]
                impl< $($t0,$t1,$t2,$t3,)* $($trailing,)* CombTuples,CombTrail >
                    CombineErrs
                for ($($t0,$t1,$t2,$t3,)* $($trailing,)*)
                where
                    $( ($t0,$t1,$t2,$t3): CombineErrs, )*
                    (
                        $( CombineErrsOut<($t0,$t1,$t2,$t3)>, )*
                    ):CombineErrs<Combined=CombTuples>,
                    CombTuples:FieldErr,
                    ($($trailing,)*):CombineErrs<Combined=CombTrail>,
                    CombTrail:FieldErr,
                    (CombTuples,CombTrail):CombineErrs,
                {
                    type Combined=CombineErrsOut<(CombTuples,CombTrail)>;
                }
            )*
        };
    }

    /*
    fn main() {
        fn as_ty(b:bool)->&'static str{
            if b {"OF"}else{"IF"}
        }

        for elem_count in 0..=4 {
            for bits in 0..1<<elem_count {
                let is_optional=(0..elem_count)
                    .map(|i| (bits>>i)&1!=0 )
                    .collect::<Vec<bool>>();

                let tup=is_optional.iter().copied().map(as_ty).collect::<Vec<_>>();
                let any_optional=is_optional.iter().cloned().any(|x|x);

                println!(
                    "({tup})={output},",
                    tup=tup.join(","),
                    output=as_ty(any_optional),
                )
            }
        }
    }
    */

    combined_err_impls! {
        small=>
        ()=IF,
        (IF,)=IF,
        (OF,)=OF,
        (IF,IF)=IF,
        (OF,IF)=OF,
        (IF,OF)=OF,
        (OF,OF)=OF,
        (IF,IF,IF)=IF,
        (OF,IF,IF)=OF,
        (IF,OF,IF)=OF,
        (OF,OF,IF)=OF,
        (IF,IF,OF)=OF,
        (OF,IF,OF)=OF,
        (IF,OF,OF)=OF,
        (OF,OF,OF)=OF,
        (IF,IF,IF,IF)=IF,
        (OF,IF,IF,IF)=OF,
        (IF,OF,IF,IF)=OF,
        (OF,OF,IF,IF)=OF,
        (IF,IF,OF,IF)=OF,
        (OF,IF,OF,IF)=OF,
        (IF,OF,OF,IF)=OF,
        (OF,OF,OF,IF)=OF,
        (IF,IF,IF,OF)=OF,
        (OF,IF,IF,OF)=OF,
        (IF,OF,IF,OF)=OF,
        (OF,OF,IF,OF)=OF,
        (IF,IF,OF,OF)=OF,
        (OF,IF,OF,OF)=OF,
        (IF,OF,OF,OF)=OF,
        (OF,OF,OF,OF)=OF,
    }

    /*
    fn main() {
        fn as_ty(b:bool)->&'static str{
            if b {"OF"}else{"IF"}
        }
        let tup_size=4;

        for elem_count in 5..=12 {
            print!("(");
            for which_tup in 0..elem_count/tup_size {
                let start=which_tup*tup_size;
                print!("(");
                for e in start..start+tup_size{
                    print!("e{},",e);
                }
                print!("),");
            }
            for e in elem_count/tup_size*tup_size..elem_count{
                print!("e{},",e);
            }
            println!(")");
        }
    }

    */

    combined_err_impls! {
        large=>
        ((e0,e1,e2,e3,),e4,)
        ((e0,e1,e2,e3,),e4,e5,)
        ((e0,e1,e2,e3,),e4,e5,e6,)
        ((e0,e1,e2,e3,),(e4,e5,e6,e7,),)
        ((e0,e1,e2,e3,),(e4,e5,e6,e7,),e8,)
        ((e0,e1,e2,e3,),(e4,e5,e6,e7,),e8,e9,)
        ((e0,e1,e2,e3,),(e4,e5,e6,e7,),e8,e9,e10,)
        ((e0,e1,e2,e3,),(e4,e5,e6,e7,),(e8,e9,e10,e11,),)
    }
}
