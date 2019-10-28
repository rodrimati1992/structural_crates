/*!
Accessor and extension traits for fields.
*/

use crate::{
    mut_ref::MutRef,
    type_level::MultiTString,
    Structural,
};


mod tuple_impls;
mod most_impls;
pub mod multi_fields;


use self::multi_fields::{
    GetMultiField,
    GetMultiFieldMut,
};



/// Allows accessing the `FieldName` field.
///
/// `FieldName` represents the name of the field on the type level,
/// It is a type because a `FIELD_NAME:&'static str` const parameter 
/// was neither stable nor worked in nightly at the time this was defined.
///
/// # Usage as Bound Example
///
/// This example demonstrates how you can use this trait as a bound.
///
/// If you have a lot of field accessor bounds you could use `structural_alias` macro
/// to alias those bounds and use that alias instead.
///
/// ```rust
/// use structural::{GetField,GetFieldExt,TStr,tstr};
/// 
/// fn formatted_value<T,S>(this:&T)->String
/// where
///     T:GetField<TStr!(v a l u e), Ty=S>,
///     S:std::fmt::Debug,
/// {
///     format!("{:#?}",this.field_(tstr!("value")) )
/// }
///
/// #[derive(structural::Structural)]
/// struct Huh<T>{
///     #[struc(access="mut")]
///     value:T,
/// }
/// 
/// fn main(){
///     let this=Huh{value:"Hello, World!"};
///     assert!( formatted_value(&this).contains("Hello, World!") );
/// }
///
/// ```
///
/// # Manual Implementation Example
///
/// While this trait is intended to be implemented using the `Structural` derive macro,
/// you can manually implement it like this:
///
/// ```rust
/// use structural::{GetField,Structural,TStr};
/// use structural::structural_trait::FieldInfo;
///
/// struct Huh<T>{
///     value:T,
/// }
///
/// impl<T> Structural for Huh<T>{
///     const FIELDS:&'static[FieldInfo]=&[FieldInfo::not_renamed("value")];
/// }
///
/// impl<T> GetField<TStr!(v a l u e)> for Huh<T>{
///     type Ty=T;
///
///     fn get_field_(&self)->&Self::Ty{
///         &self.value
///     }
/// }
///
///
/// ```
///
pub trait GetField<FieldName>:Structural{
    /// The type of the `FieldName` field.
    type Ty;

    /// Accesses the `FieldName` field by reference.
    fn get_field_(&self)->&Self::Ty;
}


/// Queries the type of a field.
///
/// # Example
///
/// Here is one way you can get the type of a field.
///
/// ```
/// use structural::{GetField,GetFieldExt,GetFieldType,TStr,tstr};
///
/// fn get_name<T>(this:&T)->&GetFieldType<T,TStr!(n a m e)>
/// where
///     T:GetField<TStr!(n a m e)>
/// {
///     this.field_(tstr!("name"))
/// }
///
///
/// #[derive(structural::Structural)]
/// struct Huh<T>{
///     #[struc(public)]
///     #[struc(rename="name")]
///     value:T,
/// }
///
/// fn main(){
///     let this=Huh{ value:"ooh".to_string() };
///     
///     assert_eq!( get_name(&this), "ooh" );
/// }
/// ```
/// 
/// Another way `get_name` could have been written is like this:
///
/// ```
/// use structural::{GetField,GetFieldExt,GetFieldType,TStr,tstr};
///
/// fn get_name<T,O>(this:&T)->&O
/// where
///     T:GetField<TStr!(n a m e), Ty=O>
/// {
///     this.field_(tstr!("name"))
/// }
/// ```
/// A potential downside of adding another type parameter is that it 
/// makes it less ergonomic to specify the type of `T` while ignoring the field type,
/// since one has to write it as `get_name::<Foo,_>(&foo)`.
/// 
///
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
/// # Usage as Bound Example
///
/// This example demonstrates how you can use this trait as a bound.
///
/// If you have a lot of field accessor bounds you could use `structural_alias` macro
/// to alias those bounds and use that alias instead.
///
/// ```rust
/// use structural::{GetFieldMut,GetFieldExt,TStr,tstr};
/// 
/// fn take_value<T,V>(this:&mut T)->V
/// where
///     T:GetFieldMut<TStr!(v a l u e), Ty=V>,
///     V:Default,
/// {
///     std::mem::replace( this.field_mut(tstr!("value")), Default::default() )
/// }
///
/// #[derive(structural::Structural)]
/// struct Huh<T>{
///     #[struc(access="mut")]
///     value:T,
/// }
/// 
/// fn main(){
///     let mut this=Huh{value:"Hello, World!"};
///     assert_eq!(take_value(&mut this),"Hello, World!");
///     assert_eq!(this.value,"");
/// }
///
/// ```
///
/// # Manual Implementation Example
///
/// While this trait is intended to be implemented using the `Structural` derive macro,
/// you can manually implement it like this:
///
/// ```rust
/// use structural::{GetField,GetFieldMut,Structural,TStr};
/// use structural::structural_trait::FieldInfo;
/// use structural::mut_ref::MutRef;
///
/// struct Huh<T>{
///     value:T,
/// }
///
/// impl<T> Structural for Huh<T>{
///     const FIELDS:&'static[FieldInfo]=&[FieldInfo::not_renamed("value")];
/// }
///
/// impl<T> GetField<TStr!(v a l u e)> for Huh<T>{
///     type Ty=T;
///
///     fn get_field_(&self)->&Self::Ty{
///         &self.value
///     }
/// }
///
/// unsafe impl<T> GetFieldMut<TStr!(v a l u e)> for Huh<T>{
///     fn get_field_mut_(&mut self)->&mut Self::Ty{
///         &mut self.value
///     }
///
///     unsafe fn raw_get_mut_field<'a>(this:MutRef<'a,Self>)->&'a mut Self::Ty
///     where
///         Self::Ty:'a,
///     {
///         &mut (*this.ptr).value
///     }
/// }
///
/// ```
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
    where 
        Self:Sized,
        Self::Ty:'a;
}

/// Converts this type into its `FieldName` field.
///
/// # Usage as Bound Example
///
/// This example demonstrates how you can use this trait as a bound.
///
/// If you have a lot of field accessor bounds you could use `structural_alias` macro
/// to alias those bounds and use that alias instead.
///
/// ```rust
/// use structural::{IntoField,GetFieldExt,GetFieldType,TStr,tstr};
/// 
/// fn into_value<T,V>(this:T)->V
/// where
///     T:IntoField<TStr!(v a l u e), Ty=V>,
/// {
///     this.into_field(tstr!("value"))
/// }
///
/// #[derive(structural::Structural)]
/// struct Huh<T>{
///     #[struc(access="move")]
///     value:T,
/// }
/// 
/// fn main(){
///     let this=Huh{value:"Hello, World!"};
///     assert_eq!(into_value(this),"Hello, World!");
/// }
/// 
/// ```
///
/// # Manual Implementation Example
///
/// While this trait is intended to be implemented using the `Structural` derive macro,
/// you can manually implement it like this:
///
/// ```rust
/// use structural::{GetField,IntoField,Structural,TStr};
/// use structural::structural_trait::FieldInfo;
/// use structural::mut_ref::MutRef;
///
/// struct Huh<T>{
///     value:T,
/// }
///
///
/// impl<T> Structural for Huh<T>{
///     const FIELDS:&'static[FieldInfo]=&[FieldInfo::not_renamed("value")];
/// }
///
/// impl<T> GetField<TStr!(v a l u e)> for Huh<T>{
///     type Ty=T;
///
///     fn get_field_(&self)->&Self::Ty{
///         &self.value
///     }
/// }
///
/// impl<T> IntoField<TStr!(v a l u e)> for Huh<T>{
///     fn into_field_(self)->Self::Ty{
///         self.value
///     }
/// }
///
/// ```
///
pub trait IntoField<FieldName>:GetField<FieldName>+Sized{
    /// Converts self into the field.
    fn into_field_(self)->Self::Ty;
}


/// An alias for a shared,mutable,and by-value accessor for a field.
pub trait IntoFieldMut<FieldName>:IntoField<FieldName>+GetFieldMut<FieldName>{}

impl<This,FieldName> IntoFieldMut<FieldName> for This
where
    This:IntoField<FieldName>+GetFieldMut<FieldName>
{}



/// An extension trait,which defines methods for accessing fields generically.
pub trait GetFieldExt{
    /// Gets a reference to the ´FieldName´ field.
    ///
    /// This is named `field_` instead of `field`
    /// because `field` collides with the `DebugTuple`/`DebugStruct` method
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,tstr};
    ///
    /// let tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.field_(tstr!("0")), &1 );
    /// assert_eq!( tup.field_(tstr!("1")), &1 );
    /// assert_eq!( tup.field_(tstr!("2")), &2 );
    /// assert_eq!( tup.field_(tstr!("3")), &3 );
    /// assert_eq!( tup.field_(tstr!("4")), &5 );
    /// assert_eq!( tup.field_(tstr!("5")), &8 );
    ///
    /// ```
    fn field_<FieldName>(&self,_:FieldName)->&Self::Ty
    where 
        Self:GetField<FieldName>
    {
        self.get_field_()
    }

    /// Gets multiple references to fields.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,tstr};
    ///
    /// let tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.fields(tstr!("0","1")), (&1,&1) );
    /// assert_eq!( tup.fields(tstr!("3","2")), (&3,&2) );
    /// assert_eq!( tup.fields(tstr!("4","5","3")), (&5,&8,&3) );
    ///
    /// ```
    fn fields<'a,Fields>(&'a self,_:MultiTString<Fields>)->Fields::MultiTy
    where
        Fields:GetMultiField<'a,Self>
    {
        Fields::multi_get_field_(self)
    }

    /// Gets a mutable reference to the ´FieldName´ field.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,tstr};
    ///
    /// let mut tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.field_mut(tstr!("0")), &mut 1 );
    /// assert_eq!( tup.field_mut(tstr!("1")), &mut 1 );
    /// assert_eq!( tup.field_mut(tstr!("2")), &mut 2 );
    /// assert_eq!( tup.field_mut(tstr!("3")), &mut 3 );
    /// assert_eq!( tup.field_mut(tstr!("4")), &mut 5 );
    /// assert_eq!( tup.field_mut(tstr!("5")), &mut 8 );
    ///
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,tstr};
    ///
    /// let mut tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.fields_mut(tstr!("0","1")), (&mut 1,&mut 1) );
    /// assert_eq!( tup.fields_mut(tstr!("3","2")), (&mut 3,&mut 2) );
    /// assert_eq!( tup.fields_mut(tstr!("4","5","3")), (&mut 5,&mut 8,&mut 3) );
    ///
    /// ```
    fn fields_mut<'a,Fields>(&'a mut self,ms:MultiTString<Fields>)->Fields::MultiTy
    where
        Fields:GetMultiFieldMut<'a,Self>,
        Self:Sized,
    {
        Fields::multi_get_field_mut_(self,ms)
    }

    /// Converts ´self´ into the ´FieldName´ field.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,tstr};
    ///
    /// let tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.clone().into_field(tstr!("0")), 1 );
    /// assert_eq!( tup.clone().into_field(tstr!("1")), 1 );
    /// assert_eq!( tup.clone().into_field(tstr!("2")), 2 );
    /// assert_eq!( tup.clone().into_field(tstr!("3")), 3 );
    /// assert_eq!( tup.clone().into_field(tstr!("4")), 5 );
    /// assert_eq!( tup.clone().into_field(tstr!("5")), 8 );
    ///
    /// ```
    fn into_field<FieldName>(self,_:FieldName)->Self::Ty
    where 
        Self:IntoField<FieldName>
    {
        self.into_field_()
    }
}


impl<T:?Sized> GetFieldExt for T{}




