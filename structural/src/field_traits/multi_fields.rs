/*!
Contains traits for accessing multiple fields at once.
*/
#![allow(non_snake_case)]

use super::*;

use crate::field_path::{IsMultiFieldPath, NestedFieldPathSet, UniquePaths};

use core_extensions::SelfOps;

/// Queries the type returned by the `RevGetMultiField::rev_get_multi_field` method.
/// This is some collection of references.
pub type RevGetMultiFieldOut<'a, Field, This> = <Field as RevGetMultiField<'a, This>>::Fields;

/// Queries the type returned by the `RevGetMultiFieldMut::rev_get_multi_field_mut` method.
/// This is some collection of mutable references.
pub type RevGetMultiFieldMutOut<'a, Field, This> =
    <Field as RevGetMultiFieldMut<'a, This>>::FieldsMut;

/// Queries the type returned by the `RevGetMultiFieldMut::rev_get_multi_field_raw_mut` method.
/// This is some collection of mutable pointers.
pub type RevGetMultiFieldMutRaw<'a, Field, This> =
    <Field as RevGetMultiFieldMut<'a, This>>::FieldsRawMut;

/// Gets references to multiple fields from `This`.
///
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// This is used by the
/// [`GetFieldExt::fields`](../../trait.GetFieldExt.html#method.fields).
/// and [`GetFieldExt::cloned_fields`](../../trait.GetFieldExt.html#method.cloned_fields).
/// methods.
///
pub trait RevGetMultiField<'a, This: ?Sized + 'a>: IsMultiFieldPath {
    /// A collection of references to fields.
    type Fields: 'a + NormalizeFields;

    /// Gets references to multiple fields from `this`.
    fn rev_get_multi_field(self, this: &'a This) -> Self::Fields;
}

/// Gets mutable references to multiple fields from `This`.
///
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// This is used by the
/// [`GetFieldExt::fields_mut`](../../trait.GetFieldExt.html#method.fields_mut).
/// method.
///
/// # Safety
///
/// The `rev_get_multi_field_raw_mut` function must return non-aliasing pointers,
/// where all of them are safe to dereference.
///
/// As a reminder: mutable references imply uniqueness,
/// which means that it's undefined behavior for implementors to
/// return multiple mutable references to the same field.
pub unsafe trait RevGetMultiFieldMut<'a, This: ?Sized + 'a>:
    IsMultiFieldPath<PathUniqueness = UniquePaths>
{
    /// A collection of mutable references to fields.
    type FieldsMut: 'a + NormalizeFields;

    /// A collection of mutable pointers to fields.
    type FieldsRawMut: 'a + NormalizeFields;

    /// Gest mutable references to multiple fields from `this`
    fn rev_get_multi_field_mut(self, this: &'a mut This) -> Self::FieldsMut;

    /// Gets raw pointers to multiple fields from `This`.
    ///
    /// # Safety
    ///
    /// `this` must point to a valid instance of `This`,which lives for the `'a` lifetime.
    unsafe fn rev_get_multi_field_raw_mut(self, this: *mut This) -> Self::FieldsRawMut;
}

macro_rules! impl_get_multi_field {
    ( $(($fpath:ident $err:ident $fty:ident))* ) => (
        impl<'a,This:?Sized,$($fpath,$err,$fty,)* U>
            RevGetMultiField<'a,This>
        for FieldPathSet<($($fpath,)*),U>
        where
            This:'a,
            $(
                $fpath:RevGetFieldImpl<'a, This, Ty=$fty, Err=$err >,
                $fty:'a,
                $err:IsFieldErr,
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
                let ($($fpath,)*)=self.into_paths();
                (
                    $(
                        $fpath.rev_get_field(this),
                    )*
                )
            }
        }

        unsafe impl<'a,This:?Sized,$($fpath,$err,$fty,)*>
            RevGetMultiFieldMut<'a,This>
        for FieldPathSet<($($fpath,)*),UniquePaths>
        where
            This:'a,
            $(
                $fpath: RevGetFieldMutImpl<'a,This, Ty=$fty, Err=$err >,
                Result<&'a mut $fty,$err>: NormalizeFields,
                Result<*mut $fty,$err>: NormalizeFields,
                $fty:'a,
                $err:IsFieldErr,
                // RevFieldMutType<'a,$fpath,This>:'a,
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

            #[allow(unused_unsafe,unused_variables)]
            fn rev_get_multi_field_mut(self,this:&'a mut This)->Self::FieldsMut{
                unsafe{
                    let ($($fpath,)*)={
                        #[allow(unused_variables)]
                        let ($($fpath,)*)=self.into_paths();
                        (
                            $(
                                $fpath.rev_get_field_raw_mut(this),
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
                let ($($fpath,)*)=self.into_paths();
                (
                    $(
                        $fpath.rev_get_field_raw_mut(this),
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

////////////////////////////////////////////////////////////////////////////////

impl<'a, F, S, U, This, Mid, OutTy, OutErr> RevGetMultiField<'a, This>
    for NestedFieldPathSet<F, S, U>
where
    F: RevGetFieldImpl<'a, This, Ty = Mid, Err = OutErr>,
    FieldPathSet<S, U>: RevGetMultiField<'a, Mid, Fields = OutTy>,
    OutErr: IsFieldErr,
    This: 'a + ?Sized,
    Mid: 'a + ?Sized,
    OutTy: 'a + NormalizeFields,
    NestedFieldPathSetOutput<OutTy, OutErr>: 'a + NormalizeFields,
{
    type Fields = NestedFieldPathSetOutput<OutTy, OutErr>;

    #[inline(always)]
    fn rev_get_multi_field(self, this: &'a This) -> NestedFieldPathSetOutput<OutTy, OutErr> {
        let (nested, set) = self.into_inner();
        nested
            .rev_get_field(this)
            .map({
                #[inline(always)]
                |mid| set.rev_get_multi_field(mid)
            })
            .piped(NestedFieldPathSetOutput)
    }
}

unsafe impl<'a, F, S, This, Mid, OutTy, OutRawTy, OutErr> RevGetMultiFieldMut<'a, This>
    for NestedFieldPathSet<F, S, UniquePaths>
where
    F: RevGetFieldMutImpl<'a, This, Ty = Mid, Err = OutErr>,
    FieldPathSet<S, UniquePaths>:
        RevGetMultiFieldMut<'a, Mid, FieldsMut = OutTy, FieldsRawMut = OutRawTy>,
    This: 'a + ?Sized,
    OutErr: IsFieldErr,
    Mid: 'a + ?Sized,
    OutTy: 'a + NormalizeFields,
    OutRawTy: 'a + NormalizeFields,
    NestedFieldPathSetOutput<OutTy, OutErr>: 'a + NormalizeFields,
    NestedFieldPathSetOutput<OutRawTy, OutErr>: 'a + NormalizeFields,
{
    type FieldsMut = NestedFieldPathSetOutput<OutTy, OutErr>;
    type FieldsRawMut = NestedFieldPathSetOutput<OutRawTy, OutErr>;

    fn rev_get_multi_field_mut(
        self,
        this: &'a mut This,
    ) -> NestedFieldPathSetOutput<OutTy, OutErr> {
        let (nested, set) = self.into_inner();
        nested
            .rev_get_field_mut(this)
            .map({
                #[inline(always)]
                |mid| set.rev_get_multi_field_mut(mid)
            })
            .piped(NestedFieldPathSetOutput)
    }

    unsafe fn rev_get_multi_field_raw_mut(
        self,
        this: *mut This,
    ) -> NestedFieldPathSetOutput<OutRawTy, OutErr> {
        let (nested, set) = self.into_inner();
        nested
            .rev_get_field_raw_mut(this)
            .map({
                #[inline(always)]
                |mid| set.rev_get_multi_field_raw_mut(mid)
            })
            .piped(NestedFieldPathSetOutput)
    }
}

/// The return type of NestedFieldPathSet `Rev*MultiField*` methods,
///
/// This implements NormalizeFields so that a `Result<TupleType,Err>`
/// also normalizes the tuple type itself,
/// turning each individual `Result<T,E>` in the tuple into `T` or `Option<T>`.
pub struct NestedFieldPathSetOutput<T, E>(pub Result<T, E>);

impl<T, E> NormalizeFields for NestedFieldPathSetOutput<T, E>
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
