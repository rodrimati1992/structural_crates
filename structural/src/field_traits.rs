/*!
Accessor and extension traits for fields.
*/

use crate::{
    mut_ref::MutRef,
    type_level::{FieldPath,FieldPathSet,IsFieldPath,IsFieldPathSet,UniquePaths},
    Structural,
    StructuralDyn,
};

use core_extensions::collection_traits::Cloned;

use std_::marker::PhantomData;


pub mod for_arrays;
pub mod for_tuples;
mod tuple_impls;
mod most_impls;
pub mod rev_get_field;
mod multi_fields;


pub use self::rev_get_field::{
    RevGetFieldType,
    RevGetFieldType_,
    RevFieldRefType,
    RevFieldMutType,
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
///     structural_trait::{FieldInfo},
///     z_impl_structural_dyn,
/// };
///
/// struct Huh<T>{
///     value:T,
/// }
///
/// impl<T> Structural for Huh<T>{
///     const FIELDS:&'static[FieldInfo]=&[FieldInfo::not_renamed("value")];
///     
/// }
///
/// z_impl_structural_dyn!{ impl[T] Huh<T> }
///
/// // This could also be written as `FP!(value)` from 1.40 onwards
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
///     // `FP!(n a m e)` can be written as `FP!(name)` from 1.40 onwards
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
///     // `FP!(n a m e)` can be written as `FP!(name)` from 1.40 onwards
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



/// Queries the type of a double nested field (eg:`.a.b`).
pub type GetFieldType2<This,FieldName,FieldName2>=
    GetFieldType<
        GetFieldType<This,FieldName>,
        FieldName2
    >;

/// Queries the type of a triple nested field (eg:`.a.b.c`).
pub type GetFieldType3<This,FieldName,FieldName2,FieldName3>=
    GetFieldType<
        GetFieldType2<This,FieldName,FieldName2>,
        FieldName3
    >;

/// Queries the type of a quadruple nested field (eg:`.a.b.c.d`).
pub type GetFieldType4<This,FieldName,FieldName2,FieldName3,FieldName4>=
    GetFieldType2<
        GetFieldType2<This,FieldName,FieldName2>,
        FieldName3,FieldName4,
    >;



/// Allows accessing the `FieldName` field mutably.
///
/// # Safety
/// 
/// These are requirements for manual implementations.
/// 
/// It is recommended that you use the `z_unsafe_impl_get_field_raw_mut_method` macro 
/// if you only borrow a field of the type.
/// 
/// Your implementation of `GetFieldMut::get_field_raw_mut` must ensure these properties:
/// 
/// - It must be side-effect free,
///
/// - The field you borrow must always be the same one.
/// 
/// - That no implementation of `GetFieldMut::get_field_raw_mut`
/// returns a pointer to a field that other ones also return,
/// 
/// Your implementation of the `get_field_raw_mut_func` method must only return a
/// function pointer for the `GetFieldMut::get_field_raw_mut` method from the same 
/// implementation of the `GetFieldMut` trait.
/// 
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
///     // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
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
///     structural_trait::{FieldInfo},
///     mut_ref::MutRef,
///     z_impl_structural_dyn,
/// };
///
/// struct Huh<T>{
///     value:T,
/// }
///
/// impl<T> Structural for Huh<T>{
///     const FIELDS:&'static[FieldInfo]=&[FieldInfo::not_renamed("value")];
///
/// }
///
/// z_impl_structural_dyn!{ impl[T] Huh<T> }
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// impl<T> GetField<FP!(v a l u e)> for Huh<T>{
///     type Ty=T;
///
///     fn get_field_(&self)->&Self::Ty{
///         &self.value
///     }
/// }
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// unsafe impl<T> GetFieldMut<FP!(v a l u e)> for Huh<T>{
///     fn get_field_mut_(&mut self)->&mut Self::Ty{
///         &mut self.value
///     }
///     structural::z_unsafe_impl_get_field_raw_mut_method!{
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

    /// Gets a mutable pointer for the field.
    /// 
    /// # Safety
    /// 
    /// You must pass a pointer casted from `*mut Self` to `*mut ()`.
    unsafe fn get_field_raw_mut(ptr:*mut (),_:PhantomData<FieldName>)->*mut Self::Ty
    where 
        Self:Sized;

    /// Gets the `get_field_raw_mut` associated function as a function pointer.
    fn get_field_raw_mut_func(&self)->GetFieldMutRefFn<FieldName,Self::Ty>;
}


/////////////////////////////////////////////////


/// The type of `GetFieldMut::get_field_raw_mut`
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
///     // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
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
///     structural_trait::{FieldInfo},
///     mut_ref::MutRef,
///     z_impl_structural_dyn,
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
/// }
///
/// z_impl_structural_dyn!{ impl[T] Huh<T> }
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// impl<T> GetField<FP!(v a l u e)> for Huh<T>{
///     type Ty=T;
///
///     fn get_field_(&self)->&Self::Ty{
///         &self.value
///     }
/// }
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// impl<T> IntoField<FP!(v a l u e)> for Huh<T>{
///     fn into_field_(self)->Self::Ty{
///         self.value
///     }
///
///     structural::z_impl_box_into_field_method!{FP!(v a l u e)}
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



/// A trait defining the primary way to call methods from structural traits.
pub trait GetFieldExt{
    /// Gets a reference to a field,determined by `path`.
    ///
    /// This is named `field_` instead of `field`
    /// because `field` collides with the `DebugTuple`/`DebugStruct` method
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,fp,structural_alias};
    ///
    /// structural_alias!{
    ///     trait EvenFields<A,B,C>{
    ///         0:A,
    ///         2:B,
    ///         4:C,
    ///     }
    /// }
    ///
    /// fn with_even<T>(this:&T)
    /// where
    ///     T:EvenFields<u32,u32,u32>
    /// {
    ///     assert_eq!( this.field_(fp!(0)), &1 );
    ///     assert_eq!( this.field_(fp!(2)), &2 );
    ///     assert_eq!( this.field_(fp!(4)), &5 );
    /// }
    ///
    /// fn main(){
    ///     with_even( &(1,0,2,0,5) );
    ///     with_even( &(1,0,2,0,5,0) );
    ///     with_even( &(1,0,2,0,5,0,0) );
    ///     with_even( &(1,0,2,0,5,0,0,0) );
    /// }
    ///
    /// ```
    #[inline(always)]
    fn field_<'a,P>(&'a self,path:P)->RevFieldRefType<'a,P,Self>
    where
        P:IsFieldPath,
        P:RevGetField<'a,Self>
    {
        path.rev_get_field(self)
    }

    /// Gets references to multiple fields,determined by `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,fp,structural_alias};
    ///
    /// structural_alias!{
    ///     trait OddFields<A,B,C>{
    ///         1:A,
    ///         3:B,
    ///         5:C,
    ///     }
    /// }
    ///
    /// fn with_even(this:&impl OddFields<u32,u32,u32>){
    ///     assert_eq!( this.fields(fp!(1,3,5)), (&22,&44,&77) );
    /// }
    ///
    /// fn main(){
    ///     with_even( &(0,22,0,44,0,77) );
    ///     with_even( &(0,22,0,44,0,77,0) );
    ///     with_even( &(0,22,0,44,0,77,0,0) );
    ///     with_even( &(0,22,0,44,0,77,0,0,0) );
    /// }
    ///
    /// ```
    #[inline(always)]
    fn fields<'a,P>(&'a self,path:P)->RevFieldRefType<'a,P,Self>
    where
        P:RevGetField<'a,Self>
    {
        path.rev_get_field(self)
    }

    /// Gets clones of multiple fields,determined by `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp,make_struct};
    /// use structural::reexports::IntoArray;
    /// 
    /// // The `Fruits_SI` trait was declared by the `Structural` derive on `Fruits`.
    /// fn total_fruit_count(fruits:&dyn Fruits_SI)->u32{
    ///     fruits
    ///         .cloned_fields(fp!( apples, oranges, tangerines, tomatoes ))
    ///         .into_array() // Converts a homogeneous tuple to an array
    ///         .iter()
    ///         .sum()
    /// }
    /// 
    /// {
    ///     let fruits=Fruits{
    ///         apples:1,
    ///         oranges:2,
    ///         tangerines:3,
    ///         tomatoes:5,
    ///     };
    ///     
    ///     assert_eq!( total_fruit_count(&fruits), 11 );
    /// }
    /// 
    /// {
    ///     let fruits=make_struct!{
    ///         apples:8,
    ///         oranges:13,
    ///         tangerines:21,
    ///         tomatoes:34,
    ///     };
    ///     
    ///     assert_eq!( total_fruit_count(&fruits), 76 );
    /// }
    /// 
    /// #[derive(Structural)]
    /// // We only get read access to the fields.
    /// #[struc(public,access="ref")]
    /// struct Fruits{
    ///     apples:u32,
    ///     oranges:u32,
    ///     tangerines:u32,
    ///     tomatoes:u32,
    /// }
    /// 
    /// ```
    fn cloned_fields<'a,P>(&'a self,path:P)-> <RevFieldRefType<'a,P,Self> as Cloned>::Cloned
    where
        P:RevGetField<'a,Self>,
        RevFieldRefType<'a,P,Self>:Cloned,
    {
        path.rev_get_field(self)
            .cloned_()
    }

    /// Gets a mutable reference to a field,determined by `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,fp,make_struct,Structural};
    ///
    /// #[derive(Structural)]
    /// struct Human{
    ///     pub x:i32,
    ///     pub y:i32,
    ///     pub health:u32,
    ///     flags:u32,
    /// }
    ///
    /// // The `Human_SI` trait was declared by the `Structural` derive on `Human`.
    /// fn move_human( this:&mut dyn Human_SI, dx:i32, dy:i32 ){
    ///     *this.field_mut(fp!(x))+=dx;
    ///     *this.field_mut(fp!(y))+=dy;
    /// }
    ///
    /// {
    ///     let mut entity=make_struct!{
    ///         x: 0, 
    ///         y: 0, 
    ///         health: 100,
    ///     };
    ///     move_human(&mut entity,-100,300);
    ///     assert_eq!( entity.fields(fp!(x,y,health)), (&-100,&300,&100) )
    /// }
    /// {
    ///     let mut entity=Human{
    ///         x: -1000,
    ///         y: 1000,
    ///         health: 1,
    ///         flags: 0b11111,
    ///     };
    ///     
    ///     move_human(&mut entity,500,-200);
    ///
    ///     assert_eq!( entity.x, -500 );
    ///     assert_eq!( entity.y, 800 );
    ///     assert_eq!( entity.health, 1 );
    ///     assert_eq!( entity.flags, 0b11111 );
    /// }
    ///
    /// ```
    ///
    #[inline(always)]
    fn field_mut<'a,P>(&'a mut self,path:P)->RevFieldMutType<'a,P,Self>
    where 
        P:IsFieldPath,
        P:RevGetFieldMut<'a,Self>
    {
        path.rev_get_field_mut(self)
    }

    /// Gets mutable references to multiple fields,determined by `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{
    ///     GetFieldExt,GetFieldMut,GetFieldType,Structural,
    ///     fp,field_path_aliases_module,
    /// };
    ///
    /// field_path_aliases_module!{ 
    ///     mod names{x,y}
    /// }
    ///
    /// fn swap_coordinates<T,U>(this:&mut T)
    /// where
    ///     T:GetFieldMut<names::x,Ty=U>,
    ///     T:GetFieldMut<names::y,Ty=U>,
    /// {
    ///     let (x,y)=this.fields_mut(fp!(x,y));
    ///     std::mem::swap(x,y);
    /// }
    /// 
    /// {
    ///     let mut this=Point2D{ x:100, y:300 };
    ///     swap_coordinates(&mut this);
    ///     assert_eq!( this.x, 300 );
    ///     assert_eq!( this.y, 100 );
    /// }
    /// 
    /// {
    ///     let mut this=Point3D{ x:30, y:0, z:500 };
    ///     swap_coordinates(&mut this);
    ///     assert_eq!( this.x, 0 );
    ///     assert_eq!( this.y, 30 );
    ///     assert_eq!( this.z, 500 );
    /// }
    ///
    /// #[derive(Structural)]
    /// struct Point2D<T>{
    ///     pub x:T,
    ///     pub y:T,
    /// }
    ///
    /// #[derive(Structural)]
    /// struct Point3D<T>{
    ///     pub x:T,
    ///     pub y:T,
    ///     pub z:T,
    /// }
    ///
    ///
    /// ```
    ///
    /// # Example
    ///
    /// An example of how this method does not allow multiple mutable borrows 
    /// of the same field.
    ///
    /// ```compile_fail
    /// use structural::{GetFieldExt,fp};
    ///
    /// let mut tup=(1,1,2,3,5,8);
    ///
    /// let _=tup.fields_mut(fp!(4,4));
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

    /// Converts ´self´ into a field,determined by `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp};
    /// 
    /// 
    /// #[derive(Structural,Clone)]
    /// #[struc(public,access="move")]
    /// struct Tupled<T>(T,T,T,T);
    /// 
    /// // The `Tupled_SI` trait was declared by the `Structural` derive on `Tupled`.
    /// fn pick_index<T>(this:impl Tupled_SI<T>,which_one:u32)->T{
    ///     match which_one % 4 {
    ///         0=>this.into_field(fp!(0)),
    ///         1=>this.into_field(fp!(1)),
    ///         2=>this.into_field(fp!(2)),
    ///         _=>this.into_field(fp!(3)),
    ///     }
    /// }
    /// 
    /// {
    ///     let tup=Tupled(13,21,34,55);
    ///     
    ///     assert_eq!( pick_index(tup.clone(),0), 13 );
    ///     assert_eq!( pick_index(tup.clone(),1), 21 );
    ///     assert_eq!( pick_index(tup.clone(),2), 34 );
    ///     assert_eq!( pick_index(tup        ,3), 55 );
    /// }
    /// 
    /// {
    ///     let array=[13,21,34,55];
    ///     
    ///     assert_eq!( pick_index(array.clone(),0), 13 );
    ///     assert_eq!( pick_index(array.clone(),1), 21 );
    ///     assert_eq!( pick_index(array.clone(),2), 34 );
    ///     assert_eq!( pick_index(array        ,3), 55 );
    /// }
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

    /// Converts a boxed ´self´ into a field,determined by `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp,make_struct,structural_alias};
    /// 
    /// structural_alias!{
    ///     trait Pair<T>{
    ///         left :T,
    ///         right:T,
    ///     }
    /// }
    /// 
    /// fn pick_from_pair<T>(this:Box<dyn Pair<T>>,which_one:u32)->T{
    ///     match which_one%2 {
    ///         0=>this.box_into_field(fp!(left )),
    ///         _=>this.box_into_field(fp!(right)),
    ///     }
    /// }
    /// 
    /// fn main(){
    ///     let this=Box::new(make_struct!{
    ///         #![derive(Clone)]
    ///         left: "foo".to_string(),
    ///         right: "bar".to_string(),
    ///     });
    /// 
    ///     assert_eq!( pick_from_pair(this.clone(),0), "foo" );
    ///     assert_eq!( pick_from_pair(this        ,1), "bar" );
    /// }
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
            T:Structural+?Sized
        {
            const FIELDS:&'static [FieldInfo]=T::FIELDS;

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
                ptr:*mut (),
                name:PhantomData<FieldName>,
            )->*mut Self::Ty{
                let this=ptr as *mut Self;
                T::get_field_raw_mut(
                    &mut **this as *mut T as *mut (),
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


