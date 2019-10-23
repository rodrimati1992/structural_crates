use crate::{
    mut_ref::MutRef,
    type_level::MultiTString,
};


mod tuple_impls;


/// Allows accessing the `FieldName` field.
///
/// `FieldName` represents the name of the field on the type level,
/// It is a type because a `FIELD_NAME:&'static str` const parameter 
/// was neither stable nor worked in nightly at the time this was defined..
pub trait GetField<FieldName>{
    /// The type of the `FieldName` field.
    type Ty;

    /// Accesses the `FieldName` field by reference.
    fn get_field_(&self)->&Self::Ty;
}


pub type GetFieldType<This,FieldName>=<This as GetField<FieldName>>::Ty;


/// Allows accessing the `FieldName` field mutably.
///
/// # Safety
///
/// This trait must be implemented for a field of the `FieldName` name.
///
/// The `raw_get_mut_field` method must only access the `FieldName` field.
/// It's definition must always be `&mut (*this.ptr).$field_name`.
///
pub unsafe trait GetFieldMut<FieldName>:GetField<FieldName>{
    /// Accesses the `FieldName` field by mutable reference.
    fn get_field_mut_(&mut self)->&mut Self::Ty;

    /// Accesses the `FieldName` field mutably.
    ///
    /// # Safety
    ///
    /// Once you call this function,it must not be called again for the same `FieldName`
    /// until the returned mutable reference is dropped.
    unsafe fn raw_get_mut_field<'a>(this:MutRef<'a,Self>)->&'a mut Self::Ty
    where Self::Ty:'a;
}

/// Converts this type into its `FieldName` field.
pub trait IntoField<FieldName>:GetFieldMut<FieldName>+Sized{
    /// Converts self into the field.
    fn into_field_(self)->Self::Ty;
}



/// This trait allows a MultiTString to borrow the fields it names.
pub trait GetMultiField<'a,This:?Sized>{
    type MultiTy:'a;

    fn multi_get_field_(this:&'a This)->Self::MultiTy;
}

/// This trait allows a MultiTString to borrow the fields it names mutably.
pub trait GetMultiFieldMut<'a,This:?Sized>:Sized{
    type MultiTy:'a;

    fn multi_get_field_mut_(this:&'a mut This,_:MultiTString<Self>)->Self::MultiTy;
}


macro_rules! impl_get_multi_field {
    ( $($fname:ident)* ) => (
        impl<'a,This:?Sized,$($fname,)*> GetMultiField<'a,This> for ($($fname,)*)
        where
            $(
                This:GetField<$fname>,
                GetFieldType<This,$fname>:'a,
            )*
        {
            type MultiTy=(
                $(
                    &'a GetFieldType<This,$fname>,
                )*
            );

            fn multi_get_field_(this:&'a This)->Self::MultiTy{
                (
                    $(
                        GetField::<$fname>::get_field_(this),
                    )*
                )
            }
        }

        impl<'a,This:?Sized,$($fname,)*> GetMultiFieldMut<'a,This> for ($($fname,)*)
        where
            $(
                This:GetFieldMut<$fname>,
                GetFieldType<This,$fname>:'a,
            )*
        {
            type MultiTy=(
                $(
                    &'a mut GetFieldType<This,$fname>,
                )*
            );

            fn multi_get_field_mut_(this:&'a mut This,_:MultiTString<Self>)->Self::MultiTy{
                let this=MutRef::new(this);
                unsafe{
                    (
                        $(
                            GetFieldMut::<$fname>::raw_get_mut_field(this.clone()),
                        )*
                    )
                }
            }
        }
    )
}


impl_get_multi_field!{F0}
impl_get_multi_field!{F0 F1}
impl_get_multi_field!{F0 F1 F2}
impl_get_multi_field!{F0 F1 F2 F3}
impl_get_multi_field!{F0 F1 F2 F3 F4}
impl_get_multi_field!{F0 F1 F2 F3 F4 F5}
impl_get_multi_field!{F0 F1 F2 F3 F4 F5 F6}
impl_get_multi_field!{F0 F1 F2 F3 F4 F5 F6 F7}



/// An extension trait,which defines methods for accessing fields generically.
pub trait GetFieldExt{
    /// Gets a reference to the ´FieldName´ field.
    ///
    /// This is named `field_` instead of `field`
    /// because `field` collides with the `DebugTuple`/`DebugStruct` method
    fn field_<FieldName>(&self,_:FieldName)->&Self::Ty
    where 
        Self:GetField<FieldName>
    {
        self.get_field_()
    }

    /// Gets multiple references to fields.
    fn fields<'a,Fields>(&'a self,_:MultiTString<Fields>)->Fields::MultiTy
    where
        Fields:GetMultiField<'a,Self>
    {
        Fields::multi_get_field_(self)
    }

    /// Gets a mutable reference to the ´FieldName´ field.
    fn field_mut<FieldName>(&mut self,_:FieldName)->&mut Self::Ty
    where 
        Self:GetFieldMut<FieldName>
    {
        self.get_field_mut_()
    }

    /// Gets multiple mutable references to fields.
    ///
    /// This is safe since `MultiTString` requires its strings 
    /// to be checked for uniqueness before being constructed
    /// (the safety invariant of `MultiTString`).
    fn fields_mut<'a,Fields>(&'a mut self,ms:MultiTString<Fields>)->Fields::MultiTy
    where
        Fields:GetMultiFieldMut<'a,Self>
    {
        Fields::multi_get_field_mut_(self,ms)
    }

    /// Converts ´self´ into the ´FieldName´ field.
    fn into_field<FieldName>(self,_:FieldName)->Self::Ty
    where 
        Self:IntoField<FieldName>
    {
        self.into_field_()
    }
}


impl<T:?Sized> GetFieldExt for T{}




