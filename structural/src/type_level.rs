/*!
types that represent values.
*/
// The types here represent values,
// so warning about "type complexity" for them is silly .
#![allow(clippy::type_complexity)]

pub mod cmp;
pub mod collection_traits;

pub mod integer;
#[doc(hidden)]
pub mod list;

pub mod to_value_traits;

#[doc(hidden)]
pub use self::list::{TList, TNil};
