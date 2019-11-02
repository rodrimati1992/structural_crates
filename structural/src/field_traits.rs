/*!
Accessor and extension traits for fields.
*/

use crate::std_::marker::PhantomData;

use crate::{
    mut_ref::MutRef,
    type_level::TStringSet,
    Structural,
    StructuralDyn,
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
/// use structural::{GetField,GetFieldExt,TI,ti};
/// 
/// fn formatted_value<T,S>(this:&T)->String
/// where
///     T:GetField<TI!(v a l u e), Ty=S>,
///     S:std::fmt::Debug,
/// {
///     format!("{:#?}",this.field_(ti!(value)) )
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
/// use structural::{
///     GetField,Structural,TI,TList,
///     structural_trait::{FieldInfo,TField},
///     impl_structural_dyn,
/// };
///
/// struct Huh<T>{
///     value:T,
/// }
///
/// impl<T> Structural for Huh<T>{
///     const FIELDS:&'static[FieldInfo]=&[FieldInfo::not_renamed("value")];
///     
///     type Fields=TList![ TField<TI!(v a l u e),T> ];
/// }
///
/// impl_structural_dyn!{ impl[T] Huh<T> }
///
/// impl<T> GetField<TI!(v a l u e)> for Huh<T>{
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
pub trait GetField<FieldName>:StructuralDyn{
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
/// use structural::{GetField,GetFieldExt,GetFieldType,TI,ti};
///
/// fn get_name<T>(this:&T)->&GetFieldType<T,TI!(n a m e)>
/// where
///     T:GetField<TI!(n a m e)>
/// {
///     this.field_(ti!(name))
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
/// use structural::{GetField,GetFieldExt,GetFieldType,TI,ti};
///
/// fn get_name<T,O>(this:&T)->&O
/// where
///     T:GetField<TI!(n a m e), Ty=O>
/// {
///     this.field_(ti!(name))
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
/// # Usage as Bound Example
///
/// This example demonstrates how you can use this trait as a bound.
///
/// If you have a lot of field accessor bounds you could use `structural_alias` macro
/// to alias those bounds and use that alias instead.
///
/// ```rust
/// use structural::{GetFieldMut,GetFieldExt,TI,ti};
/// 
/// fn take_value<T,V>(this:&mut T)->V
/// where
///     T:GetFieldMut<TI!(v a l u e), Ty=V>,
///     V:Default,
/// {
///     std::mem::replace( this.field_mut(ti!(value)), Default::default() )
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
/// use structural::{
///     GetField,GetFieldMut,Structural,TI,TList,
///     structural_trait::{FieldInfo,TField},
///     mut_ref::MutRef,
///     impl_structural_dyn,
/// };
///
/// struct Huh<T>{
///     value:T,
/// }
///
/// impl<T> Structural for Huh<T>{
///     const FIELDS:&'static[FieldInfo]=&[FieldInfo::not_renamed("value")];
///
///     type Fields=TList![ TField<TI!(v a l u e),T> ];
/// }
///
/// impl_structural_dyn!{ impl[T] Huh<T> }
///
/// impl<T> GetField<TI!(v a l u e)> for Huh<T>{
///     type Ty=T;
///
///     fn get_field_(&self)->&Self::Ty{
///         &self.value
///     }
/// }
///
/// unsafe impl<T> GetFieldMut<TI!(v a l u e)> for Huh<T>{
///     fn get_field_mut_(&mut self)->&mut Self::Ty{
///         &mut self.value
///     }
///     structural::unsafe_impl_get_field_raw_mut_method!{
///         Self,
///         field_name=value,
///         name_generic=TI!(v a l u e)
///     }
/// }
///
/// ```
///
pub unsafe trait GetFieldMut<FieldName>:GetField<FieldName>{
    /// Accesses the `FieldName` field by mutable reference.
    fn get_field_mut_(&mut self)->&mut Self::Ty;

    /// Gets a mutable reference to the field.
    /// 
    /// # Safety
    /// 
    /// For the `ptr` argument,you must pass the return value of the
    /// `as_mutref` method for this field.
    /// 
    /// For the `getter` argument,you must pass the return value of the
    /// `get_field_mutref_func` method for this field.
    ///
    /// The `getter` argument is necessary for boxed trait objects.
    unsafe fn get_field_mutref(
        ptr:MutRef<'_,()>,
        getter:GetFieldMutRefFn<FieldName,Self::Ty>,
    )->&mut Self::Ty
    where 
        Self:Sized;

    /// Gets a pointer to the struct that contains this field.
    /// 
    /// Implementors must return a pointer to the same type that 
    /// `GetFieldMut::get_field_mutref` casts the pointer to.
    fn as_mutref(&mut self)->MutRef<'_,()>;

    /// Gets the `get_field_mutref` associated function as a function pointer.
    fn get_field_mutref_func(&self)->GetFieldMutRefFn<FieldName,Self::Ty>;
}


/////////////////////////////////////////////////

#[repr(transparent)]
pub struct GetFieldMutRefFn<FieldName,FieldTy>{
    pub func:unsafe fn(MutRef<'_,()>,Self)->&mut FieldTy,
    marker:PhantomData<FieldName>,
}

impl<FieldName,FieldTy> GetFieldMutRefFn<FieldName,FieldTy>{
    pub fn new(func:unsafe fn(MutRef<'_,()>,Self)->&mut FieldTy)->Self{
        Self{
            func,
            marker:PhantomData,
        }
    }
}

impl<FieldName,FieldTy> Copy for GetFieldMutRefFn<FieldName,FieldTy>{}

impl<FieldName,FieldTy> Clone for GetFieldMutRefFn<FieldName,FieldTy>{
    #[inline(always)]
    fn clone(&self)->Self{
        *self
    }
}

/////////////////////////////////////////////////


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
/// use structural::{IntoField,GetFieldExt,GetFieldType,TI,ti};
/// 
/// fn into_value<T,V>(this:T)->V
/// where
///     T:IntoField<TI!(v a l u e), Ty=V>,
/// {
///     this.into_field(ti!(value))
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
/// use structural::{
///     GetField,IntoField,Structural,TI,TList,
///     structural_trait::{FieldInfo,TField},
///     mut_ref::MutRef,
///     impl_structural_dyn,
/// };
///
/// struct Huh<T>{
///     value:T,
/// }
///
///
/// impl<T> Structural for Huh<T>{
///     const FIELDS:&'static[FieldInfo]=&[FieldInfo::not_renamed("value")];
///
///     type Fields=TList![ TField<TI!(v a l u e),T> ];
/// }
///
/// impl_structural_dyn!{ impl[T] Huh<T> }
///
/// impl<T> GetField<TI!(v a l u e)> for Huh<T>{
///     type Ty=T;
///
///     fn get_field_(&self)->&Self::Ty{
///         &self.value
///     }
/// }
///
/// impl<T> IntoField<TI!(v a l u e)> for Huh<T>{
///     fn into_field_(self)->Self::Ty{
///         self.value
///     }
///
///     structural::impl_box_into_field_method!{TI!(v a l u e)}
/// }
///
/// ```
///
pub trait IntoField<FieldName>:GetField<FieldName>{
    /// Converts self into the field.
    fn into_field_(self)->Self::Ty
    where Self:Sized;

    /// Converts a boxed self into the field.
    #[cfg(feature="alloc")]
    fn box_into_field_(self: crate::alloc::boxed::Box<Self>)->Self::Ty;
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
    /// use structural::{GetFieldExt,ti};
    ///
    /// let tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.field_(ti!(0)), &1 );
    /// assert_eq!( tup.field_(ti!(1)), &1 );
    /// assert_eq!( tup.field_(ti!(2)), &2 );
    /// assert_eq!( tup.field_(ti!(3)), &3 );
    /// assert_eq!( tup.field_(ti!(4)), &5 );
    /// assert_eq!( tup.field_(ti!(5)), &8 );
    ///
    /// ```
    #[inline(always)]
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
    /// use structural::{GetFieldExt,ti};
    ///
    /// let tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.fields(ti!(0,1)),  (&1,&1) );
    /// assert_eq!( tup.fields(ti!(3,2)),  (&3,&2) );
    /// assert_eq!( tup.fields(ti!(4,5,3)),(&5,&8,&3) );
    ///
    /// ```
    #[inline(always)]
    fn fields<'a,Fields>(&'a self,_:TStringSet<Fields>)->Fields::MultiTy
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
    /// use structural::{GetFieldExt,ti};
    ///
    /// let mut tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.field_mut(ti!(0)), &mut 1 );
    /// assert_eq!( tup.field_mut(ti!(1)), &mut 1 );
    /// assert_eq!( tup.field_mut(ti!(2)), &mut 2 );
    /// assert_eq!( tup.field_mut(ti!(3)), &mut 3 );
    /// assert_eq!( tup.field_mut(ti!(4)), &mut 5 );
    /// assert_eq!( tup.field_mut(ti!(5)), &mut 8 );
    ///
    /// ```
    #[inline(always)]
    fn field_mut<FieldName>(&mut self,_:FieldName)->&mut Self::Ty
    where 
        Self:GetFieldMut<FieldName>
    {
        self.get_field_mut_()
    }

    /// Gets multiple mutable references to fields.
    ///
    /// This is safe since `TStringSet` requires its strings 
    /// to be checked for uniqueness before being constructed
    /// (the safety invariant of `TStringSet`).
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,ti};
    ///
    /// let mut tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.fields_mut(ti!(0,1)), (&mut 1,&mut 1) );
    /// assert_eq!( tup.fields_mut(ti!(3,2)), (&mut 3,&mut 2) );
    /// assert_eq!( tup.fields_mut(ti!(4,5,3)), (&mut 5,&mut 8,&mut 3) );
    ///
    /// ```
    #[inline(always)]
    fn fields_mut<'a,Fields>(&'a mut self,ms:TStringSet<Fields>)->Fields::MultiTy
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
    /// use structural::{GetFieldExt,ti};
    ///
    /// let tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.clone().into_field(ti!(0)), 1 );
    /// assert_eq!( tup.clone().into_field(ti!(1)), 1 );
    /// assert_eq!( tup.clone().into_field(ti!(2)), 2 );
    /// assert_eq!( tup.clone().into_field(ti!(3)), 3 );
    /// assert_eq!( tup.clone().into_field(ti!(4)), 5 );
    /// assert_eq!( tup.clone().into_field(ti!(5)), 8 );
    ///
    /// ```
    #[inline(always)]
    fn into_field<FieldName>(self,_:FieldName)->Self::Ty
    where 
        Self:IntoField<FieldName>+Sized,
    {
        self.into_field_()
    }

    /// Converts a boxed ´self´ into the ´FieldName´ field.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,ti};
    ///
    /// let tup=Box::new((1,1,2,3,5,8));
    ///
    /// assert_eq!( tup.clone().box_into_field(ti!(0)), 1 );
    /// assert_eq!( tup.clone().box_into_field(ti!(1)), 1 );
    /// assert_eq!( tup.clone().box_into_field(ti!(2)), 2 );
    /// assert_eq!( tup.clone().box_into_field(ti!(3)), 3 );
    /// assert_eq!( tup.clone().box_into_field(ti!(4)), 5 );
    /// assert_eq!( tup.clone().box_into_field(ti!(5)), 8 );
    ///
    /// ```
    #[cfg(feature="alloc")]
    #[inline(always)]
    fn box_into_field<FieldName>(self:crate::alloc::boxed::Box<Self>,_:FieldName)->Self::Ty
    where 
        Self:IntoField<FieldName>,
    {
        self.box_into_field_()
    }
}


impl<T:?Sized> GetFieldExt for T{}



///////////////////////////////////////////////////////////////////////////////


macro_rules! unsized_impls {
    ( shared,$ptr:ident ) => {

        impl<T> Structural for $ptr<T>
        where
            T:Structural
        {
            const FIELDS:&'static [FieldInfo]=T::FIELDS;

            type Fields=T::Fields;
        }

        impl<T> StructuralDyn for $ptr<T>
        where
            T:StructuralDyn+?Sized
        {
            fn fields_info(&self)->&'static[FieldInfo]{
                (**self).fields_info()
            }
        }


        impl<This,Name,Ty> GetField<Name> for $ptr<This>
        where
            This:GetField<Name,Ty=Ty>+?Sized
        {
            type Ty=Ty;

            fn get_field_(&self)->&Self::Ty{
                (**self).get_field_()
            }
        }
    };
    (mutable,$ptr:ident)=>{

        unsized_impls!{ shared,$ptr }

        unsafe impl<T,FieldName,Ty> GetFieldMut<FieldName> for Box<T>
        where
            T:GetFieldMut<FieldName,Ty=Ty>+?Sized
        {
            /// Accesses the `FieldName` field by mutable reference.
            fn get_field_mut_(&mut self)->&mut Self::Ty{
                (**self).get_field_mut_()
            }

            default_if!{
                cfg(feature="specialization")
                unsafe fn get_field_mutref(
                    ptr:MutRef<'_,()>,
                    get_field:GetFieldMutRefFn<FieldName,Self::Ty>
                )->&mut Self::Ty{
                    (get_field.func)(ptr,get_field)
                }
            }

            fn as_mutref(&mut self)->MutRef<'_,()>{
                (**self).as_mutref()
            }

            fn get_field_mutref_func(&self)->GetFieldMutRefFn<FieldName,Ty>{
                (**self).get_field_mutref_func()
            }
        }


        #[cfg(feature="specialization")]
        unsafe impl<T,FieldName,Ty> GetFieldMut<FieldName> for Box<T>
        where
            T:GetFieldMut<FieldName,Ty=Ty>
        {
            unsafe fn get_field_mutref(
                ptr:MutRef<'_,()>,
                get_field:GetFieldMutRefFn<FieldName,Self::Ty>
            )->&mut Self::Ty{
                T::get_field_mutref(ptr,get_field)
            }
        }
    };
    (value,$ptr:ident)=>{
        
        unsized_impls!{ mutable,$ptr }

    };
}

#[cfg(feature="alloc")]
mod alloc_impls{
    use super::*;

    use crate::{
        alloc::{
            boxed::Box,
            rc::Rc,
            sync::Arc,
        },
        structural_trait::FieldInfo,
    };

    unsized_impls!{value,Box}
    unsized_impls!{shared,Arc}
    unsized_impls!{shared,Rc}
}