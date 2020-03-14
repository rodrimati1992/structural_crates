use super::*;

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

impl<'a, This, T> RevGetFieldImpl<'a, This> for TStr<T>
where
    This: ?Sized + 'a + GetField<Self>,
    This::Ty: 'a,
{
    type Err = StructField;

    #[inline(always)]
    fn rev_get_field(self, this: &'a This) -> Result<&'a This::Ty, StructField> {
        Ok(GetField::get_field_(this, self))
    }
}

unsafe impl<'a, This, T> RevGetFieldMutImpl<'a, This> for TStr<T>
where
    This: ?Sized + 'a + GetFieldMut<Self>,
    This::Ty: 'a,
{
    #[inline(always)]
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut This::Ty, StructField> {
        Ok(GetFieldMut::get_field_mut_(this, self))
    }

    #[inline(always)]
    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut This::Ty, StructField> {
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
        )-> Result<*mut This::Ty,StructField>{
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
    ) -> Result<*mut This::Ty, StructField> {
        Ok(<This as GetFieldMut<Self>>::get_field_raw_mut(
            this as *mut (),
            self,
        ))
    }
}

impl<'a, This, T> RevIntoFieldImpl<'a, This> for TStr<T>
where
    This: ?Sized + 'a + IntoField<Self>,
    This::Ty: 'a,
{
    type BoxedTy = This::Ty;

    #[inline(always)]
    fn rev_into_field(self, this: This) -> Result<This::Ty, StructField>
    where
        This: Sized,
    {
        Ok(this.into_field_(self))
    }

    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn rev_box_into_field(self, this: crate::pmr::Box<This>) -> Result<This::Ty, StructField> {
        Ok(this.box_into_field_(self))
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

impl<'a, This, _V, _F> RevGetFieldImpl<'a, This> for VariantField<_V, _F>
where
    This: ?Sized + 'a + GetVariantField<_V, _F>,
    This::Ty: 'a,
{
    type Err = EnumField;

    #[inline(always)]
    fn rev_get_field(self, this: &'a This) -> Result<&'a This::Ty, EnumField> {
        ok_or_of!(GetVariantField::get_vfield_(this, self.variant, self.field))
    }
}

unsafe impl<'a, This, _V, _F> RevGetFieldMutImpl<'a, This> for VariantField<_V, _F>
where
    This: ?Sized + 'a + GetVariantFieldMut<_V, _F>,
    This::Ty: 'a,
{
    #[inline(always)]
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut This::Ty, EnumField> {
        ok_or_of!(GetVariantFieldMut::get_vfield_mut_(
            this,
            self.variant,
            self.field
        ))
    }

    #[inline(always)]
    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut This::Ty, EnumField> {
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
        )-> Result<*mut This::Ty,EnumField>{
            let func=(*this).get_vfield_raw_mut_fn();
            match func( this as *mut  (), self.variant, self.field ) {
                Some(x) => Ok(x.as_ptr()),
                None => Err(EnumField),
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
    ) -> Result<*mut This::Ty, EnumField> {
        let ret = <This as GetVariantFieldMut<_V, _F>>::get_vfield_raw_mut_(
            this as *mut (),
            self.variant,
            self.field,
        );
        match ret {
            Some(x) => Ok(x.as_ptr()),
            None => Err(EnumField),
        }
    }
}

impl<'a, This, _V, _F> RevIntoFieldImpl<'a, This> for VariantField<_V, _F>
where
    This: ?Sized + 'a + IntoVariantField<_V, _F>,
    This::Ty: 'a,
{
    type BoxedTy = This::Ty;

    #[inline(always)]
    fn rev_into_field(self, this: This) -> Result<This::Ty, EnumField>
    where
        This: Sized,
    {
        ok_or_of!(this.into_vfield_(self.variant, self.field))
    }

    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn rev_box_into_field(self, this: crate::pmr::Box<This>) -> Result<This::Ty, EnumField> {
        ok_or_of!(this.box_into_vfield_(self.variant, self.field))
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

impl<'a, This, S> RevGetFieldImpl<'a, This> for VariantName<TStr<S>>
where
    This: ?Sized + 'a + IsVariant<TStr<S>>,
    S: 'static,
{
    type Err = EnumField;

    #[inline(always)]
    fn rev_get_field(self, this: &'a This) -> Result<&'a VariantProxy<This, TStr<S>>, EnumField> {
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
    ) -> Result<&'a mut VariantProxy<This, TStr<S>>, EnumField> {
        map_of!(this.as_mut_variant(self.name))
    }

    #[inline(always)]
    unsafe fn rev_get_field_raw_mut(
        self,
        this: *mut This,
    ) -> Result<*mut VariantProxy<This, TStr<S>>, EnumField> {
        map_of!(EnumExt::as_raw_mut_variant(this, self.name))
    }
}

impl<'a, This, S> RevIntoFieldImpl<'a, This> for VariantName<TStr<S>>
where
    This: ?Sized + 'a + IsVariant<TStr<S>>,
    S: 'static,
{
    type BoxedTy = VariantProxy<Box<This>, TStr<S>>;

    #[inline(always)]
    fn rev_into_field(self, this: This) -> Result<VariantProxy<This, TStr<S>>, EnumField>
    where
        This: Sized,
    {
        map_of!(this.into_variant(self.name))
    }

    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn rev_box_into_field(
        self,
        this: crate::pmr::Box<This>,
    ) -> Result<VariantProxy<Box<This>, TStr<S>>, EnumField> {
        map_of!(this.box_into_variant(self.name))
    }
}
