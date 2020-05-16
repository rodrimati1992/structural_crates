use super::{RevGetMultiFieldImpl, RevGetMultiFieldMutImpl, RevIntoMultiFieldImpl};

use crate::{
    field::{
        multi_fields::RevMoveOutMultiFieldImpl, ownership::IntoFieldsWrapper, DropFields,
        IsFieldErr, MovedOutFields, NormalizeFields, NormalizeFieldsOut, RevGetFieldImpl,
        RevGetFieldMutImpl, RevIntoFieldImpl, RevMoveOutFieldImpl,
    },
    path::{
        AliasedPaths, FieldPathSet, LargePathSet, NestedFieldPathSet, ShallowFieldPath,
        SmallPathSet, UniquePaths,
    },
    utils::DerefNested,
};

#[allow(unused_imports)]
use core_extensions::SelfOps;

macro_rules! impl_get_multi_field {
    (
        $(
            (
                $(($fpath:ident $err:ident $fty:ident))*
            )
        )*
    )=>{
        $(
            impl_get_multi_field!{
                @inner
                TParam=($($fpath,)*),
                self=this,
                destructurer=this.into_paths(),
                $(($fpath $err $fty))*
            }

            impl_get_multi_field!{
                @inner
                TParam=SmallPathSet<($($fpath,)*)>,
                self=this,
                destructurer=this.into_paths().0,
                $(($fpath $err $fty))*
            }
        )*
    };
    (
        @inner
        TParam=$TParam:ty,
        self=$self:ident,
        destructurer=$destructurer:expr,
        $(($fpath:ident $err:ident $fty:ident))*
    ) => (
        impl<'a,This:?Sized,$($fpath,$err,$fty,)* U>
            RevGetMultiFieldImpl<'a,This>
        for FieldPathSet<$TParam,U>
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
                let $self=self;
                let ($($fpath,)*)=$destructurer;
                (
                    $(
                        $fpath.rev_get_field(this),
                    )*
                )
            }
        }

        unsafe impl<'a,This:?Sized,$($fpath,$err,$fty,)*>
            RevGetMultiFieldMutImpl<'a,This>
        for FieldPathSet<$TParam,UniquePaths>
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
                        let $self=self;
                        let ($($fpath,)*)=$destructurer;
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
                let $self=self;
                let ($($fpath,)*)=$destructurer;
                (
                    $(
                        $fpath.rev_get_field_raw_mut(this),
                    )*
                )
            }
        }

        impl<'a,This,$($fpath,$err,$fty,)*>
            RevIntoMultiFieldImpl<This>
        for FieldPathSet<$TParam,UniquePaths>
        where
            This: DropFields,
            $(
                $fpath: RevMoveOutFieldImpl<This, Ty=$fty, Err=$err >,
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
                unsafe{
                    let mut this=IntoFieldsWrapper::new(this);

                    #[allow(unused_variables)]
                    let (this, moved)=this.inner_and_moved_mut();

                    self.rev_move_out_multi_field(this,moved)
                }
            }
        }


        unsafe impl<'a,This,$($fpath,$err,$fty,)*>
            RevMoveOutMultiFieldImpl<This>
        for FieldPathSet<$TParam,UniquePaths>
        where
            This: DropFields,
            $(
                $fpath: RevMoveOutFieldImpl<This, Ty=$fty, Err=$err >,
                Result<$fty,$err>: NormalizeFields,
                $err:IsFieldErr,
            )*
        {
            #[allow(unused_variables)]
            unsafe fn rev_move_out_multi_field(
                self,
                this: &mut This,
                moved: &mut MovedOutFields,
            ) -> Self::UnnormIntoFields {
                let $self=self;
                let ($($fpath,)*)=$destructurer;

                (
                    $(
                        $fpath.rev_move_out_field(this, moved),
                    )*
                )
            }
        }


        unsafe impl<$($fpath,)* U> ShallowFieldPath for FieldPathSet<$TParam,U>
        where
            $($fpath: ShallowFieldPath,)*
        {}

    )
}

impl_get_multi_field! {
    ((F0 E0 T0))
    ((F0 E0 T0) (F1 E1 T1))
    ((F0 E0 T0) (F1 E1 T1) (F2 E2 T2))
    ((F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3))
    ((F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4))
    ((F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5))
    ((F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5) (F6 E6 T6))
    ((F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5) (F6 E6 T6) (F7 E7 T7))
}

////////////////////////////////////////////////////////////////////////////////

impl<'a, This: ?Sized + 'a, U> RevGetMultiFieldImpl<'a, This> for FieldPathSet<(), U> {
    type UnnormFields = ();

    fn rev_get_multi_field_impl(self, _this: &'a This) {}
}

unsafe impl<'a, This: ?Sized + 'a> RevGetMultiFieldMutImpl<'a, This>
    for FieldPathSet<(), UniquePaths>
{
    type UnnormFieldsMut = ();
    type UnnormFieldsRawMut = ();

    #[inline(always)]
    fn rev_get_multi_field_mut_impl(self, _this: &'a mut This) {}

    #[inline(always)]
    unsafe fn rev_get_multi_field_raw_mut_impl(self, _this: *mut This) {}
}

impl<'a, This> RevIntoMultiFieldImpl<This> for FieldPathSet<(), UniquePaths> {
    type UnnormIntoFields = ();

    #[inline(always)]
    fn rev_into_multi_field_impl(self, _this: This) {}
}

unsafe impl<'a, This> RevMoveOutMultiFieldImpl<This> for FieldPathSet<(), UniquePaths> {
    #[inline(always)]
    unsafe fn rev_move_out_multi_field(self, _this: &mut This, _moved: &mut MovedOutFields) {}
}

unsafe impl<U> ShallowFieldPath for FieldPathSet<(), U> {}

////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_get_multi_field_large {
    ( $(($fpath:ident $unnorm_a:ident $unnorm_b:ident))* ) => (

        /////////////////////////////////////////////////////////////////////////////
        ////                    Impls for large field path sets
        /////////////////////////////////////////////////////////////////////////////

        #[allow(unused_parens)]
        impl<'a,This:?Sized,$($fpath, $unnorm_a,)* U>
            RevGetMultiFieldImpl<'a,This>
        for FieldPathSet<LargePathSet<($($fpath,)*)>,U>
        where
            This:'a,
            $(
                FieldPathSet<SmallPathSet<$fpath>, AliasedPaths>:
                    RevGetMultiFieldImpl<'a, This,UnnormFields= $unnorm_a>,
                $unnorm_a: 'a + NormalizeFields,
            )*
        {
            type UnnormFields=( $( $unnorm_a ),* );

            #[allow(unused_variables)]
            fn rev_get_multi_field_impl(self,this:&'a This)-> Self::UnnormFields {
                let LargePathSet(($($fpath,)*))=self.into_paths();
                (
                    $(
                        FieldPathSet::many(SmallPathSet($fpath))
                            .rev_get_multi_field_impl(this)
                    ),*
                )
            }
        }

        #[allow(unused_parens)]
        unsafe impl<'a,This:?Sized,$($fpath,$unnorm_a,$unnorm_b,)*>
            RevGetMultiFieldMutImpl<'a,This>
        for FieldPathSet<LargePathSet<($($fpath,)*)>,UniquePaths>
        where
            This:'a,
            $(
                FieldPathSet<SmallPathSet<$fpath>, UniquePaths>: RevGetMultiFieldMutImpl<
                    'a,
                    This,
                    UnnormFieldsMut = $unnorm_a,
                    UnnormFieldsRawMut = $unnorm_b,
                >,
                $unnorm_a: 'a + NormalizeFields,
                $unnorm_b: 'a + NormalizeFields + DerefNested<'a,Dereffed= $unnorm_a >,
            )*
        {
            type UnnormFieldsMut=( $( $unnorm_a ),* );
            type UnnormFieldsRawMut=( $( $unnorm_b ),* );

            #[allow(unused_unsafe,unused_variables)]
            fn rev_get_multi_field_mut_impl(
                self,
                this:&'a mut This,
            )-> Self::UnnormFieldsMut {
                unsafe{
                    DerefNested::deref_nested(self.rev_get_multi_field_raw_mut_impl(this))
                }
            }

            #[allow(unused_variables)]
            unsafe fn rev_get_multi_field_raw_mut_impl(
                self,
                this:*mut This,
            )-> Self::UnnormFieldsRawMut {
                let LargePathSet(($($fpath,)*))=self.into_paths();
                (
                    $(
                        FieldPathSet::many(SmallPathSet($fpath))
                            .upgrade_unchecked()
                            .rev_get_multi_field_raw_mut_impl(this)
                    ),*
                )
            }
        }

        #[allow(unused_parens)]
        impl<'a,This,$($fpath,$unnorm_a,)*>
            RevIntoMultiFieldImpl<This>
        for FieldPathSet<LargePathSet<($($fpath,)*)>,UniquePaths>
        where
            This: DropFields,
            $(
                FieldPathSet<SmallPathSet<$fpath>, UniquePaths>:
                    RevMoveOutMultiFieldImpl<This,UnnormIntoFields= $unnorm_a>,

                $unnorm_a: NormalizeFields,
            )*
        {
            type UnnormIntoFields=( $( $unnorm_a ),* );

            fn rev_into_multi_field_impl(self, this: This) -> Self::UnnormIntoFields{
                let LargePathSet(($($fpath,)*))=self.into_paths();

                #[allow(unused_unsafe)]
                unsafe{
                    let mut this=IntoFieldsWrapper::new(this);

                    #[allow(unused_variables)]
                    let (this, moved)=this.inner_and_moved_mut();
                    (
                        $(
                            FieldPathSet::many(SmallPathSet($fpath))
                                .upgrade_unchecked()
                                .rev_move_out_multi_field(this, moved)
                        ),*
                    )
                }
            }
        }


        unsafe impl<$($fpath,)* U> ShallowFieldPath for FieldPathSet<LargePathSet<($($fpath,)*)>,U>
        where
            $( FieldPathSet<SmallPathSet<$fpath>, U>: ShallowFieldPath,)*
        {}
    )
}

impl_get_multi_field_large! {
    (F0 E0 T0)
}
impl_get_multi_field_large! {
    (F0 E0 T0) (F1 E1 T1)
}
impl_get_multi_field_large! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2)
}
impl_get_multi_field_large! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3)
}
impl_get_multi_field_large! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4)
}
impl_get_multi_field_large! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5)
}
impl_get_multi_field_large! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5) (F6 E6 T6)
}
impl_get_multi_field_large! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5) (F6 E6 T6) (F7 E7 T7)
}

////////////////////////////////////////////////////////////////////////////////

impl<'a, This: ?Sized + 'a, U> RevGetMultiFieldImpl<'a, This>
    for FieldPathSet<LargePathSet<()>, U>
{
    type UnnormFields = ();

    fn rev_get_multi_field_impl(self, _this: &'a This) {}
}

unsafe impl<'a, This: ?Sized + 'a> RevGetMultiFieldMutImpl<'a, This>
    for FieldPathSet<LargePathSet<()>, UniquePaths>
{
    type UnnormFieldsMut = ();
    type UnnormFieldsRawMut = ();

    #[allow(unused_unsafe, unused_variables)]
    fn rev_get_multi_field_mut_impl(self, _this: &'a mut This) {}

    #[allow(unused_variables)]
    unsafe fn rev_get_multi_field_raw_mut_impl(self, _this: *mut This) {}
}

impl<'a, This> RevIntoMultiFieldImpl<This> for FieldPathSet<LargePathSet<()>, UniquePaths> {
    type UnnormIntoFields = ();

    fn rev_into_multi_field_impl(self, _this: This) -> Self::UnnormIntoFields {}
}

unsafe impl<U> ShallowFieldPath for FieldPathSet<LargePathSet<()>, U> {}

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

impl<F, S, This, Mid, OutTy, OutErr> RevIntoMultiFieldImpl<This>
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
