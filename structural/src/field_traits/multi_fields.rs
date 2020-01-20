/*!
Contains traits for accessing multiple fields at once.
*/
#![allow(non_snake_case)]

use super::*;

use crate::type_level::IsFieldPathSet;

use std_::marker::PhantomData;

pub type RevGetMultiFieldOut<'a, Field, This> = <Field as RevGetMultiField<'a, This>>::Fields;

pub type RevGetMultiFieldMutOut<'a, Field, This> =
    <Field as RevGetMultiFieldMut<'a, This>>::FieldsMut;

pub type RevGetMultiFieldMutRaw<'a, Field, This> =
    <Field as RevGetMultiFieldMut<'a, This>>::FieldsRawMut;

pub trait RevGetMultiField<'a, This: ?Sized + 'a>: IsFieldPathSet {
    type Fields: 'a + NormalizeFields;

    fn rev_get_multi_field(self, this: &'a This) -> Self::Fields;

    #[doc(hidden)]
    const __UNIMPLEMENTABLE_OUTSIDE__: RGMF<'a, Self, This>;
}

pub trait RevGetMultiFieldMut<'a, This: ?Sized + 'a>: IsFieldPathSet {
    type FieldsMut: 'a + NormalizeFields;
    type FieldsRawMut: 'a + NormalizeFields;

    fn rev_get_multi_field_mut(self, this: &'a mut This) -> Self::FieldsMut;

    unsafe fn rev_get_multi_field_raw_mut(self, this: *mut This) -> Self::FieldsRawMut;

    #[doc(hidden)]
    const __UNIMPLEMENTABLE_OUTSIDE__: RGMF<'a, Self, This>;
}

macro_rules! impl_get_multi_field {
    ( $(($fpath:ident $err:ident $fty:ident))* ) => (
        impl<'a,This:?Sized,$($fpath,$err,$fty,)* U>
            RevGetMultiField<'a,This>
        for FieldPathSet<($(FieldPath<$fpath>,)*),U>
        where
            This:'a,
            $(
                FieldPath<$fpath>:RevGetField<'a, This, Ty=$fty, Err=$err >,
                $fty:'a,
                $err:'a,
                Result<&'a $fty,$err>: NormalizeFields,
            )*
        {
            type Fields=(
                $(
                    Result<&'a $fty,$err>,
                )*
            );

            #[allow(unused_variables)]
            fn rev_get_multi_field(self,this:&'a This)->Self::Fields{
                (
                    $(
                        FieldPath::<$fpath>::NEW.rev_get_field(this),
                    )*
                )
            }

            #[doc(hidden)]
            const __UNIMPLEMENTABLE_OUTSIDE__:RGMF<'a,Self,This>=RGMF(PhantomData);
        }

        impl<'a,This:?Sized,$($fpath,$err,$fty,)*>
            RevGetMultiFieldMut<'a,This>
        for FieldPathSet<($(FieldPath<$fpath>,)*),UniquePaths>
        where
            This:'a,
            $(
                FieldPath<$fpath>: RevGetFieldMut<'a,This, Ty=$fty, Err=$err >,
                Result<&'a mut $fty,$err>: NormalizeFields,
                Result<*mut $fty,$err>: NormalizeFields,
                $fty:'a,
                $err:'a,
                // RevFieldMutType<'a,FieldPath<$fpath>,This>:'a,
            )*
        {
            type FieldsMut=(
                $(
                    Result<&'a mut $fty,$err>,
                )*
            );
            type FieldsRawMut=(
                $(
                    Result<*mut $fty,$err>,
                )*
            );

            #[allow(unused_unsafe)]
            fn rev_get_multi_field_mut(self,this:&'a mut This)->Self::FieldsMut{
                unsafe{
                    let ($($fpath,)*)={
                        #[allow(unused_variables)]
                        let this=this as *mut This;
                        (
                            $(
                                FieldPath::<$fpath>::NEW.rev_get_field_raw_mut(this),
                            )*
                        )
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
            unsafe fn rev_get_multi_field_raw_mut(self,this:*mut This)->Self::FieldsRawMut{
                (
                    $(
                        FieldPath::<$fpath>::NEW.rev_get_field_raw_mut(this),
                    )*
                )
            }

            #[doc(hidden)]
            const __UNIMPLEMENTABLE_OUTSIDE__:RGMF<'a,Self,This>=RGMF(PhantomData);
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

#[doc(hidden)]
pub struct RGMF<'a, Name, This: ?Sized>(PhantomData<(PhantomData<Name>, PhantomData<&'a This>)>);
