/*!
Contains traits implemented on field paths,taking Structural types as parameters.
*/

#![allow(non_snake_case)]

use crate::{
    enums::{EnumExt, IsVariant, VariantProxy},
    field_path::{IsSingleFieldPath, NestedFieldPath, TStr, VariantField, VariantName},
    field_traits::{
        errors::{CombinedErrs, IntoFieldErr, IsFieldErr},
        EnumField, StructField,
    },
    FieldType, GetField, GetFieldMut, GetFieldType, GetVariantField, GetVariantFieldMut, IntoField,
    IntoVariantField,
};

#[cfg(feature = "alloc")]
use crate::pmr::Box;

/////////////////////////////////////////////////////////////////////////////

mod components;
mod nested_field_path;

/////////////////////////////////////////////////////////////////////////////

/// Queries the type of a nested field in This,
/// what field is queried is determined by `FieldName`,
///
/// # Example
///
/// ```
/// use structural::{
///     field_traits::RevGetFieldType,
///     GetFieldType3,GetFieldExt,Structural,
///     FP,fp,
/// };
///
/// fn main(){
///     let this=TopLevel::default();
///     
///     let baz: &RevGetFieldType<FP!(foo.bar.baz), TopLevel>=
///         this.field_(fp!(foo.bar.baz));
///     assert_eq!( *baz, Vec::new() );
///     
///     let strand: &RevGetFieldType<FP!(foo.bar.strand), TopLevel>=
///         this.field_(fp!(foo.bar.strand));
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

/// Queries the type returned by `RevIntoFieldImpl::rev_box_into_field`.
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

/// Like `Get*Field`,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// # Use as bound
///
/// For examples of using `RevGetFieldImpl` as a bound look at example for
/// [RevGetField](./trait.RevGetField.html)
/// (for getting a field in a struct,not inside of a nested enum) or
/// [OptRevGetField](./trait.OptRevGetField.html) (for getting a field in a (nested) enum).
///
/// # Example
///
/// This example demonstrates implementing RevGetField to index a slice from the end.
///
/// ```rust
/// use structural::field_traits::{EnumField,RevFieldType,RevGetFieldImpl};
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
///     type Err = EnumField;
///
///     fn rev_get_field(self, this: &'a [T]) -> Result<&'a T, EnumField>{
///         let len=this.len();
///         this.get(len.wrapping_sub(self.0 + 1))
///             .ok_or(EnumField)
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
    ///
    /// This can be either:
    ///
    /// - [`StructField`]:For a field in a struct (not inside a nested enum).
    ///
    /// - [`EnumField`]: For a field inside a (potentially nested) enum.
    ///
    /// [`StructField`]: ../errors/enum.StructField.html
    /// [`EnumField`]: ../errors/struct.EnumField.html
    type Err: IsFieldErr;

    /// Accesses the field that `self` represents inside of `this`,by reference.
    fn rev_get_field(self, this: &'a This) -> Result<&'a Self::Ty, Self::Err>;
}

/////////////////////////////////////////////////////////////////////////////

/// Like Get*FieldMut,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// # Safety
///
/// The `rev_get_field_raw_mut` function must return a valid pointer derived
/// from the passed in pointer, that is safe to dereference mutably.
///
/// # Use as bound
///
/// For examples of using `RevGetFieldMutImpl` as a bound look at example for
/// [RevGetFieldMut](./trait.RevGetFieldMut.html)
/// (for getting a field in a struct,not inside of a nested enum) or
/// [OptRevGetFieldMut](./trait.OptRevGetFieldMut.html) (for getting a field in a (nested) enum).
///
/// # Example
///
/// This example demonstrates implementing RevGetFieldMut to choose between 2 fields
/// based on a runtime value.
///
/// ```rust
/// use structural::field_traits::{EnumField,RevFieldType,RevGetFieldImpl,RevGetFieldMutImpl};
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
///     unsafe fn rev_get_field_raw_mut(self, this: *mut  T) -> Result<*mut P0::Ty, P0::Err>{
///         match self.0 {
///             Which::First=>self.1.rev_get_field_raw_mut(this),
///             Which::Second=>self.2.rev_get_field_raw_mut(this),
///         }
///     }
/// }
///
/// ```
pub unsafe trait RevGetFieldMutImpl<'a, This: ?Sized>: RevGetFieldImpl<'a, This> {
    /// Accesses the field that `self` represents inside of `this`,by mutable reference.
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut Self::Ty, Self::Err>;

    /// Accesses the field that `self` represents inside of `this`,by raw pointer.
    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut Self::Ty, Self::Err>;
}

/////////////////////////////////////////////////////////////////////////////

/// Like Into*Field,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// # Use as bound
///
/// For examples of using `RevIntoFieldImpl` as a bound look at example for
/// [RevIntoField](./trait.RevIntoField.html)
/// (for getting a field in a struct,not inside of a nested enum) or
/// [OptRevIntoField](./trait.OptRevIntoField.html) (for getting a field in a (nested) enum).
///
/// # Example
///
/// This example demonstrates implementing RevGetFieldMut to choose between 2 fields
/// based on a runtime value.
///
/// ```rust
/// use structural::field_traits::{EnumField,RevFieldType,RevGetFieldImpl,RevIntoFieldImpl};
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
    /// The type returned by `rev_box_into_field`,often the same as `Self::Tỳ`.
    ///
    /// The only type from `structural` where `Self::Ty` isn't the same as `Self::BoxedTy`
    /// is `VariantName`.
    type BoxedTy;

    /// Accesses the field that `self` represents inside of `this`,by value.
    fn rev_into_field(self, this: This) -> Result<Self::Ty, Self::Err>
    where
        This: Sized,
        Self::Ty: Sized;

    /// Accesses the field that `self` represents inside of `this`,by value.
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
    /// This example shows how you can access a nested field by reference.
    ///
    /// ```rust
    /// use structural::field_traits::RevGetField;
    /// use structural::{GetFieldExt,FP,fp};
    ///
    ///
    /// # fn main(){
    /// let tup=(3,5,(8,(13,21)));
    ///     assert_eq!( get_nested(&tup), &13 );
    /// # }
    ///
    /// fn get_nested<T>(this:&T)->&i32
    /// where
    ///     FP!(2.1.0): for<'a> RevGetField<'a,T,Ty=i32>
    /// {
    ///     this.field_(fp!(2.1.0))
    /// }
    ///
    ///
    /// ```
    pub trait RevGetField<'a,This>=
        RevGetFieldImpl<'a,This,Err=StructField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like GetVariantField with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access a field inside of a nested enum by reference.
    ///
    /// ```rust
    /// use structural::field_traits::OptRevGetField;
    /// use structural::{GetFieldExt,FP,fp};
    ///
    /// let tup1=(3,5,(8,(Some(13),21)));
    /// let tup2=(3,5,(8,(None,21)));
    ///
    /// assert_eq!( get_nested(&tup1), Some(&13) );
    /// assert_eq!( get_nested(&tup2), None );
    ///
    /// fn get_nested<T>(this:&T)->Option<&i32>
    /// where
    ///     FP!(2.1.0?): for<'a> OptRevGetField<'a,T,Ty=i32>
    /// {
    ///     this.field_(fp!(2.1.0?))
    /// }
    ///
    ///
    /// ```
    pub trait OptRevGetField<'a,This>=
        RevGetFieldImpl<'a,This,Err=EnumField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like GetFieldMut with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access a nested field by
    /// mutable reference.
    ///
    /// ```rust
    /// use structural::field_traits::RevGetFieldMut;
    /// use structural::for_examples::{StructFoo, StructBar, Struct3};
    /// use structural::{GetFieldExt,FP,fp};
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
    /// fn get_nested<T>(this:&mut T)->&mut i32
    /// where
    ///     FP!(baz.bar.foo): for<'a> RevGetFieldMut<'a,T,Ty=i32>
    /// {
    ///     this.field_mut(fp!(baz.bar.foo))
    /// }
    ///
    ///
    /// ```
    pub trait RevGetFieldMut<'a,This>=
        RevGetFieldMutImpl<'a,This,Err=StructField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like GetVariantFieldMut with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access a field inside of a nested enum by
    /// mutable reference.
    ///
    /// ```rust
    /// use structural::field_traits::OptRevGetFieldMut;
    /// use structural::for_examples::{StructFoo, StructBar, Struct3};
    /// use structural::{GetFieldExt,FP,fp};
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
    /// fn get_nested<T>(this:&mut T)->Option<&mut &'static str>
    /// where
    ///     FP!(baz.bar.foo?): for<'a> OptRevGetFieldMut<'a,T,Ty=&'static str>
    /// {
    ///     this.field_mut(fp!(baz.bar.foo?))
    /// }
    ///
    ///
    /// ```
    pub trait OptRevGetFieldMut<'a,This>=
        RevGetFieldMutImpl<'a,This,Err=EnumField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like IntoField with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access a nested field by value.
    ///
    /// ```rust
    /// use structural::field_traits::RevIntoField;
    /// use structural::for_examples::StructBar;
    /// use structural::{GetFieldExt,FP,fp};
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
    /// fn get_nested<T>(this:T)->Ordering
    /// where
    ///     FP!(bar.bar.bar): for<'a> RevIntoField<'a,T,Ty=Ordering>
    /// {
    ///     this.into_field(fp!(bar.bar.bar))
    /// }
    ///
    ///
    /// ```
    pub trait RevIntoField<'a,This>=
        RevIntoFieldImpl<'a,This,Err=StructField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like IntoVariantField with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access a field inside of a nested enum by value.
    ///
    /// ```rust
    /// use structural::field_traits::OptRevIntoField;
    /// use structural::for_examples::{StructFoo,WithBoom};
    /// use structural::{GetFieldExt,FP,fp};
    ///
    /// let nope=StructFoo{ foo: WithBoom::Nope };
    /// let boom=StructFoo{ foo: WithBoom::Boom{  a: "hello", b: &[3,5,8,13]  } };
    ///
    /// assert_eq!( get_nested(nope), None );
    /// assert_eq!( get_nested(boom), Some(&[3,5,8,13][..]) );
    ///
    /// fn get_nested<T>(this:T)->Option<&'static [u16]>
    /// where
    ///     FP!(foo::Boom.b): for<'a> OptRevIntoField<'a,T,Ty=&'static [u16]>
    /// {
    ///     this.into_field(fp!(foo::Boom.b))
    /// }
    ///
    ///
    /// ```
    pub trait OptRevIntoField<'a,This>=
        RevIntoFieldImpl<'a,This,Err=EnumField>
}

declare_accessor_trait_alias! {
    /// A trait alias,like IntoFieldMut with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how to access a nested field by
    /// mutable reference,and by value.
    ///
    /// Also,how to write extension traits with `Rev*` traits.
    ///
    /// ```rust
    /// use structural::field_traits::{RevIntoFieldMut,RevGetFieldType};
    /// use structural::for_examples::StructBar;
    /// use structural::reexports::{ConstDefault,const_default};
    /// use structural::{GetFieldExt,FP,fp};
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
    /// trait GetNested: Sized {
    ///     fn get_nested_mut<'a,Ty>(&'a mut self)->&'a mut Ty
    ///     where
    ///         FP!(bar.0.1): RevIntoFieldMut<'a,Self,Ty=Ty>
    ///     {
    ///         self.field_mut(fp!(bar.0.1))
    ///     }
    ///
    ///     fn into_nested<'a,Ty>(self)->Ty
    ///     where
    ///         FP!(bar.0.1): RevIntoFieldMut<'a,Self,Ty=Ty>
    ///     {
    ///         self.into_field(fp!(bar.0.1))
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
    /// A trait alias,like IntoVariantFieldMut with the parameters reversed.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how to access a field inside of a nested enum
    /// by mutable reference,and by value.
    ///
    /// Also,how to write extension traits with `Rev*` traits.
    ///
    /// ```rust
    /// use structural::field_traits::{OptRevIntoFieldMut,RevGetFieldType};
    /// use structural::for_examples::{StructFoo,WithBoom};
    /// use structural::{GetFieldExt,FP,fp};
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
    /// trait GetNested: Sized {
    ///     fn get_nested_mut<'a,Ty>(&'a mut self)->Option<&'a mut Ty>
    ///     where
    ///         FP!(foo::Boom.b): OptRevIntoFieldMut<'a,Self,Ty=Ty>
    ///     {
    ///         self.field_mut(fp!(foo::Boom.b))
    ///     }
    ///
    ///     fn into_nested<'a,Ty>(self)->Option<Ty>
    ///     where
    ///         FP!(foo::Boom.b): OptRevIntoFieldMut<'a,Self,Ty=Ty>
    ///     {
    ///         self.into_field(fp!(foo::Boom.b))
    ///     }
    /// }
    ///
    /// impl<T> GetNested for T {}
    ///
    /// ```
    pub trait OptRevIntoFieldMut<'a,This>=
        OptRevIntoField<'a,This> + OptRevGetFieldMut<'a,This>
}
