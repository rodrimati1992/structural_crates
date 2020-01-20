mod enum_ext;
pub mod variant_count;
mod variant_proxy;

pub use self::{
    enum_ext::EnumExt,
    variant_count::{VariantCount, VariantCountOut},
    variant_proxy::VariantProxy,
};

/// Queries whether an enum is the `V` variant
///
/// # Safety
///
/// An implementation of `IsVariant<FP!(Foo)>`
/// must only return true if the enum is the `Foo` variant.
///
/// Implementing this trait wrong will result in undefined behavior with
/// the VariantProxy for the `V` variant.
pub unsafe trait IsVariant<V> {
    fn is_variant_(&self, variant: V) -> bool;
}
