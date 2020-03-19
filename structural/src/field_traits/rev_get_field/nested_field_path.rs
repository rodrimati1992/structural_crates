use super::*;

macro_rules! impl_get_nested_field_inner {
    (inner;
        receivers( $($receiver:ident)* )
        first($fname0:ident $ferr0:ident $fty0:ident)
        second(
            ($fname1:ident $ferr1:ident $fty1:ident)
            $($rem_000:tt)*
        )
        middle(
            $(($fname_m:ident $ferr_m:ident $fty_m:ident))*
        )
        suffix(
            $(($fname_s:ident $ferr_s:ident $fty_s:ident))*
        )
        all(
            $(($fname_a:ident $ferr_a:ident $fty_a:ident))*
        )
        last($fname_l:ident $ferr_l:ident $fty_l:ident)
    )=>{
        impl<$($fname_a,$fty_a,)* This> RevFieldType<This> for NestedFieldPath<($($fname_a,)*)>
        where
            This:?Sized,
            $(
                $fname_a: RevFieldType<$receiver, Ty=$fty_a>,
                $fty_a:?Sized,
            )*
        {
            type Ty=$fty_l;
        }

        impl<'a,$($fname_a,$fty_a, $ferr_a,)* This,CombErr>
            RevGetFieldImpl<'a,This>
        for NestedFieldPath<($($fname_a,)*)>
        where
            This:?Sized+'a,
            $(
                $fname_a: RevGetFieldImpl<'a,$receiver, Ty=$fty_a, Err=$ferr_a>,
                $fty_a:?Sized+'a,
                $ferr_a:IntoFieldErr< CombErr >,
            )*
            ( $($ferr_a,)* ): CombinedErrs<Combined= CombErr >,
            CombErr:IsFieldErr,
        {
            type Err=CombErr;

            #[inline(always)]
            fn rev_get_field(self,field:&'a This)->Result<&'a $fty_l,CombErr>{
                let ($($fname_a,)*)=self.list;
                $(
                    let field=try_fe!( $fname_a.rev_get_field(field) );
                )*
                Ok(field)
            }
        }


        unsafe impl<'a,$($fname_a,$fty_a, $ferr_a,)* This,CombErr>
            RevGetFieldMutImpl<'a,This>
        for NestedFieldPath<($($fname_a,)*)>
        where
            This:?Sized+'a,
            $(
                $fname_a: RevGetFieldMutImpl<'a,$receiver, Ty=$fty_a, Err=$ferr_a>,
                $fty_a:'a,
                $ferr_a:IntoFieldErr< CombErr >,
            )*
            Self:RevGetFieldImpl<'a,This,Ty=$fty_l,Err=CombErr>,
            $fty_l: Sized,
            CombErr:IsFieldErr,
        {
            #[inline(always)]
            fn rev_get_field_mut(self,field:&'a mut This)->Result<&'a mut $fty_l,CombErr >{
                let ($($fname_a,)*)=self.list;
                $(
                    let field=try_fe!( $fname_a.rev_get_field_mut(field) );
                )*
                Ok(field)
            }

            unsafe fn rev_get_field_raw_mut(
                self,
                field:*mut  This
            )->Result<*mut $fty_l,CombErr>{
                let ($($fname_a,)*)=self.list;
                $(
                    #[allow(unused_mut)]
                    let mut field={
                        try_fe!($fname_a.rev_get_field_raw_mut(field))
                    };
                )*
                Ok(field)
            }
        }


        impl<'a,$($fname_a, $fty_a:'a, $ferr_a,)* This ,CombErr>
            RevIntoFieldImpl<'a,This>
        for NestedFieldPath<($($fname_a,)*)>
        where
            Self:RevGetFieldImpl<'a,This,Ty=$fty_l,Err=CombErr>,
            CombErr:IsFieldErr,

            This:?Sized+'a,
            $fname0: RevIntoFieldImpl< 'a, This, Ty=$fty0, Err=$ferr0>,

            $(
                $fname_s: RevIntoFieldImpl<'a, $fty_m, Ty=$fty_s, Err=$ferr_s>,
            )*

            $( $ferr_a:IntoFieldErr< CombErr >, )*
        {
            #[inline(always)]
            fn rev_into_field(self,field:This)->Result<$fty_l,CombErr>
            where
                This:Sized
            {
                let ($($fname_a,)*)=self.list;
                $(
                    let field=try_fe!( $fname_a.rev_into_field(field) );
                )*
                Ok(field)
            }
        }

    };
    (
        ($fname0:ident $ferr0:ident $fty0:ident)
        $(($fname:ident $ferr:ident $fty:ident))*
        ;last=($fname_l:ident $ferr_l:ident $fty_l:ident)
    ) => {
        impl_get_nested_field_inner!{
            inner;
            receivers( This $fty0 $($fty)* )
            first ($fname0 $ferr0 $fty0)
            second (
                $(($fname $ferr $fty))*
                ($fname_l $ferr_l $fty_l)
            )
            middle(
                ($fname0 $ferr0 $fty0)
                $(($fname $ferr $fty))*
            )
            suffix(
                $(($fname $ferr $fty))*
                ($fname_l $ferr_l $fty_l)
            )
            all(
                ($fname0 $ferr0 $fty0)
                $(($fname $ferr $fty))*
                ($fname_l $ferr_l $fty_l)
            )
            last($fname_l $ferr_l $fty_l)
        }
    }
}

impl_get_nested_field_inner! {
    (F0 E0 T0)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    (F4 E4 T4)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    (F4 E4 T4)
    (F5 E5 T5)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    (F4 E4 T4)
    (F5 E5 T5)
    (F6 E6 T6)
    ;last=(FL EL TL)
}

////////////////////////////////////////////////////////////////////////////////
/////           Implementations for FP!() (An empty NestedFieldPath)
////////////////////////////////////////////////////////////////////////////////

impl<This> RevFieldType<This> for NestedFieldPath<()>
where
    This: ?Sized,
{
    type Ty = This;
}

impl<'a, This> RevGetFieldImpl<'a, This> for NestedFieldPath<()>
where
    This: ?Sized + 'a,
{
    type Err = InfallibleAccess;

    fn rev_get_field(self, this: &'a This) -> Result<&'a Self::Ty, Self::Err> {
        Ok(this)
    }
}

unsafe impl<'a, This> RevGetFieldMutImpl<'a, This> for NestedFieldPath<()>
where
    This: ?Sized + 'a,
{
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut Self::Ty, Self::Err> {
        Ok(this)
    }

    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut Self::Ty, Self::Err> {
        Ok(this)
    }
}

impl<'a, This> RevIntoFieldImpl<'a, This> for NestedFieldPath<()>
where
    This: Sized + 'a,
{
    fn rev_into_field(self, this: This) -> Result<Self::Ty, Self::Err> {
        Ok(this)
    }
}

////////////////////////////////////////////////////////////////////////////////
/////           Implementations for single path component NestedFieldPath
////////////////////////////////////////////////////////////////////////////////

impl<This, F0> RevFieldType<This> for NestedFieldPath<(F0,)>
where
    This: ?Sized,
    F0: RevFieldType<This>,
{
    type Ty = F0::Ty;
}

impl<'a, This, F0> RevGetFieldImpl<'a, This> for NestedFieldPath<(F0,)>
where
    This: ?Sized + 'a,
    F0: RevGetFieldImpl<'a, This>,
{
    type Err = F0::Err;

    fn rev_get_field(self, this: &'a This) -> Result<&'a F0::Ty, F0::Err> {
        self.list.0.rev_get_field(this)
    }
}

unsafe impl<'a, This, F0> RevGetFieldMutImpl<'a, This> for NestedFieldPath<(F0,)>
where
    This: ?Sized + 'a,
    F0: RevGetFieldMutImpl<'a, This>,
{
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut F0::Ty, F0::Err> {
        self.list.0.rev_get_field_mut(this)
    }

    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut F0::Ty, F0::Err> {
        self.list.0.rev_get_field_raw_mut(this)
    }
}

impl<'a, This, F0> RevIntoFieldImpl<'a, This> for NestedFieldPath<(F0,)>
where
    This: ?Sized + 'a,
    F0: RevIntoFieldImpl<'a, This>,
{
    fn rev_into_field(self, this: This) -> Result<F0::Ty, F0::Err>
    where
        This: Sized,
        F0::Ty: Sized,
    {
        self.list.0.rev_into_field(this)
    }
}
