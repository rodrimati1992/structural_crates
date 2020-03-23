/*!
Contains traits implemented on field paths,taking Structural types as parameters.
*/

#![allow(non_snake_case)]

use crate::{
    field::{errors::IsFieldErr, FailedAccess, InfallibleAccess},
    path::IsSingleFieldPath,
};

/////////////////////////////////////////////////////////////////////////////

mod components;
mod nested_field_path;
mod path_set_types;

/////////////////////////////////////////////////////////////////////////////

/// Queries the type of a nested field in This,
/// what field is queried is determined by `FieldPath`,
///
/// # Example
///
/// ```
/// use structural::{
///     field::RevGetFieldType,
///     GetFieldType3,StructuralExt,Structural,
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
pub type RevGetFieldType<FieldPath, This> = <FieldPath as RevFieldType<This>>::Ty;

/// Queries the error type returned by `Rev*Field` methods.
pub type RevGetFieldErr<'a, FieldPath, This> = <FieldPath as RevGetFieldImpl<'a, This>>::Err;

/////////////////////////////////////////////////////////////////////////////

/// Like `FieldType`,except that the parameters are reversed.
/// `This` is the type we are accessing,and `Self` is a field path.
pub trait RevFieldType<This: ?Sized>: IsSingleFieldPath {
    /// The type of the field.
    type Ty: ?Sized;
}

/////////////////////////////////////////////////////////////////////////////

/// Like `Get*Field`,except that the parameters are reversed,
///
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// This is used by the [`StructuralExt::field_`](../../trait.StructuralExt.html#method.field_)
/// method.
///
/// # Use as bound
///
/// For examples of using `RevGetFieldImpl` as a bound look at example for:
///
/// - [RevGetField](./trait.RevGetField.html):
/// For infallible field access,generally struct fields,not inside of a nested enum.
///
/// - [OptRevGetField](./trait.OptRevGetField.html):
/// Fallible field access,generally for getting a field in a (potentially nested) enum.
///
/// # Example
///
/// This example demonstrates implementing `RevGetFieldImpl` to index a slice from the end.
///
///
/// ```rust
/// use structural::field::{FailedAccess,RevFieldType,RevGetFieldImpl};
/// use structural::path::IsSingleFieldPath;
/// use structural::StructuralExt;
///
/// struct FromEnd(usize);
///
/// impl IsSingleFieldPath for FromEnd{}
///
/// impl<'a,T> RevFieldType<[T]> for FromEnd {
///     type Ty = T;
/// }
/// impl<'a,T> RevGetFieldImpl<'a,[T]> for FromEnd {
///     type Err = FailedAccess;
///
///     fn rev_get_field(self, this: &'a [T]) -> Result<&'a T, FailedAccess>{
///         let len=this.len();
///         this.get(len.wrapping_sub(self.0 + 1))
///             .ok_or(FailedAccess)
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
    /// - [`InfallibleAccess`]:
    ///     For `Rev*` accessors that return a field that always exists,
    ///     most often in a struct.
    ///
    /// - [`FailedAccess`]:
    ///     For `Rev*` accessors that attempt to return a field that may not exist,
    ///     most often inside an enum.
    ///
    /// [`InfallibleAccess`]: ../errors/enum.InfallibleAccess.html
    /// [`FailedAccess`]: ../errors/struct.FailedAccess.html
    type Err: IsFieldErr;

    /// Accesses the field that `self` represents inside of `this`,by reference.
    fn rev_get_field(self, this: &'a This) -> Result<&'a Self::Ty, Self::Err>;
}

/////////////////////////////////////////////////////////////////////////////

/// Like Get*FieldMut,except that the parameters are reversed,
///
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// This is used by the [`StructuralExt::field_mut`](../../trait.StructuralExt.html#method.field_mut)
/// method.
///
/// # Safety
///
/// The `rev_get_field_raw_mut` function must return a valid pointer derived
/// from the passed in pointer, that is safe to dereference mutably.
///
/// # Use as bound
///
/// For examples of using `RevGetFieldMutImpl` as a bound look at example for:
///
/// - [RevGetFieldMut](./trait.RevGetFieldMut.html):
/// For infallible field access,generally struct fields,not inside of a nested enum.
///
/// - [OptRevGetFieldMut](./trait.OptRevGetFieldMut.html):
/// Fallible field access,generally for getting a field in a (potentially nested) enum.
///
/// # Example
///
/// This example demonstrates implementing `RevGetFieldMutImpl` to choose between 2 fields
/// based on a runtime value.
///
/// ```rust
/// use structural::field::{FailedAccess,RevFieldType,RevGetFieldImpl,RevGetFieldMutImpl};
/// use structural::path::IsSingleFieldPath;
/// use structural::{StructuralExt,ts};
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
    ///
    /// # Safety
    ///
    /// You must pass a pointer to a fully initialized instance of `This`.
    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut Self::Ty, Self::Err>;
}

/////////////////////////////////////////////////////////////////////////////

/// Like Into*Field,except that the parameters are reversed,
///
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// This is used by the
/// [`StructuralExt::into_field`](../../trait.StructuralExt.html#method.into_field)
/// and [`StructuralExt::box_into_field`](../../trait.StructuralExt.html#method.box_into_field)
/// method.
///
/// # Use as bound
///
/// For examples of using `RevIntoFieldImpl` as a bound look at example for:
///
/// - [RevIntoField](./trait.RevIntoField.html):
/// For infallible field access,generally struct fields,not inside of a nested enum.
///
/// - [OptRevIntoField](./trait.OptRevIntoField.html):
/// Fallible field access,generally for getting a field in a (potentially nested) enum.
///
/// # Example
///
/// This example demonstrates implementing `RevIntoFieldImpl`.
///
/// The transmute in this example is only sound because of the
/// `#[repr(transparent)]` attribute on the `Wrapper` struct,
/// and the reference to` T` is converted into a reference to `Wrapper<T>`.<br>
/// Transmutes like that are not sound in the more general case,
/// like transmuting from an arbitrary `Foo<T>` to `Foo<Newtype<T>>` using `std::mem::transmute`,
/// since the layout of `Foo` is allowed to change.
///
/// ```rust
/// use structural::field::{FailedAccess,RevFieldType,RevGetFieldImpl,RevIntoFieldImpl};
/// use structural::path::IsSingleFieldPath;
/// use structural::{StructuralExt,fp};
///
/// use core::mem;
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
///     fn rev_into_field(self, this: T) -> Result<Self::Ty, Self::Err>
///     where
///         Self::Ty: Sized
///     {
///         self.0.rev_into_field(this)
///             .map(Newtype)
///     }
/// }
/// ```
pub trait RevIntoFieldImpl<'a, This: ?Sized>: RevGetFieldImpl<'a, This> {
    /// Accesses the field that `self` represents inside of `this`,by value.
    fn rev_into_field(self, this: This) -> Result<Self::Ty, Self::Err>
    where
        This: Sized,
        Self::Ty: Sized;
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
    /// A trait alias for an infallible [`RevGetFieldImpl`](./trait.RevGetFieldImpl.html),
    /// generally used to access fields in structs(not in a nested enum inside the struct).
    ///
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how to access a nested struct field by reference.
    ///
    /// ```rust
    /// use structural::field::RevGetField;
    /// use structural::{StructuralExt,FP,fp};
    ///
    ///
    /// # fn main(){
    /// let tup=(3,5,(8,(13,21)));
    ///     assert_eq!( get_nested(&tup), &13 );
    /// # }
    ///
    /// fn get_nested<'a,T>(this:&'a T)->&'a i32
    /// where
    ///     FP!(2.1.0): RevGetField<'a,T,Ty=i32>
    /// {
    ///     this.field_(fp!(2.1.0))
    /// }
    ///
    ///
    /// ```
    pub trait RevGetField<'a,This>=
        RevGetFieldImpl<'a,This,Err=InfallibleAccess>
}

declare_accessor_trait_alias! {
    /// A trait alias for a fallible [`RevGetFieldImpl`](./trait.RevGetFieldImpl.html),
    /// generally used to access fields inside enums.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access an enum field by reference.
    ///
    /// ```rust
    /// use structural::field::OptRevGetField;
    /// use structural::{StructuralExt,FP,fp};
    ///
    /// let tup1=(3,5,(8,(Some(13),21)));
    /// let tup2=(3,5,(8,(None,21)));
    ///
    /// assert_eq!( get_nested(&tup1), Some(&13) );
    /// assert_eq!( get_nested(&tup2), None );
    ///
    /// fn get_nested<'a,T>(this:&'a T)->Option<&'a i32>
    /// where
    ///     FP!(2.1.0?): OptRevGetField<'a,T,Ty=i32>
    /// {
    ///     this.field_(fp!(2.1.0?))
    /// }
    ///
    ///
    /// ```
    pub trait OptRevGetField<'a,This>=
        RevGetFieldImpl<'a,This,Err=FailedAccess>
}

declare_accessor_trait_alias! {
    /// A trait alias for an infallible [`RevGetFieldMutImpl`](./trait.RevGetFieldMutImpl.html),
    /// generally used to access fields in structs(not in a nested enum inside the struct).
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how to access a nested struct field by mutable reference.
    ///
    /// ```rust
    /// use structural::field::RevGetFieldMut;
    /// use structural::for_examples::{StructFoo, StructBar, Struct3};
    /// use structural::{StructuralExt,FP,fp};
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
    /// fn get_nested<'a,T>(this:&'a mut T)->&'a mut i32
    /// where
    ///     FP!(baz.bar.foo): RevGetFieldMut<'a,T,Ty=i32>
    /// {
    ///     this.field_mut(fp!(baz.bar.foo))
    /// }
    ///
    ///
    /// ```
    pub trait RevGetFieldMut<'a,This>=
        RevGetFieldMutImpl<'a,This,Err=InfallibleAccess>
}

declare_accessor_trait_alias! {
    /// A trait alias for a fallible [`RevGetFieldMutImpl`](./trait.RevGetFieldMutImpl.html),
    /// generally used to access fields inside enums.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access an enum field by mutable reference.
    ///
    /// ```rust
    /// use structural::field::OptRevGetFieldMut;
    /// use structural::for_examples::{StructFoo, StructBar, Struct3};
    /// use structural::{StructuralExt,FP,fp};
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
    /// fn get_nested<'a,T>(this:&'a mut T)->Option<&'a mut &'static str>
    /// where
    ///     FP!(baz.bar.foo?): OptRevGetFieldMut<'a,T,Ty=&'static str>
    /// {
    ///     this.field_mut(fp!(baz.bar.foo?))
    /// }
    ///
    ///
    /// ```
    pub trait OptRevGetFieldMut<'a,This>=
        RevGetFieldMutImpl<'a,This,Err=FailedAccess>
}

declare_accessor_trait_alias! {
    /// A trait alias for an infallible [`RevIntoFieldImpl`](./trait.RevIntoFieldImpl.html),
    /// generally used to access fields in structs(not in a nested enum inside the struct).
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how to access a nested struct field by value.
    ///
    /// ```rust
    /// use structural::field::RevIntoField;
    /// use structural::for_examples::StructBar;
    /// use structural::{StructuralExt,FP,fp};
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
    /// fn get_nested<'a,T>(this:T)->Ordering
    /// where
    ///     FP!(bar.bar.bar): RevIntoField<'a,T,Ty=Ordering>
    /// {
    ///     this.into_field(fp!(bar.bar.bar))
    /// }
    ///
    ///
    /// ```
    pub trait RevIntoField<'a,This>=
        RevIntoFieldImpl<'a,This,Err=InfallibleAccess>
}

declare_accessor_trait_alias! {
    /// A trait alias for a fallible [`RevIntoFieldImpl`](./trait.RevIntoFieldImpl.html),
    /// generally used to access fields inside enums.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// # Example
    ///
    /// This example shows how you can access an enum field by value.
    ///
    /// ```rust
    /// use structural::field::OptRevIntoField;
    /// use structural::for_examples::{StructFoo,WithBoom};
    /// use structural::{StructuralExt,FP,fp};
    ///
    /// let nope=StructFoo{ foo: WithBoom::Nope };
    /// let boom=StructFoo{ foo: WithBoom::Boom{  a: "hello", b: &[3,5,8,13]  } };
    ///
    /// assert_eq!( get_nested(nope), None );
    /// assert_eq!( get_nested(boom), Some(&[3,5,8,13][..]) );
    ///
    /// fn get_nested<'a,T>(this:T)->Option<&'static [u16]>
    /// where
    ///     FP!(foo::Boom.b): OptRevIntoField<'a,T,Ty=&'static [u16]>
    /// {
    ///     this.into_field(fp!(foo::Boom.b))
    /// }
    ///
    ///
    /// ```
    pub trait OptRevIntoField<'a,This>=
        RevIntoFieldImpl<'a,This,Err=FailedAccess>
}

declare_accessor_trait_alias! {
    /// A trait alias for infallible [`RevIntoFieldImpl`] + [`RevGetFieldMutImpl`],
    /// generally used to access fields in structs(not in a nested enum inside the struct).
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// [`RevIntoFieldImpl`]: ./trait.RevIntoFieldImpl.html
    /// [`RevGetFieldMutImpl`]: ./trait.RevGetFieldMutImpl.html
    ///
    /// # Example
    ///
    /// This example shows how to access a struct field by mutable reference,and by value.
    ///
    /// Also,how to write extension traits with `Rev*` traits.
    ///
    /// ```rust
    /// use structural::field::{RevIntoFieldMut,RevGetFieldType};
    /// use structural::for_examples::StructBar;
    /// use structural::reexports::{ConstDefault,const_default};
    /// use structural::{StructuralExt,FP,fp};
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
    /// A trait alias for fallible [`RevIntoFieldImpl`] + [`RevGetFieldMutImpl`],
    /// generally used to access fields inside enums.
    ///
    /// `This` is the type we are accessing,and `Self` is a field path.
    ///
    /// [`RevIntoFieldImpl`]: ./trait.RevIntoFieldImpl.html
    /// [`RevGetFieldMutImpl`]: ./trait.RevGetFieldMutImpl.html
    ///
    /// # Example
    ///
    /// This example shows how to access an enum field by mutable reference,and by value.
    ///
    /// Also,how to write extension traits with `Rev*` traits.
    ///
    /// ```rust
    /// use structural::field::{OptRevIntoFieldMut,RevGetFieldType};
    /// use structural::for_examples::{StructFoo,WithBoom};
    /// use structural::{StructuralExt,FP,fp};
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
