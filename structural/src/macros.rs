#[macro_use]
mod delegate_structural;

#[macro_use]
mod drop_fields;

#[macro_use]
mod enum_derivation;

#[macro_use]
mod field_paths;

#[macro_use]
mod impl_struct;

#[macro_use]
mod list;

#[macro_use]
mod make_struct;

#[macro_use]
mod structural_alias;

#[macro_use]
mod struct_derivation;

#[macro_use]
mod switch;

#[macro_use]
mod tstr_macros;

#[macro_use]
mod type_level_internal;

#[macro_export]
#[doc(hidden)]
macro_rules! default_if {
    (
        $(#[$attr:meta])*
        cfg($($cfg_attr:tt)*)
        $($default_impl:tt)*
    ) => (
        #[cfg($($cfg_attr)*)]
        $(#[$attr])*
        default $($default_impl)*

        #[cfg(not($($cfg_attr)*))]
        $(#[$attr])*
        $($default_impl)*
    )
}

// Implements the Structural traits
#[doc(hidden)]
#[macro_export]
macro_rules! _private_impl_structural{
    (
        $(#[doc=$docs:literal])*
        impl[$($typarams:tt)*] Structural for $self_:ty
        where[$($where_:tt)*]
    )=>{
        $(#[doc=$docs])*
        impl<$($typarams)*> $crate::Structural for $self_
        where $($where_)*
        {}
    };
}

/// Asserts that the `$left` bounds are the same as the `$right` bounds
#[cfg(feature = "testing")]
#[macro_export]
#[doc(hidden)]
macro_rules! assert_equal_bounds {
    (
        trait $trait_:ident $([$($trait_params:tt)*])? ,
        ( $($left:tt)* ),
        ( $($right:tt)* )$(,)?
        $( where[ $($where_preds:tt)* ] )?
    ) => (
        trait $trait_< $($($trait_params)*)? >: $($left)*
        where
            $($($where_preds)*)?
        {
            const DUMMY:()=();

            fn foo<_T>()
            where
                _T: ?Sized+$($left)*,
                $($($where_preds)*)?;
        }

        impl<$($($trait_params)*)? _This> $trait_<$($($trait_params)*)?> for _This
        where
            _This: ?Sized+$($right)*,
            $($($where_preds)*)?
        {
            fn foo<_T>()
            where
                _T:?Sized+$($right)*,
                $($($where_preds)*)?
            {}
        }

    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! try_fe {
    ( $expr:expr ) => {
        match $expr {
            Ok(x) => x,
            Err(e) => return Err($crate::field::IntoFieldErr::into_field_err(e)),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! map_of {
    ( $expr:expr ) => {
        match $expr {
            Ok(x) => Ok(x),
            Err(_) => Err($crate::field::FailedAccess),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! ok_or_of {
    ( $expr:expr ) => {
        match $expr {
            Some(x) => Ok(x),
            None => Err($crate::field::FailedAccess),
        }
    };
}

/// Using this to test implemented traits.
#[cfg(feature = "testing")]
#[doc(hidden)]
#[macro_export]
macro_rules! declare_querying_trait {
    (
        trait $trait_name:ident $([$($params:tt)*])?
        $( implements[ $($supertraits:tt)* ] )?
        $( where[ $($where_:tt)* ] )?
        fn $impls_fn:ident;
    ) => (
        trait $trait_name<$($($params)*)?>:Sized{
            type Impls: $crate::pmr::Boolean;
            fn $impls_fn(self)->Self::Impls{
                <Self::Impls as $crate::pmr::MarkerType>::MTVAL
            }
        }

        impl<$($($params)*)? __This> $trait_name<$($($params)*)?>
        for $crate::pmr::PhantomData<__This>
        where
            $( __This:$($supertraits)*, )?
            $( $($where_)* )?
        {
            type Impls=$crate::pmr::True;
        }

        impl<$($($params)*)? __This> $trait_name<$($($params)*)?>
        for &'_ $crate::pmr::PhantomData<__This>
        {
            type Impls=$crate::pmr::False;
        }
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! abort_on_return {
    (
        error_context=$context:expr,
        code{
            $($code:tt)*
        }
    ) => (
        let guard={
            use $crate::utils::{AbortBomb,PanicInfo};
            #[allow(dead_code)]
            const BOMB:AbortBomb=AbortBomb{
                fuse:&PanicInfo{
                    file:file!(),
                    line:line!(),
                    context:$context,
                }
            };
            BOMB
        };
        let ret={
            $($code)*
        };

        $crate::pmr::forget(guard);

        ret
    )
}

#[macro_export]
#[doc(hidden)]
macro_rules! reverse_code {
    ( $( ($($block:tt)*) )* ) => {
        $crate::reverse_code!{@inner [] $(( $($block)* ))* }
    };
    (@inner [$(( $($rem:tt)* ))*] ) => {
        $($($rem)*)*
    };
    (@inner [$($rem:tt)*] $first:tt $($block:tt)* ) => {
        $crate::reverse_code!{@inner [$first $($rem)*] $($block)* }
    };
}
