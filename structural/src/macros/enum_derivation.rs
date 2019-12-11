/// Implements enum variant+field getter(s)
#[doc(hidden)]
#[macro_export]
macro_rules! impl_getter_enum{
    ////////////////////////////////////////////////////////////////////////////
    ////            All the ways to implement GetFieldImpl
    ////////////////////////////////////////////////////////////////////////////
    (
        $(unsafe)? , GetFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        unit(  )
    )=>{
        impl<$($typarams)*>
            $crate::GetFieldImpl<$crate::pmr::FieldPath1<$variant_name_str>>
        for $self_
        where $($where_)*
        {
            type Ty=();
            type Err=$crate::field_traits::OptionalField;

            fn get_field_(&self)->Result<&(),$crate::field_traits::OptionalField>{
                match self {
                    $enum_::$variant{..}=>Ok(&()),
                    _=>Err($crate::field_traits::OptionalField),
                }
            }
        }
    };
    (
        $(unsafe)? , GetFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype( $field_name:tt : $field_ty:ty $( , )* )
    )=>{
        impl_getter_enum!{
            ,GetFieldImpl
            shared(
                impl[$($typarams)*] $self_
                where [$($where_)*]

                enum=$enum_
                variant=$variant
                variant_name($variant_name_str)
            )
            field (
                $field_name : $field_ty, $crate::pmr::FieldPath1<$variant_name_str>
            )
        }

        impl<$($typarams)*__FieldName>
            $crate::GetFieldImpl<
                $crate::pmr::VariantFieldPath<$variant_name_str,__FieldName>
            >
        for $self_
        where
            $field_ty: $crate::GetFieldImpl<$crate::pmr::FieldPath1<__FieldName>>
            $( $($where_)* )?
        {
            type Ty=$crate::GetFieldType<$field_ty,$crate::pmr::FieldPath1<__FieldName>>;
            type Err=$crate::field_traits::OptionalField;

            fn get_field_(
                &self
            )->Result<
                &$crate::GetFieldType<$field_ty,$crate::pmr::FieldPath1<__FieldName>>,
                $crate::field_traits::OptionalField
            >{
                match self {
                    $enum_::$variant{$field_name:field,..}=>map_of!(
                        GetFieldImpl::<$crate::pmr::FieldPath1<__FieldName>>::get_field_(field)
                    ),
                    _=>Err($crate::field_traits::OptionalField),
                }
            }
        }
    };
    (
        $(unsafe)? , GetFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        field(
            $field_name:tt : $field_ty:ty, $field_name_param:ty  $( , )*
        )
    )=>{
        impl<$($typarams)*>
            $crate::GetFieldImpl< $field_name_param >
        for $self_
        where
            $( $($where_)* )?
        {
            type Ty=$field_ty;
            type Err=$crate::field_traits::OptionalField;

            fn get_field_(&self)->Result<&$field_ty,$crate::field_traits::OptionalField>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>Ok(field),
                    _=>Err($crate::field_traits::OptionalField),
                }
            }
        }
    };


    ////////////////////////////////////////////////////////////////////////////
    ////            All the ways to implement GetFieldMutImpl
    ////////////////////////////////////////////////////////////////////////////
    (
        unsafe,GetFieldMutImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        unit(  )
    )=>{
        unsafe impl<$($typarams)*>
            $crate::GetFieldMutImpl<$crate::pmr::FieldPath1<$variant_name_str>>
        for $self_
        where $($where_)*
        {
            fn get_field_mut_(&mut self)->Result<&mut (),$crate::field_traits::OptionalField>{
                match self {
                    $enum_::$variant { .. } => Ok($crate::utils::unit_mut_ref()),
                    _ => Err($crate::field_traits::OptionalField),
                }
            }

            z_unsafe_impl_get_field_raw_mut_method_enum!{
                Self,
                variant($enum_,$variant,()),
                name_generic=$crate::pmr::FieldPath1<$variant_name_str>,
                field_ty=(),
            }
        }
    };
    (
        unsafe,GetFieldMutImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype( $field_name:tt : $field_ty:ty $( , )* )
    )=>{
        impl_getter_enum!{
            unsafe,GetFieldMutImpl
            shared(
                impl[$($typarams)*] $self_
                where [$($where_)*]

                enum=$enum_
                variant=$variant
                variant_name($variant_name_str)
            )
            field (
                $field_name : $field_ty, $crate::pmr::FieldPath1<$variant_name_str>
            )
        }

        unsafe impl<$($typarams)*__FieldName>
            $crate::GetFieldMutImpl<
                $crate::pmr::VariantFieldPath<$variant_name_str,__FieldName>
            >
        for $self_
        where
            $field_ty: $crate::GetFieldMutImpl<$crate::pmr::FieldPath1<__FieldName>>
            $( $($where_)* )?
        {
            fn get_field_mut_(
                &mut self
            )->Result<
                &mut $crate::GetFieldType<$field_ty,$crate::pmr::FieldPath1<__FieldName>>,
                $crate::field_traits::OptionalField
            >{
                match self {
                    $enum_::$variant{$field_name:field,..}=>map_of!(
                        GetFieldMutImpl::<
                            $crate::pmr::FieldPath1<__FieldName>
                        >::get_field_mut_(field)
                    ),
                    _=>Err($crate::field_traits::OptionalField),
                }
            }

            unsafe fn get_field_raw_mut(
                this:*mut (),
                _:$crate::pmr::PhantomData<
                    $crate::pmr::VariantFieldPath<$variant_name_str,__FieldName>
                >,
            )->Result<
                *mut $crate::GetFieldType<$field_ty,$crate::pmr::FieldPath1<__FieldName>>,
                $crate::field_traits::OptionalField,
            >{
                match *(this as *mut Self) {
                    $enum_::$variant{$field_name:ref mut this,..    }=>{
                        let res=<$field_ty as
                                GetFieldMutImpl<$crate::pmr::FieldPath1<__FieldName>>
                            >::get_field_raw_mut(
                                this as *mut _ as *mut (),
                                $crate::pmr::PhantomData,
                            );
                        map_of!(res)
                    }
                    _=>{
                        Err( $crate::field_traits::OptionalField )
                    }
                }
            }

            fn get_field_raw_mut_func(
                &self
            )->$crate::field_traits::GetFieldMutRefFn<
                $crate::pmr::VariantFieldPath<$variant_name_str,__FieldName>,
                $crate::GetFieldType<$field_ty,$crate::pmr::FieldPath1<__FieldName>>,
                $crate::field_traits::OptionalField
            >{
                <Self as
                    $crate::field_traits::GetFieldMutImpl<
                        $crate::pmr::VariantFieldPath<$variant_name_str,__FieldName>
                    >
                >::get_field_raw_mut
            }
        }
    };
    (
        unsafe,GetFieldMutImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        field(
            $field_name:tt : $field_ty:ty, $field_name_param:ty  $( , )*
        )
    )=>{
        unsafe impl<$($typarams)*>
            $crate::GetFieldMutImpl< $field_name_param >
        for $self_
        where

            $( $($where_)* )?
        {
            fn get_field_mut_(
                &mut self
            )->Result<&mut $field_ty,$crate::field_traits::OptionalField>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>Ok(field),
                    _=>Err($crate::field_traits::OptionalField),
                }
            }

            z_unsafe_impl_get_field_raw_mut_method_enum!{
                Self,
                variant($enum_,$variant,$field_name),
                name_generic=$field_name_param,
                field_ty=$field_ty,
            }
        }
    };

    ////////////////////////////////////////////////////////////////////////////
    ////            All the ways to implement IntoFieldImpl
    ////////////////////////////////////////////////////////////////////////////
    (
        $(unsafe)? , IntoFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        unit(  )
    )=>{
        impl<$($typarams)*>
            $crate::IntoFieldImpl<$crate::pmr::FieldPath1<$variant_name_str>>
        for $self_
        where $($where_)*
        {
            fn into_field_(self)->Result<(),$crate::field_traits::OptionalField>{
                match self {
                    $enum_::$variant{..}=>Ok(()),
                    _=>Err($crate::field_traits::OptionalField),
                }
            }

            z_impl_box_into_field_method!{
                $crate::pmr::FieldPath1<$variant_name_str>,
                (),
                $crate::field_traits::OptionalField,
            }
        }
    };
    (
        $(unsafe)? , IntoFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype( $field_name:tt : $field_ty:ty $( , )* )
    )=>{
        impl_getter_enum!{
            ,IntoFieldImpl
            shared(
                impl[$($typarams)*] $self_
                where [$($where_)*]

                enum=$enum_
                variant=$variant
                variant_name($variant_name_str)
            )
            field (
                $field_name : $field_ty, $crate::pmr::FieldPath1<$variant_name_str>
            )
        }

        impl<$($typarams)*__FieldName>
            $crate::IntoFieldImpl<
                $crate::pmr::VariantFieldPath<$variant_name_str,__FieldName>
            >
        for $self_
        where
            $field_ty: $crate::IntoFieldImpl<$crate::pmr::FieldPath1<__FieldName>>
            $( $($where_)* )?
        {
            fn into_field_(
                self
            )->Result<
                $crate::GetFieldType<$field_ty,$crate::pmr::FieldPath1<__FieldName>>,
                $crate::field_traits::OptionalField
            >{
                match self {
                    $enum_::$variant{$field_name:field,..}=>map_of!(
                        IntoFieldImpl::<
                            $crate::pmr::FieldPath1<__FieldName>
                        >::into_field_(field)
                    ),
                    _=>Err($crate::field_traits::OptionalField),
                }
            }

            z_impl_box_into_field_method!{
                $crate::pmr::VariantFieldPath<$variant_name_str,__FieldName>,
                $crate::GetFieldType<$field_ty,$crate::pmr::FieldPath1<__FieldName>>,
                $crate::field_traits::OptionalField,
            }
        }
    };
    (
        $(unsafe)? , IntoFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        field(
            $field_name:tt : $field_ty:ty, $field_name_param:ty  $( , )*
        )
    )=>{
        impl<$($typarams)*>
            $crate::IntoFieldImpl< $field_name_param >
        for $self_
        where
            $( $($where_)* )?
        {
            fn into_field_(self)->Result<$field_ty,$crate::field_traits::OptionalField>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>Ok(field),
                    _=>Err($crate::field_traits::OptionalField),
                }
            }

            z_impl_box_into_field_method!{
                $field_name_param,
                $field_ty,
                $crate::field_traits::OptionalField,
            }
        }
    };


    ////////////////////////////////////////////////////////////////////////////
    ////                Shared code
    ////////////////////////////////////////////////////////////////////////////
    (
        $(unsafe)? , $trait_:ident
        shared(
            impl $typarams:tt $self_:ty
            where $where_:tt

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        fields($((
            $field_name:tt : $field_ty:ty, $field_name_str:ty $( , )*
        ))*)
    )=>{
        $(
            impl_getter_enum!{
                unsafe,$trait_

                shared(
                    impl $typarams $self_
                    where $where_
                    enum=$enum_
                    variant=$variant
                    variant_name($variant_name_str)
                )
                field (
                    $field_name : $field_ty,
                    $crate::pmr::VariantFieldPath<$variant_name_str, $field_name_str>
                )
            }
        )*
    };

    (top;
        $(unsafe)?,GetFieldImpl
        shared $shared:tt
        $variant_kind:ident $variant_kind_params:tt
    )=>{
        impl_getter_enum!{
            ,GetFieldImpl
            shared $shared
            $variant_kind $variant_kind_params
        }
    };
    (top;
        unsafe,GetFieldMutImpl
        shared $shared:tt
        $variant_kind:ident $variant_kind_params:tt
    )=>{
        impl_getter_enum!{
            ,GetFieldImpl
            shared $shared
            $variant_kind $variant_kind_params
        }
        impl_getter_enum!{
            unsafe,GetFieldMutImpl
            shared $shared
            $variant_kind $variant_kind_params
        }
    };
    (top;
        ,IntoFieldImpl
        shared $shared:tt
        $variant_kind:ident $variant_kind_params:tt
    )=>{
        impl_getter_enum!{
            ,GetFieldImpl
            shared $shared
            $variant_kind $variant_kind_params
        }
        impl_getter_enum!{
            ,IntoFieldImpl
            shared $shared
            $variant_kind $variant_kind_params
        }
    };
    (top;
        unsafe,IntoFieldMut
        shared $shared:tt
        $variant_kind:ident $variant_kind_params:tt
    )=>{
        impl_getter_enum!{
            ,GetFieldImpl
            shared $shared
            $variant_kind $variant_kind_params
        }
        impl_getter_enum!{
            ,IntoFieldImpl
            shared $shared
            $variant_kind $variant_kind_params
        }
        impl_getter_enum!{
            unsafe,GetFieldMutImpl
            shared $shared
            $variant_kind $variant_kind_params
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_getters_for_derive_enum{
    (
        impl $typarams:tt $self_:ty
        where $where_preds:tt
        {
            enum=$enum_:ident
            $((
                $getter_trait:ident,
                $variant:ident,
                $variant_tstr:ty,
                $variant_kind:ident $variant_kind_params:tt
            ))*
        }

    )=>{

        $crate::impl_structural!{
            impl $typarams Structural for $self_
            where $where_preds
            {
                variants=[ $( $variant, )* ]
            }
        }

        $(
            $crate::impl_getter_enum!{
                top;
                unsafe,$getter_trait
                shared(
                    impl $typarams $self_
                    where $where_preds

                    enum=$enum_
                    variant=$variant
                    variant_name($variant_tstr)
                )
                $variant_kind $variant_kind_params
            }
        )*
    }
}
