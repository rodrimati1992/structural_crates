/*!
Traits used to get a field from an enum variant.

# Safety

All of the functions in the traits traits from this module are
unsafe to call because the enum must be the variant specified by
the `V` generic parameter.
*/

use crate::{
    enum_traits::IsVariant,
    field_traits::{NonOptField, OptGetField, OptGetFieldMut, OptIntoField, OptionalField},
    type_level::{FieldPath1, UncheckedVariantField, VariantFieldPath},
    GetFieldImpl, GetFieldMutImpl, IntoFieldImpl,
};

#[cfg(feature = "alloc")]
pub use crate::alloc::boxed::Box;

macro_rules! declare_trait_alias {
    (
        $(#[$attr:meta])*
        $vis:vis trait $trait_name:ident< $vari:ident, $field:ident >=
        $($supertraits:tt)*
    ) => (
        $(#[$attr])*
        $vis trait $trait_name< $vari, $field >:$($supertraits)* {}

        impl<This,$vari,$field> $trait_name<$vari,$field> for This
        where
            This:?Sized+$($supertraits)*
        {}
    )
}

///////////////////////////

/// Gets a shared reference to the `F` field  from the `V` variant
///
/// # Safety
///
/// TODO
pub unsafe trait GetVariantFieldImpl<V, F>:
    IsVariant<FieldPath1<V>> + GetFieldImpl<VariantFieldPath<V, F>, UncheckedVariantField<V, F>>
{
}

/// Gets a mutable reference to the `F` field  from the `V` variant
///
/// # Safety
///
/// This has the safety requirements as `GetFieldMutImpl`.
///
/// TODO: requirements for the VariantField traits.
pub unsafe trait GetVariantFieldMutImpl<V, F>:
    GetVariantFieldImpl<V, F> + GetFieldMutImpl<VariantFieldPath<V, F>, UncheckedVariantField<V, F>>
{
}

/// Converts this into the `F` field  from the `V` variant
///
/// # Safety
///
/// This has the safety requirements as `GetFieldMutImpl`.
///
/// TODO: requirements for the VariantField traits.
pub unsafe trait IntoVariantFieldImpl<V, F>:
    GetVariantFieldImpl<V, F> + IntoFieldImpl<VariantFieldPath<V, F>, UncheckedVariantField<V, F>>
{
}

///////////////////////////

declare_trait_alias! {
    pub trait GetVariantField<V,F>=
        OptGetField<VariantFieldPath<V, F>> +
        GetVariantFieldImpl<V, F, Err= NonOptField> +
}

declare_trait_alias! {
    pub trait GetVariantFieldMut<V, F>=
        OptGetFieldMut<VariantFieldPath<V, F>> +
        GetVariantFieldMutImpl<V, F, Err= NonOptField> +
}

declare_trait_alias! {
    pub trait IntoVariantField<V, F>=
        OptIntoField<VariantFieldPath<V, F>> +
        IntoVariantFieldImpl<V, F, Err= NonOptField> +
}

declare_trait_alias! {
    pub trait IntoVariantFieldMut<V, F>=
        GetVariantFieldMut<V, F>+
        IntoVariantField<V, F>+
}

///////////////////////////

declare_trait_alias! {
    pub trait OptGetVariantField<V, F>=
        OptGetField<VariantFieldPath<V, F>> +
        GetVariantFieldImpl<V, F, Err= OptionalField> +
}

declare_trait_alias! {
    pub trait OptGetVariantFieldMut<V, F>=
        OptGetFieldMut<VariantFieldPath<V, F>> +
        GetVariantFieldMutImpl<V, F, Err= OptionalField> +
}

declare_trait_alias! {
    pub trait OptIntoVariantField<V, F>=
        OptIntoField<VariantFieldPath<V, F>> +
        IntoVariantFieldImpl<V, F, Err= OptionalField> +
}

declare_trait_alias! {
    pub trait OptIntoVariantFieldMut<V, F>=
        OptGetVariantFieldMut<V, F>+
        OptIntoVariantField<V, F>+
}
