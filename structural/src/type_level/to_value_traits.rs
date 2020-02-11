//! Traits for converting type-level-value to a value.

use crate::chars;

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
    chars::_0=0,
    chars::_1=1,
    chars::_2=2,
    chars::_3=3,
    chars::_4=4,
    chars::_5=5,
    chars::_6=6,
    chars::_7=7,
    chars::_8=8,
    chars::_9=9,
}
