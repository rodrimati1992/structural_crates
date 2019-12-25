/*!
Contains traits for accessing multiple fields at once.
*/
#![allow(non_snake_case)]

use super::*;

/*
pub trait RevGetField<'a,This:?Sized>{
    type Field:'a;

    fn rev_get_field(self,this:&'a This)->&'a Self::Field;
}

pub unsafe trait RevGetFieldMut<'a,This:?Sized>{
    type Field:'a;

    fn rev_get_field_mut(self,this:&'a mut This)->&'a mut Self::Field;

    unsafe fn rev_get_field_raw_mut(
        self,
        field:MutRef<'a,This>,
    )->MutRef<'a,Self::Field>;
}
*/

macro_rules! impl_get_multi_field {
    ( $(($fpath:ident $err:ident $fty:ident))* ) => (
        impl<'a,This:?Sized,$($fpath,$err,$fty,)* U>
            RevGetField<'a,This>
        for FieldPathSet<($(FieldPath<$fpath>,)*),U>
        where
            $(
                FieldPath<$fpath>:RevGetField<'a,This,Field=Result<&'a $fty,$err>>,
                Result<&'a $fty,$err>:NormalizeFields,
                $fty:'a,
                $err:'a,
            )*
        {
            type Field=(
                $(
                    Result<&'a $fty,$err>,
                )*
            );

            #[allow(unused_variables)]
            fn rev_get_field(self,this:&'a This)->Self::Field{
                (
                    $(
                        FieldPath::<$fpath>::NEW.rev_get_field(this),
                    )*
                )
            }
        }

        unsafe impl<'a,This:?Sized,$($fpath,$err,$fty,)*>
            RevGetFieldMut<'a,This>
        for FieldPathSet<($(FieldPath<$fpath>,)*),UniquePaths>
        where
            $(
                FieldPath<$fpath>:RevGetFieldMut<
                    'a,
                    This,
                    Field=Result<&'a mut $fty,$err>,
                    FieldRawMut=Result<*mut $fty,$err>,
                >,
                Result<&'a mut $fty,$err>:NormalizeFields,
                Result<*mut $fty,$err>:NormalizeFields,
                $fty:'a,
                $err:'a,
                // RevFieldMutType<'a,FieldPath<$fpath>,This>:'a,
            )*
        {
            type Field=(
                $(
                    Result<&'a mut $fty,$err>,
                )*
            );
            type FieldRawMut=(
                $(
                    Result<*mut $fty,$err>,
                )*
            );

            fn rev_get_field_mut(self,this:&'a mut This)->Self::Field{
                unsafe{
                    let ($($fpath,)*)={
                        self.rev_get_field_raw_mut(this)
                    };

                    (
                        $(
                            match $fpath {
                                Ok($fpath)=>Ok(&mut *$fpath),
                                Err(e)=>Err(e),
                            },
                        )*
                    )
                }
            }

            #[allow(unused_variables)]
            unsafe fn rev_get_field_raw_mut(self,this:*mut This)->Self::FieldRawMut{
                (
                    $(
                        FieldPath::<$fpath>::NEW.rev_get_field_raw_mut(this),
                    )*
                )
            }
        }
    )
}

impl_get_multi_field! {}
impl_get_multi_field! {
    (F0 E0 T0)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5) (F6 E6 T6)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5) (F6 E6 T6) (F7 E7 T7)
}
