/*!
Contains types representing values.
*/

// pub use core_extensions::type_level_bool::{self,True,False,Boolean};

pub mod cmp;
pub mod collection_traits;

#[doc(hidden)]
#[deprecated]
pub mod ident {
    pub use crate::type_level::field_path::*;
}

pub mod field_path;

pub mod integer;
#[doc(hidden)]
pub mod list;

#[doc(hidden)]
pub use self::{
    field_path::FieldPath1,
    list::{TList, TNil},
};

pub use self::field_path::{
    AliasedPaths, FieldPath, FieldPathSet, IsFieldPath, IsFieldPathSet, UniquePaths,
};

// Importing stuff from this module anywhere other than
// `structural_derive` or `structural`  is
// explicitly disallowed,and is likely to break.
#[doc(hidden)]
pub mod _private {

    use crate::std_::marker::PhantomData;
    use crate::type_level::collection_traits::Flatten;
    use crate::type_level::FieldPath;

    /// A type-level string,represented as a tuple of type-level bytes.
    ///
    /// This is an implementation detail of structural,
    /// so that it's possible to replace it with `pub struct TString<const NAME:&'static str>`
    ///
    /// This cannot be converted to a `&'static str` constant
    /// (if you can figure out a cheap way to do that please create an issue/pull request).
    ///
    pub struct TString<T>(pub(crate) PhantomData<T>);

    pub type FlattenedFieldPath<Tuple> = FieldPath<Flatten<Tuple>>;
}
