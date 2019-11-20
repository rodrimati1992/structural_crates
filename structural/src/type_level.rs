/*!
Contains types representing values.
*/

// pub use core_extensions::type_level_bool::{self,True,False,Boolean};

pub mod collection_traits;
pub mod ident;
pub mod list;

#[doc(hidden)]
pub use self::ident::FieldPathString;

pub use self::{
    ident::{
        IsFieldPath,IsFieldPathSet,
        FieldPath,FieldPath1,FieldPathSet,
        UniquePaths,AliasedPaths,
        TString,
    },
    list::{TList,TNil},
};



////////////////////////////////////////////////////////////////////////////////


#[doc(hidden)]
pub mod proc_macro_aliases{
    use crate::type_level::*;
    use crate::type_level::collection_traits::*;

    #[doc(hidden)]
    pub type FlattenedFieldPath<Tuple>=
        FieldPath<Flatten<Tuple>>;

}

