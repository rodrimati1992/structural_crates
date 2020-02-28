/*!
Accessor and extension traits for fields.

# GetFieldExt

The [GetFieldExt] trait,which is the way you're expected to call accessor methods.

# Implementable Traits

These traits are intended to only be implemented.

### For structs and enums

The [FieldType],[GetFieldImpl],[GetFieldMutImpl],[IntoFieldImpl] accessor traits,
that defines how a field is accessed.

### For enums

The [GetVariantFieldImpl], [GetVariantFieldMutImpl], [IntoVariantFieldImpl] marker traits.

# Traits for bounds

These traits can be used as bound aliases(they can't be directly implemented).

### For structs

The [GetField],[GetFieldMut],[IntoField],[IntoFieldMut] traits,
for accessing non-optional fields.

The [OptGetField],[OptGetFieldMut],[OptIntoField],[OptIntoFieldMut] traits,
for accessing optional fields.

### For enums

The [GetVariantField],[GetVariantFieldMut],[IntoVariantField],
[IntoVariantFieldMut] traits,for accessing non-optional fields.

The [OptGetVariantField],[OptGetVariantFieldMut],[OptIntoVariantField],
[OptIntoVariantFieldMut] traits,for accessing optional fields.

# Rev* traits

The `Rev*` traits,implemented by field paths,accessing field(s) from the passed-in type.

There are two kinds of `Rev*` traits:

- Single field traits:
Which are [RevGetFieldImpl], [RevGetFieldMutImpl], and [RevIntoFieldImpl],
mirroring the regular field accessor traits.

- Multiple field traits:
Which are [RevGetMultiField], and [RevGetMultiFieldMut] (no RevIntoMultiField for now),
allowing access to multiple fields at once.

The [GetFieldExt] trait uses the `Rev*` impls of the passed-in path to access the
fields in `Self`.

# Additional items

### Array Traits

This module re-exports these traits from [for_arrays],with:

- The `Array*` structural aliases to use any type with accessors from 0
until the size of the array,in which all the field types are the same,

- The `Array*Variant` structural aliases to use any enum variant with accessors from 0
until the size of the array,in which all the field types are the same.

### Tuple Traits

This module re-exports these traits from [for_tuples],with:

- The `Tuple*` structural aliases to use any type with accessors from `TS!(0)`
until the size of the tuple,in which all field types can be different,

- The `Tuple*Variant` structural aliases to use any enum variant with accessors from `TS!(0)`
until the size of the tuple,in which all field types can be different.

### type aliases

The [GetFieldErr] type alias allows querying the `GetFieldImpl::Err` associated type,
useful when delegating the `*Impl` accessor traits.

The [GetFieldType], [GetFieldType2], [GetFieldType3], [GetFieldType4]
type aliases allow querying the type of a field up to 4 levels of nesting.

The [RevGetFieldType] type alias gets the type of a nested field
(which one is determined by the field path).

### Errors

The [errors](self::errors) module contains the error-related items used in accessor trait impls.

### Normalize Fields

[NormalizeFields] transforms the `Result<T,_>`s in a type into either an
`Option<T>` or a `T` depending on its error type.

[NormalizeFieldsOut] The type that `Foo` is converted into when calling
`Foo::normalize_fields( foo )`.


*/

use crate::{
    field_path::{FieldPath, FieldPathSet},
    IsStructural, Structural,
};

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
        OptRevGetField, OptRevGetFieldMut, OptRevIntoField, OptRevIntoFieldMut, RevFieldType,
        RevGetField, RevGetFieldImpl, RevGetFieldMut, RevGetFieldMutImpl, RevGetFieldType,
        RevIntoBoxedFieldType, RevIntoField, RevIntoFieldImpl, RevIntoFieldMut,
    },
    variant_field::{
        GetVariantField, GetVariantFieldImpl, GetVariantFieldMut, GetVariantFieldMutImpl,
        GetVariantFieldType, IntoVariantField, IntoVariantFieldImpl, IntoVariantFieldMut,
        OptGetVariantField, OptGetVariantFieldMut, OptIntoVariantField, OptIntoVariantFieldMut,
    },
};

////////////////////////////////////////////////////////////////////////////////

/// For querying the type of the `FieldName` field.
pub trait FieldType<FieldName> {
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
    /// The error type returned by the accessor methods.
    type Err: IsFieldErr;

    /// Accesses the `FieldName` field by reference.
    fn get_field_(&self, field_name: FieldName, param: P) -> Result<&Self::Ty, Self::Err>;
}

declare_accessor_trait_alias! {
    /// A bound for shared access to the `FieldName` field.
    ///
    /// This is only usable as a bound,
    /// to access the field you can use the [GetFieldExt] methods
    /// with `&self` receivers.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,GetField,FP,fp};
    ///
    /// fn example(this:impl GetField<FP!(0), Ty=u32>){
    ///     assert_eq!( this.field_(fp!(0)), &99_u32 );
    /// }
    ///
    /// example((99,));
    /// example((99,100,101,102,));
    ///
    /// ```
    pub trait GetField<FieldName>=
        GetFieldImpl<FieldName, Err = NonOptField> + IsStructural
}

declare_accessor_trait_alias! {
    /// A bound for optional and shared access to the `FieldName` field.
    ///
    /// This is only usable as a bound,
    /// to access the field you can use the [GetFieldExt] methods
    /// with `&self` receivers.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,OptGetField,FP,fp};
    /// use structural::for_examples::{Tuple1,Tuple2,Tuple3};
    ///
    /// fn example(
    ///     with_some:&dyn OptGetField<FP!(0), Ty=u32>,
    ///     with_none:&dyn OptGetField<FP!(0), Ty=u32>,
    /// ){
    ///     // If this was just an Option field,without the `#[struc(optional)]` attribute,
    ///     // it would be a `&Some(99_u32)` instead.
    ///     assert_eq!( with_some.field_(fp!(0)), Some(&99_u32) );
    ///
    ///     // If this was just an Option field,without the `#[struc(optional)]` attribute,
    ///     // it would be a `&None` instead.
    ///     assert_eq!( with_none.field_(fp!(0)), None );
    /// }
    ///
    /// example(&Tuple1(Some(99)), &Tuple1(None));
    /// example(&Tuple2(Some(99),100), &Tuple2(None,100));
    /// example(&Tuple3(Some(99),100,101), &Tuple3(None,100,101));
    ///
    /// ```
    pub trait OptGetField<FieldName>=
        GetFieldImpl<FieldName, Err = OptionalField> + IsStructural
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
///
/// Example usage(before Rust 1.40.0):
/// `GetFieldType2<T,FP!(f o o),FP!(b a r)>`<br>
/// Example usage(since  Rust 1.40.0):
/// `GetFieldType2<T,FP!(foo),FP!(bar)>`
pub type GetFieldType2<This, FieldName, FieldName2> =
    GetFieldType<GetFieldType<This, FieldName>, FieldName2>;

/// Queries the type of a triple nested field (eg:`.a.b.c`).
///
/// Example usage(before Rust 1.40.0):
/// `GetFieldType3<T,FP!(f o o),FP!(b a r),FP!(b a z)>`<br>
/// Example usage(since  Rust 1.40.0):
/// `GetFieldType3<T,FP!(foo),FP!(bar),FP!(baz)>`
pub type GetFieldType3<This, FieldName, FieldName2, FieldName3> =
    GetFieldType<GetFieldType2<This, FieldName, FieldName2>, FieldName3>;

/// Queries the type of a quadruple nested field (eg:`.a.b.c.d`).
///
/// Example usage(before Rust 1.40.0):
/// `GetFieldType4<T,FP!(f o o),FP!(b a r),FP!(b a z),FP!(b o o m)>`<br>
/// Example usage(since  Rust 1.40.0):
/// `GetFieldType4<T,FP!(foo),FP!(bar),FP!(baz),FP!(boom)>`
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

/// A `GetFieldMutImpl` specifically used for specialization internally.
///
/// Moving the specialization to a separate impl somehow improves the error messages
/// when calling `GetFieldExt::{field_mut,fields_mut}` methods.
///
/// # Safety
///
/// This trait has the same safety requirements as `GetFieldMutImpl`.
#[doc(hidden)]
pub unsafe trait SpecGetFieldMut<FieldName, P = ()>: GetFieldImpl<FieldName, P> {
    unsafe fn get_field_raw_mut_inner(
        ptr: *mut (),
        field_name: FieldName,
        param: P,
    ) -> Result<*mut Self::Ty, Self::Err>
    where
        Self: Sized;
}

declare_accessor_trait_alias! {
    /// A bound for shared and mutable access to the `FieldName` field.
    ///
    /// This is only usable as a bound,
    /// to access the field you can use the [GetFieldExt] methods
    /// with `&self` or `&mut self` receivers.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,GetFieldMut,FP,fp};
    /// use structural::for_examples::{Struct2,Struct3};
    ///
    /// // You can write `FP!(bar)` instead of `FP!(b a r)` since Rust 1.40 .
    /// fn example(this:&mut dyn GetFieldMut<FP!(b a r), Ty=&'static str>){
    ///     assert_eq!( this.field_(fp!(bar)), &"oh boy" );
    ///     assert_eq!( this.field_mut(fp!(bar)), &mut "oh boy" );
    /// }
    ///
    /// example(&mut Struct2{ foo:Some(21), bar: "oh boy" });
    /// example(&mut Struct3{ foo:Some(21), bar: "oh boy", baz:5 });
    ///
    /// ```
    pub trait GetFieldMut<FieldName>=
        GetFieldMutImpl<FieldName, Err = NonOptField> + IsStructural
}

declare_accessor_trait_alias! {
    /// A bound for optional, shared and mutable access to the `FieldName` field.
    ///
    /// This is only usable as a bound,
    /// to access the field you can use the [GetFieldExt] methods
    /// with `&self` or `&mut self` receivers.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,OptGetFieldMut,FP,fp};
    /// use structural::for_examples::{Struct2,Struct3};
    ///
    /// fn example(
    ///     // You can write `FP!(foo)` instead of `FP!(f o o)` since Rust 1.40 .
    ///     with_some:&mut impl OptGetFieldMut<FP!(f o o), Ty=u128>,
    ///     with_none:&mut impl OptGetFieldMut<FP!(f o o), Ty=u128>,
    /// ){
    ///     // If this was just an Option field,without the `#[struc(optional)]` attribute,
    ///     // it would be `&Some(5)` and `&mut Some(5)` instead.
    ///     assert_eq!( with_some.field_(fp!(foo)), Some(&5) );
    ///     assert_eq!( with_some.field_mut(fp!(foo)), Some(&mut 5) );
    ///
    ///     // If this was just an Option field,without the `#[struc(optional)]` attribute,
    ///     // it would be `&None` and `&mut None` instead.
    ///     assert_eq!( with_none.field_(fp!(foo)), None );
    ///     assert_eq!( with_none.field_mut(fp!(foo)), None );
    /// }
    ///
    /// example(
    ///     &mut Struct2{ foo:Some(5), bar: () },
    ///     &mut Struct3{ foo:None, bar: (), baz:() },
    /// );
    ///
    /// ```
    pub trait OptGetFieldMut<FieldName>=
        GetFieldMutImpl<FieldName, Err = OptionalField> + IsStructural
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
    /// A bound for shared and by-value access to the `FieldName` field.
    ///
    /// This is only usable as a bound,
    /// to access the field you can use the [GetFieldExt] methods
    /// with `&self` or `self` receivers.
    ///
    /// The `FieldName` type parameter is usually a [TStr](crate::TStr)
    /// for the name of a field.<br>
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,IntoField,FP,fp};
    /// use structural::for_examples::{Struct2,Struct3};
    ///
    /// fn example<T>(this: T)
    /// where
    ///     // You can write `FP!(bar)` instead of `FP!(b a r)` since Rust 1.40 .
    ///     T: IntoField<FP!(b a r), Ty=&'static str>
    /// {
    ///     assert_eq!( this.field_(fp!(bar)), &"what" );
    ///
    ///     // This can't be called with `IntoField` you need `IntoFieldMut` for that.
    ///     // assert_eq!( this.field_mut(fp!(bar)), &mut "what" );
    ///
    ///     assert_eq!( this.into_field(fp!(bar)), "what" );
    /// }
    ///
    /// example(Struct2{ foo:Some(0), bar: "what" });
    /// example(Struct3{ foo:Some(0), bar: "what", baz:5 });
    ///
    /// ```
    pub trait IntoField<FieldName>=
        IntoFieldImpl<FieldName, Err = NonOptField> +
        IsStructural+
}

declare_accessor_trait_alias! {
    /// A bound for optional, shared and by-value access to the `FieldName` field.
    ///
    /// This is only usable as a bound,
    /// to access the field you can use the [GetFieldExt] methods
    /// with `&self` or `self` receivers.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,OptIntoField,FP,fp};
    /// use structural::for_examples::{Struct2,Struct3};
    ///
    /// fn example<T>(this: T)
    /// where
    ///     // You can write `FP!(foo)` instead of `FP!(f o o)` since Rust 1.40 .
    ///     T: OptIntoField<FP!(f o o), Ty=i8>
    /// {
    ///     // If this was just an Option field,without the `#[struc(optional)]` attribute,
    ///     // it would be a `&Some(51)` instead.
    ///     assert_eq!( this.field_(fp!(foo)), Some(&51) );
    ///
    ///     // This can't be called with `OptIntoField` you need `OptIntoFieldMut` for that.
    ///     // assert_eq!( this.field_mut(fp!(foo)), Some(&mut 51) );
    ///
    ///     assert_eq!( this.into_field(fp!(foo)), Some(51) );
    /// }
    ///
    /// example(Struct2{ foo:Some(51), bar: "huh?" });
    /// example(Struct3{ foo:Some(51), bar: "huh?", baz:5 });
    ///
    /// ```
    pub trait OptIntoField<FieldName>=
        IntoFieldImpl<FieldName, Err = OptionalField>+
        IsStructural+
}

declare_accessor_trait_alias! {
    /// A bound for shared, mutable,and by-value access to the `FieldName` field.
    ///
    /// This is only usable as a bound,
    /// to access the field you can use any [GetFieldExt] method.
    ///
    /// # Example
    ///
    ///
    /// ```
    /// use structural::{GetFieldExt,IntoFieldMut,FP,fp};
    /// use structural::for_examples::{Struct2,Struct3};
    ///
    /// // You can write `FP!(bar)` instead of `FP!(b a r)` since Rust 1.40 .
    /// fn example(mut this:Box<dyn IntoFieldMut<FP!(b a r), Ty=&'static str>>){
    ///     assert_eq!( this.field_(fp!(bar)), &"oh boy" );
    ///     assert_eq!( this.field_mut(fp!(bar)), &mut "oh boy" );
    ///
    ///     // You need to use `box_into_field` to unwrap a `Box<dyn Trait>`.
    ///     assert_eq!( this.box_into_field(fp!(bar)), "oh boy" );
    /// }
    ///
    /// example(Box::new(Struct2{ foo:Some(21), bar: "oh boy" }));
    /// example(Box::new(Struct3{ foo:Some(21), bar: "oh boy", baz:5 }));
    ///
    /// ```
    pub trait IntoFieldMut<FieldName>=
        IntoFieldImpl<FieldName, Err = NonOptField> +
        GetFieldMutImpl<FieldName, Err = NonOptField> +
        IsStructural+
}

declare_accessor_trait_alias! {
    /// A bound for optional, shared, mutable,and by-value access to the `FieldName` field.
    ///
    /// This is only usable as a bound,
    /// to access the field you can use any [GetFieldExt] method.
    ///
    /// # Example
    ///
    ///
    /// ```
    /// use structural::{GetFieldExt,OptIntoFieldMut,FP,fp};
    /// use structural::for_examples::{Struct2,Struct3};
    ///
    /// // You can write `FP!(foo)` instead of `FP!(f o o)` since Rust 1.40 .
    /// fn example(
    ///     mut some: Box<impl OptIntoFieldMut<FP!(f o o), Ty=char>>,
    ///     mut none: Box<impl OptIntoFieldMut<FP!(f o o), Ty=char>>,
    /// ){
    ///     // If this was just an Option field,without the `#[struc(optional)]` attribute,
    ///     // it would be `&Some('g')` and `&mut Some('g')` instead.
    ///     assert_eq!( some.field_(fp!(foo)), Some(&'g') );
    ///     assert_eq!( some.field_mut(fp!(foo)), Some(&mut 'g') );
    ///     assert_eq!( some.into_field(fp!(foo)), Some('g') );
    ///
    ///     assert_eq!( none.field_(fp!(foo)), None );
    ///     assert_eq!( none.field_mut(fp!(foo)), None );
    ///     assert_eq!( none.into_field(fp!(foo)), None );
    /// }
    ///
    /// example(
    ///     Box::new(Struct2{ foo:Some('g'), bar: "oh boy" }),
    ///     Box::new(Struct2{ foo:None, bar: "oh boy" }),
    /// );
    /// example(
    ///     Box::new(Struct3{ foo:Some('g'), bar: "oh boy", baz:5 }),
    ///     Box::new(Struct3{ foo:None, bar: "oh boy", baz:5 }),
    /// );
    ///
    /// ```
    pub trait OptIntoFieldMut<FieldName>=
        IntoFieldImpl<FieldName, Err = OptionalField> +
        GetFieldMutImpl<FieldName, Err = OptionalField> +
        IsStructural+
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "alloc")]
mod alloc_impls {
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
        specialization_params(specialize_cfg(feature="specialization"));
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


        IntoFieldImpl{
            *this
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
