/*!
Contains traits for accessing multiple fields at once.
*/

use super::*;

/// This trait allows a MultiTString to borrow the fields it names.
pub trait GetMultiField<'a,This:?Sized>{
    type MultiTy:'a;

    fn multi_get_field_(this:&'a This)->Self::MultiTy;
}

/// This trait allows a MultiTString to borrow the fields it names mutably.
pub trait GetMultiFieldMut<'a,This:?Sized>:Sized{
    type MultiTy:'a;

    fn multi_get_field_mut_(this:&'a mut This,_:MultiTString<Self>)->Self::MultiTy
    where This:Sized;
}


macro_rules! impl_get_multi_field {
    ( $($fname:ident)* ) => (
        impl<'a,This:?Sized,$($fname,)*> GetMultiField<'a,This> for ($($fname,)*)
        where
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

        impl<'a,This:?Sized,$($fname,)*> GetMultiFieldMut<'a,This> for ($($fname,)*)
        where
            $(
                This:GetFieldMut<$fname>,
                GetFieldType<This,$fname>:'a,
            )*
        {
            type MultiTy=(
                $(
                    &'a mut GetFieldType<This,$fname>,
                )*
            );

            fn multi_get_field_mut_(this:&'a mut This,_:MultiTString<Self>)->Self::MultiTy
            where
                This:Sized,
            {
                let this=MutRef::new(this);
                unsafe{
                    (
                        $(
                            GetFieldMut::<$fname>::raw_get_mut_field(this.clone()),
                        )*
                    )
                }
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



