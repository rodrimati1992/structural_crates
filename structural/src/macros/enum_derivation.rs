/// Implements enum variant+field getter(s)
#[doc(hidden)]
#[macro_export]
macro_rules! _private_impl_getter_enum{
    ////////////////////////////////////////////////////////////////////////////
    ////            All the ways to implement GetVariantField
    ////////////////////////////////////////////////////////////////////////////
    (
        GetVariantField
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype(
            $field_name:tt : $field_ty:ty
            $(,$field_name_param:ty)? $( , )*
        )
    )=>{
        impl<$($typarams)* __F>
            $crate::FieldType< $crate::VariantField<$variant_name_str,__F> >
        for $self_
        where
            $field_ty: $crate::GetField<__F>,
            $($where_)*
        {
            type Ty=$crate::GetFieldType<$field_ty,__F>;
        }

        unsafe impl<$($typarams)* __F,__Ty>
            $crate::pmr::GetVariantField<$variant_name_str,__F>
        for $self_
        where
            $field_ty: $crate::GetField<__F,Ty=__Ty>,
            $($where_)*
        {
            #[inline(always)]
            fn get_vfield_(
                &self,
                _:$variant_name_str,
                fname:__F
            )->Option<&__Ty>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        Some($crate::GetField::get_field_(field,fname))
                    }
                    #[allow(unreachable_patterns)]
                    _=>None,
                }
            }
        }
    };
    (
        GetVariantField
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        regular(
            $field_name:tt : $field_ty:ty,
            $field_name_param:ty $( , )*
        )
    )=>{

        impl<$($typarams)*>
            $crate::FieldType<
                $crate::VariantField<$variant_name_str,$field_name_param>
            >
        for $self_
        where
            $($where_)*
        {
            type Ty=$field_ty;
        }

        unsafe impl<$($typarams)*>
            $crate::pmr::GetVariantField<$variant_name_str,$field_name_param>
        for $self_
        where
            $($where_)*
        {
            #[inline(always)]
            fn get_vfield_(
                &self,
                _:$variant_name_str,
                _:$field_name_param,
            )->Option<&$field_ty>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>
                        Some(field),
                    #[allow(unreachable_patterns)]
                    _=>None
                }
            }
        }
    };

    ////////////////////////////////////////////////////////////////////////////
    ////            All the ways to implement GetVariantFieldMut
    ////////////////////////////////////////////////////////////////////////////
    (
        GetVariantFieldMut
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype(
            $field_name:tt : $field_ty:ty
            $(,$field_name_param:ty)? $( , )*
        )
    )=>{
        unsafe impl<$($typarams)* __F,__Ty>
            $crate::GetVariantFieldMut<$variant_name_str,__F>
        for $self_
        where
            $field_ty: $crate::GetFieldMut<__F,Ty=__Ty>,
            $($where_)*
        {
            #[inline(always)]
            fn get_vfield_mut_(
                &mut self,
                _: $variant_name_str,
                name: __F,
            )->Option<&mut __Ty>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>
                        Some($crate::GetFieldMut::get_field_mut_(field,name)),
                    #[allow(unreachable_patterns)]
                    _=>None
                }
            }

            #[inline(always)]
            unsafe fn get_vfield_raw_mut_(
                this:*mut  (),
                _:$variant_name_str,
                name:__F,
            )->Option<$crate::pmr::NonNull<__Ty>> {
                match *(this as *mut  Self) {
                    $enum_::$variant{$field_name:ref mut field,..}=>{
                        let field=field as *mut $field_ty as *mut  ();
                        let ptr=<$field_ty as $crate::GetFieldMut<__F>>::get_field_raw_mut(
                            field,
                            name,
                        );
                        // Safety:
                        // Calling `NonNull::new_unchecked` because the pointer returned by
                        // `get_field_raw_mut` must be valid.
                        Some($crate::pmr::NonNull::new_unchecked(ptr))
                    }
                    #[allow(unreachable_patterns)]
                    _=>None
                }
            }

            #[inline(always)]
            fn get_vfield_raw_mut_fn(
                &self,
            )->$crate::pmr::GetVFieldRawMutFn<
                $variant_name_str,
                __F,
                __Ty,
            >{
                <Self as $crate::GetVariantFieldMut<$variant_name_str,__F>>::get_vfield_raw_mut_
            }

            #[inline(always)]
            fn get_vfield_raw_mut_unchecked_fn(
                &self,
            )->$crate::pmr::GetFieldRawMutFn<
                __F,
                __Ty,
            >{
                <Self as
                    $crate::GetVariantFieldMut<$variant_name_str,__F>
                >::get_vfield_raw_mut_unchecked
            }
        }


    };
    (
        GetVariantFieldMut
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        regular(
            $field_name:tt : $field_ty:ty,
            $field_name_param:ty $( , )*
        )
    )=>{
        unsafe impl<$($typarams)*>
            $crate::GetVariantFieldMut<$variant_name_str,$field_name_param>
        for $self_
        where
            $($where_)*
        {
            #[inline(always)]
            fn get_vfield_mut_(
                &mut self,
                _: $variant_name_str,
                _: $field_name_param,
            )->Option<&mut $field_ty>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>Some(field),
                    #[allow(unreachable_patterns)]
                    _=>None
                }
            }

            #[inline(always)]
            unsafe fn get_vfield_raw_mut_(
                this:*mut  (),
                _:$variant_name_str,
                _:$field_name_param,
            )->Option<$crate::pmr::NonNull<$field_ty>> {
                $crate::z_raw_borrow_enum_field!(
                    this as *mut  Self,
                    $enum_::$variant.$field_name : $field_ty
                )
            }

            #[inline(always)]
            fn get_vfield_raw_mut_fn(
                &self,
            )->$crate::pmr::GetVFieldRawMutFn<
                $variant_name_str,
                $field_name_param,
                $field_ty,
            >{
                <Self as
                    $crate::GetVariantFieldMut<$variant_name_str,$field_name_param>
                >::get_vfield_raw_mut_
            }

            #[inline(always)]
            fn get_vfield_raw_mut_unchecked_fn(
                &self,
            )->$crate::pmr::GetFieldRawMutFn<
                $field_name_param,
                $field_ty,
            >{
                <Self as
                    $crate::GetVariantFieldMut<$variant_name_str,$field_name_param>
                >::get_vfield_raw_mut_unchecked
            }
        }
    };
    ////////////////////////////////////////////////////////////////////////////
    ////            All the ways to implement IntoVariantField
    ////////////////////////////////////////////////////////////////////////////
    (
        IntoVariantField
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype(
            $field_name:tt : $field_ty:ty
            $(,$field_name_param:ty)? $( , )*
        )
    )=>{
        unsafe impl<$($typarams)* __F,__Ty>
            $crate::pmr::IntoVariantField<$variant_name_str,__F>
        for $self_
        where
            $field_ty: $crate::IntoField<__F,Ty=__Ty>,
            $($where_)*
        {
            #[inline(always)]
            fn into_vfield_(
                self,
                _:$variant_name_str,
                name:__F,
            )->Option<__Ty>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>
                        Some($crate::IntoField::into_field_(field,name)),
                    #[allow(unreachable_patterns)]
                    _=>None
                }
            }

            $crate::z_impl_box_into_variant_field_method!{
                variant_tstr= $variant_name_str,
                field_tstr= __F,
                field_type= __Ty,
            }
        }
    };
    (
        IntoVariantField
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        regular(
            $field_name:tt : $field_ty:ty,
            $field_name_param:ty $( , )*
        )
    )=>{
        unsafe impl<$($typarams)*>
            $crate::pmr::IntoVariantField<$variant_name_str,$field_name_param>
        for $self_
        where
            $($where_)*
        {
            #[inline(always)]
            fn into_vfield_(
                self,
                _:$variant_name_str,
                _:$field_name_param,
            )->Option<$field_ty>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>Some(field),
                    #[allow(unreachable_patterns)]
                    _=>None
                }
            }

            $crate::z_impl_box_into_variant_field_method!{
                variant_tstr= $variant_name_str,
                field_tstr= $field_name_param,
                field_type= $field_ty,
            }
        }
    };
    ////////////////////////////////////////////////////////////////////////////
    ////                delegate_to
    ////////////////////////////////////////////////////////////////////////////
    (
        shared $shared:tt
        kind=$kind:tt

        delegate_to( GetVariantField, $($field_params:tt)* )
    )=>{
        $crate::_private_impl_getter_enum!{
            GetVariantField
            shared $shared
            $kind($($field_params)*)
        }
    };
    (
        shared $shared:tt
        kind=$kind:tt
        delegate_to( GetVariantFieldMut, $($field_params:tt)* )
    )=>{
        $crate::_private_impl_getter_enum!{
            GetVariantField
            shared $shared
            $kind($($field_params)*)
        }
        $crate::_private_impl_getter_enum!{
            GetVariantFieldMut
            shared $shared
            $kind($($field_params)*)
        }
    };
    (
        shared $shared:tt
        kind=$kind:tt
        delegate_to( IntoVariantField, $($field_params:tt)* )
    )=>{
        $crate::_private_impl_getter_enum!{
            GetVariantField
            shared $shared
            $kind($($field_params)*)
        }
        $crate::_private_impl_getter_enum!{
            IntoVariantField
            shared $shared
            $kind($($field_params)*)
        }
    };
    (
        shared $shared:tt
        kind=$kind:tt
        delegate_to( IntoVariantFieldMut, $($field_params:tt)* )
    )=>{
        $crate::_private_impl_getter_enum!{
            GetVariantField
            shared $shared
            $kind($($field_params)*)
        }
        $crate::_private_impl_getter_enum!{
            GetVariantFieldMut
            shared $shared
            $kind($($field_params)*)
        }
        $crate::_private_impl_getter_enum!{
            IntoVariantField
            shared $shared
            $kind($($field_params)*)
        }
    };
    (
        $trait_:ident
        shared $shared:tt
        newtype_as_field(
            shared (
                impl[$($typarams:tt)*] $self_:ty
                where[$($where_:tt)*]

                enum=$enum_:ident
                variant=$variant:ident
                variant_name($variant_name_str:ty)
            )

            $field_name:tt : $field_ty:ty
            $(,$field_name_param:ty)? $( , )*
        )
    )=>{
        $crate::_private_impl_getter_enum!{
            $trait_
            shared $shared
            newtype(
                $field_name : $field_ty $(,$field_name_param)?
            )
        }
    };
    ////////////////////////////////////////////////////////////////////////////
    ////                variants code
    ////////////////////////////////////////////////////////////////////////////
    (
        shared $shared:tt
        kind=newtype
        fields(
            (
                $trait_:ident ,
                $field_name:tt : $field_ty:ty
                    $(,$field_name_param:ty)? $( , )*
            )
        )
    )=>{
        $crate::_private_impl_getter_enum!{
            shared $shared
            kind=newtype_as_field
            delegate_to(
                $trait_,

                shared $shared

                $field_name : $field_ty $(,$field_name_param)?
            )
        }
    };

    (
        shared $shared:tt
        kind=regular
        fields( $($fields:tt)* )
    )=>{
        $(
            $crate::_private_impl_getter_enum!{
                shared $shared
                kind=regular
                delegate_to $fields
            }
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _private_impl_getters_for_derive_enum{
    (
        $(#[doc=$docs:literal])*
        impl $typarams:tt $self_:ty
        where $where_preds:tt
        {
            enum=$enum_:ident
            variant_count=$variant_count:ty,
            $((
                $variant:ident,
                $variant_tstr:ty,
                kind=$variant_kind:ident,
                fields($( $field:tt )*)
            ))*
        }

    )=>{

        $crate::_private_impl_structural!{
            $(#[doc=$docs])*
            impl $typarams Structural for $self_
            where $where_preds
            {
                variants=[ $( $variant, )* ]
            }
        }

        $(
            $crate::_private_impl_getter_enum!{
                shared(
                    impl $typarams $self_
                    where $where_preds

                    enum=$enum_
                    variant=$variant
                    variant_name($variant_tstr)
                )
                kind= $variant_kind
                fields(
                    $( $field )*
                )
            }

            $crate::_private_impl_getters_for_derive_enum!{
                @impl_is_variant
                impl $typarams $self_
                where $where_preds

                enum=$enum_
                variant=$variant
                variant_name($variant_tstr)
            }
        )*

        $crate::_private_impl_getters_for_derive_enum!{
            @inner
            impl $typarams $self_
            where $where_preds

            variant_count=$variant_count,
        }
    };
    (@inner
        impl[$($typarams:tt)*] $self_:ty
        where[$($where_:tt)*]
        variant_count=$variant_count:ty,
    )=>{
        unsafe impl<$($typarams)*> $crate::pmr::VariantCount for $self_
        where
            $($where_)*
        {
            type Count=$variant_count;
        }
    };
    (@impl_is_variant
        impl[$($typarams:tt)*] $self_:ty
        where[$($where_:tt)*]

        enum=$enum_:ident
        variant=$variant:ident
        variant_name($variant_name_str:ty)
    )=>{
        unsafe impl<$($typarams)*>
            $crate::pmr::IsVariant<$variant_name_str>
        for $self_
        where $($where_)*
        {
            #[inline]
            #[allow(unreachable_patterns)]
            fn is_variant_(&self,_:$variant_name_str)->bool{
                match self {
                    $enum_::$variant{..}=>true,
                    #[allow(unreachable_patterns)]
                    _=>false,
                }
            }
        }
    };
}
