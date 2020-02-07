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
///
/// # Example
///
/// ```rust
/// use structural::enum_traits::IsVariant;
/// use structural::{Structural,fp};
///
/// #[derive(Structural)]
/// # #[struc(no_trait)]
/// enum Enum{
///     Foo,
///     Bar(u8),
///     Boom{x:u32,y:bool},
/// }
///
/// assert_eq!( Enum::Foo.is_variant_(fp!(Foo)), true );
/// assert_eq!( Enum::Foo.is_variant_(fp!(Bar)), false );
/// assert_eq!( Enum::Foo.is_variant_(fp!(Boom)), false );
///
/// let bar=Enum::Bar(0);
/// assert_eq!( bar.is_variant_(fp!(Foo)), false );
/// assert_eq!( bar.is_variant_(fp!(Bar)), true );
/// assert_eq!( bar.is_variant_(fp!(Boom)), false );
///
/// let boom=Enum::Boom{x:0,y:false};
/// assert_eq!( boom.is_variant_(fp!(Foo)), false );
/// assert_eq!( boom.is_variant_(fp!(Bar)), false );
/// assert_eq!( boom.is_variant_(fp!(Boom)), true );
///
/// ```
pub unsafe trait IsVariant<V> {
    fn is_variant_(&self, variant: V) -> bool;
}

/// Enums used for examples in documentation
pub mod example_enums {
    use crate::Structural;
    use std_::cmp::Ordering;

    #[derive(Structural, Copy, Clone, Debug, PartialEq)]
    pub enum Variants {
        Foo(u32, u64),
        Bar(&'static str),
        Baz(#[struc(optional)] Option<Ordering>),
        Boom {
            a: Option<&'static [u8]>,
            b: &'static [u16],
        },
    }
}
