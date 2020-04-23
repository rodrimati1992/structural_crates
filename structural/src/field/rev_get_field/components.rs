use crate::{
    enums::{EnumExt, IsVariant, VariantProxy},
    field::{
        FailedAccess, FieldType, GetField, GetFieldMut, GetFieldType, GetVariantField,
        GetVariantFieldMut, InfallibleAccess, IntoField, IntoVariantField, MovedOutFields,
        RevFieldErr, RevFieldType, RevGetFieldImpl, RevGetFieldMutImpl, RevIntoFieldImpl,
        RevMoveOutFieldImpl,
    },
    TStr, VariantField, VariantName,
};

////////////////////////////////////////////////////////////////////////////////////////
////                    TStr
////////////////////////////////////////////////////////////////////////////////////////

// Used as an implementation detail of the specialized RevGetFieldMutImpl impls.
//
// This was created because the error messages with specialization enabled were worse,
// it said `VariantField<_,_> does not implement RevGetFieldMutImpl<'_,Foo>`,
// when it should have said `Foo does not implement GetFieldMut<VariantField<_,_>>`,
// which it does with this.
#[doc(hidden)]
unsafe trait SpecRevGetFieldMut<'a, This: ?Sized>: RevGetFieldImpl<'a, This> {
    unsafe fn rev_get_field_raw_mut_inner(
        self,
        this: *mut This,
    ) -> Result<*mut Self::Ty, Self::Err>;
}

impl<This, T> RevFieldType<This> for TStr<T>
where
    This: ?Sized + FieldType<Self>,
{
    type Ty = GetFieldType<This, Self>;
}

impl<This, T> RevFieldErr<This> for TStr<T>
where
    This: ?Sized + FieldType<Self>,
{
    type Err = InfallibleAccess;
}

impl<'a, This, T> RevGetFieldImpl<'a, This> for TStr<T>
where
    This: ?Sized + 'a + GetField<Self>,
    This::Ty: 'a,
{
    #[inline(always)]
    fn rev_get_field(self, this: &'a This) -> Result<&'a This::Ty, InfallibleAccess> {
        Ok(GetField::get_field_(this, self))
    }
}

unsafe impl<'a, This, T> RevGetFieldMutImpl<'a, This> for TStr<T>
where
    This: ?Sized + 'a + GetFieldMut<Self>,
    This::Ty: 'a,
{
    #[inline(always)]
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut This::Ty, InfallibleAccess> {
        Ok(GetFieldMut::get_field_mut_(this, self))
    }

    #[inline(always)]
    unsafe fn rev_get_field_raw_mut(
        self,
        this: *mut This,
    ) -> Result<*mut This::Ty, InfallibleAccess> {
        SpecRevGetFieldMut::<'a, This>::rev_get_field_raw_mut_inner(self, this)
    }
}

unsafe impl<'a, This, T> SpecRevGetFieldMut<'a, This> for TStr<T>
where
    This: ?Sized + 'a + GetFieldMut<Self>,
    This::Ty: 'a,
{
    default_if! {
        #[inline(always)]
        cfg(feature="specialization")
        unsafe fn rev_get_field_raw_mut_inner(
            self,
            this:*mut  This
        )-> Result<*mut This::Ty,InfallibleAccess>{
            let func=(*this).get_field_raw_mut_fn();
            Ok(func(
                this as *mut  (),
                self,
            ))
        }
    }
}

#[cfg(feature = "specialization")]
unsafe impl<'a, This, T> SpecRevGetFieldMut<'a, This> for TStr<T>
where
    This: 'a + GetFieldMut<Self>,
    This::Ty: 'a,
{
    #[inline(always)]
    unsafe fn rev_get_field_raw_mut_inner(
        self,
        this: *mut This,
    ) -> Result<*mut This::Ty, InfallibleAccess> {
        Ok(<This as GetFieldMut<Self>>::get_field_raw_mut(
            this as *mut (),
            self,
        ))
    }
}

impl<This, T> RevIntoFieldImpl<This> for TStr<T>
where
    This: ?Sized + IntoField<Self>,
{
    #[inline(always)]
    fn rev_into_field(self, this: This) -> Result<This::Ty, InfallibleAccess>
    where
        This: Sized,
        Self::Ty: Sized,
    {
        Ok(this.into_field_(self))
    }
}

unsafe impl<This, T> RevMoveOutFieldImpl<This> for TStr<T>
where
    This: ?Sized + IntoField<Self>,
{
    unsafe fn rev_move_out_field(
        self,
        this: &mut This,
        moved: &mut MovedOutFields,
    ) -> Result<Self::Ty, Self::Err>
    where
        Self::Ty: Sized,
    {
        Ok(this.move_out_field_(self, moved))
    }
}

////////////////////////////////////////////////////////////////////////////////////////
////                    VariantField
////////////////////////////////////////////////////////////////////////////////////////

impl<This, _V, _F> RevFieldType<This> for VariantField<_V, _F>
where
    This: ?Sized + FieldType<Self>,
{
    type Ty = GetFieldType<This, Self>;
}

impl<This, _V, _F> RevFieldErr<This> for VariantField<_V, _F>
where
    This: ?Sized + FieldType<Self>,
{
    type Err = FailedAccess;
}

impl<'a, This, _V, _F> RevGetFieldImpl<'a, This> for VariantField<_V, _F>
where
    This: ?Sized + 'a + GetVariantField<_V, _F>,
    This::Ty: 'a,
{
    #[inline(always)]
    fn rev_get_field(self, this: &'a This) -> Result<&'a This::Ty, FailedAccess> {
        ok_or_of!(GetVariantField::get_vfield_(this, self.variant, self.field))
    }
}

unsafe impl<'a, This, _V, _F> RevGetFieldMutImpl<'a, This> for VariantField<_V, _F>
where
    This: ?Sized + 'a + GetVariantFieldMut<_V, _F>,
    This::Ty: 'a,
{
    #[inline(always)]
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut This::Ty, FailedAccess> {
        ok_or_of!(GetVariantFieldMut::get_vfield_mut_(
            this,
            self.variant,
            self.field
        ))
    }

    #[inline(always)]
    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut This::Ty, FailedAccess> {
        SpecRevGetFieldMut::<'a, This>::rev_get_field_raw_mut_inner(self, this)
    }
}

unsafe impl<'a, This, _V, _F> SpecRevGetFieldMut<'a, This> for VariantField<_V, _F>
where
    This: ?Sized + 'a + GetVariantFieldMut<_V, _F>,
    This::Ty: 'a,
{
    default_if! {
        #[inline(always)]
        cfg(feature="specialization")
        unsafe fn rev_get_field_raw_mut_inner(
            self,
            this:*mut  This
        )-> Result<*mut This::Ty,FailedAccess>{
            let func=(*this).get_vfield_raw_mut_fn();
            match func( this as *mut  (), self.variant, self.field ) {
                Some(x) => Ok(x.as_ptr()),
                None => Err(FailedAccess),
            }
        }
    }
}

#[cfg(feature = "specialization")]
unsafe impl<'a, This, _V, _F> SpecRevGetFieldMut<'a, This> for VariantField<_V, _F>
where
    This: 'a + GetVariantFieldMut<_V, _F>,
    This::Ty: 'a,
{
    #[inline(always)]
    unsafe fn rev_get_field_raw_mut_inner(
        self,
        this: *mut This,
    ) -> Result<*mut This::Ty, FailedAccess> {
        let ret = <This as GetVariantFieldMut<_V, _F>>::get_vfield_raw_mut_(
            this as *mut (),
            self.variant,
            self.field,
        );
        match ret {
            Some(x) => Ok(x.as_ptr()),
            None => Err(FailedAccess),
        }
    }
}

impl<This, _V, _F> RevIntoFieldImpl<This> for VariantField<_V, _F>
where
    This: ?Sized + IntoVariantField<_V, _F>,
{
    #[inline(always)]
    fn rev_into_field(self, this: This) -> Result<This::Ty, FailedAccess>
    where
        This: Sized,
    {
        ok_or_of!(this.into_vfield_(self.variant, self.field))
    }
}

unsafe impl<This, _V, _F> RevMoveOutFieldImpl<This> for VariantField<_V, _F>
where
    This: ?Sized + IntoVariantField<_V, _F>,
{
    #[inline(always)]
    unsafe fn rev_move_out_field(
        self,
        this: &mut This,
        moved: &mut MovedOutFields,
    ) -> Result<This::Ty, FailedAccess>
    where
        This::Ty: Sized,
    {
        ok_or_of!(this.move_out_vfield_(self.variant, self.field, moved))
    }
}

////////////////////////////////////////////////////////////////////////////////////////
////                    VariantName
////////////////////////////////////////////////////////////////////////////////////////

impl<This, S> RevFieldType<This> for VariantName<TStr<S>>
where
    This: ?Sized + IsVariant<TStr<S>>,
    S: 'static,
{
    type Ty = VariantProxy<This, TStr<S>>;
}

impl<This, S> RevFieldErr<This> for VariantName<TStr<S>>
where
    This: ?Sized + IsVariant<TStr<S>>,
    S: 'static,
{
    type Err = FailedAccess;
}

impl<'a, This, S> RevGetFieldImpl<'a, This> for VariantName<TStr<S>>
where
    This: ?Sized + 'a + IsVariant<TStr<S>>,
    S: 'static,
{
    #[inline(always)]
    fn rev_get_field(
        self,
        this: &'a This,
    ) -> Result<&'a VariantProxy<This, TStr<S>>, FailedAccess> {
        map_of!(this.as_variant(self.name))
    }
}

unsafe impl<'a, This, S> RevGetFieldMutImpl<'a, This> for VariantName<TStr<S>>
where
    This: ?Sized + 'a + IsVariant<TStr<S>>,
    S: 'static,
{
    #[inline(always)]
    fn rev_get_field_mut(
        self,
        this: &'a mut This,
    ) -> Result<&'a mut VariantProxy<This, TStr<S>>, FailedAccess> {
        map_of!(this.as_mut_variant(self.name))
    }

    #[inline(always)]
    unsafe fn rev_get_field_raw_mut(
        self,
        this: *mut This,
    ) -> Result<*mut VariantProxy<This, TStr<S>>, FailedAccess> {
        map_of!(EnumExt::as_raw_mut_variant(this, self.name))
    }
}

impl<This, S> RevIntoFieldImpl<This> for VariantName<TStr<S>>
where
    This: ?Sized + IsVariant<TStr<S>>,
    S: 'static,
{
    #[inline(always)]
    fn rev_into_field(self, this: This) -> Result<VariantProxy<This, TStr<S>>, FailedAccess>
    where
        This: Sized,
    {
        map_of!(this.into_variant(self.name))
    }
}
