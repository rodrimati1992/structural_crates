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
        variant_count=TS!(2),
        (Some,strings::Some,kind=regular,fields((IntoVariantFieldMut,0:T,strings::field0)))
        (None,strings::None,kind=regular,fields())
    }
}

///////////////////////////////////////////////////////////////////////////////

_private_impl_getters_for_derive_enum! {
    impl[T,E,] Result<T,E>
    where[]
    {
        enum=Result
        variant_count=TS!(2),
        (Ok,strings::Ok,kind=regular,fields((IntoVariantFieldMut,0:T,strings::field0)))
        (Err,strings::Err,kind=regular,fields((IntoVariantFieldMut,0:E,strings::field0)))
    }
}

///////////////////////////////////////////////////////////////////////////////
