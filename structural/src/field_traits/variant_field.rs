/*!
Traits for getting a field from an enum variant.
*/

use crate::{
    enums::IsVariant,
    field_path::{FieldPath1, UncheckedVariantField, VariantFieldPath},
    field_traits::{
        FieldType, NonOptField, OptGetField, OptGetFieldMut, OptIntoField, OptionalField,
    },
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
/// This trait is designed for generic implementations,
/// the (Opt)GetVariantField traits are preferrable for bounds
/// (so long as the optionality of fields isn't abstracted over).
///
/// # Safety
///
/// The `GetFieldImpl<VariantFieldPath<V, F>, UncheckedVariantField<V, F>>` impl
/// for this type must return the `F` field from the `V` variant,
/// calling `std::hint::unrechable_unchecked` if this is not the `V` variant.
pub unsafe trait GetVariantFieldImpl<V, F>:
    IsVariant<FieldPath1<V>> + GetFieldImpl<VariantFieldPath<V, F>, UncheckedVariantField<V, F>>
{
}

/// Gets the type of a variant field,
///
/// Example(since 1.40): `GetVariantFieldType<This, TStr!(Foo), TStr!(0)>`
///
/// Example(before 1.40): `GetVariantFieldType<This, TStr!(F o o), TStr!(0)>`
pub type GetVariantFieldType<This, Variant, Field> =
    <This as FieldType<VariantFieldPath<Variant, Field>>>::Ty;

/// Gets a mutable reference to the `F` field  from the `V` variant
///
/// This trait is designed for generic implementations,
/// the (Opt)GetVariantFieldMut traits are preferrable for bounds
/// (so long as the optionality of fields isn't abstracted over).
///
/// # Safety
///
/// This has the safety requirements as `GetFieldMutImpl`.
///
/// The `GetFieldMutImpl<VariantFieldPath<V, F>, UncheckedVariantField<V, F>>` impl
/// for this type must return the `F` field from the `V` variant,
/// calling `std::hint::unrechable_unchecked` if this is not the `V` variant.
pub unsafe trait GetVariantFieldMutImpl<V, F>:
    GetVariantFieldImpl<V, F> + GetFieldMutImpl<VariantFieldPath<V, F>, UncheckedVariantField<V, F>>
{
}

/// Converts this into the `F` field  from the `V` variant
///
/// This trait is designed for generic implementations,
/// the (Opt)IntoVariantField traits are preferrable for bounds
/// (so long as the optionality of fields isn't abstracted over).
///
/// # Safety
///
/// This has the safety requirements as `GetFieldMutImpl`.
///
/// The `IntoFieldImpl<VariantFieldPath<V, F>, UncheckedVariantField<V, F>>` impl
/// for this type must return the `F` field from the `V` variant,
/// calling `std::hint::unrechable_unchecked` if this is not the `V` variant.
pub unsafe trait IntoVariantFieldImpl<V, F>:
    GetVariantFieldImpl<V, F> + IntoFieldImpl<VariantFieldPath<V, F>, UncheckedVariantField<V, F>>
{
}

///////////////////////////

declare_trait_alias! {
    /// A bound for shared access to the field `F` inside of the `V` variant.
    ///
    /// This takes TStr as parameters,eg: `GetVariantField<TStr!(Foo),TStr!(x)>`
    pub trait GetVariantField<V,F>=
        OptGetField<VariantFieldPath<V, F>> +
        GetVariantFieldImpl<V, F, Err= NonOptField> +
}

declare_trait_alias! {
    /// A bound for mutable access to the field `F` inside of the `V` variant.
    ///
    /// This takes TStr as parameters,eg: `GetVariantFieldMut<TStr!(Bar),TStr!(y)>`
    pub trait GetVariantFieldMut<V, F>=
        OptGetFieldMut<VariantFieldPath<V, F>> +
        GetVariantFieldMutImpl<V, F, Err= NonOptField> +
}

declare_trait_alias! {
    /// A bound for by-value access to the field `F` inside of the `V` variant.
    ///
    /// This takes TStr as parameters,eg: `IntoVariantField<TStr!(Baz),TStr!(z)>`
    pub trait IntoVariantField<V, F>=
        OptIntoField<VariantFieldPath<V, F>> +
        IntoVariantFieldImpl<V, F, Err= NonOptField> +
}

declare_trait_alias! {
    /// A bound for mutable and by-value access to the field `F` inside of the `V` variant.
    ///
    /// This takes TStr as parameters,eg: `IntoVariantFieldMut<TStr!(Boom),TStr!(dynamite)>`
    pub trait IntoVariantFieldMut<V, F>=
        GetVariantFieldMut<V, F>+
        IntoVariantField<V, F>+
}

///////////////////////////

declare_trait_alias! {
    /// A bound for optional shared access to the field `F` inside of the `V` variant.
    ///
    /// This takes TStr as parameters,eg: `OptGetVariantField<TStr!(Illegal),TStr!(errors)>`
    pub trait OptGetVariantField<V, F>=
        OptGetField<VariantFieldPath<V, F>> +
        GetVariantFieldImpl<V, F, Err= OptionalField> +
}

declare_trait_alias! {
    /// A bound for optional mutable access to the field `F` inside of the `V` variant.
    ///
    /// This takes TStr as parameters,eg: `OptGetVariantFieldMut<TStr!(Other),TStr!(0)>`
    pub trait OptGetVariantFieldMut<V, F>=
        OptGetFieldMut<VariantFieldPath<V, F>> +
        GetVariantFieldMutImpl<V, F, Err= OptionalField> +
}

declare_trait_alias! {
    /// A bound for optional by-value access to the field `F` inside of the `V` variant.
    ///
    /// This takes TStr as parameters,eg: `OptIntoVariantField<TStr!(Enum),TStr!(variants)>`
    pub trait OptIntoVariantField<V, F>=
        OptIntoField<VariantFieldPath<V, F>> +
        IntoVariantFieldImpl<V, F, Err= OptionalField> +
}

declare_trait_alias! {
    /// A bound for optional mutable and by-value access to the
    /// field `F` inside of the `V` variant.
    ///
    /// This takes TStr as parameters,eg: `OptIntoVariantFieldMut<TStr!(Struct),TStr!(value)>`
    pub trait OptIntoVariantFieldMut<V, F>=
        OptGetVariantFieldMut<V, F>+
        OptIntoVariantField<V, F>+
}
