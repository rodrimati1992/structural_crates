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
        drop_fields=just_fields,
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

///////////////////////////////////////////////////////////////////////////////

_private_impl_getters_for_derive_enum! {
    impl[T,E,] Result<T,E>
    where[]
    {
        enum=Result
        drop_fields=just_fields,
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

///////////////////////////////////////////////////////////////////////////////
