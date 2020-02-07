use crate::type_level::IsTStr;

use std_::marker::PhantomData;

/// For querying the ammount of variants of an enum.
///
/// # Safety
///
/// Count must be a type-level string with the amount of enum variants written in decimal.
/// For example:`TStr!(9)` would be used for an enum with 9 variant.
///
/// Specifying fewer variants than the enum actually has may result in undefined behavior
/// when the enum is matched in the `switch` macro.
///
pub unsafe trait VariantCount {
    /// This is a type-level string(eg:`TStr!(3)`)
    /// representing the amount of variants of the enum.
    ///
    /// This is a type instead of a constant so that it can be a supertrait of
    /// dyn-compatible traits.
    type Count: IsTStr;
}

/// Queries the ammount of variants of `This`.
///
/// This evaluates to a TStr,like `TStr!("hello")`
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
