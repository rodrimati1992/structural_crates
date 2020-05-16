use crate::{
    convert::{EmptyTryFromError, FromStructural, TryFromError, TryFromStructural},
    structural_aliases as sa,
};

tstr_aliases! {
    mod strings {
        Ok,
        Err,
        Some,
        None,
        field0=0,
    }
}

///////////////////////////////////////////////////////////////////////////////

_private_impl_getters_for_derive_enum! {
    impl[T,] Option<T>
    where[]
    {
        enum=Option
        drop_fields={just_fields,}
        variant_count=TS!(2),
        (
            Some,
            strings::Some,
            kind=regular,
            not_public(),
            fields((IntoVariantFieldMut,0:T,dropping(f0, 0),strings::field0))
        )
        (None,strings::None,kind=regular,not_public(),fields())
    }
}

impl<F, T> FromStructural<F> for Option<T>
where
    F: sa::OptionMove_ESI<T>,
{
    fn from_structural(this: F) -> Self {
        switch! {this;
            Some(x)=>Some(x),
            None=>None,
        }
    }
}

impl<F, T> TryFromStructural<F> for Option<T>
where
    F: sa::OptionMove_SI<T>,
{
    type Error = EmptyTryFromError;

    fn try_from_structural(this: F) -> Result<Self, TryFromError<F, Self::Error>> {
        Ok(switch! {this;
            Some(x)=>Some(x),
            None=>None,
            _=>return Err(TryFromError::with_empty_error(this)),
        })
    }
}

///////////////////////////////////////////////////////////////////////////////

_private_impl_getters_for_derive_enum! {
    impl[T,E,] Result<T,E>
    where[]
    {
        enum=Result
        drop_fields={just_fields,}
        variant_count=TS!(2),
        (
            Ok,
            strings::Ok,
            kind=regular,
            not_public(),
            fields((IntoVariantFieldMut,0:T,dropping(f0, 0),strings::field0))
        )
        (
            Err,
            strings::Err,
            kind=regular,
            not_public(),
            fields((IntoVariantFieldMut,0:E,dropping(f0, 0),strings::field0))
        )
    }
}

impl<F, T, E> FromStructural<F> for Result<T, E>
where
    F: sa::ResultMove_ESI<T, E>,
{
    fn from_structural(this: F) -> Self {
        switch! {this;
            Ok(x)=>Ok(x),
            Err(x)=>Err(x),
        }
    }
}

impl<F, T, E> TryFromStructural<F> for Result<T, E>
where
    F: sa::ResultMove_SI<T, E>,
{
    type Error = EmptyTryFromError;

    fn try_from_structural(this: F) -> Result<Self, TryFromError<F, Self::Error>> {
        Ok(switch! {this;
            Ok(x)=>Ok(x),
            Err(x)=>Err(x),
            _=>return Err(TryFromError::with_empty_error(this)),
        })
    }
}

///////////////////////////////////////////////////////////////////////////////
