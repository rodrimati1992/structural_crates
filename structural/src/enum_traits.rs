/// Queries whether an enum is the `V` variant
pub trait IsVariant<V> {
    fn is_variant_(&self) -> bool;
}
