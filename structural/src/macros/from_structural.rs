/// For implementing [`FromStructural`],
/// and delegating the implementation of [`TryFromStructural`] to it.
///
/// [`FromStructural`]: ./trait.FromStructural.html
/// [`TryFromStructural`]: ./trait.TryFromStructural.html
#[macro_export]
macro_rules! z_impl_from_structural {
    (
        impl[ $($impl_params:tt)* ] FromStructural<$from:ident> for $self:ty
        where [ $($where_preds:tt)* ]
        {
            fn from_structural($from_var:ident){
                $($code:tt)*
            }
        }
    ) => {
        impl< $($impl_params)* > $crate::pmr::FromStructural<$from> for $self
        where
            $($where_preds)*
        {
            fn from_structural($from_var: $from) -> Self {
                $($code)*
            }
        }

        impl< $($impl_params)*> $crate::pmr::TryFromStructural<$from> for $self
        where
            $($where_preds)*
        {
            type Error = $crate::pmr::Infallible;

            #[inline(always)]
            fn try_from_structural(
                $from_var: $from,
            ) -> Result<Self, $crate::pmr::TryFromError<$from,$crate::pmr::Infallible>> {
                Ok(<Self as $crate::pmr::FromStructural<$from>>::from_structural($from_var))
            }
        }
    };
}

/// For implementing [`TryFromStructural`],
/// and delegating the implementation of [`FromStructural`] to it.
///
/// The implementation of [`FromStructural`] inherits all the constraints of the
/// `TryFromStructural` impl,
/// and assumes that the error branch is unreachable, panicking if it's not.
///
/// [`FromStructural`]: ./trait.FromStructural.html
/// [`TryFromStructural`]: ./trait.TryFromStructural.html
#[macro_export]
macro_rules! z_impl_try_from_structural_for_enum {
    (
        impl[ $($impl_params:tt)* ] TryFromStructural<$from:ident> for $self:ty
        where [ $($where_preds:tt)* ]
        {
            type Error= $err:ty;
            fn try_from_structural($from_var:ident){
                $($code:tt)*
            }
        }

        FromStructural
        where [ $($from_where_preds:tt)* ]
    ) => {
        impl< $($impl_params)* > $crate::pmr::FromStructural<$from> for $self
        where
            $($where_preds)*
            $($from_where_preds)*
        {
            fn from_structural(from_var: $from) -> Self {
                let res=<Self as $crate::pmr::TryFromStructural<$from>>::try_from_structural(
                    from_var
                );
                match res {
                    Ok(x) => x,
                    Err(e) => unreachable!(
                        "expected type to implement `TryFromStructural::try_from_structural`
                         such that it doesn't return an error in `FromStructural`.\n\
                        type:\n\t{}\n\
                        error:\n\t{}\n",
                        std::any::type_name::<Self>(),
                        e.error,
                    ),
                }
            }
        }

        impl< $($impl_params)*> $crate::pmr::TryFromStructural<$from> for $self
        where
            $($where_preds)*
        {
            type Error = $err;

            #[inline(always)]
            fn try_from_structural(
                $from_var: $from,
            ) -> Result<Self, $crate::pmr::TryFromError<$from,$err>> {
                $($code)*
            }
        }
    };
}
