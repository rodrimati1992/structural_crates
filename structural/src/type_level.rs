/*!
Contains types representing values.
*/

// pub use core_extensions::type_level_bool::{self,True,False,Boolean};

pub mod collection_traits;
pub mod ident;
#[doc(hidden)]
pub mod list;

#[doc(hidden)]
pub use self::{
    ident::{FieldPathString,FieldPath1,TString},
    list::{TList,TNil},
};

pub use self::{
    ident::{
        IsFieldPath,IsFieldPathSet,
        FieldPath,FieldPathSet,
        UniquePaths,AliasedPaths,
    },
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

