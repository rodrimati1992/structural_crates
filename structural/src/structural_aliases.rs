structural_alias! {
    /// Structural alias for `Option`-like enums
    pub trait Option_SI<T>{
        Some(T),
        None,
    }

    /// Structural alias for `Result`-like enums
    pub trait Result_SI<T,E>{
        Ok(T),
        Err(E),
    }

    /// Structural alias for `std::ops::Range`-like structs
    pub trait Range_SI<T>{
        start: T,
        end: T,
    }

    /// Structural alias for `std::ops::Range`-like structs,with only shared access.
    pub trait RangeRef_SI<T>{
        ref start: T,
        ref end: T,
    }

    /// Structural alias for `std::ops::RangeFrom`-like structs
    pub trait RangeFrom_SI<T>{
        start: T,
    }

    /// Structural alias for `std::ops::RangeTo`-like structs
    pub trait RangeTo_SI<T>{
        end: T,
    }

}
