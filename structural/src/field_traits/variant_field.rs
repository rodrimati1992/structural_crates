/*!
Traits for getting a field from an enum variant.
*/

use crate::{
    enums::IsVariant,
    field_path::VariantField,
    field_traits::{FieldType, GetFieldRawMutFn},
};

use core_extensions::ConstDefault;

use std_::ptr::NonNull;

///////////////////////////

/// Provides shared access to an enum variant field.
///
/// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
///
/// # Example
///
/// ```
/// use structural::field_traits::GetVariantField;
/// use structural::for_examples::{Variants,WithBar};
/// use structural::{GetFieldExt,TS,fp};
///
/// fn example(this: impl GetVariantField<TS!(Bar),TS!(0),Ty= &'static str>){
///     assert_eq!( this.field_(fp!(::Bar.0)), Some(&"why?") );
///
///     // You can use `fp!(::Foo=>bar,baz)` to access multiple fields inside
///     // an enum variant.
///     assert_eq!( this.fields(fp!(::Bar=>0,0)), Some((&"why?",&"why?")) );
///
///     assert_eq!( this.is_variant(fp!(Bar)), true );
/// }
///
/// example(Variants::Bar("why?"));
/// example(WithBar::Bar("why?"));
///
/// ```
///
/// # Safety
///
/// TODO
pub unsafe trait GetVariantField<V, F>:
    IsVariant<V> + FieldType<VariantField<V, F>>
{
    fn get_vfield_(&self, variant: V, field: F) -> Option<&Self::Ty>;

    #[inline(always)]
    unsafe fn get_vfield_unchecked(&self, variant: V, field: F) -> &Self::Ty {
        match self.get_vfield_(variant, field) {
            Some(x) => x,
            None => crate::utils::unreachable_unchecked(),
        }
    }
}

/// Gets the type of a variant field,
///
/// The `Variant` and `Field` type parameters are expected to be [TStr](../../struct.TStr.html).
///
/// Example: `GetVariantFieldType<This, TS!(Foo), TS!(0)>`
///
/// # Example
///
/// ```
/// use structural::{GetFieldExt,TS,fp};
/// use structural::field_traits::GetVariantFieldType;
/// use structural::for_examples::Variants;
///
/// let this = Variants::Foo(8,13);
///
/// let value: u32= this.into_field(fp!(::Foo.0)).unwrap();
///
/// let value: GetVariantFieldType<Variants, TS!(Foo), TS!(0)>= value;
///
/// assert_eq!( value, 8_u32 );
///
/// ```
pub type GetVariantFieldType<This, Variant, Field> =
    <This as FieldType<VariantField<Variant, Field>>>::Ty;

/// Provides shared and mutable access to an enum variant field.
///
/// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
///
/// # Example
///
/// ```
/// use structural::field_traits::GetVariantFieldMut;
/// use structural::for_examples::{Bomb,WithBoom};
/// use structural::{GetFieldExt,TS,fp};
///
/// fn example<T>(this: &mut T)
/// where
///     T: GetVariantFieldMut<TS!(Boom),TS!(a),Ty= &'static str>+
///        GetVariantFieldMut<TS!(Boom),TS!(b),Ty= &'static [u16]>,
/// {
///     assert_eq!( this.field_(fp!(::Boom.a)), Some(&"why?") );
///     assert_eq!( this.field_mut(fp!(::Boom.a)), Some(&mut "why?") );
///
///     // You can use `fp!(::Foo=>bar,baz)` to access multiple fields inside
///     // an enum variant.
///     assert_eq!(
///         this.fields_mut(fp!(::Boom=>a,b)),
///         Some(( &mut "why?", &mut &[0,1,2][..] )),
///     );
///
///     assert_eq!( this.is_variant(fp!(Boom)), true );
/// }
///
/// example(&mut WithBoom::Boom{ a:"why?", b:&[0,1,2] });
/// example(&mut Bomb::Boom{ a:"why?", b:&[0,1,2] });
///
/// ```
///
/// # Safety
///
/// This has the safety requirements as `GetFieldMut`.
///
/// TODO
pub unsafe trait GetVariantFieldMut<V, F>: GetVariantField<V, F> {
    fn get_vfield_mut_(&mut self, variant: V, field: F) -> Option<&mut Self::Ty>;

    #[inline(always)]
    unsafe fn get_vfield_mut_unchecked(&mut self, variant: V, field: F) -> &mut Self::Ty {
        match self.get_vfield_mut_(variant, field) {
            Some(x) => x,
            None => crate::utils::unreachable_unchecked(),
        }
    }

    ///
    /// This function returns a `NonNull` purely as an optimization detail,
    /// functions that return raw pointers (`*mut _`) are also
    /// expected to return pointers to valid fields.
    ///
    unsafe fn get_vfield_raw_mut_(ptr: *mut (), variant: V, field: F) -> Option<NonNull<Self::Ty>>
    where
        Self: Sized;

    // This function takes only the `F` parameter so that its parameters are
    // the same as `GetFieldMut::get_field_raw_mut`.
    #[inline(always)]
    unsafe fn get_vfield_raw_mut_unchecked(ptr: *mut (), field: F) -> *mut Self::Ty
    where
        Self: Sized,
        V: ConstDefault,
    {
        match Self::get_vfield_raw_mut_(ptr, V::DEFAULT, field) {
            Some(x) => x.as_ptr(),
            None => crate::utils::unreachable_unchecked(),
        }
    }

    fn get_vfield_raw_mut_fn(&self) -> GetVFieldRawMutFn<V, F, Self::Ty>;

    fn get_vfield_raw_mut_unchecked_fn(&self) -> GetFieldRawMutFn<F, Self::Ty>;
}

pub type GetVFieldRawMutFn<VariantName, FieldName, FieldTy> =
    unsafe fn(*mut (), VariantName, FieldName) -> Option<NonNull<FieldTy>>;

/// Provides shared and by-value access to an enum variant field.
///
/// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
///
///
/// # Example
///
/// ```
/// use structural::field_traits::IntoVariantField;
/// use structural::for_examples::{Bomb,WithBoom};
/// use structural::{GetFieldExt,TS,fp};
///
/// fn example<T>(mut this: T)
/// where
///     T: IntoVariantField<TS!(Boom),TS!(a),Ty= &'static str>+
///        IntoVariantField<TS!(Boom),TS!(b),Ty= &'static [u16]>,
/// {
///     assert_eq!( this.is_variant(fp!(Boom)), true );
///     assert_eq!( this.field_(fp!(::Boom.a)), Some(&"Because.") );
///     assert_eq!( this.fields(fp!(::Boom=>a,b)), Some(( &"Because.", &&[13,21][..] )) );
///     assert_eq!( this.into_field(fp!(::Boom.a)), Some("Because.") );
/// }
///
/// example(WithBoom::Boom{ a:"Because.", b:&[13,21] });
/// example(Bomb::Boom{ a:"Because.", b:&[13,21] });
///
/// ```
///
/// # Safety
///
/// This has the safety requirements of `GetFieldMut`.
///
/// TODO
pub unsafe trait IntoVariantField<V, F>: GetVariantField<V, F> {
    fn into_vfield_(self, variant_name: V, field_name: F) -> Option<Self::Ty>
    where
        Self: Sized;

    #[inline(always)]
    unsafe fn into_vfield_unchecked(self, variant_name: V, field_name: F) -> Self::Ty
    where
        Self: Sized,
    {
        match self.into_vfield_(variant_name, field_name) {
            Some(x) => x,
            None => crate::utils::unreachable_unchecked(),
        }
    }

    #[cfg(feature = "alloc")]
    fn box_into_vfield_(
        self: crate::alloc::boxed::Box<Self>,
        variant_name: V,
        field_name: F,
    ) -> Option<Self::Ty>;

    #[cfg(feature = "alloc")]
    #[inline(always)]
    unsafe fn box_into_vfield_unchecked(
        self: crate::alloc::boxed::Box<Self>,
        variant_name: V,
        field_name: F,
    ) -> Self::Ty {
        match self.box_into_vfield_(variant_name, field_name) {
            Some(x) => x,
            None => crate::utils::unreachable_unchecked(),
        }
    }
}

///////////////////////////

/// A bound for shared,mutable,and by-value access to the field `F` inside of the `V` variant.
///
/// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
///
/// # Example
///
/// ```
/// use structural::field_traits::IntoVariantFieldMut;
/// use structural::for_examples::{Bomb,WithBoom};
/// use structural::{GetFieldExt,TS,fp};
///
/// fn example<T>(mut this: T)
/// where
///     T: IntoVariantFieldMut<TS!(Boom),TS!(a),Ty= &'static str>+
///        IntoVariantFieldMut<TS!(Boom),TS!(b),Ty= &'static [u16]>,
/// {
///     assert_eq!( this.is_variant(fp!(Boom)), true );
///     assert_eq!( this.field_(fp!(::Boom.a)), Some(&"Because.") );
///     assert_eq!( this.fields(fp!(::Boom=>a,b)), Some(( &"Because.", &&[13,21][..] )) );
///     assert_eq!( this.field_mut(fp!(::Boom.a)), Some(&mut "Because.") );
///     assert_eq!( this.fields_mut(
///         fp!(::Boom=>a,b)),
///         Some(( &mut "Because.", &mut &[13,21][..] )),
///     );
///     assert_eq!( this.into_field(fp!(::Boom.a)), Some("Because.") );
/// }
///
/// example(WithBoom::Boom{ a:"Because.", b:&[13,21] });
/// example(Bomb::Boom{ a:"Because.", b:&[13,21] });
///
/// ```
pub trait IntoVariantFieldMut<V, F>: GetVariantFieldMut<V, F> + IntoVariantField<V, F> {}

impl<This, V, F> IntoVariantFieldMut<V, F> for This where
    This: ?Sized + GetVariantFieldMut<V, F> + IntoVariantField<V, F>
{
}

///////////////////////////

/// A `GetVariantFieldMut` specifically used for specialization internally.
///
/// # Safety
///
/// This trait has the same safety requirements as `GetVariantFieldMut`.
#[doc(hidden)]
pub unsafe trait SpecGetVariantFieldMut<V, F>: GetVariantField<V, F> {
    unsafe fn get_vfield_raw_mut_inner(
        ptr: *mut (),
        variant_name: V,
        field_name: F,
    ) -> Option<NonNull<Self::Ty>>
    where
        Self: Sized;
}
