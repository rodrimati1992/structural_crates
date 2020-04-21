use crate::{
    field::{
        DroppedFields, IntoFieldErr, RevFieldErr, RevFieldType, RevGetFieldImpl,
        RevGetFieldMutImpl, RevIntoFieldImpl, RevMoveOutFieldImpl,
    },
    FieldPathSet, NestedFieldPathSet,
};

impl<This, T, U> RevFieldType<This> for FieldPathSet<(T,), U>
where
    This: ?Sized,
    T: RevFieldType<This>,
{
    type Ty = T::Ty;
}

impl<This, T, U> RevFieldErr<This> for FieldPathSet<(T,), U>
where
    This: ?Sized,
    T: RevFieldErr<This>,
{
    type Err = T::Err;
}

impl<'a, This, T, U> RevGetFieldImpl<'a, This> for FieldPathSet<(T,), U>
where
    This: ?Sized + 'a,
    T: RevGetFieldImpl<'a, This>,
{
    #[inline(always)]
    fn rev_get_field(self, this: &'a This) -> Result<&'a T::Ty, T::Err> {
        self.into_path().rev_get_field(this)
    }
}

unsafe impl<'a, This, T, U> RevGetFieldMutImpl<'a, This> for FieldPathSet<(T,), U>
where
    This: ?Sized + 'a,
    T: RevGetFieldMutImpl<'a, This>,
{
    #[inline(always)]
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut T::Ty, T::Err> {
        self.into_path().rev_get_field_mut(this)
    }

    #[inline(always)]
    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut T::Ty, T::Err> {
        self.into_path().rev_get_field_raw_mut(this)
    }
}

impl<This, T, U> RevIntoFieldImpl<This> for FieldPathSet<(T,), U>
where
    This: ?Sized,
    T: RevIntoFieldImpl<This>,
{
    #[inline(always)]
    fn rev_into_field(self, this: This) -> Result<T::Ty, T::Err>
    where
        This: Sized,
        T::Ty: Sized,
    {
        self.into_path().rev_into_field(this)
    }
}

unsafe impl<This, T, U> RevMoveOutFieldImpl<This> for FieldPathSet<(T,), U>
where
    This: ?Sized,
    T: RevMoveOutFieldImpl<This>,
{
    #[inline(always)]
    unsafe fn rev_move_out_field(
        self,
        this: &mut This,
        dropped: &mut DroppedFields,
    ) -> Result<Self::Ty, Self::Err>
    where
        T::Ty: Sized,
    {
        self.into_path().rev_move_out_field(this, dropped)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<This, A, B, U> RevFieldType<This> for NestedFieldPathSet<A, (B,), U>
where
    This: ?Sized,
    A: RevFieldType<This>,
    B: RevFieldType<A::Ty>,
{
    type Ty = B::Ty;
}

impl<This, A, B, U> RevFieldErr<This> for NestedFieldPathSet<A, (B,), U>
where
    This: ?Sized,
    A: RevFieldErr<This>,
    B: RevFieldErr<A::Ty>,
{
    type Err = B::Err;
}

impl<'a, This, A, B, U> RevGetFieldImpl<'a, This> for NestedFieldPathSet<A, (B,), U>
where
    This: ?Sized + 'a,
    A: RevGetFieldImpl<'a, This>,
    B: RevGetFieldImpl<'a, A::Ty>,
    A::Ty: 'a,
    A::Err: IntoFieldErr<B::Err>,
{
    #[inline(always)]
    fn rev_get_field(self, this: &'a This) -> Result<&'a B::Ty, B::Err> {
        let (nested, set) = self.into_inner();
        let x = try_fe!(nested.rev_get_field(this));
        set.into_path().rev_get_field(x)
    }
}

unsafe impl<'a, This, A, B, U> RevGetFieldMutImpl<'a, This> for NestedFieldPathSet<A, (B,), U>
where
    This: ?Sized + 'a,
    A: RevGetFieldMutImpl<'a, This>,
    B: RevGetFieldMutImpl<'a, A::Ty>,
    A::Ty: 'a,
    A::Err: IntoFieldErr<B::Err>,
{
    #[inline(always)]
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut B::Ty, B::Err> {
        let (nested, set) = self.into_inner();
        let x = try_fe!(nested.rev_get_field_mut(this));
        set.into_path().rev_get_field_mut(x)
    }

    #[inline(always)]
    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut B::Ty, B::Err> {
        let (nested, set) = self.into_inner();
        let x = try_fe!(nested.rev_get_field_raw_mut(this));
        set.into_path().rev_get_field_raw_mut(x)
    }
}

impl<This, A, B, U> RevIntoFieldImpl<This> for NestedFieldPathSet<A, (B,), U>
where
    This: ?Sized,
    A: RevIntoFieldImpl<This>,
    B: RevIntoFieldImpl<A::Ty>,
    A::Ty: Sized,
    A::Err: IntoFieldErr<B::Err>,
{
    #[inline(always)]
    fn rev_into_field(self, this: This) -> Result<B::Ty, B::Err>
    where
        This: Sized,
        B::Ty: Sized,
    {
        let (nested, set) = self.into_inner();
        let x = try_fe!(nested.rev_into_field(this));
        set.into_path().rev_into_field(x)
    }
}
