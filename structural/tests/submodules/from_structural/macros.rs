//! Tests that use macros to implement FromStructural traits.

use structural::{convert::TryFromError, switch, Structural, StructuralExt};

use std::fmt::{self, Display};

#[test]
fn impl_try_from_structural_for_enum() {
    {
        assert_eq!(Variants::Foo.into_struc::<MatchesAll>(), MatchesAll::Foo);
        assert_eq!(Variants::Bar.into_struc::<MatchesAll>(), MatchesAll::Bar);
        assert_eq!(Variants::Baz.into_struc::<MatchesAll>(), MatchesAll::Baz);

        assert_eq!(
            MoreVariants::Foo.try_into_struc::<MatchesAll>().unwrap(),
            MatchesAll::Foo
        );
        assert_eq!(
            MoreVariants::Bar.try_into_struc::<MatchesAll>().unwrap(),
            MatchesAll::Bar
        );
        assert_eq!(
            MoreVariants::Baz.try_into_struc::<MatchesAll>().unwrap(),
            MatchesAll::Baz
        );
        assert_eq!(
            MoreVariants::Qux
                .try_into_struc::<MatchesAll>()
                .unwrap_err(),
            TryFromError::new(MoreVariants::Qux, MyError)
        );
    }

    {
        assert_eq!(Variants::Foo.into_struc::<MatchesFew>(), MatchesFew::Foo);

        assert_eq!(
            Variants::Foo.try_into_struc::<MatchesFew>().unwrap(),
            MatchesFew::Foo
        );
        assert_eq!(
            Variants::Bar.try_into_struc::<MatchesFew>().unwrap(),
            MatchesFew::Bar
        );
        assert_eq!(
            Variants::Baz.try_into_struc::<MatchesFew>().unwrap_err(),
            TryFromError::new(Variants::Baz, MyError)
        );
    }
}

#[test]
#[should_panic]
fn impl_try_from_structural_for_enum_panics() {
    Variants::Baz.into_struc::<MatchesFew>();
}

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub enum Variants {
    Foo,
    Bar,
    Baz,
}

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
#[struc(no_trait)]
pub enum MoreVariants {
    Foo,
    Bar,
    Baz,
    Qux,
}

#[derive(Structural, Debug, Copy, Clone, PartialEq)]
pub enum MatchesAll {
    Foo,
    Bar,
    Baz,
}

#[allow(dead_code)]
#[derive(Structural, Debug, Copy, Clone, PartialEq)]
pub enum MatchesFew {
    Foo,
    Bar,
    Baz,
}

structural::z_impl_try_from_structural_for_enum! {
    impl[F] TryFromStructural<F> for MatchesAll
    where[ F: MatchesAll_SI, ]
    {
     type Error = MyError;
        fn try_from_structural(this){
            switch!{this;
                Foo => Ok(Self::Foo),
                Bar => Ok(Self::Bar),
                Baz => Ok(Self::Baz),
                _ => Err(TryFromError::new(this, MyError)),
            }
        }
    }
    FromStructural
    where[ F: MatchesAll_ESI, ]
}

structural::z_impl_try_from_structural_for_enum! {
    impl[F] TryFromStructural<F> for MatchesFew
    where[ F: MatchesFew_SI, ]
    {
     type Error = MyError;
        fn try_from_structural(this){
            switch!{this;
                Foo => Ok(Self::Foo),
                Bar => Ok(Self::Bar),
                _ => Err(TryFromError::new(this, MyError)),
            }
        }
    }
    FromStructural
    where[ F: MatchesFew_ESI, ]
}

#[derive(Debug, Clone, PartialEq)]
pub struct MyError;

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt("MyError", f)
    }
}

impl std::error::Error for MyError {}
