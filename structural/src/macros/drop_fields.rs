// Implementation details:
//
// The reason that this uses the `RunDrop` type instead of `std::ptr::drop_in_place` is
// to avoid leaks,since the remaining field destructors may otherwise not run.
#[doc(hidden)]
#[macro_export]
macro_rules! _private_impl_drop_fields{
    ////////////////////////////////////////////////////////////////////////////
    //// The different way that structural types are dropped
    (
        struct_or_enum($soe:tt),
        drop_kind=just_fields,

        impl[$($typarams:tt)*] DropFields for $self_:ty
        where[$($where:tt)*]
        $impl_details:tt
    )=>{

        unsafe impl<$($typarams)*> $crate::pmr::PrePostDropFields for $self_
        where
            $($where)*
        {}

        $crate::_private_impl_drop_fields_inner!{
            @compute_where_clause
            struct_or_enum($soe),

            impl[$($typarams)*] DropFields for $self_
            where[$($where)*]
            $impl_details
        }
    };
    (
        struct_or_enum($soe:tt),
        drop_kind=pre_post_drop,

        impl[$($typarams:tt)*] DropFields for $self_:ty
        where[$($where:tt)*]
        $impl_details:tt
    )=>{
        $crate::_private_impl_drop_fields_inner!{
            @compute_where_clause
            struct_or_enum($soe),

            impl[$($typarams)*] DropFields for $self_
            where[$($where)*]
            $impl_details
        }
    };
    (
        struct_or_enum($soe:tt),
        drop_kind=$(custom_drop)?,

        impl[$($typarams:tt)*] DropFields for $self_:ty
        where[$($where:tt)*]
        $impl_details:tt
    )=>{};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _private_impl_drop_fields_inner{
    // Structs don't need to compute any where clauses
    (@compute_where_clause
        struct_or_enum(struct),

        impl[$($typarams:tt)*] DropFields for $self_:ty
        where[$($where:tt)*]
        {
            not_public( $($drop_uncond:tt)* ),
            field_names($(
                (
                    $field_name:tt,
                    $field_index:expr,
                ),
            )*),
        }
    )=>{
        unsafe impl<$($typarams)*> $crate::pmr::DropFields for $self_
        where
            Self: $crate::pmr::PrePostDropFields,
            $($where)*
        {
            #[inline(always)]
            unsafe fn drop_fields(&mut self,dropped: $crate::pmr::DroppedFields) {
                let mut this=$crate::pmr::RunPostDrop::new(self);
                let this=this.get_mut();

                let mut this=$crate::pmr::RunPreDrop::new(this);
                let this=this.get_mut();

                use $crate::pmr::{RunDrop,DropBit};

                $(
                    let _a=RunDrop::new( &mut this.$drop_uncond );
                )*

                $(
                    let _a;
                    {
                        const __DROP_BIT:DropBit=DropBit::new($field_index);
                        if !dropped.is_dropped(__DROP_BIT) {
                            _a=RunDrop::new( &mut this.$field_name )
                        }
                    }
                )*
            }
        }
    };
    (@compute_where_clause
        struct_or_enum(enum),

        impl $impl_params:tt DropFields for $self_:ty
        where $where:tt
        $impl_details:tt
    )=>{
        $crate::_private_impl_drop_fields_inner!{
            @step
            dropped_variable=dropped,
            impl $impl_params DropFields for $self_
            where $where
            branches()
            $impl_details
        }
    };
    ////////////////////////////////////////////////////////////////////////////
    //// How enums are dropped
    (@step
        dropped_variable=$dropped:ident,
        impl $impl_params:tt DropFields for $self_:ty
        where [$($where:tt)*]
        branches($($branches:tt)*)
        {
            $variant:ident(
                kind=regular,

                not_public($(
                    ($drop_uncond:tt = $drop_uncond_var:ident)
                )*),

                fields($(
                    (
                        $trait:ident,
                        $field_name:tt : $field_ty:ty,
                        dropping($field_var:ident, $field_index:expr)
                        $($rem_asdfgh:tt)*
                    )$(,)?
                )*),
            )
            $($rest:tt)*
        }
    )=>{
        $crate::_private_impl_drop_fields_inner!{
            @step
            dropped_variable=$dropped,
            impl $impl_params DropFields for $self_
            where [$($where)*]
            branches(
                $($branches)*

                Self::$variant{
                    $($drop_uncond: $drop_uncond_var,)*
                    $($field_name: $field_var,)*
                    ..
                }=>{
                    $(
                        let _a=RunDrop::new(&mut $drop_uncond_var);
                    )*

                    $(
                        let _a;
                        {
                            use $crate::pmr::{RunDrop, DropBit};
                            const __DROP_BIT:DropBit=DropBit::new($field_index);
                            if !$dropped.is_dropped(__DROP_BIT) {
                                _a=RunDrop::new( $field_var );
                            }
                        }
                    )*
                }
            )
            {
                $($rest)*
            }
        }
    };
    // Dropping a newtype variant
    (@step
        dropped_variable=$dropped:ident,
        impl $impl_params:tt DropFields for $self_:ty
        where [$($where:tt)*]
        branches($($branches:tt)*)
        {
            $variant:ident(
                kind=newtype,

                not_public($(
                    ($drop_uncond:tt = $drop_uncond_var:ident)
                )*),

                fields(
                    (
                        $trait:ident,
                        $field_name:tt : $field_ty:ty,
                        dropping($field_var:ident, $field_index:expr)
                        $($rem_asdfgh:tt)*
                    ) $(,)?
                ),
            )
            $($rest:tt)*
        }
    )=>{
        $crate::_private_impl_drop_fields_inner!{
            @step
            dropped_variable=$dropped,
            impl $impl_params DropFields for $self_
            where [
                $field_ty: $crate::pmr::DropFields,
                $($where)*
            ]
            branches(
                $($branches)*

                Self::$variant{
                    $($drop_uncond: $drop_uncond_var,)*
                    $field_name: $field_var,
                    ..
                }=>{
                    $(
                        let _a=$crate::pmr::RunDrop::new(&mut $drop_uncond_var);
                    )*

                    $crate::pmr::DropFields::drop_fields( $field_var, $dropped );
                }
            )
            {
                $($rest)*
            }
        }
    };
    (@step
        dropped_variable=$dropped:ident,
        impl[$($typarams:tt)*] DropFields for $self_:ty
        where [$($where:tt)*]
        branches($($branches:tt)*)
        {}
    )=>{
        unsafe impl<$($typarams)*> $crate::pmr::DropFields for $self_
        where
            Self: $crate::pmr::PrePostDropFields,
            $($where)*
        {
            #[inline(always)]
            unsafe fn drop_fields(&mut self, $dropped: $crate::pmr::DroppedFields) {
                let mut this=$crate::pmr::RunPostDrop::new(self);
                let this=this.get_mut();

                let mut this=$crate::pmr::RunPreDrop::new(this);
                let this=this.get_mut();

                match this {
                    $($branches)*
                }
            }
        }
    };
    ( $($stuff:tt)* )=>{
        compile_error!{concat!(
            "Unrecognized `_private_impl_drop_fields_inner` arguments:",
            $(stringify!($stuff),)*
        )}
    }
}
