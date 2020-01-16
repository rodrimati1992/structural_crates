/*!
Accessor and extension traits for fields.
*/

use crate::{
    type_level::{FieldPath, FieldPathSet, IsFieldPath, IsFieldPathSet, UniquePaths},
    IsStructural, Structural,
};

use core_extensions::collection_traits::Cloned;

use std_::marker::PhantomData;

mod enum_impls;
pub mod errors;
pub mod for_arrays;
pub mod for_tuples;
mod get_field_ext;
mod most_impls;
pub mod multi_fields;
mod normalize_fields;
pub mod rev_get_field;
mod slice_impls;
mod tuple_impls;
pub mod variant_field;

pub use self::{
    errors::{CombinedErrs, CombinedErrsOut, IntoFieldErr, IsFieldErr, NonOptField, OptionalField},
    for_arrays::array_traits::*,
    for_tuples::*,
    get_field_ext::GetFieldExt,
    multi_fields::{RevGetMultiField, RevGetMultiFieldMut},
    normalize_fields::{NormalizeFields, NormalizeFieldsOut},
    rev_get_field::{
        RevGetField, RevGetFieldMut, RevGetFieldType, RevIntoBoxedFieldType, RevIntoField,
    },
    variant_field::{
        GetVariantField, GetVariantFieldImpl, GetVariantFieldMut, GetVariantFieldMutImpl,
        IntoVariantField, IntoVariantFieldImpl, IntoVariantFieldMut, OptGetVariantField,
        OptGetVariantFieldMut, OptIntoVariantField, OptIntoVariantFieldMut,
    },
};

////////////////////////////////////////////////////////////////////////////////

/// For querying the type of the `FieldName` field.
pub trait FieldType<FieldName>: IsStructural {
    /// The type of the `FieldName` field.
    type Ty;
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! declare_accessor_trait_alias {
    (
        $(#[$attr:meta])*
        $vis:vis trait $trait_name:ident<$name:ident>=
        $($supertraits:tt)*
    ) => (
        $(#[$attr])*
        $vis trait $trait_name< $name >:$($supertraits)* {}

        impl<This,$name> $trait_name< $name > for This
        where
            This:?Sized+$($supertraits)*
        {}
    )
}

/// Allows accessing the `FieldName` field.
///
/// `FieldName` represents the name of the field on the type level,
/// It is a type because a `FIELD_NAME:&'static str` const parameter
/// was neither stable nor worked in nightly at the time this was defined.
///
/// # Safety
///
/// While this trait is not unsafe,
/// implementors ought not mutate fields inside accessor trait impls.
///
/// Mutating fields is only advisable if those fields don't have field accessor impls.
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
///     FieldType,GetFieldImpl,IsStructural,Structural,FP,TList,
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
///
/// impl<T> IsStructural for Huh<T>{}
///
/// // This could also be written as `FP!(value)` from 1.40 onwards
/// impl<T> FieldType<FP!(v a l u e)> for Huh<T>{
///     type Ty=T;
/// }
///
/// // This could also be written as `FP!(value)` from 1.40 onwards
/// impl<T> GetFieldImpl<FP!(v a l u e)> for Huh<T>{
///     type Err=NonOptField;
///
///     fn get_field_(&self,_:FP!(v a l u e),_:())->Result<&Self::Ty,Self::Err>{
///         Ok(&self.value)
///     }
/// }
///
///
/// ```
///
pub trait GetFieldImpl<FieldName, P = ()>: FieldType<FieldName> {
    type Err: IsFieldErr;

    /// Accesses the `FieldName` field by reference.
    fn get_field_(&self, field_name: FieldName, param: P) -> Result<&Self::Ty, Self::Err>;
}

declare_accessor_trait_alias! {
    pub trait GetField<FieldName>=
        GetFieldImpl<FieldName, Err = NonOptField>
}

declare_accessor_trait_alias! {
    pub trait OptGetField<FieldName>=
        GetFieldImpl<FieldName, Err = OptionalField>
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
pub type GetFieldType<This, FieldName> = <This as FieldType<FieldName>>::Ty;

pub type GetFieldErr<This, FieldName, P = ()> = <This as GetFieldImpl<FieldName, P>>::Err;

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
/// Implementors ought not mutate fields inside their accessor trait impls,
/// or the accessor trait impls of other fields.
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
///     FieldType,GetFieldImpl,GetFieldMutImpl,IsStructural,Structural,FP,TList,
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
/// impl<T> IsStructural for Huh<T>{}
///
/// // This could also be written as `FP!(value)` from 1.40 onwards
/// impl<T> FieldType<FP!(v a l u e)> for Huh<T>{
///     type Ty=T;
/// }
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// impl<T> GetFieldImpl<FP!(v a l u e)> for Huh<T>{
///     type Err=NonOptField;
///
///     fn get_field_(&self,_:FP!(v a l u e),_:())->Result<&Self::Ty,Self::Err>{
///         Ok(&self.value)
///     }
/// }
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// unsafe impl<T> GetFieldMutImpl<FP!(v a l u e)> for Huh<T>{
///     fn get_field_mut_(&mut self,_:FP!(v a l u e),_:())->Result<&mut Self::Ty,Self::Err>{
///         Ok(&mut self.value)
///     }
///     structural::z_unsafe_impl_get_field_raw_mut_method!{
///         Self,
///         field_name=value,
///         name_generic=FP!(v a l u e),
///         optionality=nonopt,
///     }
/// }
///
/// ```
///
pub unsafe trait GetFieldMutImpl<FieldName, P = ()>: GetFieldImpl<FieldName, P> {
    /// Accesses the `FieldName` field by mutable reference.
    fn get_field_mut_(
        &mut self,
        field_name: FieldName,
        param: P,
    ) -> Result<&mut Self::Ty, Self::Err>;

    /// Gets a mutable pointer for the field.
    ///
    /// # Safety
    ///
    /// You must pass a pointer casted from `*mut Self` to `*mut ()`.
    unsafe fn get_field_raw_mut(
        ptr: *mut (),
        field_name: FieldName,
        param: P,
    ) -> Result<*mut Self::Ty, Self::Err>
    where
        Self: Sized;

    /// Gets the `get_field_raw_mut` associated function as a function pointer.
    fn get_field_raw_mut_func(&self) -> GetFieldRawMutFn<FieldName, P, Self::Ty, Self::Err>;
}

declare_accessor_trait_alias! {
    pub trait GetFieldMut<FieldName>=
        GetFieldMutImpl<FieldName, Err = NonOptField>
}

declare_accessor_trait_alias! {
    pub trait OptGetFieldMut<FieldName>=
        GetFieldMutImpl<FieldName, Err = OptionalField>
}

/////////////////////////////////////////////////

/// The type of `GetFieldMutImpl::get_field_raw_mut`
pub type GetFieldRawMutFn<FieldName, P, FieldTy, E> =
    unsafe fn(*mut (), FieldName, P) -> Result<*mut FieldTy, E>;

/////////////////////////////////////////////////

/// Converts this type into its `FieldName` field.
///
/// # Safety
///
/// While this trait is not unsafe,
/// implementors ought not mutate fields inside accessor trait impls.
///
/// Mutating fields is only advisable if those fields don't have field accessor impls.
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
///     FieldType,GetFieldImpl,IntoFieldImpl,IsStructural,Structural,FP,TList,
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
/// impl<T> IsStructural for Huh<T>{}
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// impl<T> FieldType<FP!(v a l u e)> for Huh<T>{
///     type Ty=T;
/// }
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// impl<T> GetFieldImpl<FP!(v a l u e)> for Huh<T>{
///     type Err=NonOptField;
///
///     fn get_field_(&self,_:FP!(v a l u e),_:())->Result<&Self::Ty,Self::Err>{
///         Ok(&self.value)
///     }
/// }
///
/// // `FP!(v a l u e)` can be written as `FP!(value)` from 1.40 onwards
/// impl<T> IntoFieldImpl<FP!(v a l u e)> for Huh<T>{
///     fn into_field_(self,_:FP!(v a l u e),_:())->Result<Self::Ty,Self::Err>{
///         Ok(self.value)
///     }
///
///     structural::z_impl_box_into_field_method!{FP!(v a l u e)}
/// }
///
/// ```
///
pub trait IntoFieldImpl<FieldName, P = ()>: GetFieldImpl<FieldName, P> {
    /// Converts self into the field.
    fn into_field_(self, field_name: FieldName, param: P) -> Result<Self::Ty, Self::Err>
    where
        Self: Sized;

    /// Converts a boxed self into the field.
    #[cfg(feature = "alloc")]
    fn box_into_field_(
        self: crate::alloc::boxed::Box<Self>,
        field_name: FieldName,
        param: P,
    ) -> Result<Self::Ty, Self::Err>;
}

declare_accessor_trait_alias! {
    pub trait IntoField<FieldName>=
        IntoFieldImpl<FieldName, Err = NonOptField>
}

declare_accessor_trait_alias! {
    pub trait OptIntoField<FieldName>=
        IntoFieldImpl<FieldName, Err = OptionalField>
}

declare_accessor_trait_alias! {
    pub trait IntoFieldMut<FieldName>=
        IntoFieldImpl<FieldName, Err = NonOptField> +
        GetFieldMutImpl<FieldName, Err = NonOptField> +
}

declare_accessor_trait_alias! {
    pub trait OptIntoFieldMut<FieldName>=
        IntoFieldImpl<FieldName, Err = OptionalField> +
        GetFieldMutImpl<FieldName, Err = OptionalField> +
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;

    use crate::alloc::{boxed::Box, rc::Rc, sync::Arc};

    macro_rules! impl_shared_ptr_accessors {
        ( $this:ident ) => {
            unsafe_delegate_structural_with! {
                impl[T,] $this<T>
                where[T:?Sized,]

                self_ident=this;
                delegating_to_type=T;
                field_name_param=( field_name : FieldName );

                GetFieldImpl {
                    &*this
                }
            }
        };
    }
    impl_shared_ptr_accessors! {Arc}
    impl_shared_ptr_accessors! {Rc}

    unsafe_delegate_structural_with! {
        impl[T,] Box<T>
        where[T:?Sized,]

        self_ident=this;
        delegating_to_type=T;
        field_name_param=( field_name : FieldName );

        GetFieldImpl {
            &*this
        }

        unsafe GetFieldMutImpl{
            &mut **this
        }
        as_delegating_raw{
            *(this as *mut Box<T> as *mut *mut T)
        }
        raw_mut_impl(specialize_cfg(feature="specialization"))


        IntoFieldImpl{
            *this
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
