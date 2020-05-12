use crate::{
    field::{DropFields, GetField, GetVariantField, IntoField, IntoVariantField, MovedOutFields},
    TStr,
};

use core_extensions::ConstDefault;

/// Wrapper that emulates by-value access to fields by cloning them.
///
/// This allows using types that only provide shared access to fields
/// (implementing `GetField`/`GetVariantField`)
/// to be passed to functions expecting by-value access to them
/// (requiring `IntoField`/`IntoVariantField`),
/// by cloning those fields.
///
/// # Example
///
/// ```rust
/// use structural::{FieldCloner, Structural, StructuralExt, fp, structural_alias};
///
/// # fn main(){
///
/// let expected = ("what".to_string(), vec![0,1,2]);
///
/// let this = TheStruct{foo: "what".to_string(), bar: vec![0,1,2]};
///
/// // This doesn't compile,because `TheStruct` only provides shared access to the fields,
/// // implementing `GetField` to access both the `foo` and `bar` fields.
/// //
/// // assert_eq!( into_foo_bar(this), expected.clone() );
///
/// assert_eq!( into_foo_bar(FieldCloner(this.clone())), expected.clone() );
///
/// assert_eq!( into_foo_bar(FieldCloner(&this)), expected.clone() );
///
/// assert_eq!( into_foo_bar(FieldCloner(&&&&&this)), expected.clone() );
///
/// # }
///
/// fn into_foo_bar<P, T>(this: P)-> (String, Vec<T>)
/// where
///     P: TypeMove<String, Vec<T>>,
/// {
///     this.into_fields(fp!(foo, bar))
/// }
///
/// #[derive(Structural,Clone)]
/// // Makes this struct only implement `GetField` for the fields,
/// // providing shared access to them.
/// #[struc(access="ref")]
/// struct TheStruct<T, U>{
///     pub foo: T,
///     pub bar: U,
/// }
///
/// structural::structural_alias!{
///     // The same fields as TheStruct, with shared and by-value access to the fields.
///     //
///     // This trait isn't implemented by `TheStruct` because it only
///     // provides shared access to the fields.
///     trait TypeMove<T, U>{
///         move foo: T,
///         move bar: U,
///     }
/// }
/// ```
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FieldCloner<T>(pub T);

impl<T> FieldCloner<T> {
    /// Turns a `&FieldCloner<T>` into a `FieldCloner<&T>`.
    #[inline(always)]
    pub fn as_ref(&self) -> FieldCloner<&T> {
        FieldCloner(&self.0)
    }

    /// Turns a `&mut FieldCloner<T>` into a `FieldCloner<&mut T>`.
    #[inline(always)]
    pub fn as_mut(&mut self) -> FieldCloner<&mut T> {
        FieldCloner(&mut self.0)
    }

    /// Transforms the wrapped value with the `func` function.
    #[inline(always)]
    pub fn map<F, U>(self, f: F) -> FieldCloner<U>
    where
        F: FnOnce(T) -> U,
    {
        FieldCloner(f(self.0))
    }

    /// Calls `func` with `self`,rewrapping its return value in a `FieldCloner<U>`
    #[inline(always)]
    pub fn then<F, U>(self, f: F) -> FieldCloner<U>
    where
        F: FnOnce(Self) -> U,
    {
        FieldCloner(f(self))
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<T> ConstDefault for FieldCloner<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = FieldCloner(T::DEFAULT);
}

////////////////////////////////////////////////////////////////////////////////

unsafe impl<T, P> IntoField<P> for FieldCloner<T>
where
    T: GetField<P>,
    T::Ty: Clone,
{
    #[inline(always)]
    fn into_field_(self, path: P) -> Self::Ty {
        self.0.get_field_(path).clone()
    }

    #[inline(always)]
    unsafe fn move_out_field_(&mut self, path: P, _: &mut MovedOutFields) -> Self::Ty {
        self.0.get_field_(path).clone()
    }
}

unsafe impl<T, V, F> IntoVariantField<TStr<V>, F> for FieldCloner<T>
where
    T: GetVariantField<TStr<V>, F>,
    T::Ty: Clone,
{
    #[inline(always)]
    fn into_vfield_(self, variant_name: TStr<V>, field_name: F) -> Option<Self::Ty> {
        match self.0.get_vfield_(variant_name, field_name) {
            Some(x) => Some(x.clone()),
            _ => None,
        }
    }

    #[inline(always)]
    unsafe fn move_out_vfield_(
        &mut self,
        variant_name: TStr<V>,
        field_name: F,
        _: &mut MovedOutFields,
    ) -> Option<Self::Ty> {
        match self.0.get_vfield_(variant_name, field_name) {
            Some(x) => Some(x.clone()),
            _ => None,
        }
    }
}

unsafe impl<T> DropFields for FieldCloner<T> {
    #[inline(always)]
    fn pre_move(&mut self) {}

    #[inline(always)]
    unsafe fn drop_fields(&mut self, _: MovedOutFields) {
        // No field was moved out, so we can just drop Self.
        std::ptr::drop_in_place(self);
    }
}

unsafe_delegate_structural_with! {
    impl[T,] FieldCloner<T>
    where[]

    self_ident=this;
    specialization_params(Sized);
    delegating_to_type=T;

    GetField { &this.0 }

    GetFieldMut{
        &mut this.0
    }
    as_delegating_raw{
        this as *mut T
    }

    FromStructural {
        constructor = FieldCloner;
    }
}
