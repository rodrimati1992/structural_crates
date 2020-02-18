//! Traits for converting type-level-value to a value.

///////////////////////////////////////////////////////////////////////////////

/// Converts this type-level value to a usize.
pub trait ToUsize {
    const USIZE: usize;
}

///////////////////////////////////////////////////////////////////////////////

mod sealed {
    pub trait Sealed {}
}

use self::sealed::Sealed;

/// Converts this type-level digit to a u8.
#[doc(hidden)]
pub trait ToDigit: Sealed {
    const DIGIT: u8;
}

macro_rules! impl_to_digit {
    ( $($self:ty=$value:literal,)* ) => (
        $(
            impl Sealed for $self {}
            impl ToDigit for $self {
                const DIGIT:u8=$value;
            }
        )*
    )
}

impl_to_digit! {
    crate::p::_0=0,
    crate::p::_1=1,
    crate::p::_2=2,
    crate::p::_3=3,
    crate::p::_4=4,
    crate::p::_5=5,
    crate::p::_6=6,
    crate::p::_7=7,
    crate::p::_8=8,
    crate::p::_9=9,
}
