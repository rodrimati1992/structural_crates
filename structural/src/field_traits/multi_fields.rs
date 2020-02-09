/*!
Contains traits for accessing multiple fields at once.
*/
#![allow(non_snake_case)]

use super::*;

use crate::type_level::{IsFieldPathSet, NestedFieldSet};

use core_extensions::SelfOps;

use std_::marker::PhantomData;

pub type RevGetMultiFieldOut<'a, Field, This> = <Field as RevGetMultiField<'a, This>>::Fields;

pub type RevGetMultiFieldMutOut<'a, Field, This> =
    <Field as RevGetMultiFieldMut<'a, This>>::FieldsMut;

pub type RevGetMultiFieldMutRaw<'a, Field, This> =
    <Field as RevGetMultiFieldMut<'a, This>>::FieldsRawMut;

pub unsafe trait RevGetMultiField<'a, This: ?Sized + 'a> {
    type Fields: 'a + NormalizeFields;

    fn rev_get_multi_field(self, this: &'a This) -> Self::Fields;
}

pub unsafe trait RevGetMultiFieldMut<'a, This: ?Sized + 'a> {
    type FieldsMut: 'a + NormalizeFields;
    type FieldsRawMut: 'a + NormalizeFields;

    fn rev_get_multi_field_mut(self, this: &'a mut This) -> Self::FieldsMut;

    unsafe fn rev_get_multi_field_raw_mut(self, this: *mut This) -> Self::FieldsRawMut;
}

macro_rules! impl_get_multi_field {
    ( $(($fpath:ident $err:ident $fty:ident))* ) => (
        unsafe impl<'a,This:?Sized,$($fpath,$err,$fty,)* U>
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
        }

        unsafe impl<'a,This:?Sized,$($fpath,$err,$fty,)*>
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

impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5) (F6 E6 T6) (F7 E7 T7)
    (F8 E8 T8) (F9 E9 T9) (F10 E10 T10) (F11 E11 T11) (F12 E12 T12)
}

unsafe impl<'a, F, S, U, This, Mid, OutTy, OutErr> RevGetMultiField<'a, This>
    for NestedFieldSet<F, S, U>
where
    FieldPath<F>: RevGetField<'a, This, Ty = Mid, Err = OutErr>,
    FieldPathSet<S, U>: RevGetMultiField<'a, Mid, Fields = OutTy>,
    This: 'a + ?Sized,
    Mid: 'a + ?Sized,
    OutTy: 'a + NormalizeFields,
    NestedFieldSetOutput<OutTy, OutErr>: 'a + NormalizeFields,
{
    type Fields = NestedFieldSetOutput<OutTy, OutErr>;

    #[inline(always)]
    fn rev_get_multi_field(self, this: &'a This) -> NestedFieldSetOutput<OutTy, OutErr> {
        self.path
            .rev_get_field(this)
            .map({
                #[inline(always)]
                |mid| self.path_set.rev_get_multi_field(mid)
            })
            .piped(NestedFieldSetOutput)
    }
}

unsafe impl<'a, F, S, U, This, Mid, OutTy, OutRawTy, OutErr> RevGetMultiFieldMut<'a, This>
    for NestedFieldSet<F, S, U>
where
    FieldPath<F>: RevGetFieldMut<'a, This, Ty = Mid, Err = OutErr>,
    FieldPathSet<S, U>: RevGetMultiFieldMut<'a, Mid, FieldsMut = OutTy, FieldsRawMut = OutRawTy>,
    This: 'a + ?Sized,
    Mid: 'a + ?Sized,
    OutTy: 'a + NormalizeFields,
    OutRawTy: 'a + NormalizeFields,
    NestedFieldSetOutput<OutTy, OutErr>: 'a + NormalizeFields,
    NestedFieldSetOutput<OutRawTy, OutErr>: 'a + NormalizeFields,
{
    type FieldsMut = NestedFieldSetOutput<OutTy, OutErr>;
    type FieldsRawMut = NestedFieldSetOutput<OutRawTy, OutErr>;

    fn rev_get_multi_field_mut(self, this: &'a mut This) -> NestedFieldSetOutput<OutTy, OutErr> {
        self.path
            .rev_get_field_mut(this)
            .map({
                #[inline(always)]
                |mid| self.path_set.rev_get_multi_field_mut(mid)
            })
            .piped(NestedFieldSetOutput)
    }

    unsafe fn rev_get_multi_field_raw_mut(
        self,
        this: *mut This,
    ) -> NestedFieldSetOutput<OutRawTy, OutErr> {
        self.path
            .rev_get_field_raw_mut(this)
            .map({
                #[inline(always)]
                |mid| self.path_set.rev_get_multi_field_raw_mut(mid)
            })
            .piped(NestedFieldSetOutput)
    }
}

/// The return type of NestedFieldSet `Rev*MultiField*` methods,
///
/// This implements NormalizeFields so that a `Result<TupleType,Err>`
/// also normalizes the tuple type itself
/// (instead of just turning it into either `TupleType` or `Option<TupleType>`),
/// this is so that the tuple is composed of `Option<T>` and `T` instead
/// of `Result<E,impl IsFieldErr>`.
pub struct NestedFieldSetOutput<T, E>(pub Result<T, E>);

impl<T, E> NormalizeFields for NestedFieldSetOutput<T, E>
where
    T: NormalizeFields,
    Result<T::Output, E>: NormalizeFields,
{
    type Output = NormalizeFieldsOut<Result<T::Output, E>>;

    #[inline(always)]
    fn normalize_fields(self) -> Self::Output {
        match self.0 {
            Ok(x) => Ok(x.normalize_fields()),
            Err(e) => Err(e),
        }
        .normalize_fields()
    }
}
