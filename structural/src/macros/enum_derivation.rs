/// Implements enum variant+field getter(s)
#[doc(hidden)]
#[macro_export]
macro_rules! impl_getter_enum{
    ////////////////////////////////////////////////////////////////////////////
    ////            All the ways to implement GetFieldImpl
    ////////////////////////////////////////////////////////////////////////////
    (
        GetFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype( $field_name:tt : $field_ty:ty $(,$field_name_param:ty)? $( , )* )
    )=>{
        impl<$($typarams)*>
            $crate::GetFieldImpl< $crate::pmr::FieldPath1<$variant_name_str> >
        for $self_
        where
            $($where_)*
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
    (
        GetFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        regular(
            $field_name:tt : $field_ty:ty, $field_name_param:ty  $( , )*
        )
    )=>{
        impl<$($typarams)*>
            $crate::GetFieldImpl< $crate::pmr::FieldPath1<$field_name_param> >
        for $proxy<$self_,$variant_name_str>
        where
            $($where_)*
        {
            type Ty=$field_ty;
            type Err=$crate::field_traits::NonOptField;

            #[inline(always)]
            fn get_field_(&self)->Result<&$field_ty,$crate::field_traits::NonOptField>{
                match self.value {
                    $enum_::$variant{$field_name:ref field,..}=>Ok(field),
                    _=>unsafe{
                        // The proxies for each variant ought only be constructible
                        // when the enum is that particular variant.
                        $crate::pmr::unreachable_unchecked()
                    },
                }
            }
        }
    };
    (
        GetFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        make_proxy $anything:tt
    )=>{
        impl<$($typarams)*>
            $crate::GetFieldImpl< $crate::pmr::FieldPath1<$variant_name_str> >
        for $self_
        where
            $($where_)*
        {
            type Ty=$proxy<$self_,$variant_name_str>;
            type Err=$crate::field_traits::OptionalField;

            #[inline(always)]
            fn get_field_(&self)->Result<
                &$proxy<$self_,$variant_name_str>,
                $crate::field_traits::OptionalField
            >{
                match self {
                    $enum_::$variant{..}=>unsafe{
                        Ok($proxy::<$self_,$variant_name_str>::from_ref(self))
                    }
                    _=>Err($crate::field_traits::OptionalField),
                }
            }
        }
    };


    ////////////////////////////////////////////////////////////////////////////
    ////            All the ways to implement GetFieldMutImpl
    ////////////////////////////////////////////////////////////////////////////
    (
        GetFieldMutImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype( $field_name:tt : $field_ty:ty $(,$field_name_param:ty)? $( , )* )
    )=>{
        unsafe impl<$($typarams)*>
            $crate::GetFieldMutImpl< $crate::pmr::FieldPath1<$variant_name_str> >
        for $self_
        where
            $($where_)*
        {
            #[inline(always)]
            fn get_field_mut_(
                &mut self
            )->Result<&mut $field_ty,$crate::field_traits::OptionalField>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>Ok(field),
                    _=>Err($crate::field_traits::OptionalField),
                }
            }

            #[inline(always)]
            unsafe fn get_field_raw_mut(
                this:*mut (),
                _:$crate::pmr::PhantomData< $crate::pmr::FieldPath1<$variant_name_str> >,
            )->Result<*mut $field_ty,$crate::field_traits::OptionalField>{
                let this=this as *mut Self;
                match *this {
                    $enum_::$variant{$field_name:ref mut field}=>Ok(field as *mut $field_ty),
                    _=>Err( $crate::field_traits::OptionalField ),
                }
            }

            #[inline(always)]
            fn get_field_raw_mut_func(
                &self
            )->$crate::field_traits::GetFieldRawMutFn<
                $crate::pmr::FieldPath1<$variant_name_str>,
                $field_ty,
                $crate::field_traits::OptionalField,
            >{
                <Self as
                    $crate::field_traits::GetFieldMutImpl<
                        $crate::pmr::FieldPath1<$variant_name_str>
                    >
                >::get_field_raw_mut
            }
        }
    };
    (
        GetFieldMutImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        regular(
            $field_name:tt : $field_ty:ty, $field_name_param:ty  $( , )*
        )
    )=>{
        unsafe impl<$($typarams)*>
            $crate::GetFieldMutImpl< $crate::pmr::FieldPath1<$field_name_param> >
        for $proxy<$self_,$variant_name_str>
        where

            $($where_)*
        {
            #[inline(always)]
            fn get_field_mut_(
                &mut self
            )->Result<&mut $field_ty,$crate::field_traits::NonOptField>{
                match self.value {
                    $enum_::$variant{$field_name:ref mut field,..}=>Ok(field),
                    _=>unsafe{
                        // The proxies for each variant ought only be constructible
                        // when the enum is that particular variant.
                        $crate::pmr::unreachable_unchecked()
                    },
                }
            }

            unsafe fn get_field_raw_mut(
                this:*mut (),
                _:$crate::pmr::PhantomData<$crate::pmr::FieldPath1<$field_name_param>>,
            )->Result<*mut $field_ty,$crate::field_traits::NonOptField>{
                match *(this as *mut Self as *mut $self_) {
                    $enum_::$variant{$field_name:ref mut this,..    }=>{
                        Ok( this as *mut $field_ty )
                    }
                    _=>{
                        // The proxies for each variant ought only be constructible
                        // when the enum is that particular variant.
                        $crate::pmr::unreachable_unchecked()
                    }
                }
            }

            fn get_field_raw_mut_func(
                &self
            )->$crate::field_traits::GetFieldRawMutFn<
                $crate::pmr::FieldPath1<$field_name_param>,
                $field_ty,
                $crate::field_traits::NonOptField,
            >{
                <Self as
                    $crate::field_traits::GetFieldMutImpl<
                        $crate::pmr::FieldPath1<$field_name_param>
                    >
                >::get_field_raw_mut
            }
        }
    };
    (
        GetFieldMutImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        make_proxy $anything:tt
    )=>{
        unsafe impl<$($typarams)*>
            $crate::GetFieldMutImpl< $crate::pmr::FieldPath1<$variant_name_str> >
        for $self_
        where
            $($where_)*
        {
            #[inline(always)]
            fn get_field_mut_(
                &mut self
            )->Result<
                &mut $proxy<$self_,$variant_name_str>,
                $crate::field_traits::OptionalField
            >{
                match self {
                    $enum_::$variant{..}=>unsafe{
                        Ok($proxy::<$self_,$variant_name_str>::from_mut(self))
                    }
                    _=>Err($crate::field_traits::OptionalField),
                }
            }

            #[inline(always)]
            unsafe fn get_field_raw_mut(
                this:*mut (),
                _:$crate::pmr::PhantomData< $crate::pmr::FieldPath1<$variant_name_str> >,
            )->Result<
                *mut $proxy<$self_,$variant_name_str>,
                $crate::field_traits::OptionalField,
            >{
                let this=this as *mut Self;
                match *this {
                    $enum_::$variant{..}=>{
                        Ok($proxy::<$self_,$variant_name_str>::from_raw_mut(this))
                    }
                    _=>{
                        Err( $crate::field_traits::OptionalField )
                    }
                }
            }

            #[inline(always)]
            fn get_field_raw_mut_func(
                &self
            )->$crate::field_traits::GetFieldRawMutFn<
                $crate::pmr::FieldPath1<$variant_name_str>,
                $proxy<$self_,$variant_name_str>,
                $crate::field_traits::OptionalField
            >{
                <Self as
                    $crate::field_traits::GetFieldMutImpl<
                        $crate::pmr::FieldPath1<$variant_name_str>
                    >
                >::get_field_raw_mut
            }
        }
    };
    ////////////////////////////////////////////////////////////////////////////
    ////            All the ways to implement IntoFieldImpl
    ////////////////////////////////////////////////////////////////////////////
    (
        IntoFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype( $field_name:tt : $field_ty:ty $(,$field_name_param:ty)? $( , )* )
    )=>{
        impl<$($typarams)*>
            $crate::IntoFieldImpl< $crate::pmr::FieldPath1<$variant_name_str> >
        for $self_
        where
            $($where_)*
        {
            fn into_field_(self)->Result<$field_ty,$crate::field_traits::OptionalField>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>Ok(field),
                    _=>Err($crate::field_traits::OptionalField),
                }
            }

            $crate::z_impl_box_into_field_method!{
                $crate::pmr::FieldPath1<$variant_name_str>,
                $field_ty,
                $crate::field_traits::OptionalField,
            }
        }
    };
    (
        IntoFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        regular(
            $field_name:tt : $field_ty:ty, $field_name_param:ty  $( , )*
        )
    )=>{
        impl<$($typarams)*>
            $crate::IntoFieldImpl< $crate::pmr::FieldPath1<$field_name_param> >
        for $proxy<$self_,$variant_name_str>
        where
            $($where_)*
        {
            #[inline(always)]
            fn into_field_(self)->Result<$field_ty,$crate::field_traits::NonOptField>{
                match self.value {
                    $enum_::$variant{$field_name:field,..}=>Ok(field),
                    _=>unsafe{
                        // The proxies for each variant ought only be constructible
                        // when the enum is that particular variant.
                        $crate::pmr::unreachable_unchecked()
                    }
                }
            }

            $crate::z_impl_box_into_field_method!{
                $crate::pmr::FieldPath1<$field_name_param>,
                $field_ty,
                $crate::field_traits::NonOptField,
            }
        }
    };

    (
        IntoFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        make_proxy $anything:tt
    )=>{
        impl<$($typarams)*>
            $crate::IntoFieldImpl< $crate::pmr::FieldPath1<$variant_name_str> >
        for $self_
        where
            $($where_)*
        {
            #[inline(always)]
            fn into_field_(self)->Result<
                $proxy<$self_,$variant_name_str>,
                $crate::field_traits::OptionalField
            >{
                match self {
                    $enum_::$variant{..}=>unsafe{ Ok($proxy::new(self)) }
                    _=>Err($crate::field_traits::OptionalField),
                }
            }

            $crate::z_impl_box_into_field_method!{
                $crate::pmr::FieldPath1<$variant_name_str>,
                $proxy<$self_,$variant_name_str>,
                $crate::field_traits::OptionalField,
            }
        }
    };


    ////////////////////////////////////////////////////////////////////////////
    ////                delegate_to
    ////////////////////////////////////////////////////////////////////////////
    (
        shared $shared:tt
        kind=$kind:tt

        delegate_to( GetFieldImpl, $($field_params:tt)* )
    )=>{
        $crate::impl_getter_enum!{
            GetFieldImpl
            shared $shared
            $kind($($field_params)*)
        }
    };
    (
        shared $shared:tt
        kind=$kind:tt
        delegate_to( GetFieldMutImpl, $($field_params:tt)* )
    )=>{
        $crate::impl_getter_enum!{
            GetFieldImpl
            shared $shared
            $kind($($field_params)*)
        }
        $crate::impl_getter_enum!{
            GetFieldMutImpl
            shared $shared
            $kind($($field_params)*)
        }
    };
    (
        shared $shared:tt
        kind=$kind:tt
        delegate_to( IntoFieldImpl, $($field_params:tt)* )
    )=>{
        $crate::impl_getter_enum!{
            GetFieldImpl
            shared $shared
            $kind($($field_params)*)
        }
        $crate::impl_getter_enum!{
            IntoFieldImpl
            shared $shared
            $kind($($field_params)*)
        }
    };
    (
        shared $shared:tt
        kind=$kind:tt
        delegate_to( IntoFieldMut, $($field_params:tt)* )
    )=>{
        $crate::impl_getter_enum!{
            GetFieldImpl
            shared $shared
            $kind($($field_params)*)
        }
        $crate::impl_getter_enum!{
            GetFieldMutImpl
            shared $shared
            $kind($($field_params)*)
        }
        $crate::impl_getter_enum!{
            IntoFieldImpl
            shared $shared
            $kind($($field_params)*)
        }
    };
    /*/////////////////////////////////////////////////////////////////////////////
    This is an intermediate macro for generating an optional field accessor,
    to get the single field of a newtype variant using the name of the variant.
    */////////////////////////////////////////////////////////////////////////////
    (
        $trait_:ident
        shared $shared:tt
        newtype_as_field(
            shared (
                impl[$($typarams:tt)*] $self_:ty
                where[$($where_:tt)*]

                enum=$enum_:ident
                proxy=$proxy:ident
                variant=$variant:ident
                variant_name($variant_name_str:ty)
            )

            $field_name:tt : $field_ty:ty $(,$field_name_param:ty)? $( , )*
        )
    )=>{
        $crate::impl_getter_enum!{
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
        fields( ( $trait_:ident , $field_name:tt : $field_ty:ty $(,$field_name_param:ty)? $( , )* ) )
    )=>{
        $crate::impl_getter_enum!{
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
            $crate::impl_getter_enum!{
                shared $shared
                kind=regular
                delegate_to $fields
            }
        )*

        $crate::impl_getter_enum!{
            shared $shared
            kind=make_proxy
            delegate_to ( IntoFieldMut, )
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! delegate_to_variant_proxy {
    (
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
        )
        shared $shared:tt
        shared_impls
    )=>{
        impl<$($typarams)* _V>
            $crate::enum_traits::IsVariant<$crate::pmr::FieldPath1Str<_V>>
        for $self_
        where
            Self:$crate::GetFieldImpl<$crate::pmr::FieldPath1Str<_V>>,
        {
            #[inline]
            fn is_variant_(&self)->bool{
                $crate::GetFieldImpl::<$crate::pmr::FieldPath1Str<_V>>::get_field_(self).is_ok()
            }
        }
    };
    (top_level;
        shared $shared:tt
        $($other:tt)*
    )=>{
        $crate::delegate_to_variant_proxy!{
            shared $shared
            shared $shared
            $($other)*
        }
    };
    (
        shared $shared:tt
        shared $shared2:tt
        kind=newtype
        fields( ( $trait:ident, $field_name:tt : $field_ty:ty $(,$field_name_param:ty)? $( , )*) )
    )=>{
        delegate_to_variant_proxy!{
            shared $shared
            proxy_ty=$field_ty
        }
    };
    (
        shared (
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        shared $shared:tt
        kind=regular
        fields()
    )=>{

    };
    (
        shared (
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        shared $shared:tt
        kind=regular
        fields( $($fields:tt)+ )
    )=>{
        delegate_to_variant_proxy!{
            shared $shared
            proxy_ty=$proxy<Self,$variant_name_str>
        }
    };
    (
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            proxy=$proxy:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        proxy_ty=$proxy_ty:ty
    )=>{
        impl<$($typarams)* _F,_O>
            $crate::GetFieldImpl<$crate::pmr::VariantFieldPath<$variant_name_str,_F>>
        for $self_
        where
            Self:$crate::GetFieldImpl<$crate::pmr::FieldPath1<$variant_name_str>,Ty=$proxy_ty>,
            $proxy_ty: $crate::GetFieldImpl<$crate::pmr::FieldPath1<_F>,Ty=_O>,
        {
            type Ty=_O;
            type Err=$crate::pmr::OptionalField;

            #[inline(always)]
            fn get_field_(&self)->Result<&_O,$crate::field_traits::OptionalField>{
                let proxy=$crate::try_of!(
                    $crate::GetFieldImpl::<
                        $crate::pmr::FieldPath1<$variant_name_str>
                    >::get_field_(self)
                );
                $crate::map_of!(
                    $crate::GetFieldImpl::<$crate::pmr::FieldPath1<_F>>::get_field_(proxy) 
                )
            }
        }

        unsafe impl<$($typarams)* _F,_O>
            $crate::GetFieldMutImpl<$crate::pmr::VariantFieldPath<$variant_name_str,_F>>
        for $self_
        where
            Self:$crate::GetFieldImpl<
                $crate::pmr::VariantFieldPath<$variant_name_str,_F>,
                Ty=_O,
                Err=$crate::field_traits::OptionalField,
            >,
            Self:$crate::GetFieldMutImpl<
                $crate::pmr::FieldPath1<$variant_name_str>,
                Ty=$proxy_ty,
            >,
            $proxy_ty: $crate::GetFieldMutImpl<$crate::pmr::FieldPath1<_F>,Ty=_O>,
        {
            fn get_field_mut_(&mut self)->Result<
                &mut _O,
                $crate::field_traits::OptionalField
            >{
                let proxy=$crate::try_of!(
                    $crate::GetFieldMutImpl::<
                        $crate::pmr::FieldPath1<$variant_name_str>
                    >::get_field_mut_(self)
                );
                $crate::map_of!(
                    $crate::GetFieldMutImpl::<
                        $crate::pmr::FieldPath1<_F>
                    >::get_field_mut_(proxy)
                )
            }

            unsafe fn get_field_raw_mut(
                this:*mut (),
                _:$crate::pmr::PhantomData<
                    $crate::pmr::VariantFieldPath<$variant_name_str,_F>
                >,
            )->Result<
                *mut _O,
                $crate::field_traits::OptionalField
            >{
                let proxy=$crate::try_of!(
                    <Self as
                        $crate::GetFieldMutImpl<
                            $crate::pmr::FieldPath1<$variant_name_str>
                        >
                    >::get_field_raw_mut(this,$crate::pmr::PhantomData)
                );
                $crate::map_of!(
                    <$proxy_ty as
                        $crate::GetFieldMutImpl<
                            $crate::pmr::FieldPath1<_F>
                        >
                    >::get_field_raw_mut(proxy as *mut (),$crate::pmr::PhantomData)
                )
            }

            fn get_field_raw_mut_func(
                &self
            )->$crate::field_traits::GetFieldRawMutFn<
                $crate::pmr::VariantFieldPath<$variant_name_str,_F>,
                _O,
                $crate::field_traits::OptionalField
            >{
                <Self as
                    $crate::GetFieldMutImpl<
                        $crate::pmr::VariantFieldPath<$variant_name_str,_F>
                    >
                >::get_field_raw_mut
            }
        }

        impl<$($typarams)* _F,_O>
            $crate::IntoFieldImpl<$crate::pmr::VariantFieldPath<$variant_name_str,_F>>
        for $self_
        where
            Self:$crate::GetFieldImpl<
                $crate::pmr::VariantFieldPath<$variant_name_str,_F>,
                Ty=_O,
                Err=$crate::field_traits::OptionalField,
            >,
            Self:IntoFieldImpl<$crate::pmr::FieldPath1<$variant_name_str>,Ty=$proxy_ty>,
            $proxy_ty: $crate::IntoFieldImpl<$crate::pmr::FieldPath1<_F>,Ty=_O>,
        {
            #[inline(always)]
            fn into_field_(self)->Result<_O,$crate::field_traits::OptionalField>{
                let proxy=$crate::try_of!(
                    $crate::IntoFieldImpl::<
                        $crate::pmr::FieldPath1<$variant_name_str>
                    >::into_field_(self)
                );
                $crate::map_of!(
                    $crate::IntoFieldImpl::<$crate::pmr::FieldPath1<_F>>::into_field_(proxy)
                )
            }

            $crate::z_impl_box_into_field_method!{
                $crate::pmr::VariantFieldPath<$variant_name_str,_F>,
                _O,
                $crate::field_traits::OptionalField,
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_getters_for_derive_enum{
    (
        impl $typarams:tt $self_:ty
        where $where_preds:tt
        {
            enum=$enum_:ident
            proxy=$proxy:ident
            $((
                $variant:ident,
                $variant_tstr:ty,
                kind=$variant_kind:ident,
                fields($( $field:tt )*)
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
                shared(
                    impl $typarams $self_
                    where $where_preds

                    enum=$enum_
                    proxy=$proxy
                    variant=$variant
                    variant_name($variant_tstr)
                )
                kind= $variant_kind
                fields(
                    $( $field )*
                )
            }

            $crate::delegate_to_variant_proxy!{
                top_level;
                shared(
                    impl $typarams $self_
                    where $where_preds

                    enum=$enum_
                    proxy=$proxy
                    variant=$variant
                    variant_name($variant_tstr)
                )
                kind= $variant_kind
                fields(
                    $( $field )*
                )
            }
        )*

        $crate::delegate_to_variant_proxy!{
            top_level;
            shared(
                impl $typarams $self_
                where $where_preds

                enum=$enum_
                proxy=$proxy
            )
            shared_impls
        }


    }
}
