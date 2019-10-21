extern crate self as structural;



#[macro_use]
mod macros;

pub mod mut_ref;
pub mod field_traits;
pub mod type_level;
pub mod utils;


#[doc(hidden)]
pub use crate::type_level::ident as chars;

pub use crate::{
    field_traits::{GetField,GetFieldMut,IntoField,GetFieldExt},
};



/// Reexports from the `core_extensions` crate.
pub mod reexports{
    pub use core_extensions::{MarkerType,SelfOps};
}

// Reexports for the proc macros in structural_derive.
#[doc(hidden)]
pub mod proc_macro_reexports{
    pub use crate::type_level::ident::*;
    pub use core_extensions::MarkerType;
}
