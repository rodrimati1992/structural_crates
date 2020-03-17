/*!
Contains traits for accessing multiple fields at once.
*/
#![allow(non_snake_case)]

use crate::{
    field_path::{FieldPathSet, IsMultiFieldPath, NestedFieldPathSet, UniquePaths},
    field_traits::{
        IsFieldErr, NormalizeFields, NormalizeFieldsOut, RevGetFieldImpl, RevGetFieldMutImpl,
    },
};

#[allow(unused_imports)]
use core_extensions::SelfOps;

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
/// use structural::field_traits::{FailedAccess, RevGetMultiFieldImpl};
/// use structural::field_path::{AliasedPaths, IsMultiFieldPath};
/// use structural::GetFieldExt;
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
/// [`GetFieldExt::fields`](../../trait.GetFieldExt.html#method.fields),
/// and [`GetFieldExt::cloned_fields`](../../trait.GetFieldExt.html#method.cloned_fields)
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
/// use structural::{GetFieldExt, FP, fp};
/// use structural::field_traits::RevGetMultiField;
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
/// use structural::{GetFieldExt, FP};
/// use structural::field_path::IntoAliasingOut;
/// use structural::field_traits::{RevGetMultiField,RevGetMultiFieldOut};
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
/// use structural::field_traits::{FailedAccess, RevGetMultiFieldMutImpl};
/// use structural::field_path::{IsMultiFieldPath, UniquePaths};
/// use structural::GetFieldExt;
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
/// [`GetFieldExt::fields_mut`](../../trait.GetFieldExt.html#method.fields_mut)
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
/// use structural::{GetFieldExt, FP, fp};
/// use structural::field_traits::RevGetMultiFieldMut;
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
/// use structural::{GetFieldExt, field_path_aliases};
/// use structural::field_traits::{RevGetMultiFieldMut,RevGetMultiFieldMutOut};
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

macro_rules! impl_get_multi_field {
    ( $(($fpath:ident $err:ident $fty:ident))* ) => (
        impl<'a,This:?Sized,$($fpath,$err,$fty,)* U>
            RevGetMultiFieldImpl<'a,This>
        for FieldPathSet<($($fpath,)*),U>
        where
            This:'a,
            $(
                $fpath:RevGetFieldImpl<'a, This, Ty=$fty, Err=$err >,
                $fty:'a,
                $err:IsFieldErr,
                Result<&'a $fty,$err>: NormalizeFields,
            )*
        {
            type UnnormFields=(
                $(
                    Result<&'a $fty,$err>,
                )*
            );

            #[allow(unused_variables)]
            fn rev_get_multi_field_impl(self,this:&'a This)-> Self::UnnormFields {
                let ($($fpath,)*)=self.into_paths();
                (
                    $(
                        $fpath.rev_get_field(this),
                    )*
                )
            }
        }

        unsafe impl<'a,This:?Sized,$($fpath,$err,$fty,)*>
            RevGetMultiFieldMutImpl<'a,This>
        for FieldPathSet<($($fpath,)*),UniquePaths>
        where
            This:'a,
            $(
                $fpath: RevGetFieldMutImpl<'a,This, Ty=$fty, Err=$err >,
                Result<&'a mut $fty,$err>: NormalizeFields,
                Result<*mut $fty,$err>: NormalizeFields,
                $fty:'a,
                $err:IsFieldErr,
            )*
        {
            type UnnormFieldsMut=(
                $(
                    Result<&'a mut $fty,$err>,
                )*
            );
            type UnnormFieldsRawMut=(
                $(
                    Result<*mut $fty,$err>,
                )*
            );

            #[allow(unused_unsafe,unused_variables)]
            fn rev_get_multi_field_mut_impl(
                self,
                this:&'a mut This,
            )-> Self::UnnormFieldsMut {
                unsafe{
                    let ($($fpath,)*)={
                        #[allow(unused_variables)]
                        let ($($fpath,)*)=self.into_paths();
                        (
                            $(
                                $fpath.rev_get_field_raw_mut(this),
                            )*
                        )
                    };

                    (
                        $(
                            match $fpath {
                                Ok($fpath)=>Ok(&mut *$fpath),
                                Err(e)=>Err(e),
                            },
                        )*
                    )
                }
            }

            #[allow(unused_variables)]
            unsafe fn rev_get_multi_field_raw_mut_impl(
                self,
                this:*mut This,
            )-> Self::UnnormFieldsRawMut {
                let ($($fpath,)*)=self.into_paths();
                (
                    $(
                        $fpath.rev_get_field_raw_mut(this),
                    )*
                )
            }
        }
    )
}

impl_get_multi_field! {}
impl_get_multi_field! {
    (F0 E0 T0)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5) (F6 E6 T6)
}
impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5) (F6 E6 T6) (F7 E7 T7)
}

impl_get_multi_field! {
    (F0 E0 T0) (F1 E1 T1) (F2 E2 T2) (F3 E3 T3) (F4 E4 T4) (F5 E5 T5) (F6 E6 T6) (F7 E7 T7)
    (F8 E8 T8) (F9 E9 T9) (F10 E10 T10) (F11 E11 T11) (F12 E12 T12)
}

////////////////////////////////////////////////////////////////////////////////

impl<'a, F, S, U, This, Mid, OutTy, OutErr> RevGetMultiFieldImpl<'a, This>
    for NestedFieldPathSet<F, S, U>
where
    F: RevGetFieldImpl<'a, This, Ty = Mid, Err = OutErr>,
    FieldPathSet<S, U>: RevGetMultiFieldImpl<'a, Mid, UnnormFields = OutTy>,
    OutErr: IsFieldErr,
    This: 'a + ?Sized,
    Mid: 'a + ?Sized,
    OutTy: 'a + NormalizeFields,
    NestedFieldPathSetOutput<OutTy, OutErr>: 'a + NormalizeFields,
{
    type UnnormFields = NestedFieldPathSetOutput<OutTy, OutErr>;

    #[inline(always)]
    fn rev_get_multi_field_impl(self, this: &'a This) -> NestedFieldPathSetOutput<OutTy, OutErr> {
        let (nested, set) = self.into_inner();
        nested
            .rev_get_field(this)
            .map({
                #[inline(always)]
                |mid| set.rev_get_multi_field_impl(mid)
            })
            .piped(NestedFieldPathSetOutput)
    }
}

unsafe impl<'a, F, S, This, Mid, OutTy, OutRawTy, OutErr> RevGetMultiFieldMutImpl<'a, This>
    for NestedFieldPathSet<F, S, UniquePaths>
where
    F: RevGetFieldMutImpl<'a, This, Ty = Mid, Err = OutErr>,
    FieldPathSet<S, UniquePaths>:
        RevGetMultiFieldMutImpl<'a, Mid, UnnormFieldsMut = OutTy, UnnormFieldsRawMut = OutRawTy>,
    This: 'a + ?Sized,
    OutErr: IsFieldErr,
    Mid: 'a + ?Sized,
    OutTy: 'a + NormalizeFields,
    OutRawTy: 'a + NormalizeFields,
    NestedFieldPathSetOutput<OutTy, OutErr>: 'a + NormalizeFields,
    NestedFieldPathSetOutput<OutRawTy, OutErr>: 'a + NormalizeFields,
{
    type UnnormFieldsMut = NestedFieldPathSetOutput<OutTy, OutErr>;
    type UnnormFieldsRawMut = NestedFieldPathSetOutput<OutRawTy, OutErr>;

    #[inline(always)]
    fn rev_get_multi_field_mut_impl(
        self,
        this: &'a mut This,
    ) -> NestedFieldPathSetOutput<OutTy, OutErr> {
        let (nested, set) = self.into_inner();
        nested
            .rev_get_field_mut(this)
            .map({
                #[inline(always)]
                |mid| set.rev_get_multi_field_mut_impl(mid)
            })
            .piped(NestedFieldPathSetOutput)
    }

    #[inline(always)]
    unsafe fn rev_get_multi_field_raw_mut_impl(
        self,
        this: *mut This,
    ) -> NestedFieldPathSetOutput<OutRawTy, OutErr> {
        let (nested, set) = self.into_inner();
        nested
            .rev_get_field_raw_mut(this)
            .map({
                #[inline(always)]
                |mid| set.rev_get_multi_field_raw_mut_impl(mid)
            })
            .piped(NestedFieldPathSetOutput)
    }
}

/// The return type of NestedFieldPathSet's `Rev*MultiField*Impl` impls,
///
/// This implements NormalizeFields so that a the wrapped `Result<TupleType,Err>`
/// also normalizes the tuple type itself,
/// turning each individual `Result<T,E>` in the tuple into `T` or `Option<T>`.
pub struct NestedFieldPathSetOutput<T, E>(pub Result<T, E>);

impl<T, E> NormalizeFields for NestedFieldPathSetOutput<T, E>
where
    T: NormalizeFields,
    Result<T::Output, E>: NormalizeFields,
{
    type Output = NormalizeFieldsOut<Result<T::Output, E>>;

    #[inline(always)]
    fn normalize_fields(self) -> Self::Output {
        match self.0 {
            Ok(x) => Ok(x.normalize_fields()),
            Err(e) => Err(e),
        }
        .normalize_fields()
    }
}
