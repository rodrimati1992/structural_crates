/*!
Accessor and extension traits for fields.
*/

use crate::{
    mut_ref::MutRef,
    type_level::{FieldPath, FieldPathSet, IsFieldPath, IsFieldPathSet, UniquePaths},
    Structural,
};

use core_extensions::collection_traits::Cloned;

use std_::marker::PhantomData;

mod enum_impls;
pub mod errors;
pub mod for_arrays;
pub mod for_tuples;
mod get_field_ext;
mod most_impls;
mod multi_fields;
mod normalize_fields;
pub mod rev_get_field;
mod tuple_impls;

pub use self::{
    errors::{FieldErr, IntoFieldErr, NonOptField, OptionalField},
    get_field_ext::GetFieldExt,
    normalize_fields::{NormalizeFields, NormalizeFieldsOut},
    rev_get_field::{
        RevFieldMutType, RevFieldRefType, RevGetField, RevGetFieldMut, RevGetFieldType,
        RevGetFieldType_, RevIntoField, RevIntoFieldType,
    },
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
/// use structural::{NonOptGetField,GetFieldExt,FP,fp};
///
/// fn formatted_value<T,S>(this:&T)->String
/// where
///     T:NonOptGetField<FP!(v a l u e), Ty=S>,
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
///     GetFieldImpl,Structural,FP,TList,
///     field_traits::NonOptField,
///     structural_trait::{FieldInfo,FieldInfos},
/// };
///
/// struct Huh<T>{
///     value:T,
/// }
///
/// impl<T> Structural for Huh<T>{
///     const FIELDS:&'static FieldInfos=&FieldInfos::Struct(&[
///         FieldInfo::not_renamed("value")
///     ]);
/// }
///
/// // This could also be written as `FP!(value)` from 1.40 onwards
/// impl<T> GetFieldImpl<FP!(v a l u e)> for Huh<T>{
///     type Ty=T;
///     type Err=NonOptField;
///
///     fn get_field_(&self)->Result<&Self::Ty,Self::Err>{
///         Ok(&self.value)
///     }
/// }
///
///
/// ```
///
pub trait GetFieldImpl<FieldName> {
    /// The type of the `FieldName` field.
    type Ty;
    type Err: FieldErr;

    /// Accesses the `FieldName` field by reference.
    fn get_field_(&self) -> Result<&Self::Ty, Self::Err>;
}

pub trait NonOptGetField<FieldName>: GetFieldImpl<FieldName, Err = NonOptField> {}

impl<This: ?Sized, FieldName> NonOptGetField<FieldName> for This where
    This: GetFieldImpl<FieldName, Err = NonOptField>
{
}

pub trait OptGetField<FieldName>: GetFieldImpl<FieldName, Err = OptionalField> {}

impl<This: ?Sized, FieldName> OptGetField<FieldName> for This where
    This: GetFieldImpl<FieldName, Err = OptionalField>
{
}

/// Queries the type of a field.
///
/// # Example
///
/// Here is one way you can get the type of a field.
///
/// ```
/// use structural::{NonOptGetField,GetFieldExt,GetFieldType,FP,fp};
///
/// fn get_name<T>(this:&T)->&GetFieldType<T,FP!(n a m e)>
/// where
///     // `FP!(n a m e)` can be written as `FP!(name)` from 1.40 onwards
///     T:NonOptGetField<FP!(n a m e)>
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
/// use structural::{NonOptGetField,GetFieldExt,GetFieldType,FP,fp};
///
/// fn get_name<T,O>(this:&T)->&O
/// where
///     // `FP!(n a m e)` can be written as `FP!(name)` from 1.40 onwards
///     T:NonOptGetField<FP!(n a m e), Ty=O>
/// {
///     this.field_(fp!(name))
/// }
/// ```
/// A potential downside of adding another type parameter is that it
/// makes it less ergonomic to specify the type of `T` while ignoring the field type,
/// since one has to write it as `get_name::<Foo,_>(&foo)`.
///
///
pub type GetFieldType<This, FieldName> = <This as GetFieldImpl<FieldName>>::Ty;

pub type GetFieldErr<This, FieldName> = <This as GetFieldImpl<FieldName>>::Err;

/// Queries the type of a double nested field (eg:`.a.b`).
pub type GetFieldType2<This, FieldName, FieldName2> =
    GetFieldType<GetFieldType<This, FieldName>, FieldName2>;

/// Queries the type of a triple nested field (eg:`.a.b.c`).
pub type GetFieldType3<This, FieldName, FieldName2, FieldName3> =
    GetFieldType<GetFieldType2<This, FieldName, FieldName2>, FieldName3>;

/// Queries the type of a quadruple nested field (eg:`.a.b.c.d`).
pub type GetFieldType4<This, FieldName, FieldName2, FieldName3, FieldName4> =
    GetFieldType2<GetFieldType2<This, FieldName, FieldName2>, FieldName3, FieldName4>;

/// Allows accessing the `FieldName` field mutably.
///
/// # Safety
///
/// These are requirements for manual implementations.
///
/// It is recommended that you use the `z_unsafe_impl_get_field_raw_mut_method` macro
/// if you only borrow a field of the type.
///
/// Your implementation of `GetFieldMutImpl::get_field_raw_mut` must ensure these properties:
///
/// - It must be side-effect free,
///
/// - The field you borrow must always be the same one.
///
/// - That no implementation of `GetFieldMutImpl::get_field_raw_mut`
/// returns a pointer to a field that other ones also return,
///
/// Your implementation of the `get_field_raw_mut_func` method must only return a
/// function pointer for the `GetFieldMutImpl::get_field_raw_mut` method from the same
/// implementation of the `GetFieldMutImpl` trait.
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
/// use structural::{NonOptGetFieldMut,GetFieldExt,FP,fp};
///
/// fn take_value<T,V>(this:&mut T)->V
/// where
///     // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
///     T:NonOptGetFieldMut<FP!(v a l u e), Ty=V>,
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
///     GetFieldImpl,GetFieldMutImpl,Structural,FP,TList,
///     field_traits::NonOptField,
///     structural_trait::{FieldInfo,FieldInfos},
///     mut_ref::MutRef,
/// };
///
/// struct Huh<T>{
///     value:T,
/// }
///
/// impl<T> Structural for Huh<T>{
///     const FIELDS:&'static FieldInfos=&FieldInfos::Struct(&[
///         FieldInfo::not_renamed("value")
///     ]);
///
/// }
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// impl<T> GetFieldImpl<FP!(v a l u e)> for Huh<T>{
///     type Ty=T;
///     type Err=NonOptField;
///
///     fn get_field_(&self)->Result<&Self::Ty,Self::Err>{
///         Ok(&self.value)
///     }
/// }
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// unsafe impl<T> GetFieldMutImpl<FP!(v a l u e)> for Huh<T>{
///     fn get_field_mut_(&mut self)->Result<&mut Self::Ty,Self::Err>{
///         Ok(&mut self.value)
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
pub unsafe trait GetFieldMutImpl<FieldName>: GetFieldImpl<FieldName> {
    /// Accesses the `FieldName` field by mutable reference.
    fn get_field_mut_(&mut self) -> Result<&mut Self::Ty, Self::Err>;

    /// Gets a mutable pointer for the field.
    ///
    /// # Safety
    ///
    /// You must pass a pointer casted from `*mut Self` to `*mut ()`.
    unsafe fn get_field_raw_mut(
        ptr: *mut (),
        _: PhantomData<FieldName>,
    ) -> Result<*mut Self::Ty, Self::Err>
    where
        Self: Sized;

    /// Gets the `get_field_raw_mut` associated function as a function pointer.
    fn get_field_raw_mut_func(&self) -> GetFieldMutRefFn<FieldName, Self::Ty, Self::Err>;
}

pub trait NonOptGetFieldMut<FieldName>: GetFieldMutImpl<FieldName, Err = NonOptField> {}

impl<This: ?Sized, FieldName> NonOptGetFieldMut<FieldName> for This where
    This: GetFieldMutImpl<FieldName, Err = NonOptField>
{
}

pub trait OptGetFieldMut<FieldName>: GetFieldMutImpl<FieldName, Err = OptionalField> {}

impl<This: ?Sized, FieldName> OptGetFieldMut<FieldName> for This where
    This: GetFieldMutImpl<FieldName, Err = OptionalField>
{
}

/////////////////////////////////////////////////

/// The type of `GetFieldMutImpl::get_field_raw_mut`
pub type GetFieldMutRefFn<FieldName, FieldTy, E> =
    unsafe fn(*mut (), PhantomData<FieldName>) -> Result<*mut FieldTy, E>;

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
/// use structural::{NonOptIntoField,GetFieldExt,GetFieldType,FP,fp};
///
/// fn into_value<T,V>(this:T)->V
/// where
///     // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
///     T:NonOptIntoField<FP!(v a l u e), Ty=V>,
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
///     GetFieldImpl,IntoFieldImpl,Structural,FP,TList,
///     field_traits::NonOptField,
///     structural_trait::{FieldInfo,FieldInfos},
///     mut_ref::MutRef,
/// };
///
/// struct Huh<T>{
///     value:T,
/// }
///
///
/// impl<T> Structural for Huh<T>{
///     const FIELDS:&'static FieldInfos=&FieldInfos::Struct(&[
///         FieldInfo::not_renamed("value")
///     ]);
/// }
///
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// impl<T> GetFieldImpl<FP!(v a l u e)> for Huh<T>{
///     type Ty=T;
///     type Err=NonOptField;
///
///     fn get_field_(&self)->Result<&Self::Ty,Self::Err>{
///         Ok(&self.value)
///     }
/// }
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// impl<T> IntoFieldImpl<FP!(v a l u e)> for Huh<T>{
///     fn into_field_(self)->Result<Self::Ty,Self::Err>{
///         Ok(self.value)
///     }
///
///     structural::z_impl_box_into_field_method!{FP!(v a l u e)}
/// }
///
/// ```
///
pub trait IntoFieldImpl<FieldName>: GetFieldImpl<FieldName> {
    /// Converts self into the field.
    fn into_field_(self) -> Result<Self::Ty, Self::Err>
    where
        Self: Sized;

    /// Converts a boxed self into the field.
    #[cfg(feature = "alloc")]
    fn box_into_field_(self: crate::alloc::boxed::Box<Self>) -> Result<Self::Ty, Self::Err>;
}

pub trait NonOptIntoField<FieldName>: IntoFieldImpl<FieldName, Err = NonOptField> {}

impl<This: ?Sized, FieldName> NonOptIntoField<FieldName> for This where
    This: IntoFieldImpl<FieldName, Err = NonOptField>
{
}

pub trait OptIntoField<FieldName>: IntoFieldImpl<FieldName, Err = OptionalField> {}

impl<This: ?Sized, FieldName> OptIntoField<FieldName> for This where
    This: IntoFieldImpl<FieldName, Err = OptionalField>
{
}

/// An alias for a shared,mutable,and by-value accessor for a field.
pub trait IntoFieldMut<FieldName>: IntoFieldImpl<FieldName> + GetFieldMutImpl<FieldName> {}

impl<This, FieldName> IntoFieldMut<FieldName> for This where
    This: IntoFieldImpl<FieldName> + GetFieldMutImpl<FieldName>
{
}

pub trait NonOptIntoFieldMut<FieldName>:
    IntoFieldImpl<FieldName, Err = NonOptField> + GetFieldMutImpl<FieldName>
{
}

impl<This: ?Sized, FieldName> NonOptIntoFieldMut<FieldName> for This where
    This: IntoFieldImpl<FieldName, Err = NonOptField> + GetFieldMutImpl<FieldName>
{
}

pub trait OptIntoFieldMut<FieldName>:
    IntoFieldImpl<FieldName, Err = OptionalField> + GetFieldMutImpl<FieldName>
{
}

impl<This: ?Sized, FieldName> OptIntoFieldMut<FieldName> for This where
    This: IntoFieldImpl<FieldName, Err = OptionalField> + GetFieldMutImpl<FieldName>
{
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "alloc")]
macro_rules! unsized_impls {
    ( shared,$ptr:ident ) => {
        impl<T> Structural for $ptr<T>
        where
            T: Structural + ?Sized,
        {
            const FIELDS: &'static $crate::structural_trait::FieldInfos = { T::FIELDS };
        }

        impl<This, Name, Ty> GetFieldImpl<Name> for $ptr<This>
        where
            This: GetFieldImpl<Name, Ty = Ty> + ?Sized,
        {
            type Ty = Ty;
            type Err = GetFieldErr<This, Name>;

            fn get_field_(&self) -> Result<&Self::Ty, Self::Err> {
                (**self).get_field_()
            }
        }
    };
    (mutable,$ptr:ident) => {
        unsized_impls! { shared,$ptr }

        unsafe impl<T, FieldName, Ty> GetFieldMutImpl<FieldName> for Box<T>
        where
            T: GetFieldMutImpl<FieldName, Ty = Ty> + ?Sized,
        {
            /// Accesses the `FieldName` field by mutable reference.
            fn get_field_mut_(&mut self) -> Result<&mut Self::Ty, Self::Err> {
                (**self).get_field_mut_()
            }

            default_if! {
                cfg(feature="specialization")
                unsafe fn get_field_raw_mut(
                    this:*mut (),
                    name:PhantomData<FieldName>
                )->Result<*mut Self::Ty,Self::Err>{
                    let this=this as *mut Self;
                    let func=<T as GetFieldMutImpl<FieldName>>::get_field_raw_mut_func(&**this);
                    func( &mut **this as *mut T as *mut (), name )
                }
            }

            fn get_field_raw_mut_func(&self) -> GetFieldMutRefFn<FieldName, Ty, Self::Err> {
                <Self as GetFieldMutImpl<FieldName>>::get_field_raw_mut
            }
        }

        #[cfg(feature = "specialization")]
        unsafe impl<T, FieldName, Ty> GetFieldMutImpl<FieldName> for Box<T>
        where
            T: GetFieldMutImpl<FieldName, Ty = Ty>,
        {
            unsafe fn get_field_raw_mut(
                ptr: *mut (),
                name: PhantomData<FieldName>,
            ) -> Result<*mut Self::Ty, Self::Err> {
                let this = ptr as *mut Self;
                T::get_field_raw_mut(&mut **this as *mut T as *mut (), name)
            }
        }
    };
    (value,$ptr:ident) => {
        unsized_impls! { mutable,$ptr }
    };
}

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;

    use crate::alloc::{boxed::Box, rc::Rc, sync::Arc};

    unsized_impls! {value,Box}
    unsized_impls! {shared,Arc}
    unsized_impls! {shared,Rc}
}

////////////////////////////////////////////////////////////////////////////////
