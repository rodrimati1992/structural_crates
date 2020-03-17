/*!
Accessor and extension traits for fields.

# GetFieldExt

The [GetFieldExt](./trait.GetFieldExt.html) trait,
which is the way you're expected to call accessor methods.

# Traits

### For structs and enums

The [FieldType](./trait.FieldType.html) trait,for querying the type of a field,

### For structs

The [GetField](./trait.GetField.html),
[GetFieldMut](./trait.GetFieldMut.html),
[IntoField](./trait.IntoField.html)
accessor traits,that define how a field is accessed.

[IntoFieldMut](./trait.IntoFieldMut.html),a trait alias for `GetFieldMut` + `IntoField`.

### For enums

The [GetVariantField](./variant_field/trait.GetVariantField.html),
[GetVariantFieldMut](./variant_field/trait.GetVariantFieldMut.html),
[IntoVariantField](./variant_field/trait.IntoVariantField.html)
accessor traits,that define how a variant field is accessed.

[IntoVariantFieldMut](./variant_field/trait.IntoVariantFieldMut.html),
a trait alias for `GetVariantFieldMut` + `IntoVariantField`.

# Rev* traits

The `Rev*` traits,implemented by field paths,accessing field(s) from the passed-in type.

There are two kinds of `Rev*` traits,single field and multi field traits.

The [GetFieldExt](./trait.GetFieldExt.html) trait
uses the `Rev*` impls of the passed-in path to access the
fields in `Self`.

### Single Field traits

Which are [RevGetFieldImpl](./rev_get_field/trait.RevGetFieldImpl.html),
[RevGetFieldMutImpl](./rev_get_field/trait.RevGetFieldMutImpl.html),
and [RevIntoFieldImpl](./rev_get_field/trait.RevIntoFieldImpl.html),
mirroring the regular field accessor traits.

### Multiple field traits

For bounds to access multiple fields at once,
there's [RevGetMultiField](./multi_fields/trait.RevGetMultiField.html),
and [RevGetMultiFieldMut](./multi_fields/trait.RevGetMultiFieldMutImpl.html)
(no RevIntoMultiField for now).

For implementing a new way to access multiple fields,
there's [RevGetMultiFieldImpl](./multi_fields/trait.RevGetMultiFieldImpl.html),
and [RevGetMultiFieldMutImpl](./multi_fields/trait.RevGetMultiFieldMutImpl.html)
(no RevIntoMultiFieldImpl for now).

# Additional items

### Array Traits

This module re-exports these traits from [for_arrays](./for_arrays/index.html),with:

- The `Array*` structural aliases to use any type with accessors from 0
until the size of the array,in which all the field types are the same,

- The `Array*Variant` structural aliases to use any enum variant with accessors from 0
until the size of the array,in which all the field types are the same.

### Tuple Traits

This module re-exports these traits from [for_tuples](./for_tuples/index.html),with:

- The `Tuple*` structural aliases to use any type with accessors from `TS!(0)`
until the size of the tuple,in which all field types can be different,

- The `Tuple*Variant` structural aliases to use any enum variant with accessors from `TS!(0)`
until the size of the tuple,in which all field types can be different.

### type aliases

The [GetFieldErr](./type.GetFieldErr.html)
type alias allows querying the `RevGetField::Err` associated type,
useful when delegating the `Rev*Impl` traits.

The [GetFieldType](./type.GetFieldType.html),
[GetFieldType2](./type.GetFieldType2.html),
[GetFieldType3](./type.GetFieldType3.html),
[GetFieldType4](./type.GetFieldType4.html)
type aliases allow querying the type of a field up to 4 levels of nesting.

The [GetVariantFieldType](./variant_field/type.GetVariantFieldType.html)
for querying the type of an enum variant field,
most useful when the name of the variant and the field are passed separately.

The [RevGetFieldType](./rev_get_field/type.RevGetFieldType.html)
type alias gets the type of a nested field
(which one is determined by the field path).

### Errors

The [errors](./errors/index.html)
module contains the error-related items used in `Rev*` trait impls.

### Normalize Fields

[NormalizeFields](./trait.NormalizeFields.html)
transforms the `Result<T,_>`s from inside a type returned by `Rev` traits into either an
`Option<T>` or a `T` depending on its error type.

[NormalizeFieldsOut](./type.NormalizeFieldsOut.html)
The type that `Foo` is converted into when calling
`Foo::normalize_fields( foo )`.


*/

use crate::Structural;

mod enum_impls;
pub mod errors;
pub mod for_arrays;
pub mod for_tuples;
mod get_field_ext;
mod most_impls;
pub mod multi_fields;
mod normalize_fields;
pub mod rev_get_field;
mod tuple_impls;
pub mod variant_field;

pub use self::{
    errors::{
        CombinedErrs, CombinedErrsOut, FailedAccess, InfallibleAccess, IntoFieldErr, IsFieldErr,
    },
    for_arrays::array_traits::*,
    for_tuples::*,
    get_field_ext::GetFieldExt,
    multi_fields::{
        RevGetMultiField, RevGetMultiFieldImpl, RevGetMultiFieldMut, RevGetMultiFieldMutImpl,
        RevGetMultiFieldMutOut, RevGetMultiFieldMutRaw, RevGetMultiFieldOut,
    },
    normalize_fields::{NormalizeFields, NormalizeFieldsOut},
    rev_get_field::{
        OptRevGetField, OptRevGetFieldMut, OptRevIntoField, OptRevIntoFieldMut, RevFieldType,
        RevGetField, RevGetFieldImpl, RevGetFieldMut, RevGetFieldMutImpl, RevGetFieldType,
        RevIntoBoxedFieldType, RevIntoField, RevIntoFieldImpl, RevIntoFieldMut,
    },
    variant_field::{
        GetVFieldRawMutFn, GetVariantField, GetVariantFieldMut, GetVariantFieldType,
        IntoVariantField, IntoVariantFieldMut,
    },
};

pub use self::variant_field::SpecGetVariantFieldMut;

////////////////////////////////////////////////////////////////////////////////

/// For querying the type of the `FieldName` field.
///
/// Structs generally implement this with a `TStr` parameter,
/// while enums implement this with a `VariantField` parameter
pub trait FieldType<FieldName> {
    /// The type of the `FieldName` field.
    type Ty;
}

////////////////////////////////////////////////////////////////////////////////

/// Provides shared access to the `FieldName` field.
///
/// `FieldName` is expected to be a [TStr](../struct.TStr.html).
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
/// ```
/// use structural::{GetFieldExt,GetField,FP,fp};
///
/// fn example(this:impl GetField<FP!(0), Ty=u32> + GetField<FP!(1), Ty=&'static str>){
///     assert_eq!( this.field_(fp!(0)), &99 );
///     assert_eq!( this.field_(fp!(1)), &"world" );
///
///     assert_eq!( this.fields(fp!(0,1)), (&99, &"world") );
///     assert_eq!( this.cloned_fields(fp!(0,1)), (99, "world") );
/// }
///
/// example((99,"world",));
/// example((99,"world",100,101,102,));
///
/// ```
///
/// # Manual Implementation Example
///
/// While this trait is intended to be implemented using the `Structural` derive macro,
/// you can manually implement it like this:
///
/// ```rust
/// use structural::{FieldType,GetField,Structural,FP};
///
/// struct Huh<T>{
///     value:T,
/// }
///
/// impl<T> Structural for Huh<T>{}
///
///
/// impl<T> FieldType<FP!(value)> for Huh<T>{
///     type Ty=T;
/// }
///
/// impl<T> GetField<FP!(value)> for Huh<T>{
///     fn get_field_(&self,_:FP!(value))->&Self::Ty{
///         &self.value
///     }
/// }
///
/// ```
///
pub trait GetField<FieldName>: FieldType<FieldName> {
    /// Accesses the `FieldName` field by reference.
    fn get_field_(&self, field_name: FieldName) -> &Self::Ty;
}

/// Queries the type of a field.
///
/// # Example
///
/// Here is one way you can get the type of a `struct` field.
///
/// ```
/// use structural::{GetField,GetFieldExt,GetFieldType,FP,fp};
///
/// fn get_name<T>(this:&T)->&GetFieldType<T,FP!(name)>
/// where
///     T:GetField<FP!(name)>
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
///     T:GetField<FP!(name), Ty=O>
/// {
///     this.field_(fp!(name))
/// }
/// ```
/// A potential downside of adding another type parameter is that it
/// makes it less ergonomic to specify the type of `T` while ignoring the field type,
/// since one has to write it as `get_name::<Foo,_>(&foo)`.
///
/// # Example
///
/// Here's an example of accessing an `enum` field,using `GetFieldType` to get the field type.
///
/// This also demonstrates a way to write extension traits.
///
/// ```
/// use structural::{FP, GetFieldExt, GetFieldType, GetVariantField, Structural, TS, fp};
/// use structural::for_examples::EnumOptA;
///
/// let foo= EnumOptA::Limbs{legs:Some(9), hands:None};
/// assert_eq!( foo.get_limbs(), Some((&Some(9), &None)) );
///
/// let array=[0,1,2,3];
/// let baz=EnumGround::Limbs{legs:"many", hands:&array};
/// assert_eq!( baz.get_limbs(), Some((&"many", &&array[..])) );
///
/// trait GetLimbs:
///     GetVariantField<TS!(Limbs),TS!(legs)> +
///     GetVariantField<TS!(Limbs),TS!(hands)>
/// {
///     fn get_limbs(&self)-> Option<(
///         &GetFieldType<Self, FP!(::Limbs.legs)>,
///         &GetFieldType<Self, FP!(::Limbs.hands)>,
///     )> {
///         self.fields(fp!(::Limbs=>legs,hands))
///     }
/// }
///
/// impl<T> GetLimbs for T
/// where
///     T: ?Sized +
///        GetVariantField<TS!(Limbs),TS!(legs)> +
///        GetVariantField<TS!(Limbs),TS!(hands)>
/// {}
///
///
/// #[derive(Structural, Copy, Clone, Debug, PartialEq)]
/// #[struc(no_trait)]
/// pub enum EnumGround<'a> {
///     Limbs {
///         legs: &'static str,
///         hands: &'a [u8],
///     },
/// }
///
/// ```
pub type GetFieldType<This, FieldName> = <This as FieldType<FieldName>>::Ty;

/// Queries the type of a double nested field (eg:`.a.b`).
///
/// Example usage:
/// `GetFieldType2<T,FP!(foo),FP!(bar)>`
pub type GetFieldType2<This, FieldName, FieldName2> =
    GetFieldType<GetFieldType<This, FieldName>, FieldName2>;

/// Queries the type of a triple nested field (eg:`.a.b.c`).
///
/// Example usage:
/// `GetFieldType3<T,FP!(foo),FP!(bar),FP!(baz)>`
pub type GetFieldType3<This, FieldName, FieldName2, FieldName3> =
    GetFieldType<GetFieldType2<This, FieldName, FieldName2>, FieldName3>;

/// Queries the type of a quadruple nested field (eg:`.a.b.c.d`).
///
/// Example usage:
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
/// ### Implementing `get_field_raw_mut`
///
/// Your implementation of `GetFieldMut::get_field_raw_mut` must ensure these properties:
/// <span id="raw_mut_properties"></span>
///
/// - It must be side-effect free,
///
/// - The method must return a pointer to a fully initialized field,
///
/// - The field you access must always be the same one.
///
/// - That no implementation returns a pointer to a field that other ones also return,
///
/// You can unerase the pointer by casting it to `*mut  Self`
/// (you can also use any type that's compatible with `Self`).
///
/// Your implementation of the `get_field_raw_mut_fn` method must only return a
/// function pointer to a function that ensures the properties listed
/// [here](#raw_mut_properties).
///
///
/// # Example: Usage as Bound
///
/// ```
/// use structural::{GetFieldExt,GetFieldMut,FP,fp};
/// use structural::for_examples::{Struct2,Struct3};
///
/// fn example<T>(this:&mut T)
/// where
///     T: GetFieldMut<FP!(foo), Ty=Option<u32>> + GetFieldMut<FP!(bar), Ty=&'static str>
/// {
///     assert_eq!( this.field_(fp!(foo)), &Some(21) );
///     assert_eq!( this.field_(fp!(bar)), &"oh boy" );
///     assert_eq!( this.fields(fp!(foo,bar)), (&Some(21), &"oh boy") );
///     assert_eq!( this.cloned_fields(fp!(foo,bar)), (Some(21), "oh boy") );
///
///     assert_eq!( this.field_mut(fp!(foo)), &mut Some(21) );
///     assert_eq!( this.field_mut(fp!(bar)), &mut "oh boy" );
///     assert_eq!( this.fields_mut(fp!(foo,bar)), (&mut Some(21), &mut "oh boy") );
/// }
///
/// example(&mut Struct2{ foo:Some(21), bar: "oh boy" });
/// example(&mut Struct3{ foo:Some(21), bar: "oh boy", baz:5 });
///
/// ```
///
/// <span id="manual-implementation-example"></span>
/// # Example: Manual implementation
///
/// While this trait is intended to be implemented using the `Structural` derive macro,
/// you can also implement it like this:
///
/// ```rust
/// use structural::{FieldType,GetField,GetFieldMut,Structural,FP};
///
/// struct Huh<T>{
///     value:T,
/// }
///
/// impl<T> Structural for Huh<T>{}
///
/// impl<T> FieldType<FP!(value)> for Huh<T>{
///     type Ty=T;
/// }
///
/// impl<T> GetField<FP!(value)> for Huh<T>{
///     fn get_field_(&self,_:FP!(value))->&Self::Ty{
///         &self.value
///     }
/// }
///
/// unsafe impl<T> GetFieldMut<FP!(value)> for Huh<T>{
///     fn get_field_mut_(&mut self,_:FP!(value))->&mut Self::Ty{
///         &mut self.value
///     }
///     structural::z_unsafe_impl_get_field_raw_mut_method!{
///         Self,
///         field_tstr=value,
///         name_generic=FP!(value),
///     }
/// }
///
/// ```
///
pub unsafe trait GetFieldMut<FieldName>: GetField<FieldName> {
    /// Accesses the `FieldName` field by mutable reference.
    fn get_field_mut_(&mut self, field_name: FieldName) -> &mut Self::Ty;

    /// Gets a mutable pointer for the field.
    ///
    /// # Safety
    ///
    /// You must pass a pointer casted from `*mut  Self` to `*mut  ()`,
    /// pointing to a fully initialized instance of the type.
    unsafe fn get_field_raw_mut(ptr: *mut (), field_name: FieldName) -> *mut Self::Ty
    where
        Self: Sized;

    /// Gets the `get_field_raw_mut` associated function as a function pointer.
    fn get_field_raw_mut_fn(&self) -> GetFieldRawMutFn<FieldName, Self::Ty>;
}

/// A `GetFieldMut` specifically used for specialization internally.
///
/// Moving the specialization to a separate impl somehow improves the error messages
/// when calling `GetFieldExt::{field_mut,fields_mut}` methods.
///
/// # Safety
///
/// This trait has the same safety requirements as `GetFieldMut`.
#[doc(hidden)]
pub unsafe trait SpecGetFieldMut<FieldName>: GetField<FieldName> {
    /// Gets a mutable pointer for the field.
    ///
    /// # Safety
    ///
    /// You must pass a pointer casted from `*mut  Self` to `*mut  ()`,
    /// pointing to a fully initialized instance of the type.
    unsafe fn get_field_raw_mut_inner(ptr: *mut (), field_name: FieldName) -> *mut Self::Ty
    where
        Self: Sized;
}

/////////////////////////////////////////////////

/// The type of `GetFieldMut::get_field_raw_mut`
pub type GetFieldRawMutFn<FieldName, FieldTy> = unsafe fn(*mut (), FieldName) -> *mut FieldTy;

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
/// ```
/// use structural::{GetFieldExt,IntoField,FP,fp};
/// use structural::for_examples::{Struct2,Struct3};
///
/// fn example<T>(this: T)
/// where
///     T: Copy + IntoField<FP!(foo), Ty=Option<i8>> + IntoField<FP!(bar), Ty=&'static str>
/// {
///     assert_eq!( this.field_(fp!(foo)), &None );
///     assert_eq!( this.field_(fp!(bar)), &"great" );
///     assert_eq!( this.fields(fp!(foo,bar)), (&None, &"great") );
///     assert_eq!( this.cloned_fields(fp!(foo,bar)), (None, "great") );
///
///     // This can't be called with `IntoField` you need `IntoFieldMut` for that.
///     // assert_eq!( this.field_mut(fp!(bar)), &mut "great" );
///
///     assert_eq!( this.into_field(fp!(foo)), None );
///     assert_eq!( this.into_field(fp!(bar)), "great" );
/// }
///
/// example(Struct2{ foo:None, bar: "great" });
/// example(Struct3{ foo:None, bar: "great", baz:5 });
///
/// ```
///
/// # Manual Implementation Example
///
/// While this trait is intended to be implemented using the `Structural` derive macro,
/// you can manually implement it like this:
///
/// ```rust
/// use structural::{FieldType,GetField,IntoField,Structural,FP};
///
/// struct Huh<T>{
///     value:T,
/// }
///
///
/// impl<T> Structural for Huh<T>{}
///
/// impl<T> FieldType<FP!(value)> for Huh<T>{
///     type Ty=T;
/// }
///
/// impl<T> GetField<FP!(value)> for Huh<T>{
///     fn get_field_(&self,_:FP!(value))->&Self::Ty{
///         &self.value
///     }
/// }
///
/// impl<T> IntoField<FP!(value)> for Huh<T>{
///     fn into_field_(self,_:FP!(value))->Self::Ty{
///         self.value
///     }
///
///     structural::z_impl_box_into_field_method!{field_tstr=FP!(value)}
/// }
///
/// ```
///
pub trait IntoField<FieldName>: GetField<FieldName> {
    /// Converts self into the field.
    fn into_field_(self, field_name: FieldName) -> Self::Ty
    where
        Self: Sized;

    /// Converts a boxed self into the field.
    #[cfg(feature = "alloc")]
    fn box_into_field_(self: crate::pmr::Box<Self>, field_name: FieldName) -> Self::Ty;
}

/// A bound for shared, mutable,and by-value access to the `FieldName` field.
///
/// This is only usable as a bound,
/// to access the field you can use any [GetFieldExt](./trait.GetFieldExt.html) method.
///
/// # Example
///
///
/// ```
/// use structural::{GetFieldExt,IntoFieldMut,FP,fp};
/// use structural::for_examples::{Struct2,Struct3};
///
///
/// fn example(mut this:Box<dyn Bounds>){
///     assert_eq!( this.field_(fp!(foo)), &Some(false) );
///     assert_eq!( this.field_(fp!(bar)), &"oh boy" );
///
///     assert_eq!( this.fields(fp!(foo,bar)), (&Some(false), &"oh boy") );
///
///     assert_eq!( this.cloned_fields(fp!(foo,bar)), (Some(false), "oh boy") );
///
///     assert_eq!( this.field_mut(fp!(foo)), &mut Some(false) );
///     assert_eq!( this.field_mut(fp!(bar)), &mut "oh boy" );
///
///     assert_eq!( this.fields_mut(fp!(foo,bar)), (&mut Some(false), &mut "oh boy") );
///
///     // You need to use `box_into_field` to unwrap a `Box<dyn Trait>`.
///     assert_eq!( this.box_into_field(fp!(bar)), "oh boy" );
/// }
///
/// example(Box::new(Struct2{ foo:Some(false), bar: "oh boy" }));
/// example(Box::new(Struct3{ foo:Some(false), bar: "oh boy", baz:5 }));
///
///
/// // This trait and impl block is what the `structural_alias` macro expands to.
/// trait Bounds:
///     IntoFieldMut<FP!(foo), Ty=Option<bool>> +
///     IntoFieldMut<FP!(bar), Ty=&'static str>
/// {}
///
/// impl<This> Bounds for This
/// where
///     This:?Sized +
///         IntoFieldMut<FP!(foo), Ty=Option<bool>> +
///         IntoFieldMut<FP!(bar), Ty=&'static str>
/// {}
///
/// ```
pub trait IntoFieldMut<F>: IntoField<F> + GetFieldMut<F> {}

impl<This, F> IntoFieldMut<F> for This where This: ?Sized + IntoField<F> + GetFieldMut<F> {}
