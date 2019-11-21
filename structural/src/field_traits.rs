/*!
Accessor and extension traits for fields.
*/

use crate::{
    mut_ref::MutRef,
    type_level::{FieldPath,FieldPathSet,IsFieldPath,IsFieldPathSet,UniquePaths},
    Structural,
    StructuralDyn,
};

use std_::marker::PhantomData;


mod tuple_impls;
mod most_impls;
pub mod rev_get_field;
pub mod multi_fields;


use self::rev_get_field::{
    RevFieldType,
    RevFieldMutType,
    RevFieldMutRefType,
    RevIntoFieldType,
    RevGetField,
    RevGetFieldMut,
    RevIntoField,
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
/// use structural::{GetField,GetFieldExt,FP,fp};
/// 
/// fn formatted_value<T,S>(this:&T)->String
/// where
///     T:GetField<FP!(v a l u e), Ty=S>,
///     S:std::fmt::Debug,
/// {
///     format!("{:#?}",this.field_(fp!(value)) )
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
///     GetField,Structural,FP,TList,
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
///     type Fields=TList![ TField<FP!(v a l u e),T> ];
/// }
///
/// impl_structural_dyn!{ impl[T] Huh<T> }
///
/// impl<T> GetField<FP!(v a l u e)> for Huh<T>{
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
/// use structural::{GetField,GetFieldExt,GetFieldType,FP,fp};
///
/// fn get_name<T>(this:&T)->&GetFieldType<T,FP!(n a m e)>
/// where
///     T:GetField<FP!(n a m e)>
/// {
///     this.field_(fp!(name))
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
/// use structural::{GetField,GetFieldExt,GetFieldType,FP,fp};
///
/// fn get_name<T,O>(this:&T)->&O
/// where
///     T:GetField<FP!(n a m e), Ty=O>
/// {
///     this.field_(fp!(name))
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
/// use structural::{GetFieldMut,GetFieldExt,FP,fp};
/// 
/// fn take_value<T,V>(this:&mut T)->V
/// where
///     T:GetFieldMut<FP!(v a l u e), Ty=V>,
///     V:Default,
/// {
///     std::mem::replace( this.field_mut(fp!(value)), Default::default() )
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
///     GetField,GetFieldMut,Structural,FP,TList,
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
///     type Fields=TList![ TField<FP!(v a l u e),T> ];
/// }
///
/// impl_structural_dyn!{ impl[T] Huh<T> }
///
/// impl<T> GetField<FP!(v a l u e)> for Huh<T>{
///     type Ty=T;
///
///     fn get_field_(&self)->&Self::Ty{
///         &self.value
///     }
/// }
///
/// unsafe impl<T> GetFieldMut<FP!(v a l u e)> for Huh<T>{
///     fn get_field_mut_(&mut self)->&mut Self::Ty{
///         &mut self.value
///     }
///     structural::unsafe_impl_get_field_raw_mut_method!{
///         Self,
///         field_name=value,
///         name_generic=FP!(v a l u e)
///     }
/// }
///
/// ```
///
pub unsafe trait GetFieldMut<FieldName>:GetField<FieldName>{
    /// Accesses the `FieldName` field by mutable reference.
    fn get_field_mut_(&mut self)->&mut Self::Ty;

    /// Gets a MutRef to the field.
    /// 
    /// # Safety
    /// 
    /// For the `ptr` argument,you must pass the return value of the
    /// `as_mutref` method for this field.
    /// 
    unsafe fn get_field_raw_mut(ptr:*mut (),_:PhantomData<FieldName>)->*mut Self::Ty
    where 
        Self:Sized;

    /// Gets the `get_field_raw_mut` associated function as a function pointer.
    fn get_field_raw_mut_func(&self)->GetFieldMutRefFn<FieldName,Self::Ty>;
}


/////////////////////////////////////////////////


/// TODO
pub type GetFieldMutRefFn<FieldName,FieldTy>=
    unsafe fn(*mut (),PhantomData<FieldName>)->*mut FieldTy;

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
/// use structural::{IntoField,GetFieldExt,GetFieldType,FP,fp};
/// 
/// fn into_value<T,V>(this:T)->V
/// where
///     T:IntoField<FP!(v a l u e), Ty=V>,
/// {
///     this.into_field(fp!(value))
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
///     GetField,IntoField,Structural,FP,TList,
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
///     type Fields=TList![ TField<FP!(v a l u e),T> ];
/// }
///
/// impl_structural_dyn!{ impl[T] Huh<T> }
///
/// impl<T> GetField<FP!(v a l u e)> for Huh<T>{
///     type Ty=T;
///
///     fn get_field_(&self)->&Self::Ty{
///         &self.value
///     }
/// }
///
/// impl<T> IntoField<FP!(v a l u e)> for Huh<T>{
///     fn into_field_(self)->Self::Ty{
///         self.value
///     }
///
///     structural::impl_box_into_field_method!{FP!(v a l u e)}
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
    /// use structural::{GetFieldExt,fp};
    ///
    /// let tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.field_(fp!(0)), &1 );
    /// assert_eq!( tup.field_(fp!(1)), &1 );
    /// assert_eq!( tup.field_(fp!(2)), &2 );
    /// assert_eq!( tup.field_(fp!(3)), &3 );
    /// assert_eq!( tup.field_(fp!(4)), &5 );
    /// assert_eq!( tup.field_(fp!(5)), &8 );
    ///
    /// ```
    #[inline(always)]
    fn field_<'a,P>(&'a self,path:P)->RevFieldType<'a,P,Self>
    where
        P:IsFieldPath,
        P:RevGetField<'a,Self>
    {
        path.rev_get_field(self)
    }

    /// Gets multiple references to fields.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,fp};
    ///
    /// let tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.fields(fp!(0,1)),  (&1,&1) );
    /// assert_eq!( tup.fields(fp!(3,2)),  (&3,&2) );
    /// assert_eq!( tup.fields(fp!(4,5,3)),(&5,&8,&3) );
    ///
    /// ```
    #[inline(always)]
    fn fields<'a,P>(&'a self,path:P)->RevFieldType<'a,P,Self>
    where
        P:RevGetField<'a,Self>
    {
        path.rev_get_field(self)
    }

    /// Gets a mutable reference to the ´FieldName´ field.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,fp};
    ///
    /// let mut tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.field_mut(fp!(0)), &mut 1 );
    /// assert_eq!( tup.field_mut(fp!(1)), &mut 1 );
    /// assert_eq!( tup.field_mut(fp!(2)), &mut 2 );
    /// assert_eq!( tup.field_mut(fp!(3)), &mut 3 );
    /// assert_eq!( tup.field_mut(fp!(4)), &mut 5 );
    /// assert_eq!( tup.field_mut(fp!(5)), &mut 8 );
    ///
    /// ```
    #[inline(always)]
    fn field_mut<'a,P>(&'a mut self,path:P)->RevFieldMutType<'a,P,Self>
    where 
        P:IsFieldPath,
        P:RevGetFieldMut<'a,Self>
    {
        path.rev_get_field_mut(self)
    }

    /// Gets multiple mutable references to fields.
    ///
    /// This is safe since `FieldPathSet` requires its strings 
    /// to be checked for uniqueness before being constructed
    /// (the safety invariant of `FieldPathSet`).
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,fp};
    ///
    /// let mut tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.fields_mut(fp!(0,1)), (&mut 1,&mut 1) );
    /// assert_eq!( tup.fields_mut(fp!(3,2)), (&mut 3,&mut 2) );
    /// assert_eq!( tup.fields_mut(fp!(4,5,3)), (&mut 5,&mut 8,&mut 3) );
    ///
    /// ```
    #[inline(always)]
    fn fields_mut<'a,P>(
        &'a mut self,
        path:P,
    )->RevFieldMutType<'a,P,Self>
    where 
        P:IsFieldPathSet<PathUniqueness=UniquePaths>,
        P:RevGetFieldMut<'a,Self>
    {
        path.rev_get_field_mut(self)
    }

    /// Converts ´self´ into the ´FieldName´ field.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,fp};
    ///
    /// let tup=(1,1,2,3,5,8);
    ///
    /// assert_eq!( tup.clone().into_field(fp!(0)), 1 );
    /// assert_eq!( tup.clone().into_field(fp!(1)), 1 );
    /// assert_eq!( tup.clone().into_field(fp!(2)), 2 );
    /// assert_eq!( tup.clone().into_field(fp!(3)), 3 );
    /// assert_eq!( tup.clone().into_field(fp!(4)), 5 );
    /// assert_eq!( tup.clone().into_field(fp!(5)), 8 );
    ///
    /// ```
    #[inline(always)]
    fn into_field<'a,P>(self,path:P)->RevIntoFieldType<'a,P,Self>
    where 
        P:IsFieldPath,
        P:RevIntoField<'a,Self>,
        Self:Sized,
    {
        path.rev_into_field(self)
    }

    /// Converts a boxed ´self´ into the ´FieldName´ field.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,fp};
    ///
    /// let tup=Box::new((1,1,2,3,5,8));
    ///
    /// assert_eq!( tup.clone().box_into_field(fp!(0)), 1 );
    /// assert_eq!( tup.clone().box_into_field(fp!(1)), 1 );
    /// assert_eq!( tup.clone().box_into_field(fp!(2)), 2 );
    /// assert_eq!( tup.clone().box_into_field(fp!(3)), 3 );
    /// assert_eq!( tup.clone().box_into_field(fp!(4)), 5 );
    /// assert_eq!( tup.clone().box_into_field(fp!(5)), 8 );
    ///
    /// ```
    #[cfg(feature="alloc")]
    #[inline(always)]
    fn box_into_field<'a,P>(
        self:crate::alloc::boxed::Box<Self>,
        path:P,
    )->RevIntoFieldType<'a,P,Self>
    where 
        P:RevIntoField<'a,Self>,
    {
        path.rev_box_into_field(self)
    }
}


impl<T:?Sized> GetFieldExt for T{}



///////////////////////////////////////////////////////////////////////////////


#[cfg(feature="alloc")]
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
                unsafe fn get_field_raw_mut(
                    this:*mut (),
                    name:PhantomData<FieldName>
                )->*mut Self::Ty{
                    let this=this as *mut Self;
                    let func=<T as GetFieldMut<FieldName>>::get_field_raw_mut_func(&**this);
                    func( &mut **this as *mut T as *mut (), name )
                }
            }

            fn get_field_raw_mut_func(&self)->GetFieldMutRefFn<FieldName,Ty>{
                <Self as GetFieldMut<FieldName>>::get_field_raw_mut
            }
        }


        #[cfg(feature="specialization")]
        unsafe impl<T,FieldName,Ty> GetFieldMut<FieldName> for Box<T>
        where
            T:GetFieldMut<FieldName,Ty=Ty>
        {
            unsafe fn get_field_raw_mut(
                ptr:MutRef<'_,()>,
                name:PhantomData<FieldName>,
            )->&mut Self::Ty{
                let this=ptr.cast::<Self>();
                T::get_field_raw_mut(
                    MutRef::from_ptr(&mut **this.ptr as *mut T).cast::<()>(),
                    name,
                )
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



////////////////////////////////////////////////////////////////////////////////


