/*!
Contains traits implemented on field paths,taking Structural types as parameters.
*/

#![allow(non_snake_case)]

use crate::{
    enums::{EnumExt, IsVariant, VariantProxy},
    field_path::{FieldPath, IsSingleFieldPath, TStr, VariantField, VariantName},
    field_traits::{
        errors::{CombinedErrs, IntoFieldErr, IsFieldErr},
        NonOptField, OptionalField,
    },
    FieldType, GetFieldImpl, GetFieldMutImpl, GetFieldType, IntoFieldImpl,
};

#[cfg(feature = "alloc")]
use crate::pmr::Box;

/////////////////////////////////////////////////////////////////////////////

/// Allows querying the type of a nested field in This,
/// what field is queried is determined by `FieldName`,
///
/// # Example
///
/// ```
/// use structural::{
///     field_traits::RevGetFieldType,
///     GetFieldType3,GetFieldExt,Structural,
///     field_path_aliases,
/// };
///
/// field_path_aliases!{
///     foo_bar_baz= foo.bar.baz,
///     foo_bar_strand= foo.bar.strand,
/// }
///
/// fn main(){
///     let this=TopLevel::default();
///     
///     let baz: &RevGetFieldType<foo_bar_baz, TopLevel>=
///         this.field_(foo_bar_baz);
///     assert_eq!( *baz, Vec::new() );
///     
///     let strand: &RevGetFieldType<foo_bar_strand, TopLevel>=
///         this.field_(foo_bar_strand);
///     assert_eq!( *strand, String::new() );
/// }
///
/// #[derive(Debug,Default,Structural)]
/// struct TopLevel{
///     pub foo:Foo,
/// }
///
/// #[derive(Debug,Default,Structural)]
/// struct Foo{
///     pub bar:Bar,
/// }
///
/// #[derive(Debug,Default,Structural)]
/// struct Bar{
///     pub baz:Vec<()>,
///     pub strand:String,
/// }
/// ```
pub type RevGetFieldType<FieldName, This> = <FieldName as RevFieldType<This>>::Ty;

/// The type returned by `RevIntoFieldImpl::rev_box_into_field`.
pub type RevIntoBoxedFieldType<'a, FieldName, This> =
    <FieldName as RevIntoFieldImpl<'a, This>>::BoxedTy;

/// Queries the error type returned by `Rev*Field` methods.
pub type RevGetFieldErr<'a, FieldName, This> = <FieldName as RevGetFieldImpl<'a, This>>::Err;

/////////////////////////////////////////////////////////////////////////////

/// Like FieldType,except that the parameters are reversed.
/// `This` is the type we are accessing,and `Self` is a field path.
pub trait RevFieldType<This: ?Sized>: IsSingleFieldPath {
    /// The type of the field.
    type Ty: ?Sized;
}

/////////////////////////////////////////////////////////////////////////////

/// Like GetFieldImpl,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// # Use as bound
///
/// For examples of using `RevGetFieldImpl` as a bound look at example for
/// [RevGetField](./trait.RevGetField.html) or
/// [OptRevGetField](./trait.OptRevGetField.html).
///
/// # Example
///
/// This example demonstrates implementing RevGetField to index a slice from the end.
///
/// ```rust
/// use structural::field_traits::{OptionalField,RevFieldType,RevGetFieldImpl};
/// use structural::field_path::IsSingleFieldPath;
/// use structural::GetFieldExt;
///
/// struct FromEnd(usize);
///
/// impl IsSingleFieldPath for FromEnd{}
///
/// impl<'a,T> RevFieldType<[T]> for FromEnd {
///     type Ty = T;
/// }
/// impl<'a,T> RevGetFieldImpl<'a,[T]> for FromEnd {
///     type Err = OptionalField;
///
///     fn rev_get_field(self, this: &'a [T]) -> Result<&'a T, OptionalField>{
///         let len=this.len();
///         this.get(len.wrapping_sub(self.0 + 1))
///             .ok_or(OptionalField)
///     }
/// }
///
/// let slice=&[3,5,8,13][..];
///
/// assert_eq!( slice.field_(FromEnd(0)), Some(&13) );
/// assert_eq!( slice.field_(FromEnd(1)), Some(&8) );
/// assert_eq!( slice.field_(FromEnd(2)), Some(&5) );
/// assert_eq!( slice.field_(FromEnd(3)), Some(&3) );
/// assert_eq!( slice.field_(FromEnd(4)), None );
///
///
/// ```
pub trait RevGetFieldImpl<'a, This: ?Sized>: RevFieldType<This> {
    /// The error returned by `rev_*` methods.
    type Err: IsFieldErr;

    /// Accesses the field(s) that `self` represents inside of `this`,by reference.
    fn rev_get_field(self, this: &'a This) -> Result<&'a Self::Ty, Self::Err>;
}

/////////////////////////////////////////////////////////////////////////////

/// Like GetFieldMutImpl,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// # Use as bound
///
/// For examples of using `RevGetFieldMutImpl` as a bound look at example for
/// [RevGetFieldMut](./trait.RevGetFieldMut.html) or
/// [OptRevGetFieldMut](./trait.OptRevGetFieldMut.html).
///
/// # Safety
///
/// The `rev_get_field_raw_mut` function must return a non-aliasing pointer,
/// that is safe to dereference.
///
/// # Example
///
/// This example demonstrates implementing RevGetFieldMut to choose between 2 fields
/// based on a runtime value.
///
/// ```rust
/// use structural::field_traits::{OptionalField,RevFieldType,RevGetFieldImpl,RevGetFieldMutImpl};
/// use structural::field_path::IsSingleFieldPath;
/// use structural::{GetFieldExt,ts};
///
/// let mut tup=(3,5,8,13);
///
/// assert_eq!( tup.field_mut(Choose(Which::First, ts!(0), ts!(1))), &mut 3 );
/// assert_eq!( tup.field_mut(Choose(Which::Second, ts!(0), ts!(1))), &mut 5 );
/// assert_eq!( tup.field_mut(Choose(Which::First, ts!(1), ts!(2))), &mut 5 );
/// assert_eq!( tup.field_mut(Choose(Which::Second, ts!(1), ts!(2))), &mut 8 );
///
///
/// #[derive(Debug,Copy,Clone,PartialEq)]
/// struct Choose<P0,P1>(Which,P0,P1);
///
/// #[derive(Debug,Copy,Clone,PartialEq)]
/// enum Which{
///     First,
///     Second,
/// }
///
/// impl<P0,P1> IsSingleFieldPath for Choose<P0,P1>{}
///
/// impl<P0,P1,T> RevFieldType<T> for Choose<P0,P1>
/// where
///     P0: RevFieldType<T>,
///     P1: RevFieldType<T, Ty=P0::Ty>,
/// {
///     type Ty = P0::Ty;
/// }
///
/// impl<'a,P0,P1,T> RevGetFieldImpl<'a,T> for Choose<P0,P1>
/// where
///     P0: RevGetFieldImpl<'a,T>,
///     P1: RevGetFieldImpl<'a,T, Ty=P0::Ty, Err=P0::Err>,
/// {
///     type Err = P0::Err;
///
///     fn rev_get_field(self, this: &'a T) -> Result<&'a P0::Ty, P0::Err>{
///         match self.0 {
///             Which::First=>self.1.rev_get_field(this),
///             Which::Second=>self.2.rev_get_field(this),
///         }
///     }
/// }
///
/// unsafe impl<'a,P0,P1,T> RevGetFieldMutImpl<'a,T> for Choose<P0,P1>
/// where
///     P0: RevGetFieldMutImpl<'a,T>,
///     P1: RevGetFieldMutImpl<'a,T, Ty=P0::Ty, Err=P0::Err>,
/// {
///     fn rev_get_field_mut(self, this: &'a mut T) -> Result<&'a mut P0::Ty, P0::Err>{
///         match self.0 {
///             Which::First=>self.1.rev_get_field_mut(this),
///             Which::Second=>self.2.rev_get_field_mut(this),
///         }
///     }
///
///     unsafe fn rev_get_field_raw_mut(self, this: *mut *mut T) -> Result<*mut P0::Ty, P0::Err>{
///         match self.0 {
///             Which::First=>self.1.rev_get_field_raw_mut(this),
///             Which::Second=>self.2.rev_get_field_raw_mut(this),
///         }
///     }
/// }
///
/// ```
pub unsafe trait RevGetFieldMutImpl<'a, This: ?Sized>: RevGetFieldImpl<'a, This> {
    /// Accesses the field(s) that `self` represents inside of `this`,by mutable reference.
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut Self::Ty, Self::Err>;

    /// Accesses the field(s) that `self` represents inside of `this`,by raw pointer.
    unsafe fn rev_get_field_raw_mut(self, this: *mut *mut This)
        -> Result<*mut Self::Ty, Self::Err>;
}

/////////////////////////////////////////////////////////////////////////////

/// Like IntoFieldImpl,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// # Use as bound
///
/// For examples of using `RevIntoFieldImpl` as a bound look at example for
/// [RevIntoField](./trait.RevIntoField.html) or
/// [OptRevIntoField](./trait.OptRevIntoField.html).
///
/// # Example
///
/// This example demonstrates implementing RevGetFieldMut to choose between 2 fields
/// based on a runtime value.
///
/// ```rust
/// use structural::field_traits::{OptionalField,RevFieldType,RevGetFieldImpl,RevIntoFieldImpl};
/// use structural::field_path::IsSingleFieldPath;
/// use structural::{GetFieldExt,fp};
///
/// use std::mem;
///
/// let mut tup=(3,5,8,13);
///
/// assert_eq!( tup.field_(Wrapped(fp!(0))), &Newtype(3) );
/// assert_eq!( tup.field_(Wrapped(fp!(1))), &Newtype(5) );
/// assert_eq!( tup.field_(Wrapped(fp!(2))), &Newtype(8) );
/// assert_eq!( tup.field_(Wrapped(fp!(3))), &Newtype(13) );
///
/// assert_eq!( tup.into_field(Wrapped(fp!(0))), Newtype(3) );
/// assert_eq!( tup.into_field(Wrapped(fp!(1))), Newtype(5) );
/// assert_eq!( tup.into_field(Wrapped(fp!(2))), Newtype(8) );
/// assert_eq!( tup.into_field(Wrapped(fp!(3))), Newtype(13) );
///
///
/// #[derive(Debug,Copy,Clone,PartialEq)]
/// struct Wrapped<P>(P);
///
/// #[repr(transparent)]
/// #[derive(Debug,Copy,Clone,PartialEq)]
/// pub struct Newtype<T:?Sized>(T);
///
/// impl<P> IsSingleFieldPath for Wrapped<P>{}
///
/// impl<P,T> RevFieldType<T> for Wrapped<P>
/// where
///     P: RevFieldType<T>,
/// {
///     type Ty = Newtype<P::Ty>;
/// }
///
/// impl<'a,P,T> RevGetFieldImpl<'a,T> for Wrapped<P>
/// where
///     P: RevGetFieldImpl<'a,T>,
/// {
///     type Err = P::Err;
///
///     fn rev_get_field(self, this: &'a T) -> Result<&'a Self::Ty, P::Err>{
///         self.0.rev_get_field(this)
///             .map(|x|unsafe{ mem::transmute::<&P::Ty,&Newtype<P::Ty>>(x) })
///     }
/// }
///
/// impl<'a,P,T> RevIntoFieldImpl<'a,T> for Wrapped<P>
/// where
///     P: RevIntoFieldImpl<'a,T>,
///     P::Ty: Sized,
/// {
///     type BoxedTy = Newtype<P::BoxedTy>;
///
///     fn rev_into_field(self, this: T) -> Result<Self::Ty, Self::Err>
///     where
///         Self::Ty: Sized
///     {
///         self.0.rev_into_field(this)
///             .map(Newtype)
///     }
///     
///     fn rev_box_into_field(self, this: Box<T>) -> Result<Self::BoxedTy, Self::Err>{
///         self.0.rev_box_into_field(this)
///             .map(Newtype)
///     }
/// }
/// ```
pub trait RevIntoFieldImpl<'a, This: ?Sized>: RevGetFieldImpl<'a, This> {
    type BoxedTy;

    /// Accesses the field(s) that `self` represents inside of `this`,by value.
    fn rev_into_field(self, this: This) -> Result<Self::Ty, Self::Err>
    where
        This: Sized,
        Self::Ty: Sized;

    /// Accesses the field(s) that `self` represents inside of `this`,by value.
    #[cfg(feature = "alloc")]
    fn rev_box_into_field(self, this: Box<This>) -> Result<Self::BoxedTy, Self::Err>;
}

/////////////////////////////////////////////////////////////////////////////

macro_rules! declare_accessor_trait_alias {
    (
        $(#[$attr:meta])*
        $vis:vis trait $trait_name:ident<$lt:lifetime,$This:ident>=
        $($supertraits:tt)*
    ) => (
        $(#[$attr])*
        $vis trait $trait_name<$lt, $This >:$($supertraits)* {}

        impl<$lt,Path,$This> $trait_name<$lt, $This > for Path
        where
            Path:$($supertraits)*
        {}
    )
}

declare_accessor_trait_alias! {
    /// A trait alias,like GetField with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access a nested non-optional field by reference.
    ///
    /// ```rust
    /// use structural::field_traits::RevGetField;
    /// use structural::{GetFieldExt, field_path_aliases};
    ///
    /// let tup=(3,5,(8,(13,21)));
    ///
    /// assert_eq!( get_nested(&tup), &13 );
    ///
    /// field_path_aliases!{
    ///     mod paths{ nested=2.1.0 }
    /// }
    ///
    /// fn get_nested<T>(this:&T)->&i32
    /// where
    ///     // You can use `FP!(2.1.0)` instead of `paths::nested` from Rust 1.40 onwards.
    ///     paths::nested: for<'a> RevGetField<'a,T,Ty=i32>
    /// {
    ///     this.field_(paths::nested)
    /// }
    ///
    ///
    /// ```
    pub trait RevGetField<'a,This>=
        RevGetFieldImpl<'a,This,Err=NonOptField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like OptGetField with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access a nested optional field by reference.
    ///
    /// ```rust
    /// use structural::field_traits::OptRevGetField;
    /// use structural::{GetFieldExt, field_path_aliases};
    ///
    /// let tup1=(3,5,(8,(Some(13),21)));
    /// let tup2=(3,5,(8,(None,21)));
    ///
    /// assert_eq!( get_nested(&tup1), Some(&13) );
    /// assert_eq!( get_nested(&tup2), None );
    ///
    /// field_path_aliases!{
    ///     mod paths{ nested=2.1.0.Some }
    /// }
    ///
    /// fn get_nested<T>(this:&T)->Option<&i32>
    /// where
    ///     // You can use `FP!(2.1.0.Some)` instead of
    ///     // `paths::nested` from Rust 1.40 onwards.
    ///     paths::nested: for<'a> OptRevGetField<'a,T,Ty=i32>
    /// {
    ///     this.field_(paths::nested)
    /// }
    ///
    ///
    /// ```
    pub trait OptRevGetField<'a,This>=
        RevGetFieldImpl<'a,This,Err=OptionalField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like GetFieldMut with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access a nested non-optional field by
    /// mutable reference.
    ///
    /// ```rust
    /// use structural::field_traits::RevGetFieldMut;
    /// use structural::for_examples::{StructFoo, StructBar, Struct3};
    /// use structural::{GetFieldExt, field_path_aliases};
    ///
    /// let mut struct_=Struct3{
    ///     foo: Some(0),
    ///     bar: "hi",
    ///     baz: StructBar{
    ///         bar: StructFoo{
    ///             foo:101,
    ///         },
    ///     },
    /// };
    ///
    /// assert_eq!( get_nested(&mut struct_), &mut 101 );
    ///
    /// field_path_aliases!{
    ///     mod paths{ nested=baz.bar.foo }
    /// }
    ///
    /// fn get_nested<T>(this:&mut T)->&mut i32
    /// where
    ///     // You can use `FP!(baz.bar.foo)` instead of `paths::nested` from
    ///     // Rust 1.40 onwards.
    ///     paths::nested: for<'a> RevGetFieldMut<'a,T,Ty=i32>
    /// {
    ///     this.field_mut(paths::nested)
    /// }
    ///
    ///
    /// ```
    pub trait RevGetFieldMut<'a,This>=
        RevGetFieldMutImpl<'a,This,Err=NonOptField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like OptGetFieldMut with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access a nested optional field by
    /// mutable reference.
    ///
    /// ```rust
    /// use structural::field_traits::OptRevGetFieldMut;
    /// use structural::for_examples::{StructFoo, StructBar, Struct3};
    /// use structural::{GetFieldExt, field_path_aliases};
    ///
    /// let mut struct_=Struct3{
    ///     foo: Some(0),
    ///     bar: "hi",
    ///     baz: StructBar{
    ///         bar: StructFoo{
    ///             foo:Some("hello"),
    ///         },
    ///     },
    /// };
    ///
    /// assert_eq!( get_nested(&mut struct_), Some(&mut "hello") );
    ///
    /// field_path_aliases!{
    ///     mod paths{ nested=baz.bar.foo.Some }
    /// }
    ///
    /// fn get_nested<T>(this:&mut T)->Option<&mut &'static str>
    /// where
    ///     // You can use `FP!(baz.bar.foo.Some)` instead of `paths::nested` from
    ///     // Rust 1.40 onwards.
    ///     paths::nested: for<'a> OptRevGetFieldMut<'a,T,Ty=&'static str>
    /// {
    ///     this.field_mut(paths::nested)
    /// }
    ///
    ///
    /// ```
    pub trait OptRevGetFieldMut<'a,This>=
        RevGetFieldMutImpl<'a,This,Err=OptionalField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like IntoField with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access a nested non-optional field by value.
    ///
    /// ```rust
    /// use structural::field_traits::RevIntoField;
    /// use structural::for_examples::StructBar;
    /// use structural::{GetFieldExt, field_path_aliases};
    ///
    /// use std::cmp::Ordering;
    ///
    /// let struct_=StructBar{
    ///     bar: StructBar{
    ///         bar: StructBar{
    ///             bar: Ordering::Greater
    ///         },
    ///     },
    /// };
    ///
    /// assert_eq!( get_nested(struct_), Ordering::Greater );
    ///
    /// field_path_aliases!{
    ///     mod paths{ nested=bar.bar.bar }
    /// }
    ///
    /// fn get_nested<T>(this:T)->Ordering
    /// where
    ///     // You can use `FP!(bar.bar.bar)` instead of `paths::nested` from
    ///     // Rust 1.40 onwards.
    ///     paths::nested: for<'a> RevIntoField<'a,T,Ty=Ordering>
    /// {
    ///     this.into_field(paths::nested)
    /// }
    ///
    ///
    /// ```
    pub trait RevIntoField<'a,This>=
        RevIntoFieldImpl<'a,This,Err=NonOptField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like OptIntoField with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access a nested optional field by value.
    ///
    /// ```rust
    /// use structural::field_traits::OptRevIntoField;
    /// use structural::for_examples::{StructFoo,WithBoom};
    /// use structural::{GetFieldExt, field_path_aliases};
    ///
    /// let nope=StructFoo{ foo: WithBoom::Nope };
    /// let boom=StructFoo{ foo: WithBoom::Boom{  a: "hello", b: &[3,5,8,13]  } };
    ///
    /// assert_eq!( get_nested(nope), None );
    /// assert_eq!( get_nested(boom), Some(&[3,5,8,13][..]) );
    ///
    /// field_path_aliases!{
    ///     mod paths{ nested=foo::Boom.b }
    /// }
    ///
    /// fn get_nested<T>(this:T)->Option<&'static [u16]>
    /// where
    ///     // You can use `FP!(foo::Boom.b)` instead of `paths::nested` from
    ///     // Rust 1.40 onwards.
    ///     paths::nested: for<'a> OptRevIntoField<'a,T,Ty=&'static [u16]>
    /// {
    ///     this.into_field(paths::nested)
    /// }
    ///
    ///
    /// ```
    pub trait OptRevIntoField<'a,This>=
        RevIntoFieldImpl<'a,This,Err=OptionalField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like IntoFieldMut with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how to access a nested non-optional field by
    /// mutable reference,and by value.
    ///
    /// Also,how to write extension traits with `Rev*` traits.
    ///
    /// ```rust
    /// use structural::field_traits::{RevIntoFieldMut,RevGetFieldType};
    /// use structural::for_examples::StructBar;
    /// use structural::reexports::{ConstDefault,const_default};
    /// use structural::{GetFieldExt, field_path_aliases};
    ///
    /// let mut foo=StructBar{
    ///     bar: ([(0,3),(5,8)], [(40,50,60)]),
    /// };
    ///
    /// assert_eq!( foo.get_nested_mut(), &mut (5,8) );
    /// assert_eq!( foo.into_nested(), (5,8) );
    ///
    /// let mut oop=StructBar{
    ///     bar: [["hello","world"],["uh","no"]],
    /// };
    ///
    /// assert_eq!( oop.get_nested_mut(), &mut "world" );
    /// assert_eq!( oop.into_nested(), "world" );
    ///
    /// field_path_aliases!{
    ///     mod paths{ nested=bar.0.1 }
    /// }
    ///
    /// trait GetNested: Sized {
    ///     fn get_nested_mut<'a,Ty>(&'a mut self)->&'a mut Ty
    ///     where
    ///         // You can use `FP!(bar.0.1)` instead of `paths::nested` from
    ///         // Rust 1.40 onwards.
    ///         paths::nested: RevIntoFieldMut<'a,Self,Ty=Ty>
    ///     {
    ///         self.field_mut(paths::nested)
    ///     }
    ///
    ///     fn into_nested<'a,Ty>(self)->Ty
    ///     where
    ///         // You can use `FP!(bar.0.1)` instead of `paths::nested` from
    ///         // Rust 1.40 onwards.
    ///         paths::nested: RevIntoFieldMut<'a,Self,Ty=Ty>
    ///     {
    ///         self.into_field(paths::nested)
    ///     }
    /// }
    ///
    /// impl<T> GetNested for T {}
    ///
    /// ```
    pub trait RevIntoFieldMut<'a,This>=
        RevIntoField<'a,This> + RevGetFieldMut<'a,This>
}

declare_accessor_trait_alias! {
    /// A trait alias,like OptIntoFieldMut with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how to access a nested optional field
    /// by mutable reference,and by value.
    ///
    /// Also,how to write extension traits with `Rev*` traits.
    ///
    /// ```rust
    /// use structural::field_traits::{OptRevIntoFieldMut,RevGetFieldType};
    /// use structural::for_examples::{StructFoo,WithBoom};
    /// use structural::{GetFieldExt, field_path_aliases};
    ///
    /// let mut nope=StructFoo{ foo: WithBoom::Nope };
    /// let mut boom=StructFoo{ foo: WithBoom::Boom{  a: "hello", b: &[3,5,8,13]  } };
    ///
    /// assert_eq!( nope.get_nested_mut(), None );
    /// assert_eq!( boom.get_nested_mut(), Some(&mut &[3,5,8,13][..]) );
    ///
    /// assert_eq!( nope.into_nested(), None );
    /// assert_eq!( boom.into_nested(), Some(&[3,5,8,13][..]) );
    ///
    /// field_path_aliases!{
    ///     mod paths{ nested=foo::Boom.b }
    /// }
    ///
    /// trait GetNested: Sized {
    ///     fn get_nested_mut<'a,Ty>(&'a mut self)->Option<&'a mut Ty>
    ///     where
    ///         // You can use `FP!(foo::Boom.b)` instead of `paths::nested` from
    ///         // Rust 1.40 onwards.
    ///         paths::nested: OptRevIntoFieldMut<'a,Self,Ty=Ty>
    ///     {
    ///         self.field_mut(paths::nested)
    ///     }
    ///
    ///     fn into_nested<'a,Ty>(self)->Option<Ty>
    ///     where
    ///         // You can use `FP!(foo::Boom.b)` instead of `paths::nested` from
    ///         // Rust 1.40 onwards.
    ///         paths::nested: OptRevIntoFieldMut<'a,Self,Ty=Ty>
    ///     {
    ///         self.into_field(paths::nested)
    ///     }
    /// }
    ///
    /// impl<T> GetNested for T {}
    ///
    /// ```
    pub trait OptRevIntoFieldMut<'a,This>=
        OptRevIntoField<'a,This> + OptRevGetFieldMut<'a,This>
}

/////////////////////////////////////////////////////////////////////////////

macro_rules! impl_get_nested_field_inner {
    (inner;
        receivers( $($receiver:ident)* )
        first($fname0:ident $ferr0:ident $fty0:ident)
        second(
            ($fname1:ident $ferr1:ident $fty1:ident)
            $($rem_000:tt)*
        )
        middle(
            $(($fname_m:ident $ferr_m:ident $fty_m:ident))*
        )
        suffix(
            $(($fname_s:ident $ferr_s:ident $fty_s:ident))*
        )
        all(
            $(($fname_a:ident $ferr_a:ident $fty_a:ident))*
        )
        last($fname_l:ident $ferr_l:ident $fty_l:ident)
    )=>{
        impl<$($fname_a,$fty_a,)* This> RevFieldType<This> for FieldPath<($($fname_a,)*)>
        where
            This:?Sized,
            $(
                $fname_a: RevFieldType<$receiver, Ty=$fty_a>,
                $fty_a:?Sized,
            )*
        {
            type Ty=$fty_l;
        }

        impl<'a,$($fname_a,$fty_a, $ferr_a,)* This,CombErr>
            RevGetFieldImpl<'a,This>
        for FieldPath<($($fname_a,)*)>
        where
            This:?Sized+'a,
            $(
                $fname_a: RevGetFieldImpl<'a,$receiver, Ty=$fty_a, Err=$ferr_a>,
                $fty_a:?Sized+'a,
                $ferr_a:IntoFieldErr< CombErr >,
            )*
            ( $($ferr_a,)* ): CombinedErrs<Combined= CombErr >,
            CombErr:IsFieldErr,
        {
            type Err=CombErr;

            #[inline(always)]
            fn rev_get_field(self,field:&'a This)->Result<&'a $fty_l,CombErr>{
                let ($($fname_a,)*)=self.list;
                $(
                    let field=try_fe!( $fname_a.rev_get_field(field) );
                )*
                Ok(field)
            }
        }


        unsafe impl<'a,$($fname_a,$fty_a, $ferr_a,)* This,CombErr>
            RevGetFieldMutImpl<'a,This>
        for FieldPath<($($fname_a,)*)>
        where
            This:?Sized+'a,
            $(
                $fname_a: RevGetFieldMutImpl<'a,$receiver, Ty=$fty_a, Err=$ferr_a>,
                $fty_a:'a,
                $ferr_a:IntoFieldErr< CombErr >,
            )*
            Self:RevGetFieldImpl<'a,This,Ty=$fty_l,Err=CombErr>,
            CombErr:IsFieldErr,
        {
            #[inline(always)]
            fn rev_get_field_mut(self,field:&'a mut This)->Result<&'a mut $fty_l,CombErr >{
                let ($($fname_a,)*)=self.list;
                $(
                    let field=try_fe!( $fname_a.rev_get_field_mut(field) );
                )*
                Ok(field)
            }

            unsafe fn rev_get_field_raw_mut(
                self,
                field:*mut *mut This
            )->Result<*mut $fty_l,CombErr>{
                let ($($fname_a,)*)=self.list;
                let mut field=*field;
                $(
                    #[allow(unused_mut)]
                    let mut field={
                        let field=&mut field as *mut _;
                        try_fe!($fname_a.rev_get_field_raw_mut(field))
                    };
                )*
                Ok(field)
            }
        }


        impl<'a,$($fname_a, $fty_a:'a, $ferr_a,)* This,BoxedTy0:'a,CombErr>
            RevIntoFieldImpl<'a,This>
        for FieldPath<($($fname_a,)*)>
        where
            Self:RevGetFieldImpl<'a,This,Ty=$fty_l,Err=CombErr>,
            CombErr:IsFieldErr,

            This:?Sized+'a,
            $fname0: RevIntoFieldImpl<'a, This, Ty=$fty0, BoxedTy=BoxedTy0, Err=$ferr0>,

            $fname1: RevIntoFieldImpl<
                'a,
                BoxedTy0,
                Ty= RevGetFieldType<$fname1,$fty0>,
                Err= RevGetFieldErr<'a,$fname1,$fty0>,
            >,

            $(
                $fname_s: RevIntoFieldImpl<'a, $fty_m, Ty=$fty_s, Err=$ferr_s>,
            )*

            $( $ferr_a:IntoFieldErr< CombErr >, )*
        {
            type BoxedTy=$fty_l;

            #[inline(always)]
            fn rev_into_field(self,field:This)->Result<$fty_l,CombErr>
            where
                This:Sized
            {
                let ($($fname_a,)*)=self.list;
                $(
                    let field=try_fe!( $fname_a.rev_into_field(field) );
                )*
                Ok(field)
            }

            #[cfg(feature="alloc")]
            #[inline(always)]
            fn rev_box_into_field(
                self,
                field:crate::pmr::Box<This>,
            )->Result<$fty_l,CombErr>{
                let ($($fname_a,)*)=self.list;
                let field=try_fe!(
                    $fname0.rev_box_into_field(field)
                );
                $(
                    let field=try_fe!(
                        $fname_s.rev_into_field(field)
                    );
                )*
                Ok(field)
            }
        }

    };
    (
        ($fname0:ident $ferr0:ident $fty0:ident)
        $(($fname:ident $ferr:ident $fty:ident))*
        ;last=($fname_l:ident $ferr_l:ident $fty_l:ident)
    ) => {
        impl_get_nested_field_inner!{
            inner;

            receivers( This $fty0 $($fty)* )
            first ($fname0 $ferr0 $fty0)
            second (
                $(($fname $ferr $fty))*
                ($fname_l $ferr_l $fty_l)
            )
            middle(
                ($fname0 $ferr0 $fty0)
                $(($fname $ferr $fty))*
            )
            suffix(
                $(($fname $ferr $fty))*
                ($fname_l $ferr_l $fty_l)
            )
            all(
                ($fname0 $ferr0 $fty0)
                $(($fname $ferr $fty))*
                ($fname_l $ferr_l $fty_l)
            )
            last($fname_l $ferr_l $fty_l)
        }
    }
}

impl_get_nested_field_inner! {
    (F0 E0 T0)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    (F4 E4 T4)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    (F4 E4 T4)
    (F5 E5 T5)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    (F4 E4 T4)
    (F5 E5 T5)
    (F6 E6 T6)
    ;last=(FL EL TL)
}

////////////////////////////////////////////////////////////////////////////////
/////           Implementations for FP!() (The empty field path)
////////////////////////////////////////////////////////////////////////////////

impl<This> RevFieldType<This> for FieldPath<()>
where
    This: ?Sized,
{
    type Ty = This;
}

impl<'a, This> RevGetFieldImpl<'a, This> for FieldPath<()>
where
    This: ?Sized + 'a,
{
    type Err = NonOptField;

    fn rev_get_field(self, this: &'a This) -> Result<&'a Self::Ty, Self::Err> {
        Ok(this)
    }
}

unsafe impl<'a, This> RevGetFieldMutImpl<'a, This> for FieldPath<()>
where
    This: ?Sized + 'a,
{
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut Self::Ty, Self::Err> {
        Ok(this)
    }

    unsafe fn rev_get_field_raw_mut(
        self,
        this: *mut *mut This,
    ) -> Result<*mut Self::Ty, Self::Err> {
        Ok(*this)
    }
}

impl<'a, This> RevIntoFieldImpl<'a, This> for FieldPath<()>
where
    This: Sized + 'a,
{
    type BoxedTy = This;

    fn rev_into_field(self, this: This) -> Result<Self::Ty, Self::Err> {
        Ok(this)
    }

    #[cfg(feature = "alloc")]
    fn rev_box_into_field(self, this: Box<This>) -> Result<Self::BoxedTy, Self::Err> {
        Ok(*this)
    }
}

////////////////////////////////////////////////////////////////////////////////
/////           Implementations for single path field paths
////////////////////////////////////////////////////////////////////////////////

impl<This, F0> RevFieldType<This> for FieldPath<(F0,)>
where
    This: ?Sized,
    F0: RevFieldType<This>,
{
    type Ty = F0::Ty;
}

impl<'a, This, F0> RevGetFieldImpl<'a, This> for FieldPath<(F0,)>
where
    This: ?Sized + 'a,
    F0: RevGetFieldImpl<'a, This>,
{
    type Err = F0::Err;

    fn rev_get_field(self, this: &'a This) -> Result<&'a F0::Ty, F0::Err> {
        self.list.0.rev_get_field(this)
    }
}

unsafe impl<'a, This, F0> RevGetFieldMutImpl<'a, This> for FieldPath<(F0,)>
where
    This: ?Sized + 'a,
    F0: RevGetFieldMutImpl<'a, This>,
{
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut F0::Ty, F0::Err> {
        self.list.0.rev_get_field_mut(this)
    }

    unsafe fn rev_get_field_raw_mut(self, this: *mut *mut This) -> Result<*mut F0::Ty, F0::Err> {
        self.list.0.rev_get_field_raw_mut(this)
    }
}

impl<'a, This, F0> RevIntoFieldImpl<'a, This> for FieldPath<(F0,)>
where
    This: ?Sized + 'a,
    F0: RevIntoFieldImpl<'a, This>,
{
    type BoxedTy = F0::BoxedTy;

    fn rev_into_field(self, this: This) -> Result<F0::Ty, F0::Err>
    where
        This: Sized,
        F0::Ty: Sized,
    {
        self.list.0.rev_into_field(this)
    }

    #[cfg(feature = "alloc")]
    fn rev_box_into_field(self, this: Box<This>) -> Result<F0::BoxedTy, F0::Err> {
        self.list.0.rev_box_into_field(this)
    }
}

////////////////////////////////////////////////////////////////////////////////
/////           Implementations for field path components
////////////////////////////////////////////////////////////////////////////////

// Used as an implementation detail of the specialized RevGetFieldMutImpl impls.
//
// This was created because the error messages with specialization enabled were worse,
// it said `VariantField<_,_> does not implement RevGetFieldMutImpl<'_,Foo>`,
// when it should have said `Foo does not implement GetFieldMutImpl<VariantField<_,_>>`,
// which it does with this.
#[doc(hidden)]
unsafe trait SpecRevGetFieldMut<'a, This: ?Sized>: RevGetFieldImpl<'a, This> {
    unsafe fn rev_get_field_raw_mut_inner(
        self,
        this: *mut *mut This,
    ) -> Result<*mut Self::Ty, Self::Err>;
}

macro_rules! impl_rev_traits {
    (
        impl[ $($typarams:tt)*] $self_:ty
        where[ $($where_:tt)* ]
    ) => (
        impl<This,$($typarams)*> RevFieldType<This> for $self_
        where
            This: ?Sized + FieldType<Self>,
            $($where_)*
        {
            type Ty =GetFieldType<This,Self>;
        }

        impl<'a,This,$($typarams)*> RevGetFieldImpl<'a,This> for $self_
        where
            This: ?Sized + 'a + GetFieldImpl<Self>,
            This::Ty: 'a,
            $($where_)*
        {
            type Err=This::Err;

            #[inline(always)]
            fn rev_get_field(self, this: &'a This) -> Result<&'a This::Ty,This::Err>{
                GetFieldImpl::get_field_( this, self, () )
            }
        }


        unsafe impl<'a,This,$($typarams)*> RevGetFieldMutImpl<'a,This> for $self_
        where
            This: ?Sized + 'a + GetFieldMutImpl<Self>,
            This::Ty: 'a,
            $($where_)*
        {
            #[inline(always)]
            fn rev_get_field_mut(self,this:&'a mut This)->Result<&'a mut This::Ty,This::Err >{
                map_fe!(
                    GetFieldMutImpl::get_field_mut_( this, self, () )
                )
            }

            #[inline(always)]
            unsafe fn rev_get_field_raw_mut(
                self,
                this:*mut *mut This,
            )->Result<*mut This::Ty,This::Err>{
                SpecRevGetFieldMut::<'a,This>::rev_get_field_raw_mut_inner(
                    self,
                    this
                )
            }
        }

        unsafe impl<'a,This,$($typarams)*> SpecRevGetFieldMut<'a,This> for $self_
        where
            This: ?Sized + 'a + GetFieldMutImpl<Self>,
            This::Ty: 'a,
            $($where_)*
        {
            default_if!{
                #[inline(always)]
                cfg(feature="specialization")
                unsafe fn rev_get_field_raw_mut_inner(
                    self,
                    this:*mut *mut This
                )-> Result<*mut This::Ty,This::Err>{
                    let func=(**this).get_field_raw_mut_func();
                    func(
                        this as *mut *mut (),
                        self,
                        (),
                    )
                }
            }
        }


        #[cfg(feature="specialization")]
        unsafe impl<'a,This,$($typarams)*> SpecRevGetFieldMut<'a,This> for $self_
        where
            This: 'a + GetFieldMutImpl<Self>,
            This::Ty: 'a,
            $($where_)*
        {
            #[inline(always)]
            unsafe fn rev_get_field_raw_mut_inner(
                self,
                this:*mut *mut This,
            )->Result<*mut This::Ty,This::Err>{
                <This as
                    GetFieldMutImpl<Self>
                >::get_field_raw_mut(this as *mut *mut (), self, ())
            }
        }


        impl<'a,This,$($typarams)*> RevIntoFieldImpl<'a,This> for $self_
        where
            This: ?Sized + 'a + IntoFieldImpl<Self>,
            This::Ty: 'a,
            $($where_)*
        {
            type BoxedTy=This::Ty;

            #[inline(always)]
            fn rev_into_field(self,this:This)->Result<This::Ty,This::Err>
            where
                This:Sized
            {
                this.into_field_(self,())
            }

            #[cfg(feature="alloc")]
            #[inline(always)]
            fn rev_box_into_field(self,this:crate::pmr::Box<This>)->Result<This::Ty,This::Err>{
                this.box_into_field_(self,())
            }
        }
    )
}

impl_rev_traits! {
    impl[T] TStr<T>
    where[]
}

impl_rev_traits! {
    impl[V,T] VariantField<V,T>
    where[]
}

////////////////////////////////////////////

impl<This, S> RevFieldType<This> for VariantName<TStr<S>>
where
    This: ?Sized + IsVariant<TStr<S>>,
    S: 'static,
{
    type Ty = VariantProxy<This, TStr<S>>;
}

impl<'a, This, S> RevGetFieldImpl<'a, This> for VariantName<TStr<S>>
where
    This: ?Sized + 'a + IsVariant<TStr<S>>,
    S: 'static,
{
    type Err = OptionalField;

    #[inline(always)]
    fn rev_get_field(
        self,
        this: &'a This,
    ) -> Result<&'a VariantProxy<This, TStr<S>>, OptionalField> {
        map_of!(this.as_variant(self.name))
    }
}

unsafe impl<'a, This, S> RevGetFieldMutImpl<'a, This> for VariantName<TStr<S>>
where
    This: ?Sized + 'a + IsVariant<TStr<S>>,
    S: 'static,
{
    #[inline(always)]
    fn rev_get_field_mut(
        self,
        this: &'a mut This,
    ) -> Result<&'a mut VariantProxy<This, TStr<S>>, OptionalField> {
        map_of!(this.as_mut_variant(self.name))
    }

    #[inline(always)]
    unsafe fn rev_get_field_raw_mut(
        self,
        this: *mut *mut This,
    ) -> Result<*mut VariantProxy<This, TStr<S>>, OptionalField> {
        map_of!(EnumExt::as_raw_mut_variant(*this, self.name))
    }
}

impl<'a, This, S> RevIntoFieldImpl<'a, This> for VariantName<TStr<S>>
where
    This: ?Sized + 'a + IsVariant<TStr<S>>,
    S: 'static,
{
    type BoxedTy = VariantProxy<Box<This>, TStr<S>>;

    #[inline(always)]
    fn rev_into_field(self, this: This) -> Result<VariantProxy<This, TStr<S>>, OptionalField>
    where
        This: Sized,
    {
        map_of!(this.into_variant(self.name))
    }

    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn rev_box_into_field(
        self,
        this: crate::pmr::Box<This>,
    ) -> Result<VariantProxy<Box<This>, TStr<S>>, OptionalField> {
        map_of!(this.box_into_variant(self.name))
    }
}
