/*!
Enum related traits and types.

The `*VariantField*` traits are declared in
[the `field` module](../field/index.html),
you can use `*VariantField*` traits as bounds,
and then call `GetFieldExt` methods to access fields inside enum variants.
*/

mod enum_ext;
#[doc(hidden)]
pub mod variant_count;
mod variant_proxy;

pub use self::{
    enum_ext::EnumExt,
    variant_count::{VariantCount, VariantCountOut},
    variant_proxy::VariantProxy,
};

use crate::path::AssertTStrParam;

/// Queries whether an enum is some variant (the `V` type parameter)
///
/// Example bounds: `IsVariant<TS!(Foo)>`,`IsVariant<TS!(Bar)>`.
///
/// # Safety
///
/// An implementation of `IsVariant<TS!(Foo)>`
/// must only return true if the enum is the `Foo` variant
/// (`Foo` is just an example,it applies to all variants).
///
/// Undefined behavior will happen if this trait return `true`,
/// while the accessor for a field of that variant returns `None`.
///
///
/// # Example
///
/// ```rust
/// use structural::enums::IsVariant;
/// use structural::{TS,Structural,fp};
///
/// assertions(Enum::Foo, Enum::Bar(0), Enum::Boom{x:0,y:false});
///
/// assertions(Enum2::Foo, Enum2::Bar, Enum2::Boom);
///
/// fn assertions<T>(foo:T, bar:T, boom:T)
/// where
///     T: IsVariant<TS!(Foo)> + IsVariant<TS!(Bar)> + IsVariant<TS!(Boom)>
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
pub unsafe trait IsVariant<V>: AssertTStrParam<V> {
    /// Checks whether this enum is the variant that `V` stands for.
    fn is_variant_(&self, variant: V) -> bool;
}
