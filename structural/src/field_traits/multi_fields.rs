/*!
Contains traits for accessing multiple fields at once.
*/

use super::*;

/// This trait allows a MultiTString to borrow the fields it names.
pub trait GetMultiField<'a,This:?Sized>:'a{
    type MultiTy:'a;

    fn multi_get_field_(this:&'a This)->Self::MultiTy;
}

macro_rules! impl_get_multi_field {
    ( $($fname:ident)* ) => (
        impl<'a,This:?Sized,$($fname,)*> GetMultiField<'a,This> for ($($fname,)*)
        where
            Self:'a,
            $(
                This:GetField<$fname>,
                GetFieldType<This,$fname>:'a,
            )*
        {
            type MultiTy=(
                $(
                    &'a GetFieldType<This,$fname>,
                )*
            );

            fn multi_get_field_(this:&'a This)->Self::MultiTy{
                (
                    $(
                        GetField::<$fname>::get_field_(this),
                    )*
                )
            }
        }
    )
}


impl_get_multi_field!{F0}
impl_get_multi_field!{F0 F1}
impl_get_multi_field!{F0 F1 F2}
impl_get_multi_field!{F0 F1 F2 F3}
impl_get_multi_field!{F0 F1 F2 F3 F4}
impl_get_multi_field!{F0 F1 F2 F3 F4 F5}
impl_get_multi_field!{F0 F1 F2 F3 F4 F5 F6}
impl_get_multi_field!{F0 F1 F2 F3 F4 F5 F6 F7}



