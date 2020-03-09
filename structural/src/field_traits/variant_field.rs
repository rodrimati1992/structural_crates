/*!
Traits for getting a field from an enum variant.
*/

use crate::{
    enums::IsVariant,
    field_path::{UncheckedVariantField, VariantField},
    field_traits::{
        FieldType, NonOptField, OptGetField, OptGetFieldMut, OptIntoField, OptionalField,
    },
    GetFieldImpl, GetFieldMutImpl, IntoFieldImpl,
};

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

/// Marker trait for requiring the unchecked enum shared accessor impls to be correct.
///
/// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
///
/// This trait is designed for generic implementations,
/// the [(Opt)](./trait.OptGetVariantField.html)
/// [GetVariantField](./trait.GetVariantField.html)
/// traits are preferrable for bounds.
///
/// # Safety
///
/// The `GetFieldImpl<VariantField<V, F>, UncheckedVariantField<V, F>>` impl
/// for this type must return the `F` field from the `V` variant,
/// never returning (unwinding counts as returning) if this is not the `V` variant.
pub unsafe trait GetVariantFieldImpl<V, F>:
    IsVariant<V> + GetFieldImpl<VariantField<V, F>, UncheckedVariantField<V, F>>
{
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

/// Marker trait for requiring the unchecked enum shared+mutable accessor impls to be correct.
///
/// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
///
/// This trait is designed for generic implementations,
/// the [(Opt)](./trait.OptGetVariantFieldMut.html)
/// [GetVariantFieldMut](./trait.GetVariantFieldMut.html)
/// traits are preferrable for bounds.
///
/// # Safety
///
/// This has the safety requirements as `GetFieldMutImpl`.
///
/// The `GetFieldMutImpl<VariantField<V, F>, UncheckedVariantField<V, F>>` impl
/// for this type must return the `F` field from the `V` variant,
/// never returning (unwinding counts as returning) if this is not the `V` variant.
pub unsafe trait GetVariantFieldMutImpl<V, F>:
    GetVariantFieldImpl<V, F> + GetFieldMutImpl<VariantField<V, F>, UncheckedVariantField<V, F>>
{
}

/// Marker trait for requiring the unchecked enum shared+by-value accessor impls to be correct.
///
/// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
///
/// This trait is designed for generic implementations,
/// the [(Opt)](./trait.OptIntoVariantField.html)
/// [IntoVariantField](./trait.IntoVariantField.html)/
/// [(Opt)](./trait.OptIntoVariantFieldMut.html)
/// [IntoVariantFieldMut](./trait.IntoVariantFieldMut.html)
/// traits are preferrable for bounds.
///
/// # Safety
///
/// This has the safety requirements of `GetFieldMutImpl`.
///
/// The `IntoFieldImpl<VariantField<V, F>, UncheckedVariantField<V, F>>` impl
/// for this type must return the `F` field from the `V` variant,
/// never returning (unwinding counts as returning) if this is not the `V` variant.
pub unsafe trait IntoVariantFieldImpl<V, F>:
    GetVariantFieldImpl<V, F> + IntoFieldImpl<VariantField<V, F>, UncheckedVariantField<V, F>>
{
}

///////////////////////////

declare_trait_alias! {
    /// A bound for shared access to the field `F` inside of the `V` variant.
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
    pub trait GetVariantField<V,F>=
        OptGetField<VariantField<V, F>> +
        GetVariantFieldImpl<V, F, Err= NonOptField> +
}

declare_trait_alias! {
    /// A bound for mutable access to the field `F` inside of the `V` variant.
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
    pub trait GetVariantFieldMut<V, F>=
        OptGetFieldMut<VariantField<V, F>> +
        GetVariantFieldMutImpl<V, F, Err= NonOptField> +
}

declare_trait_alias! {
    /// A bound for by-value access to the field `F` inside of the `V` variant.
    ///
    /// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
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
    pub trait IntoVariantField<V, F>=
        OptIntoField<VariantField<V, F>> +
        IntoVariantFieldImpl<V, F, Err= NonOptField> +
}

declare_trait_alias! {
    /// A bound for mutable and by-value access to the field `F` inside of the `V` variant.
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
    pub trait IntoVariantFieldMut<V, F>=
        GetVariantFieldMut<V, F>+
        IntoVariantField<V, F>+
}

///////////////////////////

declare_trait_alias! {
    /// A bound for optional shared access to the field `F` inside of the `V` variant.
    ///
    /// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
    ///
    /// # Example
    ///
    /// ```
    /// use structural::field_traits::{GetVariantField,OptGetVariantField};
    /// use structural::for_examples::{EnumOptA,EnumOptFlying};
    /// use structural::{GetFieldExt,tstr_aliases,fp};
    ///
    /// tstr_aliases!{
    ///     mod strs{Limbs,legs,hands}
    /// }
    ///
    /// fn example(
    ///     this: impl
    ///         // `legs` is a `#[struc(optional)] legs:Option<usize>` field.
    ///         OptGetVariantField<strs::Limbs, strs::legs,Ty= usize> +
    ///         // `hands` is an `Option<usize>` field,without a `#[struc(optional)]` attribute.
    ///         GetVariantField<strs::Limbs, strs::hands,Ty= Option<usize> >
    /// ){
    ///     assert_eq!( this.field_(fp!(::Limbs.legs)), Some(&8) );
    ///     assert_eq!( this.field_(fp!(::Limbs.hands)), Some(&Some(13)) );
    ///
    ///     // You can use `fp!(::Foo=>bar,baz)` to access multiple fields inside
    ///     // an enum variant.
    ///     assert_eq!( this.fields(fp!(::Limbs=>legs,hands)), Some(( Some(&8),&Some(13) )) );
    ///
    ///     assert_eq!( this.is_variant(fp!(Limbs)), true );
    /// }
    ///
    /// example(EnumOptA::Limbs{
    ///     legs: Some(8),
    ///     hands: Some(13),
    /// });
    /// example(EnumOptFlying::Limbs{
    ///     legs: Some(8),
    ///     hands: Some(13),
    ///     noodles: 9001,
    /// });
    ///
    /// ```
    pub trait OptGetVariantField<V, F>=
        OptGetField<VariantField<V, F>> +
        GetVariantFieldImpl<V, F, Err= OptionalField> +
}

declare_trait_alias! {
    /// A bound for optional mutable access to the field `F` inside of the `V` variant.
    ///
    /// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
    ///
    /// # Example
    ///
    /// ```
    /// use structural::field_traits::{GetVariantFieldMut,OptGetVariantFieldMut};
    /// use structural::for_examples::{EnumOptA,EnumOptFlying};
    /// use structural::{GetFieldExt,tstr_aliases,fp};
    ///
    /// tstr_aliases!{
    ///     mod strs{Limbs,legs,hands}
    /// }
    ///
    /// fn example<T>(mut this:T)
    /// where
    ///     // `legs` is a `#[struc(optional)] legs:Option<usize>` field.
    ///     T: OptGetVariantFieldMut<strs::Limbs, strs::legs,Ty= usize >,
    ///     // `hands` is an `Option<usize>` field,without a `#[struc(optional)]` attribute.
    ///     T: GetVariantFieldMut<strs::Limbs, strs::hands,Ty= Option<usize> >,
    /// {
    ///     assert_eq!( this.field_(fp!(::Limbs.legs)), Some(&8) );
    ///     assert_eq!( this.field_(fp!(::Limbs.hands)), Some(&Some(13)) );
    ///
    ///     assert_eq!( this.field_mut(fp!(::Limbs.legs)), Some(&mut 8) );
    ///     assert_eq!( this.field_mut(fp!(::Limbs.hands)), Some(&mut Some(13)) );
    ///
    ///     // You can use `fp!(::Foo=>bar,baz)` to access multiple fields inside
    ///     // an enum variant.
    ///     assert_eq!(
    ///         this.fields(fp!(::Limbs=>legs,hands)),
    ///         Some(( Some(&8),&Some(13) )),
    ///     );
    ///     assert_eq!(
    ///         this.fields_mut(fp!(::Limbs=>legs,hands)),
    ///         Some(( Some(&mut 8),&mut Some(13) )),
    ///     );
    ///
    ///     assert_eq!( this.is_variant(fp!(Limbs)), true );
    /// }
    ///
    /// example(EnumOptA::Limbs{
    ///     legs: Some(8),
    ///     hands: Some(13),
    /// });
    /// example(EnumOptFlying::Limbs{
    ///     legs: Some(8),
    ///     hands: Some(13),
    ///     noodles: 9001,
    /// });
    ///
    /// ```
    pub trait OptGetVariantFieldMut<V, F>=
        OptGetFieldMut<VariantField<V, F>> +
        GetVariantFieldMutImpl<V, F, Err= OptionalField> +
}

declare_trait_alias! {
    /// A bound for optional by-value access to the field `F` inside of the `V` variant.
    ///
    /// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
    ///
    /// # Example
    ///
    /// ```
    /// use structural::field_traits::{IntoVariantField,OptIntoVariantField};
    /// use structural::for_examples::{EnumOptA,EnumOptFlying};
    /// use structural::{GetFieldExt,tstr_aliases,fp};
    ///
    /// tstr_aliases!{
    ///     mod strs{Limbs,legs,hands}
    /// }
    ///
    /// fn example<T>(this:T)
    /// where
    ///     T: Copy,
    ///     // `legs` is a `#[struc(optional)] legs:Option<usize>` field.
    ///     T: OptIntoVariantField<strs::Limbs, strs::legs,Ty= usize >,
    ///     // `hands` is an `Option<usize>` field,without a `#[struc(optional)]` attribute.
    ///     T: IntoVariantField<strs::Limbs, strs::hands,Ty= Option<usize> >,
    /// {
    ///     assert_eq!( this.field_(fp!(::Limbs.legs)), Some(&8) );
    ///     assert_eq!( this.field_(fp!(::Limbs.hands)), Some(&Some(13)) );
    ///
    ///     assert_eq!( this.into_field(fp!(::Limbs.legs)), Some(8) );
    ///     assert_eq!( this.into_field(fp!(::Limbs.hands)), Some(Some(13)) );
    ///
    ///     // You can use `fp!(::Foo=>bar,baz)` to access multiple fields inside
    ///     // an enum variant.
    ///     assert_eq!(
    ///         this.fields(fp!(::Limbs=>legs,hands)),
    ///         Some(( Some(&8),&Some(13) )),
    ///     );
    ///
    ///     assert_eq!( this.is_variant(fp!(Limbs)), true );
    /// }
    ///
    /// example(EnumOptA::Limbs{
    ///     legs: Some(8),
    ///     hands: Some(13),
    /// });
    /// example(EnumOptFlying::Limbs{
    ///     legs: Some(8),
    ///     hands: Some(13),
    ///     noodles: 9001,
    /// });
    ///
    /// ```
    pub trait OptIntoVariantField<V, F>=
        OptIntoField<VariantField<V, F>> +
        IntoVariantFieldImpl<V, F, Err= OptionalField> +
}

declare_trait_alias! {
    /// A bound for optional mutable and by-value access to the
    /// field `F` inside of the `V` variant.
    ///
    /// The `V` and `F` type parameters are expected to be [TStr](../../struct.TStr.html).
    ///
    /// # Example
    ///
    /// ```
    /// use structural::field_traits::{IntoVariantFieldMut,OptIntoVariantFieldMut};
    /// use structural::for_examples::{EnumOptA,EnumOptFlying};
    /// use structural::{GetFieldExt,tstr_aliases,fp};
    ///
    /// tstr_aliases!{
    ///     mod strs{Limbs,legs,hands}
    /// }
    ///
    /// fn example<T>(mut this:T)
    /// where
    ///     T: Copy,
    ///     // `legs` is a `#[struc(optional)] legs:Option<usize>` field.
    ///     T: OptIntoVariantFieldMut<strs::Limbs, strs::legs,Ty= usize >,
    ///     // `hands` is an `Option<usize>` field,without a `#[struc(optional)]` attribute.
    ///     T: IntoVariantFieldMut<strs::Limbs, strs::hands,Ty= Option<usize> >,
    /// {
    ///     assert_eq!( this.field_(fp!(::Limbs.legs)), Some(&8) );
    ///     assert_eq!( this.field_(fp!(::Limbs.hands)), Some(&Some(13)) );
    ///
    ///     assert_eq!( this.field_mut(fp!(::Limbs.legs)), Some(&mut 8) );
    ///     assert_eq!( this.field_mut(fp!(::Limbs.hands)), Some(&mut Some(13)) );
    ///
    ///     assert_eq!( this.into_field(fp!(::Limbs.legs)), Some(8) );
    ///     assert_eq!( this.into_field(fp!(::Limbs.hands)), Some(Some(13)) );
    ///
    ///     // You can use `fp!(::Foo=>bar,baz)` to access multiple fields inside
    ///     // an enum variant.
    ///     assert_eq!(
    ///         this.fields(fp!(::Limbs=>legs,hands)),
    ///         Some(( Some(&8),&Some(13) )),
    ///     );
    ///     assert_eq!(
    ///         this.fields_mut(fp!(::Limbs=>legs,hands)),
    ///         Some(( Some(&mut 8),&mut Some(13) )),
    ///     );
    ///
    ///     assert_eq!( this.is_variant(fp!(Limbs)), true );
    /// }
    ///
    /// example(EnumOptA::Limbs{
    ///     legs: Some(8),
    ///     hands: Some(13),
    /// });
    /// example(EnumOptFlying::Limbs{
    ///     legs: Some(8),
    ///     hands: Some(13),
    ///     noodles: 9001,
    /// });
    ///
    /// ```
    pub trait OptIntoVariantFieldMut<V, F>=
        OptGetVariantFieldMut<V, F>+
        OptIntoVariantField<V, F>+
}
