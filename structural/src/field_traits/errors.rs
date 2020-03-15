/*!
Field-accessor-related error types and traits
*/

use std_::fmt::{self, Display};

use core_extensions::collection_traits::Cloned;

mod sealed {
    pub trait Sealed {}
}
use self::sealed::Sealed;

/// Marker trait for the errors that can be returned from the `RevGetField` trait and subtraits.
///
/// The errors can be:
///
/// - [StructField](./enum.StructField.html):
///     Used when a field is inside a struct (and it's not nested within an enum).
///
/// - [EnumField](./struct.EnumField.html):
///     Used when a field is inside an enum.
///
/// This trait is sealed,and cannot be implemented outside of the `structural` crate.
pub trait IsFieldErr: Sealed + 'static + Copy + Cloned {}

/// The error type for accesses to fields inside a struct,
/// and it's not nested inside an enum.
///
/// This is used as the `Err` associated type for `Rev*Field*` implementors,
/// which return a `Result<_,StructField>`,
/// then [GetFieldExt](../trait.GetFieldExt.html) methods use
/// [NormalizeFields](../trait.NormalizeFields.html) to turn
/// `Ok(foo)` into `foo` (which can be safely done,since this type can't be constructed).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructField {}

/// The error type for accesses to fields inside an enum.
///
/// This is used as the `Err` associated type for `Rev*Field*` implementors,
/// which return a `Result<_,EnumField>`,
/// then [GetFieldExt](../trait.GetFieldExt.html) methods use
/// [NormalizeFields](../trait.NormalizeFields.html) to turn
/// `Ok(foo)` into `Some(foo)`,and `Err(EnumField)` into `None`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumField;

impl Cloned for EnumField {
    type Cloned = Self;

    #[inline(always)]
    fn cloned_(&self) -> Self {
        *self
    }
}

impl Cloned for StructField {
    type Cloned = Self;

    #[inline(always)]
    fn cloned_(&self) -> Self {
        *self
    }
}

impl Sealed for EnumField {}
impl IsFieldErr for EnumField {}

impl Sealed for StructField {}
impl IsFieldErr for StructField {}

#[cfg(feature = "std")]
mod std_impls {
    use super::*;

    use std::error::Error;

    impl Error for EnumField {
        #[inline(always)]
        fn description(&self) -> &str {
            "Some field could not be accessed"
        }
    }
    impl Error for StructField {
        #[inline(always)]
        fn description(&self) -> &str {
            "The field isn't optional,this function is uncallable"
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Display for EnumField {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Some field could not be accessed")
    }
}

impl From<StructField> for EnumField {
    #[inline(always)]
    fn from(_: StructField) -> Self {
        EnumField
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Display for StructField {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("This field can always be accessed.")
    }
}

////////////////////////////////////////////////////////////////////////////////

/// A specialized conversion trait,to convert field accessor error types to other
/// field accessor error types.
pub trait IntoFieldErr<T>: IsFieldErr {
    fn into_field_err(self) -> T
    where
        T: IsFieldErr;
}

impl<T> IntoFieldErr<T> for T
where
    T: IsFieldErr,
{
    #[inline(always)]
    fn into_field_err(self) -> T {
        self
    }
}

impl IntoFieldErr<EnumField> for StructField {
    #[inline(always)]
    fn into_field_err(self) -> EnumField {
        match self {}
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Combines multiple error types into one.
///
/// A tuple of errors is combined into a `StructField` so long as all of them are,
/// otherwise they're combined into an `EnumField` .
///
/// This is used by the `Rev*Field*` impls for [NestedFieldPath](../../struct.NestedFieldPath.html) to
/// determine whether a nested field access is optional or not.
pub trait CombinedErrs {
    type Combined: IsFieldErr;
}

/// The combination of all the error types in `This`.
///
/// A tuple of errors is combined into a `StructField` so long as all of them are,
/// otherwise they're combined into an `EnumField` .
pub type CombinedErrsOut<This> = <This as CombinedErrs>::Combined;

mod impl_combine_errs {
    use super::{EnumField as OF, StructField as IF, *};

    macro_rules! combined_err_impls {
        (small=>
            $( $ty:ty = $output:ty ,)*
        ) => {
            $(
                impl CombinedErrs for $ty {
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
                    CombinedErrs
                for ($($t0,$t1,$t2,$t3,)* $($trailing,)*)
                where
                    $( ($t0,$t1,$t2,$t3): CombinedErrs, )*
                    (
                        $( CombinedErrsOut<($t0,$t1,$t2,$t3)>, )*
                    ):CombinedErrs<Combined=CombTuples>,
                    CombTuples:IsFieldErr,
                    ($($trailing,)*):CombinedErrs<Combined=CombTrail>,
                    CombTrail:IsFieldErr,
                    (CombTuples,CombTrail):CombinedErrs,
                {
                    type Combined=CombinedErrsOut<(CombTuples,CombTrail)>;
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
