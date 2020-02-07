mod enum_ext;
#[doc(hidden)]
pub mod variant_count;
mod variant_proxy;

pub use self::{
    enum_ext::EnumExt,
    variant_count::{VariantCount, VariantCountOut},
    variant_proxy::VariantProxy,
};

/// Queries whether an enum is some variant (the `V` type parameter)
///
/// Example bounds: `IsVariant<FP!(Foo)>`,`IsVariant<FP!(Bar)>`.
///
/// # Safety
///
/// An implementation of `IsVariant<FP!(Foo)>`
/// must only return true if the enum is the `Foo` variant
/// (`Foo` is just an example,it applies to all variants).
///
/// Undefined behavior will happen if this trait is implemented wrong.<br>
/// For example:A `VariantProxy<Self,V>` has accessor impls that assume that the enum
/// is the variant that `V` stands for,causing UB if the enum is not that variant.
///
/// # Example
///
/// ```rust
/// use structural::enums::IsVariant;
/// use structural::{FP,Structural,fp};
///
/// assertions(Enum::Foo, Enum::Bar(0), Enum::Boom{x:0,y:false});
///
/// assertions(Enum2::Foo, Enum2::Bar, Enum2::Boom);
///
/// fn assertions<T>(foo:T, bar:T, boom:T)
/// where
///     // From Rust 1.40 this is equivalent to:
///     // T: IsVariant<FP!(Foo)> + IsVariant<FP!(Bar)> + IsVariant<FP!(Boom)>
///     T: IsVariant<FP!(F o o)> + IsVariant<FP!(B a r)> + IsVariant<FP!(B o o m)>
/// {
///     assert_eq!( foo.is_variant_(fp!(Foo)), true );
///     assert_eq!( foo.is_variant_(fp!(Bar)), false );
///     assert_eq!( foo.is_variant_(fp!(Boom)), false );
///    
///     assert_eq!( bar.is_variant_(fp!(Foo)), false );
///     assert_eq!( bar.is_variant_(fp!(Bar)), true );
///     assert_eq!( bar.is_variant_(fp!(Boom)), false );
///    
///     assert_eq!( boom.is_variant_(fp!(Foo)), false );
///     assert_eq!( boom.is_variant_(fp!(Bar)), false );
///     assert_eq!( boom.is_variant_(fp!(Boom)), true );
/// }
///
/// #[derive(Structural)]
/// # #[struc(no_trait)]
/// enum Enum{
///     Foo,
///     Bar(u8),
///     Boom{x:u32,y:bool},
/// }
///
/// #[derive(Structural)]
/// # #[struc(no_trait)]
/// enum Enum2{
///     Foo,
///     Bar,
///     Boom,
/// }
///
///
/// ```
pub unsafe trait IsVariant<V> {
    /// Checks whether this enum is the variant that `V` stands for.
    fn is_variant_(&self, variant: V) -> bool;
}

/// Enums used for examples in documentation
pub mod example_enums {
    use crate::Structural;
    use std_::cmp::Ordering;

    #[derive(Structural, Copy, Clone, Debug, PartialEq)]
    #[struc(no_trait)]
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
