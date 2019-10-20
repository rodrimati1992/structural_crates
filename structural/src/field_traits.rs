use crate::{
    mut_ref::MutRef,
    utils::coerce_slice,
    type_level::MultiTString,
};

use std::marker::PhantomData;


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
    unsafe fn raw_get_mut_field<'a>(this:MutRef<'a,Self>)->&'a mut Self::Ty;
}

/// Converts this type into its `FieldName` field.
pub trait IntoField<FieldName>:GetFieldMut<FieldName>+Sized{
    /// Converts self into the field.
    fn into_field_(self)->Self::Ty;
}


/// An extension trait,which defines methods for accessing fields generically.
pub trait GetFieldExt{
    /// Gets a reference to the ´FieldName´ field.
    fn field<FieldName>(&self,_:FieldName)->&Self::Ty
    where 
        Self:GetField<FieldName>
    {
        self.get_field_()
    }

    /// Gets a mutable reference to the ´FieldName´ field.
    fn field_mut<FieldName>(&mut self,_:FieldName)->&mut Self::Ty
    where 
        Self:GetFieldMut<FieldName>
    {
        self.get_field_mut_()
    }

    /// Converts ´self´ into the ´FieldName´ field.
    fn into_field<FieldName>(self,_:FieldName)->Self::Ty
    where 
        Self:IntoField<FieldName>
    {
        self.into_field_()
    }

    /// Gets mutable references to the ´Field0´ and ´Field1´ fields.
    ///
    /// # Panic
    ///
    /// This function panics if `Field0` and `Field1` are the same field.
    fn field_mut_2<Field0,Field1>(&mut self,_:MultiTString<(Field0,Field1)>)-> (
        &mut GetFieldType<Self,Field0>,
        &mut GetFieldType<Self,Field1>,
    )where
        Self:GetFieldMut<Field0>,
        Self:GetFieldMut<Field1>,
    {
        let this=MutRef::new(self);
        unsafe{
            (
                GetFieldMut::<Field0>::raw_get_mut_field(this.clone()),
                GetFieldMut::<Field1>::raw_get_mut_field(this.clone()),
            )
        }
    }
    
    /// Gets mutable references to the ´Field0´,´Field1´,and ´Field2´ fields.
    ///
    /// # Panic
    ///
    /// This function panics if a field is repeated more than once.
    fn field_mut_3<Field0,Field1,Field2>(&mut self,_:MultiTString<(Field0,Field1,Field2)>)-> (
        &mut GetFieldType<Self,Field0>,
        &mut GetFieldType<Self,Field1>,
        &mut GetFieldType<Self,Field2>,
    )where
        Self:GetFieldMut<Field0>,
        Self:GetFieldMut<Field1>,
        Self:GetFieldMut<Field2>,
    {
        let this=MutRef::new(self);
        unsafe{
            (
                GetFieldMut::<Field0>::raw_get_mut_field(this.clone()),
                GetFieldMut::<Field1>::raw_get_mut_field(this.clone()),
                GetFieldMut::<Field2>::raw_get_mut_field(this.clone()),
            )
        }
    }
    
    /// Gets mutable references to the ´Field0´,´Field1´,´Field2´,and ´Field3´ fields.
    ///
    /// # Panic
    ///
    /// This function panics if a field is repeated more than once.
    fn field_mut_4<Field0,Field1,Field2,Field3>(
        &mut self,
        _:MultiTString<(Field0,Field1,Field2,Field3)>,
    )-> (
        &mut GetFieldType<Self,Field0>,
        &mut GetFieldType<Self,Field1>,
        &mut GetFieldType<Self,Field2>,
        &mut GetFieldType<Self,Field3>,
    )where
        Self:GetFieldMut<Field0>,
        Self:GetFieldMut<Field1>,
        Self:GetFieldMut<Field2>,
        Self:GetFieldMut<Field3>,
    {
        let this=MutRef::new(self);
        unsafe{
            (
                GetFieldMut::<Field0>::raw_get_mut_field(this.clone()),        
                GetFieldMut::<Field1>::raw_get_mut_field(this.clone()),        
                GetFieldMut::<Field2>::raw_get_mut_field(this.clone()),        
                GetFieldMut::<Field3>::raw_get_mut_field(this.clone()),        
            )
        }
    }
}


impl<T:?Sized> GetFieldExt for T{}


