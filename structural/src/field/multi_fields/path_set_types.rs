use super::{RevGetMultiFieldImpl, RevGetMultiFieldMutImpl, RevIntoMultiFieldImpl};

use crate::{
    field::{
        ownership::AndDroppedFields, DropFields, IsFieldErr, NormalizeFields, NormalizeFieldsOut,
        RevGetFieldImpl, RevGetFieldMutImpl, RevIntoFieldImpl, RevMoveOutField,
    },
    path::{FieldPathSet, NestedFieldPathSet, ShallowFieldPath, UniquePaths},
};

#[allow(unused_imports)]
use core_extensions::SelfOps;

macro_rules! impl_get_multi_field {
    ( $(($fpath:ident $err:ident $fty:ident))* ) => (
        impl<'a,This:?Sized,$($fpath,$err,$fty,)* U>
            RevGetMultiFieldImpl<'a,This>
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
            type UnnormFields=(
                $(
                    Result<&'a $fty,$err>,
                )*
            );

            #[allow(unused_variables)]
            fn rev_get_multi_field_impl(self,this:&'a This)-> Self::UnnormFields {
                let ($($fpath,)*)=self.into_paths();
                (
                    $(
                        $fpath.rev_get_field(this),
                    )*
                )
            }
        }

        unsafe impl<'a,This:?Sized,$($fpath,$err,$fty,)*>
            RevGetMultiFieldMutImpl<'a,This>
        for FieldPathSet<($($fpath,)*),UniquePaths>
        where
            This:'a,
            $(
                $fpath: RevGetFieldMutImpl<'a,This, Ty=$fty, Err=$err >,
                Result<&'a mut $fty,$err>: NormalizeFields,
                Result<*mut $fty,$err>: NormalizeFields,
                $fty:'a,
                $err:IsFieldErr,
            )*
        {
            type UnnormFieldsMut=(
                $(
                    Result<&'a mut $fty,$err>,
                )*
            );
            type UnnormFieldsRawMut=(
                $(
                    Result<*mut $fty,$err>,
                )*
            );

            #[allow(unused_unsafe,unused_variables)]
            fn rev_get_multi_field_mut_impl(
                self,
                this:&'a mut This,
            )-> Self::UnnormFieldsMut {
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
            unsafe fn rev_get_multi_field_raw_mut_impl(
                self,
                this:*mut This,
            )-> Self::UnnormFieldsRawMut {
                let ($($fpath,)*)=self.into_paths();
                (
                    $(
                        $fpath.rev_get_field_raw_mut(this),
                    )*
                )
            }
        }

        unsafe impl<'a,This,$($fpath,$err,$fty,)*>
            RevIntoMultiFieldImpl<This>
        for FieldPathSet<($($fpath,)*),UniquePaths>
        where
            This: DropFields,
            $(
                $fpath: RevMoveOutField<This, Ty=$fty, Err=$err >,
                Result<$fty,$err>: NormalizeFields,
                $err:IsFieldErr,
            )*
        {
            type UnnormIntoFields=(
                $(
                    Result<$fty,$err>,
                )*
            );

            fn rev_into_multi_field_impl(self, this: This) -> Self::UnnormIntoFields{
                let ($($fpath,)*)=self.into_paths();
                unsafe{
                    let mut this=AndDroppedFields::new(this);

                    #[allow(unused_variables)]
                    let (this, dropped)=this.inner_and_dropped_mut();
                    (
                        $(
                            $fpath.rev_move_out_field(this, dropped),
                        )*
                    )
                }
            }
        }

        unsafe impl<$($fpath,)* U> ShallowFieldPath for FieldPathSet<($($fpath,)*),U>
        where
            $($fpath: ShallowFieldPath,)*
        {}


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
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<'a, F, S, U, This, Mid, OutTy, OutErr> RevGetMultiFieldImpl<'a, This>
    for NestedFieldPathSet<F, S, U>
where
    F: RevGetFieldImpl<'a, This, Ty = Mid, Err = OutErr>,
    FieldPathSet<S, U>: RevGetMultiFieldImpl<'a, Mid, UnnormFields = OutTy>,
    OutErr: IsFieldErr,
    This: 'a + ?Sized,
    Mid: 'a + ?Sized,
    OutTy: 'a + NormalizeFields,
    NestedFieldPathSetOutput<OutTy, OutErr>: 'a + NormalizeFields,
{
    type UnnormFields = NestedFieldPathSetOutput<OutTy, OutErr>;

    #[inline(always)]
    fn rev_get_multi_field_impl(self, this: &'a This) -> NestedFieldPathSetOutput<OutTy, OutErr> {
        let (nested, set) = self.into_inner();
        nested
            .rev_get_field(this)
            .map({
                #[inline(always)]
                |mid| set.rev_get_multi_field_impl(mid)
            })
            .piped(NestedFieldPathSetOutput)
    }
}

unsafe impl<'a, F, S, This, Mid, OutTy, OutRawTy, OutErr> RevGetMultiFieldMutImpl<'a, This>
    for NestedFieldPathSet<F, S, UniquePaths>
where
    F: RevGetFieldMutImpl<'a, This, Ty = Mid, Err = OutErr>,
    FieldPathSet<S, UniquePaths>:
        RevGetMultiFieldMutImpl<'a, Mid, UnnormFieldsMut = OutTy, UnnormFieldsRawMut = OutRawTy>,
    This: 'a + ?Sized,
    OutErr: IsFieldErr,
    Mid: 'a + ?Sized,
    OutTy: 'a + NormalizeFields,
    OutRawTy: 'a + NormalizeFields,
    NestedFieldPathSetOutput<OutTy, OutErr>: 'a + NormalizeFields,
    NestedFieldPathSetOutput<OutRawTy, OutErr>: 'a + NormalizeFields,
{
    type UnnormFieldsMut = NestedFieldPathSetOutput<OutTy, OutErr>;
    type UnnormFieldsRawMut = NestedFieldPathSetOutput<OutRawTy, OutErr>;

    #[inline(always)]
    fn rev_get_multi_field_mut_impl(
        self,
        this: &'a mut This,
    ) -> NestedFieldPathSetOutput<OutTy, OutErr> {
        let (nested, set) = self.into_inner();
        nested
            .rev_get_field_mut(this)
            .map({
                #[inline(always)]
                |mid| set.rev_get_multi_field_mut_impl(mid)
            })
            .piped(NestedFieldPathSetOutput)
    }

    #[inline(always)]
    unsafe fn rev_get_multi_field_raw_mut_impl(
        self,
        this: *mut This,
    ) -> NestedFieldPathSetOutput<OutRawTy, OutErr> {
        let (nested, set) = self.into_inner();
        nested
            .rev_get_field_raw_mut(this)
            .map({
                #[inline(always)]
                |mid| set.rev_get_multi_field_raw_mut_impl(mid)
            })
            .piped(NestedFieldPathSetOutput)
    }
}

unsafe impl<F, S, This, Mid, OutTy, OutErr> RevIntoMultiFieldImpl<This>
    for NestedFieldPathSet<F, S, UniquePaths>
where
    F: RevIntoFieldImpl<This, Ty = Mid, Err = OutErr>,
    FieldPathSet<S, UniquePaths>: RevIntoMultiFieldImpl<Mid, UnnormIntoFields = OutTy>,
    OutTy: NormalizeFields,
    OutErr: IsFieldErr,
    NestedFieldPathSetOutput<OutTy, OutErr>: NormalizeFields,
{
    type UnnormIntoFields = NestedFieldPathSetOutput<OutTy, OutErr>;

    fn rev_into_multi_field_impl(self, this: This) -> Self::UnnormIntoFields {
        let (nested, set) = self.into_inner();
        nested
            .rev_into_field(this)
            .map({
                #[inline(always)]
                |mid| set.rev_into_multi_field_impl(mid)
            })
            .piped(NestedFieldPathSetOutput)
    }
}

/// The return type of NestedFieldPathSet's `Rev*MultiField*Impl` impls,
///
/// This implements NormalizeFields so that a the wrapped `Result<TupleType,Err>`
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
