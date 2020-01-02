mod variant_proxy;

pub use self::variant_proxy::VariantProxy;

/// Queries whether an enum is the `V` variant
pub trait IsVariant<V> {
    fn is_variant_(&self) -> bool;
}
