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
