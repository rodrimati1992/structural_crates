/*!
Contains types representing values.
*/

// pub use core_extensions::type_level_bool::{self,True,False,Boolean};

pub mod collection_traits;
pub mod ident;
pub mod list;

pub use self::{
    ident::{FieldPath,FieldPaths,MutableAccess,SharedAccess,TString},
    list::{TList,TNil},
};
