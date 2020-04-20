/*!
Contains traits for accessing multiple fields at once.
*/
#![allow(non_snake_case)]

use crate::{
    field::NormalizeFields,
    path::{IsMultiFieldPath, UniquePaths},
    NormalizeFieldsOut,
};

#[allow(unused_imports)]
use core_extensions::SelfOps;

mod path_set_types;
mod single_field_path;

/// Queries the type returned by the
/// `RevGetMultiFieldImpl::rev_get_multi_field_impl` method.
/// This is some collection of references.
pub type RevGetMultiFieldImplOut<'a, Path, This> =
    <Path as RevGetMultiFieldImpl<'a, This>>::UnnormFields;

/// Queries the type returned by the
/// `RevGetMultiFieldMutImpl::rev_get_multi_field_mut_impl` method.
/// This is some collection of mutable references.
pub type RevGetMultiFieldMutImplOut<'a, Path, This> =
    <Path as RevGetMultiFieldMutImpl<'a, This>>::UnnormFieldsMut;

/// Queries the type returned by the
/// `RevGetMultiFieldMutImpl::rev_get_multi_field_raw_mut_impl` method.
/// This is some collection of mutable pointers.
pub type RevGetMultiFieldMutImplRaw<'a, Path, This> =
    <Path as RevGetMultiFieldMutImpl<'a, This>>::UnnormFieldsRawMut;

/// Queries the type returned by the
/// `RevIntoMultiFieldImpl::rev_into_multi_field_impl` method.
/// This is some collection of values.
pub type RevIntoMultiFieldImplOut<Path, This> =
    <Path as RevIntoMultiFieldImpl<This>>::UnnormIntoFields;

/// Queries the type returned by the
/// `RevGetMultiField::rev_get_multi_field` method.
/// This is some collection of references.
pub type RevGetMultiFieldOut<'a, Path, This> = <Path as RevGetMultiField<'a, This>>::Fields;

/// Queries the type returned by the
/// `RevGetMultiFieldMut::rev_get_multi_field_mut` method.
/// This is some collection of mutable references.
pub type RevGetMultiFieldMutOut<'a, Path, This> =
    <Path as RevGetMultiFieldMut<'a, This>>::FieldsMut;

/// Queries the type returned by the
/// `RevGetMultiFieldMut::rev_get_multi_field_raw_mut` method.
/// This is some collection of mutable pointers.
pub type RevGetMultiFieldMutRaw<'a, Path, This> =
    <Path as RevGetMultiFieldMut<'a, This>>::FieldsRawMut;

/// Queries the type returned by the
/// `RevIntoMultiField::rev_into_multi_field` method.
/// This is some collection of values.
pub type RevIntoMultiFieldOut<Path, This> = <Path as RevIntoMultiField<This>>::IntoFields;

/// Gets references to multiple fields from `This`,
/// usually a tuple of `Result<&_, E: IsFieldErr>`s,
///
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// To get `Option<&_>` or `&_` instead of each of those `Result`s,
/// you can use the [`RevGetMultiField`] as a bound instead.
///
/// [`RevGetMultiField`]: ./trait.RevGetMultiField.html
///
/// # Usage as bound
///
/// This trait shouldn't be used as a bound,except in implementations of the same trait.
///
/// For a trait that can be used in bounds,you can use [`RevGetMultiField`] (it has examples).
///
/// # Example
///
/// This example demonstrates how you can implement `RevGetMultiFieldImpl` to get a
/// pair of indices.
///
/// ```rust
/// use structural::field::{FailedAccess, RevGetMultiFieldImpl};
/// use structural::path::{AliasedPaths, IsMultiFieldPath};
/// use structural::StructuralExt;
///
/// struct Pair(usize,usize);
///
/// impl IsMultiFieldPath for Pair{
///     type PathUniqueness= AliasedPaths;
/// }
///
/// impl<'a,T:'a> RevGetMultiFieldImpl<'a,[T]> for Pair{
///     type UnnormFields=(
///         Result<&'a T,FailedAccess>,
///         Result<&'a T,FailedAccess>,
///     );
///     
///     fn rev_get_multi_field_impl(self, this: &'a [T]) -> Self::UnnormFields{
///         (
///             this.get(self.0).ok_or(FailedAccess),
///             this.get(self.1).ok_or(FailedAccess),
///         )
///     }
/// }
///
/// let arr=[2,3,5,8,13,21,34,55,89];
///
/// assert_eq!( arr[..].fields(Pair(2,1)), (Some(&5),Some(&3)) );
/// assert_eq!( arr[..].fields(Pair(0,3)), (Some(&2),Some(&8)) );
///
/// ```
///
pub trait RevGetMultiFieldImpl<'a, This: ?Sized + 'a>: IsMultiFieldPath + Sized {
    /// A collection of `Results<&'a _,_>`s referencing fields.
    type UnnormFields: 'a + NormalizeFields;

    /// Gets references to multiple fields from `this`.
    fn rev_get_multi_field_impl(self, this: &'a This) -> Self::UnnormFields;
}

/// Gets references to multiple fields from `This`,
/// usually a tuple of `Option<&_>`s and `&_`s
///
/// This trait has a blanket implementation for the [`RevGetMultiFieldImpl`] trait,
/// implementing that trait is necessary to be able to use this trait.
///
/// This is used by the
/// [`StructuralExt::fields`](../../trait.StructuralExt.html#method.fields),
/// and [`StructuralExt::cloned_fields`](../../trait.StructuralExt.html#method.cloned_fields)
/// methods.
///
/// There's also the [`RevGetMultiFieldOut`] type alias to get this trait's
/// [`Fields`] associated type.
///
/// # Example
///
/// This demonstrates how you can use `RevGetMultiField` with structs.
///
/// ```rust
/// use structural::{StructuralExt, FP, fp};
/// use structural::field::RevGetMultiField;
/// use structural::for_examples::{Tuple2, Tuple3};
///
/// use std::cmp::Ordering;
///
/// fn access_fields<'a,T,O0,O1>(this: &'a T)->(&'a O0,&'a O1)
/// where
///     // The `Fields= (&'a O0,&'a O1)` constraint ensures that the return type is a pair.
///     FP!(0,1): RevGetMultiField<'a,T,Fields= (&'a O0,&'a O1)>
/// {
///     this.fields(fp!(0,1))
/// }
///
/// let tup2 = Tuple2(Some(8), Ordering::Less);
/// let tup3 = Tuple3(Some("hello"), Some(true), 34);
///
/// assert_eq!( access_fields(&tup2), (&Some(8), &Ordering::Less) );
/// assert_eq!( access_fields(&tup3), (&Some("hello"), &Some(true)) );
///
/// ```
///
/// # Example
///
/// This demonstrates how you can use `RevGetMultiField` with enums.
///
/// ```rust
/// use structural::{StructuralExt, FP};
/// use structural::path::IntoAliasingOut;
/// use structural::field::{RevGetMultiField,RevGetMultiFieldOut};
/// use structural::for_examples::{Bomb, WithBoom};
///
/// use std::cmp::Ordering;
///
/// // Used `IntoAliasingOut` here to make the path safely constructible,
/// // since cosntructing a `NestedFieldPathSet<_,_,UniquePaths>`
/// // (what `FP!(::Boom=>a,b)` desugars into) requires unsafe code to construct,
/// // while `NestedFieldPathSet<_,_,AliasedPaths>`
/// // (what `IntoAliasingOut` turns `FP!(::Boom=>a,b)` into)
/// // does not require unsafe code to construct
/// //
/// // While this is safely constructible,it can't be used to do multiple mutable borrows,
/// // you can use `field_path_aliases` to avoid writing `unsafe` code in that case.
/// type ThePath=IntoAliasingOut<FP!(::Boom=>a,b)>;
///
/// fn access_fields<'a,T,O0,O1>(this: &'a T)->Option<(&'a O0,&'a O1)>
/// where
///     // The `Fields= Option<(&'a O0,&'a O1)>` constraint ensures that
///     // the return type is an optional pair.
///     ThePath: RevGetMultiField<'a,T,Fields= Option<(&'a O0,&'a O1)>>
/// {
///     this.fields(ThePath::NEW)
/// }
///
/// fn main(){
///     let with_0 = WithBoom::Nope;
///     let with_1 = WithBoom::Boom{a:"hi", b: &[0,1,2]};
///     assert_eq!( access_fields(&with_0), None );
///     assert_eq!( access_fields(&with_1), Some((&"hi", &&[0,1,2][..])) );
///    
///     let bomb_0 = Bomb::Nope;
///     let bomb_1 = Bomb::Boom{a:"hello", b: &[5,8,13]};
///     assert_eq!( access_fields(&bomb_0), None );
///     assert_eq!( access_fields(&bomb_1), Some((&"hello", &&[5,8,13][..])) );
/// }
///
/// ```
///
/// [`RevGetMultiFieldImpl`]: ./trait.RevGetMultiFieldImpl.html
///
/// [`Fields`]: ./trait.RevGetMultiField.html#associatedtype.Fields
///
/// [`RevGetMultiFieldImpl::rev_get_multi_field`]:
/// ./trait.RevGetMultiFieldImpl.html#tymethod.rev_get_multi_field
pub trait RevGetMultiField<'a, This: ?Sized + 'a>: RevGetMultiFieldImpl<'a, This> {
    /// This is usually a tuple of `Option<&_>`s and `&_`s.
    type Fields: 'a;

    /// Gets references to multiple fields from `this`,
    /// usually a tuple of `Option<&_>`s and `&_`s.
    fn rev_get_multi_field(self, this: &'a This) -> Self::Fields;
}

impl<'a, This, Path> RevGetMultiField<'a, This> for Path
where
    This: ?Sized + 'a,
    Path: RevGetMultiFieldImpl<'a, This>,
{
    type Fields = NormalizeFieldsOut<Path::UnnormFields>;

    fn rev_get_multi_field(self, this: &'a This) -> Self::Fields {
        let x = RevGetMultiFieldImpl::<'a, This>::rev_get_multi_field_impl(self, this);
        NormalizeFields::normalize_fields(x)
    }
}

//////////////////////////////////////////////////////////////////////////

/// Gets mutable references to multiple fields from `This`,
/// usually returning a tuple of `Result<&mut _, E: IsFieldErr>`s,
///
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// To get `Option<&mut _>`s and `&mut _`s instead of each of those `Result`s,
/// you can use the [`RevGetMultiFieldMut`] as a bound instead.
///
/// # Safety
///
/// The [`rev_get_multi_field_raw_mut_impl`] method must return non-aliasing pointers,
/// where all of them are safe to dereference.
///
/// As a reminder: mutable references imply uniqueness,
/// which means that it's undefined behavior for implementors to
/// return multiple mutable references to the same field from [`rev_get_multi_field_mut_impl`].
///
/// [`RevGetMultiFieldMut`]: ./trait.RevGetMultiFieldMut.html
/// [`rev_get_multi_field_raw_mut_impl`]: #tymethod.rev_get_multi_field_raw_mut_impl
/// [`rev_get_multi_field_mut_impl`]: #tymethod.rev_get_multi_field_mut_impl
///
///
/// # Example
///
/// This example demonstrates how you can implement `RevGetMultiFieldMutImpl` to get a
/// pair of indices.
///
/// ```rust
/// use structural::field::{FailedAccess, RevGetMultiFieldMutImpl};
/// use structural::path::{IsMultiFieldPath, UniquePaths};
/// use structural::StructuralExt;
///
/// let mut arr=[2,3,5,8,13,21,34,55,89];
///
/// assert_eq!( arr[..].fields_mut(Pair::new(2,1)), (Some(&mut 5),Some(&mut 3)) );
/// assert_eq!( arr[..].fields_mut(Pair::new(0,3)), (Some(&mut 2),Some(&mut 8)) );
///
/// mod pair{
///     pub struct Pair(usize,usize);
///     impl Pair {
///         pub fn first(&self)->usize{
///             self.0
///         }
///         pub fn second(&self)->usize{
///             self.1
///         }
///         pub fn new(first:usize, second:usize)->Self{
///             assert_ne!(first,second,"Expected disjoint indices");
///             Pair(first, second)
///         }
///     }
/// }
/// use pair::Pair;
///
/// impl IsMultiFieldPath for Pair{
///     type PathUniqueness= UniquePaths;
/// }
///
/// unsafe impl<'a,T:'a> RevGetMultiFieldMutImpl<'a,[T]> for Pair{
///     type UnnormFieldsMut=(
///         Result<&'a mut T,FailedAccess>,
///         Result<&'a mut T,FailedAccess>,
///     );
///
///     type UnnormFieldsRawMut=(
///         Result<*mut T,FailedAccess>,
///         Result<*mut T,FailedAccess>,
///     );
///
///     fn rev_get_multi_field_mut_impl(self, this: &'a mut [T]) -> Self::UnnormFieldsMut{
///         // Pair ensures that the indices are disjoint in its constructor,
///         // so there can't be overlapping mutable references.
///         unsafe{
///             let ret=self.rev_get_multi_field_raw_mut_impl(this);
///             (
///                 ret.0.map(|x|&mut *x),
///                 ret.1.map(|x|&mut *x),
///             )
///         }
///     }
///
///     unsafe fn rev_get_multi_field_raw_mut_impl(
///         self,
///         this: *mut [T]
///     ) -> Self::UnnormFieldsRawMut{
///         (
///             get_at_raw(this, self.first() ),
///             get_at_raw(this, self.second()),
///         )
///     }
/// }
///
/// /// # Safety
/// ///
/// /// The passed in `*mut [T]` must point to fully initialized memory.
/// ///
/// /// The index that you pass in must not have already been borrowed.
/// unsafe fn get_at_raw<T>(this: *mut [T], index: usize)-> Result<*mut T, FailedAccess> {
///     // There might be a better way to get the length of a `*mut [T]`.
///     let len=(*this).len();
///
///     if index < len {
///         Ok(&mut *(this as *mut T).add(index) as *mut T)
///     } else {
///         Err(FailedAccess)
///     }
/// }
///
///
/// ```
///
pub unsafe trait RevGetMultiFieldMutImpl<'a, This: ?Sized + 'a>:
    IsMultiFieldPath<PathUniqueness = UniquePaths> + Sized
{
    /// This is usually a tuple of `Result<&mut _,E: IsFieldErr>`s.
    type UnnormFieldsMut: 'a + NormalizeFields;

    /// This is usually a tuple of `Result<*mut _,E: IsFieldErr>`s.
    type UnnormFieldsRawMut: 'a + NormalizeFields;

    /// Gets mutable references to multiple fields from `this`,
    /// usually a tuple of `Result<&mut _,E: IsFieldErr>`s.
    fn rev_get_multi_field_mut_impl(self, this: &'a mut This) -> Self::UnnormFieldsMut;

    /// Gets raw pointers to multiple fields from `this`,
    /// usually a tuple of `Result<*mut _,E: IsFieldErr>`s.
    ///
    /// # Safety
    ///
    /// `this` must point to a valid instance of `This`,which lives for the `'a` lifetime.
    unsafe fn rev_get_multi_field_raw_mut_impl(self, this: *mut This) -> Self::UnnormFieldsRawMut;
}

/// Gets mutable references to multiple fields from `This`,
/// usually a tuple of `Option<&mut _>`s and `&mut _`s.
///
/// This trait has a blanket implementation for the [`RevGetMultiFieldMutImpl`] trait,
/// implementing that trait is necessary to be able to use this trait.
///
/// This is used by the
/// [`StructuralExt::fields_mut`](../../trait.StructuralExt.html#method.fields_mut)
/// method.
///
/// There's also the [`RevGetMultiFieldMutOut`] type alias to get this trait's
/// [`FieldsMut`] associated type.
///
/// # Example
///
/// This demonstrates using `RevGetMultiFieldMut` with structs.
///
/// ```rust
/// use structural::{StructuralExt, FP, fp};
/// use structural::field::RevGetMultiFieldMut;
/// use structural::for_examples::{Tuple2, Tuple3};
///
/// use std::cmp::Ordering;
///
/// fn access_fields<'a,T,O0,O1>(this: &'a mut T)->(&'a mut O0,&'a mut O1)
/// where
///     // The `FieldsMut= (&'a mut O0,&'a mut O1)` constraint ensures that
///     // the return type is a pair.
///     FP!(0,1): RevGetMultiFieldMut<'a,T,FieldsMut= (&'a mut O0,&'a mut O1)>
/// {
///     this.fields_mut(fp!(0,1))
/// }
///
/// let mut tup2 = Tuple2(Some(8), Ordering::Less);
/// let mut tup3 = Tuple3(Some("hello"), Some(true), 34);
///
/// assert_eq!( access_fields(&mut tup2), (&mut Some(8), &mut Ordering::Less) );
/// assert_eq!( access_fields(&mut tup3), (&mut Some("hello"), &mut Some(true)) );
///
/// ```
///
/// # Example
///
/// This demonstrates how you can use `RevGetMultiFieldMut` with enums.
///
/// ```rust
/// use structural::{StructuralExt, field_path_aliases};
/// use structural::field::{RevGetMultiFieldMut,RevGetMultiFieldMutOut};
/// use structural::for_examples::{Bomb, WithBoom};
///
/// use std::cmp::Ordering;
///
/// // Using the `field_path_aliases` macro is required to declare the `ThePath` constant,
/// // since it's unsafe to construct paths for borrowing multiple fields mutably
/// // (the macro ensures that they alias doesn't refer to the same field multiple times).
/// field_path_aliases!{
///     ThePath=(::Boom=>a,b),
/// }
/// fn access_fields<'a,T,O0,O1>(this: &'a mut T)->Option<(&'a mut O0,&'a mut O1)>
/// where
///     // The `FieldsMut= Option<(&'a mut O0,&'a mut O1)>` constraint ensures that
///     // the return type is an optional pair.
///     ThePath: RevGetMultiFieldMut<'a,T,FieldsMut= Option<(&'a mut O0,&'a mut O1)>>
/// {
///     this.fields_mut(ThePath)
/// }
///
/// fn main(){
///     let mut with_0 = WithBoom::Nope;
///     let mut with_1 = WithBoom::Boom{a:"hi", b: &[0,1,2]};
///     assert_eq!( access_fields(&mut with_0), None );
///     assert_eq!( access_fields(&mut with_1), Some((&mut "hi", &mut &[0,1,2][..])) );
///    
///     let mut bomb_0 = Bomb::Nope;
///     let mut bomb_1 = Bomb::Boom{a:"hello", b: &[5,8,13]};
///     assert_eq!( access_fields(&mut bomb_0), None );
///     assert_eq!( access_fields(&mut bomb_1), Some((&mut "hello", &mut &[5,8,13][..])) );
/// }
///
/// ```
///
/// [`RevGetMultiFieldMutOut`]: ./type.RevGetMultiFieldMutOut.html
///
/// [`RevGetMultiFieldMutImpl`]: ./trait.RevGetMultiFieldMutImpl.html
///
/// [`FieldsMut`]: ./trait.RevGetMultiFieldMut.html#associatedtype.FieldsMut
///
/// [`RevGetMultiFieldImpl::rev_get_multi_field`]:
/// ./trait.RevGetMultiFieldImpl.html#tymethod.rev_get_multi_field
pub trait RevGetMultiFieldMut<'a, This: ?Sized + 'a>: RevGetMultiFieldMutImpl<'a, This> {
    /// This is usually a tuple of `Option<&mut _>`s and `&mut _`s.
    type FieldsMut: 'a;

    /// This is usually a tuple of `Option<*mut _>`s and `*mut _`s.
    type FieldsRawMut: 'a;

    /// Gets mutable references to multiple fields from `this`,
    /// usually a tuple of `Option<&mut _>`s and `&mut _`s.
    fn rev_get_multi_field_mut(self, this: &'a mut This) -> Self::FieldsMut;

    /// Gets raw pointers to multiple fields from `this`,
    /// usually a tuple of `Option<*mut _>`s and `*mut _`s.
    ///
    /// # Safety
    ///
    /// `this` must point to a valid instance of `This`,which lives for the `'a` lifetime.
    unsafe fn rev_get_multi_field_raw_mut(self, this: *mut This) -> Self::FieldsRawMut;
}

impl<'a, This, Path> RevGetMultiFieldMut<'a, This> for Path
where
    This: ?Sized + 'a,
    Path: RevGetMultiFieldMutImpl<'a, This>,
{
    type FieldsMut = NormalizeFieldsOut<Path::UnnormFieldsMut>;

    type FieldsRawMut = NormalizeFieldsOut<Path::UnnormFieldsRawMut>;

    fn rev_get_multi_field_mut(self, this: &'a mut This) -> Self::FieldsMut {
        let x = RevGetMultiFieldMutImpl::<'a, This>::rev_get_multi_field_mut_impl(self, this);
        NormalizeFields::normalize_fields(x)
    }

    unsafe fn rev_get_multi_field_raw_mut(self, this: *mut This) -> Self::FieldsRawMut {
        let x = RevGetMultiFieldMutImpl::<'a, This>::rev_get_multi_field_raw_mut_impl(self, this);
        NormalizeFields::normalize_fields(x)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub unsafe trait RevIntoMultiFieldImpl<This>:
    IsMultiFieldPath<PathUniqueness = UniquePaths> + Sized
{
    /// This is usually a tuple of `Result<_, E: IsFieldErr>`s.
    type UnnormIntoFields: NormalizeFields;

    /// Converts `this` into multiple fields by value,
    /// usually a tuple of `Result<_, E: IsFieldErr>`s.
    fn rev_into_multi_field_impl(self, this: This) -> Self::UnnormIntoFields;
}

pub trait RevIntoMultiField<This>: RevIntoMultiFieldImpl<This> {
    /// This is usually a tuple of `Option<T>`s and `T`s.
    type IntoFields;

    /// Converts `this` into multiple fields by value.
    /// usually a tuple of `Option<T>`s and `T`s.
    fn rev_into_multi_field(self, this: This) -> Self::IntoFields;
}

impl<This, Path> RevIntoMultiField<This> for Path
where
    Path: RevIntoMultiFieldImpl<This>,
{
    type IntoFields = NormalizeFieldsOut<Path::UnnormIntoFields>;

    fn rev_into_multi_field(self, this: This) -> Self::IntoFields {
        self.rev_into_multi_field_impl(this).normalize_fields()
    }
}
