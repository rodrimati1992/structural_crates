//! Structural aliases for standard library types.
//!
//! ### Array Traits
//!
//! This module re-exports these traits from [arrays_traits](./array_traits/index.html),with:
//!
//! - The `Array*` structural aliases to use any type with accessors from 0
//! until the size of the array,in which all the field types are the same,
//!
//! - The `Array*Variant` structural aliases to use any enum variant with accessors from 0
//! until the size of the array,in which all the field types are the same.
//!
//! ### Tuple Traits
//!
//! This module re-exports these traits from [tuple_traits](./tuple_traits/index.html),with:
//!
//! - The `Tuple*` structural aliases to use any type with accessors from `TS!(0)`
//! until the size of the tuple,in which all field types can be different,
//!
//! - The `Tuple*Variant` structural aliases to use any enum variant with accessors from `TS!(0)`
//! until the size of the tuple,in which all field types can be different.
//!
//!

pub mod array_traits;
pub mod tuple_traits;

pub use self::{array_traits::*, tuple_traits::*};

///////////////////////////////////////////////////////////////////////////////////////////////////

structural_alias! {
    /// Structural alias for `Option`-like enums. With shared,mutable,and by value access.
    ///
    pub trait Option_SI<T>{
        Some(T),
        None,
    }

    /// Structural alias for `Option`-like enums. With shared,and by value access.
    ///
    #[struc(and_exhaustive_enum(name="OptionMove_ESI"))]
    pub trait OptionMove_SI<T>{
        Some(move T),
        None,
    }

    /// Structural alias for `Result`-like enums. With shared,mutable,and by value access.
    ///
    #[struc(and_exhaustive_enum(name="Result_ESI"))]
    pub trait Result_SI<T,E>{
        Ok(T),
        Err(E),
    }

    /// Structural alias for `Result`-like enums. With shared,and by value access.
    ///
    #[struc(and_exhaustive_enum(name="ResultMove_ESI"))]
    pub trait ResultMove_SI<T,E>{
        Ok(move T),
        Err(move E),
    }

    /// Structural alias for `std::ops::Range`-like structs
    ///
    pub trait Range_SI<T>{
        start: T,
        end: T,
    }

    /// Structural alias for `std::ops::Range`-like structs,with only shared access.
    ///
    pub trait RangeRef_SI<T>{
        ref start: T,
        ref end: T,
    }

    /// Structural alias for `std::ops::RangeFrom`-like structs
    ///
    pub trait RangeFrom_SI<T>{
        start: T,
    }

    /// Structural alias for `std::ops::RangeTo`-like structs
    ///
    pub trait RangeTo_SI<T>{
        end: T,
    }

}
