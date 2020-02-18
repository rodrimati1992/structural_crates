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
