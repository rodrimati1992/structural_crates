/*!
Contains types representing values.
*/

// pub use core_extensions::type_level_bool::{self,True,False,Boolean};

pub mod ident;
pub mod list;

pub use self::{
    ident::{TStringSet,TString},
    list::{TList,TNil},
};
