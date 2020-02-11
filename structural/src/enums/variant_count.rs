use crate::field_path::IsTStr;

use std_::marker::PhantomData;

/// For querying the amount of variants of an enum.
///
/// # Safety
///
/// Count must be a type-level string with the amount of enum variants written in decimal.
/// For example:`TStr!(9)` would be used for an enum with 9 variant.
///
/// Specifying fewer variants than the enum actually has may result in undefined behavior
/// when the enum is matched in the `switch` macro.
///
/// # Example
///
/// This example demonstrates using `VariantCount` as a bound to
/// restrict a pre-existing structural alias.
///
/// ```rust
/// use structural::{Structural,TStr,structural_alias,switch};
/// use structural::enums::VariantCount;
///
/// # fn main(){
/// {
///     // Enum
///
///     assert_eq!( nonexhaustive(Enum::Foo), Some(0) );
///     assert_eq!( exhaustive_a(Enum::Foo), 0 );
///     assert_eq!( exhaustive_b(Enum::Foo), 0 );
///    
///     let bar=Enum::Bar(0);
///     assert_eq!( nonexhaustive(bar.clone()), Some(1) );
///     assert_eq!( exhaustive_a(bar.clone()), 1 );
///     assert_eq!( exhaustive_b(bar), 1 );
///    
///     let baz=Enum::Baz{wha:"whoah".into()};
///     assert_eq!( nonexhaustive(baz.clone()), Some(2) );
///     assert_eq!( exhaustive_a(baz.clone()), 2 );
///     assert_eq!( exhaustive_b(baz), 2 );
/// }
/// {   
///     // HyperEnum:
///     // This enum has a superset of the variants required by `Ternary`.
///     // The commented out lines below don't compile
///     // because the `exhaustive_*` functions require the enum to only have
///     // the `Foo`,`Bar`,and `Baz` variants
///
///     assert_eq!( nonexhaustive(HyperEnum::Foo), Some(0) );
///     // assert_eq!( exhaustive_a(HyperEnum::Foo), 0 );
///     // assert_eq!( exhaustive_b(HyperEnum::Foo), 0 );
///    
///     assert_eq!( nonexhaustive(HyperEnum::Bar), Some(1) );
///     // assert_eq!( exhaustive_a(HyperEnum::Bar), 1 );
///     // assert_eq!( exhaustive_b(HyperEnum::Bar), 1 );
///    
///     assert_eq!( nonexhaustive(HyperEnum::Baz), Some(2) );
///     // assert_eq!( exhaustive_a(HyperEnum::Baz), 2 );
///     // assert_eq!( exhaustive_b(HyperEnum::Baz), 2 );
///
///     assert_eq!( nonexhaustive(HyperEnum::Boom), None );
/// }
/// # }
///
/// // This function returns the index of the current variant of the enum,
/// // but because `Ternary` is a nonexhaustive structural trait,
/// // it returns None to handle the case where the enum is
/// // none of the three variants.
/// //
/// fn nonexhaustive<T>(this: T)->Option<u8>
/// where
///     T: Ternary,
/// {
///     // The VariantCount bound allow this switch to be exhaustive.
///     switch!{this;
///         Foo=>Some(0),
///         Bar=>Some(1),
///         Baz=>Some(2),
///         // This branch is required,
///         // because `Ternary` doesn't require the enum to have exactly 3 variants
///         _=>None,
///     }
/// }
///
/// // This function returns the index of the current variant of the enum,
/// fn exhaustive_a<T>(this: T)->u8
/// where
///     T: Ternary + VariantCount<Count=TStr!(3)>,
/// {
///     // The VariantCount bound allow this switch to be exhaustive.
///     switch!{this;
///         Foo=>0,
///         Bar=>1,
///         Baz=>2,
///     }
/// }
///
/// fn exhaustive_b<T>(this: T)->u8
/// where
///     // `TernaryExhaustive` is equivalent to `Ternary + VariantCount<Count=TStr!(3)>`.
///     //
///     // You would use a `+ VariantCount<Count=_>` bound if all of thse happen:
///     // - The structural alias came from somewhere else.
///     // - It's a nonexhaustive structural alias.
///     // - You don't want to do declare another alias,like `TernarySuper`.
///     T: TernaryExhaustive
/// {
///     exhaustive_a(this)
/// }
///
/// structural_alias!{
///     // `#[struc(and_exhaustive_enum(...))]` generates a subtrait with
///     //`VariantCount<Count=TStr!($variant_count)>` as an additional bound
///     // (the `$variant_count` stands for the number of variants in the structural alias)
///     #[struc(and_exhaustive_enum(name="TernaryExhaustive"))]
///     pub trait Ternary{
///         Foo,
///         Bar,
///         Baz,
///     }
///
///     pub trait TernarySuper: Ternary + VariantCount<Count=TStr!(3)> {}
/// }
///
/// #[derive(Structural,Clone)]
/// # #[struc(no_trait)]
/// enum Enum{
///     Foo,
///     Bar(u32),
///     Baz{wha:String},
/// }
///
/// #[derive(Structural,Clone)]
/// # #[struc(no_trait)]
/// enum HyperEnum{
///     Foo,
///     Bar,
///     Baz,
///     Boom,
/// }
///
///
/// ```
pub unsafe trait VariantCount {
    /// This is a type-level string(eg:`TStr!(3)`)
    /// representing the amount of variants of the enum.
    ///
    /// This is a type instead of a `&'static str` constant so that
    /// it can be a supertrait of dyn-compatible traits.
    type Count: IsTStr;
}

/// Queries the amount of variants of `This`.
///
/// This evaluates to a TStr,like `TStr!(9)`
///
/// # Example
///
/// This demonstrates `VariantCountOut` by
/// making a function that requires two enums to have the same number of variants.
///
/// ```rust
/// use structural::Structural;
/// use structural::enums::{VariantCount,VariantCountOut};
///
/// same_sized_enums( &Enum1::Foo, &Enum2::A );
///
/// // This does not compile because `Enum1` has 3 variants and `Result` has 2 variants.
/// // same_sized_enums( &Enum1::Foo, &Result::<(),()>::Ok(()) );
///
/// // This function asserts at compile-time that both enums have the same number of variants.
/// fn same_sized_enums<L,R>(left:&L,right:&R)
/// where
///     L:VariantCount,
///     R:VariantCount<Count=VariantCountOut<L>>,
/// {}
///
///     
/// #[derive(Structural)]
/// # #[struc(no_trait)]
/// enum Enum1{
///     Foo,
///     Bar,
///     Baz,
/// }
///     
/// #[derive(Structural)]
/// # #[struc(no_trait)]
/// enum Enum2{
///     A,
///     B,
///     C,
/// }
/// ```
pub type VariantCountOut<This> = <This as VariantCount>::Count;

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////

mod sealed_gvch {
    pub trait Sealed {}
}

/// Exploits autoref-based specialization,
/// based on whether the `T` in this `PhantomData<T>` implements `VariantCount` .
///
/// This trait's single method returns either:
///
/// - `ExpectedVariantCount<_>`: If `T` implements `VariantCount`
///
/// . `ExpectedDefaultBranch`: If `T` does not implement `VariantCount`
///
pub trait GetVariantCountHack<Found>: sealed_gvch::Sealed {
    type MaybeCount;

    /// Constructs the `Self::MaybeCount`.
    ///
    /// This method name is long to prevent accidental colisions.
    fn structural_get_variant_count(self, found: Found) -> Self::MaybeCount;
}

impl<T> sealed_gvch::Sealed for PhantomData<T> where T: VariantCount {}

impl<T, Found> GetVariantCountHack<Found> for PhantomData<T>
where
    T: VariantCount,
{
    type MaybeCount = ExpectedVariantCount<Self, T::Count, Found>;

    #[inline(always)]
    fn structural_get_variant_count(self, _found: Found) -> Self::MaybeCount {
        ExpectedVariantCount(PhantomData)
    }
}

impl<T> sealed_gvch::Sealed for &'_ PhantomData<T> {}

impl<T, Found> GetVariantCountHack<Found> for &'_ PhantomData<T> {
    type MaybeCount = ExpectedDefaultBranch<Self>;

    #[inline(always)]
    fn structural_get_variant_count(self, _found: Found) -> Self::MaybeCount {
        ExpectedDefaultBranch(PhantomData)
    }
}

/// Helper type for the `switch` macro,
/// to assert that all variants have been matched
/// when the user does not write a default branch.
pub struct ExpectedVariantCount<This, Expected, Found>(PhantomData<(This, Expected, Found)>);

impl<This, Expected, Found> Copy for ExpectedVariantCount<This, Expected, Found> {}

impl<This, Expected, Found> Clone for ExpectedVariantCount<This, Expected, Found> {
    fn clone(&self) -> Self {
        *self
    }
}

/////////////////////////////////////

/// Helper type for the `switch` macro,
pub struct ExpectedDefaultBranch<This>(PhantomData<This>);

impl<This> Copy for ExpectedDefaultBranch<This> {}

impl<This> Clone for ExpectedDefaultBranch<This> {
    fn clone(&self) -> Self {
        *self
    }
}

/////////////////////////////////////

/// This is a hack used to produce moderately readable error messages for the `switch` macro.
#[allow(non_camel_case_types)]
mod messages {
    use super::*;

    pub struct switch_that_matches_on_all_variants<Count>(PhantomData<Count>);

    pub struct switch_that_does_not_match_on_all_variants<Count>(PhantomData<Count>);

    pub struct switch_with_a_default_branch<T>(PhantomData<T>);

    pub struct switch_without_a_default_branch<T>(PhantomData<T>);
}

//////////////////////////////////////////////////////////////////////////

pub trait GetElseValue<T>: Sized {
    type ExpectedMsg;
    type FoundMsg;

    /// Ensures that an enum has been matched exhaustively,
    /// by asserting that the actual amount of variants with the
    /// amount of variants matched in `switch!` arms.
    ///
    /// # Safety
    ///
    /// This function must only be called in the default branch after
    /// all enum variants of `This` have been matched.
    ///
    /// The value passed as the `rhs` parameter must be the amount of variants
    /// that were matched (represented as a field path,eg:`fp!(9)`).
    ///
    /// This function must be unreachable at runtime,
    /// since the `Expected` type parameter is the amount of variants in the enum,
    /// as well as the amount of variants matched.
    #[inline(always)]
    unsafe fn get_else_values(self) -> (Self::ExpectedMsg, Self::FoundMsg, T) {
        unreachable!()
    }
}

///////////////////////

impl<This, T, Expected> GetElseValue<T> for ExpectedVariantCount<This, Expected, Expected> {
    type ExpectedMsg = messages::switch_that_matches_on_all_variants<Expected>;
    type FoundMsg = messages::switch_that_matches_on_all_variants<Expected>;

    #[inline(always)]
    unsafe fn get_else_values(self) -> (Self::ExpectedMsg, Self::FoundMsg, T) {
        std::hint::unreachable_unchecked()
    }
}

impl<This, T, Expected, Found> GetElseValue<T> for &'_ ExpectedVariantCount<This, Expected, Found> {
    type ExpectedMsg = messages::switch_that_matches_on_all_variants<Expected>;
    type FoundMsg = messages::switch_that_does_not_match_on_all_variants<Found>;
}

///////////////////////

impl<This, T> GetElseValue<T> for ExpectedDefaultBranch<This> {
    type ExpectedMsg = messages::switch_with_a_default_branch<()>;
    type FoundMsg = messages::switch_without_a_default_branch<()>;
}
