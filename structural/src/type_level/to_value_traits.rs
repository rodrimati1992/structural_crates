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
    crate::__0=0,
    crate::__1=1,
    crate::__2=2,
    crate::__3=3,
    crate::__4=4,
    crate::__5=5,
    crate::__6=6,
    crate::__7=7,
    crate::__8=8,
    crate::__9=9,
}
