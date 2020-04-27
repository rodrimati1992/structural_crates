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
        for_drop={
            just_fields,
            $($for_drop:tt)*
        }

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
            for_drop($($for_drop)*)

            impl[$($typarams)*] DropFields for $self_
            where[$($where)*]
            $impl_details
        }
    };
    (
        struct_or_enum($soe:tt),
        for_drop={
            pre_post_drop,
            $($for_drop:tt)*
        }

        impl[$($typarams:tt)*] DropFields for $self_:ty
        where[$($where:tt)*]
        $impl_details:tt
    )=>{
        $crate::_private_impl_drop_fields_inner!{
            @compute_where_clause
            struct_or_enum($soe),
            for_drop($($for_drop)*)

            impl[$($typarams)*] DropFields for $self_
            where[$($where)*]
            $impl_details
        }
    };
    (
        struct_or_enum($soe:tt),
        for_drop=$(custom_drop)?

        impl[$($typarams:tt)*] DropFields for $self_:ty
        where[$($where:tt)*]
        $impl_details:tt
    )=>{};
    ( $($stuff:tt)* )=>{
        compile_error!{concat!(
            "Unrecognized `_private_impl_drop_fields` arguments:",
            $(stringify!($stuff),)*
        )}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _private_impl_drop_fields_inner{
    // Structs don't need to compute any where clauses
    (@compute_where_clause
        struct_or_enum(struct),
        for_drop(
            $(pre_move = $pre_move_fn:expr,)?
        )

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
            fn pre_move(&mut self){
                $(
                    $pre_move_fn(self);
                )?
            }

            unsafe fn drop_fields(&mut self,moved: $crate::pmr::MovedOutFields) {
                let mut this=$crate::pmr::RunPostDrop::new(self);
                let this=this.get_mut();

                $crate::abort_on_return!{
                    error_context="Inside PrePostDropFields::pre_drop",
                    code{
                        $crate::pmr::PrePostDropFields::pre_drop(this);
                    }
                }

                use $crate::pmr::{RunDrop,FieldBit};

                $crate::reverse_code!{
                    $((
                        let _a=RunDrop::new( &mut this.$drop_uncond );
                    ))*
                }

                $crate::reverse_code!{
                    $((
                        let _a;
                        {
                            const __DROP_BIT:FieldBit=FieldBit::new($field_index);
                            if !moved.is_moved_out(__DROP_BIT) {
                                _a=RunDrop::new( &mut this.$field_name )
                            }
                        }
                    ))*
                }
            }
        }
    };
    (@compute_where_clause
        struct_or_enum(enum),
        for_drop $for_drop:tt

        impl $impl_params:tt DropFields for $self_:ty
        where $where:tt
        $impl_details:tt
    )=>{
        $crate::_private_impl_drop_fields_inner!{
            @step
            moved_fields_variable=moved_fields,
            for_drop $for_drop

            impl $impl_params DropFields for $self_
            where $where
            pre_move_branches()
            branches()
            $impl_details
        }
    };
    ////////////////////////////////////////////////////////////////////////////
    //// How enums are dropped
    (@step
        moved_fields_variable=$moved_fields:ident,
        for_drop $for_drop:tt

        impl $impl_params:tt DropFields for $self_:ty
        where [$($where:tt)*]
        pre_move_branches($($pre_move_branches:tt)*)
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
            moved_fields_variable=$moved_fields,
            for_drop $for_drop

            impl $impl_params DropFields for $self_
            where [$($where)*]
            pre_move_branches(
                $($pre_move_branches)*
            )
            branches(
                $($branches)*

                Self::$variant{
                    $($drop_uncond: $drop_uncond_var,)*
                    $($field_name: $field_var,)*
                    ..
                }=>{
                    $crate::reverse_code!{
                        $((
                            let _a=$crate::pmr::RunDrop::new($drop_uncond_var);
                        ))*
                    }

                    $crate::reverse_code!{
                        $((
                            let _a;
                            {
                                use $crate::pmr::{RunDrop, FieldBit};
                                const __DROP_BIT:FieldBit=FieldBit::new($field_index);
                                if !$moved_fields.is_moved_out(__DROP_BIT) {
                                    _a=RunDrop::new( $field_var );
                                }
                            }
                        ))*
                    }
                }
            )
            {
                $($rest)*
            }
        }
    };
    // Dropping a newtype variant
    (@step
        moved_fields_variable=$moved_fields:ident,
        for_drop $for_drop:tt

        impl $impl_params:tt DropFields for $self_:ty
        where [$($where:tt)*]
        pre_move_branches($($pre_move_branches:tt)*)
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
            moved_fields_variable=$moved_fields,
            for_drop $for_drop

            impl $impl_params DropFields for $self_
            where [
                $field_ty: $crate::pmr::DropFields,
                $($where)*
            ]
            pre_move_branches(
                $($pre_move_branches)*

                Self::$variant{$field_name: $field_var,..}=>{
                    $crate::pmr::DropFields::pre_move( $field_var );
                }
            )
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

                    $crate::pmr::DropFields::drop_fields( $field_var, $moved_fields );
                }
            )
            {
                $($rest)*
            }
        }
    };
    (@step
        moved_fields_variable=$moved_fields:ident,
        for_drop(
            $(pre_move = $pre_move_fn:expr,)?
        )

        impl[$($typarams:tt)*] DropFields for $self_:ty
        where [$($where:tt)*]
        pre_move_branches($($pre_move_branches:tt)*)
        branches($($branches:tt)*)
        {}
    )=>{
        unsafe impl<$($typarams)*> $crate::pmr::DropFields for $self_
        where
            Self: $crate::pmr::PrePostDropFields,
            $($where)*
        {
            #[inline(always)]
            fn pre_move(&mut self){
                #[allow(unused_variables,unused_mut)]
                let mut this=$crate::pmr::RunOnDrop::new(
                    self,
                    #[inline(always)]
                    |this|{
                        match this {
                            $($pre_move_branches)*
                            #[allow(unreachable_patterns)]
                            _=>{}
                        }
                    }
                );

                $(
                    $pre_move_fn(this.reborrow_mut());
                )?
            }

            #[inline(always)]
            unsafe fn drop_fields(&mut self, $moved_fields: $crate::pmr::MovedOutFields) {
                let mut this=$crate::pmr::RunPostDrop::new(self);
                let this=this.get_mut();

                $crate::abort_on_return!{
                    error_context="Inside PrePostDropFields::pre_drop",
                    code{
                        $crate::pmr::PrePostDropFields::pre_drop(this);
                    }
                }

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
