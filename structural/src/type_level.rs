/*!
types that represent values.
*/

// pub use core_extensions::type_level_bool::{self,True,False,Boolean};

pub mod cmp;
pub mod collection_traits;

pub mod integer;
#[doc(hidden)]
pub mod list;

pub mod to_value_traits;

#[doc(hidden)]
pub use self::list::{TList, TNil};

// Importing stuff from this module anywhere other than
// `structural_derive` or `structural`  is
// explicitly disallowed,and is likely to break.
#[doc(hidden)]
pub mod _private {

    use crate::field_path::{FieldPath, FieldPath1};
    use crate::std_::marker::PhantomData;
    use crate::type_level::collection_traits::Flatten;

    /// A type-level string,represented as a tuple of type-level bytes.
    ///
    /// This is an implementation detail of structural,
    /// so that it's possible to replace it with `pub struct TStr_<const NAME:&'static str>`
    ///
    /// This cannot be converted to a `&'static str` constant
    /// (if you can figure out a cheap way to do that please create an issue/pull request).
    ///
    pub struct TStr_<T>(pub(crate) PhantomData<T>);

    #[doc(hidden)]
    pub type FieldPath1Str<T> = FieldPath1<TStr_<T>>;

    pub type FlattenedFieldPath<Tuple> = FieldPath<Flatten<Tuple>>;
}
