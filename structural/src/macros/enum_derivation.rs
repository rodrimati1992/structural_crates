/// Implements enum variant+field getter(s)
#[doc(hidden)]
#[macro_export]
macro_rules! _private_impl_getter_enum{
    ////////////////////////////////////////////////////////////////////////////
    ////            All the ways to implement GetFieldImpl
    ////////////////////////////////////////////////////////////////////////////
    (
        GetFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype(
            $field_name:tt : $field_ty:ty,
            $optionality:ident
            $(,$field_name_param:ty)? $( , )*
        )
    )=>{

        impl<$($typarams)*>
            $crate::FieldType< $variant_name_str >
        for $self_
        where
            $($where_)*
        {
            type Ty=$field_ty;
        }

        impl<$($typarams)*>
            $crate::GetFieldImpl< $variant_name_str >
        for $self_
        where
            $($where_)*
        {
            type Err=$crate::pmr::OptionalField;

            #[inline(always)]
            fn get_field_(
                &self,
                _:$variant_name_str,
                _:(),
            )->Result<&$field_ty,$crate::pmr::OptionalField>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        $crate::handle_optionality!($optionality,ref,field)
                    }
                    #[allow(unreachable_patterns)]
                    _=>Err($crate::pmr::OptionalField),
                }
            }
        }

        unsafe impl<$($typarams)* _F>
            $crate::pmr::GetVariantFieldImpl<$variant_name_str,_F>
        for $self_
        where
            $field_ty: $crate::GetFieldImpl<_F>,
            $($where_)*
        {}

        impl<$($typarams)* _F>
            $crate::FieldType< $crate::pmr::VariantField<$variant_name_str,_F> >
        for $self_
        where
            $field_ty: $crate::GetFieldImpl<_F>,
            $($where_)*
        {
            type Ty=$crate::GetFieldType<$field_ty,_F>;
        }

        impl<$($typarams)* _F,_Ty>
            $crate::GetFieldImpl<
                $crate::pmr::VariantField<$variant_name_str,_F>,
                $crate::pmr::UncheckedVariantField<$variant_name_str,_F>,
            >
        for $self_
        where
            $field_ty: $crate::GetFieldImpl<_F,Ty=_Ty>,
            $($where_)*
        {
            type Err=$crate::vf_err!($optionality,$field_ty,_F);

            #[inline(always)]
            fn get_field_(
                &self,
                name:$crate::pmr::VariantField<$variant_name_str,_F>,
                _:$crate::pmr::UncheckedVariantField<$variant_name_str,_F>,
            )->Result<
                &_Ty,
                $crate::vf_err!($optionality,$field_ty,_F),
            >{
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        let name = name.field;
                        let field = $crate::try_optionality!($optionality,ref,field);
                        $crate::map_optionality!(
                            $optionality,
                            $crate::GetFieldImpl::get_field_(field,name,())
                        )
                    }
                    #[allow(unreachable_patterns)]
                    _=>unsafe{
                        // The methods in *VariantField require the receiver
                        // to be the variant specified by the first type parameter
                        $crate::pmr::unreachable_unchecked()
                    }
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
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        regular(
            $field_name:tt : $field_ty:ty,
            $optionality:ident,
            $field_name_param:ty $( , )*
        )
    )=>{
        unsafe impl<$($typarams)*>
            $crate::pmr::GetVariantFieldImpl<$variant_name_str,$field_name_param>
        for $self_
        where
            $($where_)*
        {}

        impl<$($typarams)*>
            $crate::FieldType<
                $crate::pmr::VariantField<$variant_name_str,$field_name_param>
            >
        for $self_
        where
            $($where_)*
        {
            type Ty=$field_ty;
        }

        impl<$($typarams)*>
            $crate::GetFieldImpl<
                $crate::pmr::VariantField<$variant_name_str,$field_name_param>,
                $crate::pmr::UncheckedVariantField<$variant_name_str,$field_name_param>,
            >
        for $self_
        where
            $($where_)*
        {
            type Err=$crate::err_from_opt!($optionality);

            #[inline(always)]
            fn get_field_(
                &self,
                _:$crate::pmr::VariantField<$variant_name_str,$field_name_param>,
                _:$crate::pmr::UncheckedVariantField<$variant_name_str,$field_name_param>,
            )->Result<&$field_ty,$crate::err_from_opt!($optionality)>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        $crate::handle_optionality!($optionality,ref,field)
                    }
                    #[allow(unreachable_patterns)]
                    _=>unsafe{
                        // The methods in *VariantField require the receiver
                        // to be the variant specified by the first type parameter
                        $crate::pmr::unreachable_unchecked()
                    }
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
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype(
            $field_name:tt : $field_ty:ty,
            $optionality:ident
            $(,$field_name_param:ty)? $( , )*
        )
    )=>{
        unsafe impl<$($typarams)*>
            $crate::GetFieldMutImpl< $variant_name_str >
        for $self_
        where
            $($where_)*
        {
            #[inline(always)]
            fn get_field_mut_(
                &mut self,
                _:$variant_name_str,
                _:(),
            )->Result<&mut $field_ty,$crate::pmr::OptionalField>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        $crate::handle_optionality!($optionality,mut,field)
                    }
                    #[allow(unreachable_patterns)]
                    _=>Err($crate::pmr::OptionalField),
                }
            }

            #[inline(always)]
            unsafe fn get_field_raw_mut(
                this:*mut *mut (),
                _:$variant_name_str,
                _:(),
            )->Result<*mut $field_ty,$crate::pmr::OptionalField>{
                let this=*(this as *mut *mut Self);
                match *this {
                    $enum_::$variant{$field_name:ref mut field}=>{
                        $crate::handle_optionality!(
                            $optionality,
                            raw,
                            field as *mut $crate::option_or_value_ty!($optionality,$field_ty),
                        )
                    }
                    #[allow(unreachable_patterns)]
                    _=>Err( $crate::pmr::OptionalField ),
                }
            }

            #[inline(always)]
            fn get_field_raw_mut_func(
                &self,
            )->$crate::field_traits::GetFieldRawMutFn<
                $variant_name_str,
                (),
                $field_ty,
                $crate::pmr::OptionalField,
            >{
                <Self as
                    $crate::field_traits::GetFieldMutImpl<
                        $variant_name_str
                    >
                >::get_field_raw_mut
            }
        }


        unsafe impl<$($typarams)* _F>
            $crate::pmr::GetVariantFieldMutImpl<$variant_name_str,_F>
        for $self_
        where
            $field_ty: $crate::GetFieldMutImpl<_F>,
            $($where_)*
        {}

        unsafe impl<$($typarams)* _F,_Ty>
            $crate::pmr::GetFieldMutImpl<
                $crate::pmr::VariantField<$variant_name_str,_F>,
                $crate::pmr::UncheckedVariantField<$variant_name_str,_F>,
            >
        for $self_
        where
            $field_ty: $crate::GetFieldMutImpl<_F,Ty=_Ty>,
            $($where_)*
        {
            #[inline(always)]
            fn get_field_mut_(
                &mut self,
                name:$crate::pmr::VariantField<$variant_name_str,_F>,
                _:$crate::pmr::UncheckedVariantField<$variant_name_str,_F>,
            )->Result<
                &mut _Ty,
                $crate::vf_err!($optionality,$field_ty,_F),
            >{
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        let name = name.field;
                        let field= $crate::try_optionality!($optionality,mut,field);
                        $crate::map_optionality!(
                            $optionality,
                            $crate::GetFieldMutImpl::<
                                _F
                            >::get_field_mut_(field,name,())
                        )
                    }
                    #[allow(unreachable_patterns)]
                    _=>unsafe{
                        // The methods in *VariantField require the receiver
                        // to be the variant specified by the first type parameter
                        $crate::pmr::unreachable_unchecked()
                    }
                }
            }

            #[inline(always)]
            unsafe fn get_field_raw_mut(
                this:*mut *mut (),
                name:$crate::pmr::VariantField<$variant_name_str,_F>,
                _:$crate::pmr::UncheckedVariantField<$variant_name_str,_F>,
            )->Result<
                *mut $crate::GetFieldType<$field_ty,_F>,
                $crate::vf_err!($optionality,$field_ty,_F),
            >{
                match **(this as *mut *mut Self) {
                    $enum_::$variant{$field_name:ref mut field,..}=>{
                        let name = name.field;
                        let mut field:*mut _= $crate::try_optionality!($optionality,raw,field);
                        let field=(&mut field) as *mut *mut $field_ty as *mut *mut ();
                        $crate::map_optionality!(
                            $optionality,
                            <$field_ty as $crate::GetFieldMutImpl<_>>::get_field_raw_mut(
                                field,
                                name,
                                (),
                            )
                        )
                    }
                    #[allow(unreachable_patterns)]
                    _=>{
                        // The methods in *VariantField require the receiver
                        // to be the variant specified by the first type parameter
                        $crate::pmr::unreachable_unchecked()
                    }
                }
            }

            #[inline(always)]
            fn get_field_raw_mut_func(
                &self,
            )->$crate::field_traits::GetFieldRawMutFn<
                $crate::pmr::VariantField<$variant_name_str,_F>,
                $crate::pmr::UncheckedVariantField<$variant_name_str,_F>,
                $crate::GetFieldType<$field_ty,_F>,
                $crate::vf_err!($optionality,$field_ty,_F),
            >{
                <Self as
                    $crate::field_traits::GetFieldMutImpl<
                        $crate::pmr::VariantField<$variant_name_str,_F>,
                        $crate::pmr::UncheckedVariantField<$variant_name_str,_F>,
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
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        regular(
            $field_name:tt : $field_ty:ty,
            $optionality:ident,
            $field_name_param:ty $( , )*
        )
    )=>{
        unsafe impl<$($typarams)*>
            $crate::pmr::GetVariantFieldMutImpl<$variant_name_str,$field_name_param>
        for $self_
        where
            $($where_)*
        {}

        unsafe impl<$($typarams)*>
            $crate::pmr::GetFieldMutImpl<
                $crate::pmr::VariantField<$variant_name_str,$field_name_param>,
                $crate::pmr::UncheckedVariantField<$variant_name_str,$field_name_param>,
            >
        for $self_
        where
            $($where_)*
        {
            #[inline(always)]
            fn get_field_mut_(
                &mut self,
                _:$crate::pmr::VariantField<$variant_name_str,$field_name_param>,
                _:$crate::pmr::UncheckedVariantField<$variant_name_str,$field_name_param>,
            )->Result<&mut $field_ty,$crate::err_from_opt!($optionality)>{
                match self {
                    $enum_::$variant{$field_name:ref mut field,..}=>{
                        $crate::handle_optionality!($optionality,mut,field)
                    }
                    #[allow(unreachable_patterns)]
                    _=>unsafe{
                        // The proxies for each variant ought only be constructible
                        // when the enum is that particular variant.
                        $crate::pmr::unreachable_unchecked()
                    },
                }
            }

            unsafe fn get_field_raw_mut(
                this:*mut *mut (),
                _:$crate::pmr::VariantField<$variant_name_str,$field_name_param>,
                _:$crate::pmr::UncheckedVariantField<$variant_name_str,$field_name_param>,
            )->Result<*mut $field_ty,$crate::err_from_opt!($optionality)>{
                let this=*(this as *mut *mut Self);
                match *this {
                    $enum_::$variant{$field_name:ref mut this,..    }=>{
                        $crate::handle_optionality!(
                            $optionality,
                            raw,
                            // this as *mut $crate::option_or_value_ty!($optionality,$field_ty),
                            this,
                        )
                    }
                    #[allow(unreachable_patterns)]
                    _=>{
                        // The proxies for each variant ought only be constructible
                        // when the enum is that particular variant.
                        $crate::pmr::unreachable_unchecked()
                    }
                }
            }

            #[inline(always)]
            fn get_field_raw_mut_func(
                &self
            )->$crate::field_traits::GetFieldRawMutFn<
                $crate::pmr::VariantField<$variant_name_str,$field_name_param>,
                $crate::pmr::UncheckedVariantField<$variant_name_str,$field_name_param>,
                $field_ty,
                $crate::err_from_opt!($optionality)
            >{
                <Self as
                    $crate::field_traits::GetFieldMutImpl<
                        $crate::pmr::VariantField<$variant_name_str,$field_name_param>,
                        $crate::pmr::UncheckedVariantField<$variant_name_str,$field_name_param>,
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
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        newtype(
            $field_name:tt : $field_ty:ty,
            $optionality:ident
            $(,$field_name_param:ty)? $( , )*
        )
    )=>{
        impl<$($typarams)*>
            $crate::IntoFieldImpl< $variant_name_str >
        for $self_
        where
            $($where_)*
        {
            fn into_field_(
                self,
                _:$variant_name_str,
                _:(),
            )->Result<$field_ty,$crate::pmr::OptionalField>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        $crate::handle_optionality!($optionality,move,field)
                    }
                    #[allow(unreachable_patterns)]
                    _=>Err($crate::pmr::OptionalField),
                }
            }

            $crate::z_impl_box_into_field_method!{
                $variant_name_str,
                (),
                $field_ty,
                $crate::pmr::OptionalField,
            }
        }

        unsafe impl<$($typarams)* _F>
            $crate::pmr::IntoVariantFieldImpl<$variant_name_str,_F>
        for $self_
        where
            $field_ty: $crate::IntoFieldImpl<_F>,
            $($where_)*
        {}

        impl<$($typarams)* _F>
            $crate::pmr::IntoFieldImpl<
                $crate::pmr::VariantField<$variant_name_str,_F>,
                $crate::pmr::UncheckedVariantField<$variant_name_str,_F>,
            >
        for $self_
        where
            $field_ty: $crate::IntoFieldImpl<_F>,
            $($where_)*
        {
            #[inline(always)]
            fn into_field_(
                self,
                name:$crate::pmr::VariantField<$variant_name_str,_F>,
                _:$crate::pmr::UncheckedVariantField<$variant_name_str,_F>,
            )->Result<
                $crate::GetFieldType<$field_ty,_F>,
                $crate::vf_err!($optionality,$field_ty,_F),
            >{
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        let name = name.field;
                        let field= $crate::try_optionality!($optionality,move,field);
                        $crate::map_optionality!(
                            $optionality,
                            $crate::IntoFieldImpl::<
                                _F
                            >::into_field_(field,name,())
                        )
                    }
                    #[allow(unreachable_patterns)]
                    _=>unsafe{
                        // The methods in *VariantField require the receiver
                        // to be the variant specified by the first type parameter
                        $crate::pmr::unreachable_unchecked()
                    }
                }
            }

            $crate::z_impl_box_into_field_method!{
                $crate::pmr::VariantField<$variant_name_str,_F>,
                $crate::pmr::UncheckedVariantField<$variant_name_str,_F>,
                $crate::GetFieldType<$field_ty,_F>,
                $crate::vf_err!($optionality,$field_ty,_F),
            }
        }
    };
    (
        IntoFieldImpl
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
        regular(
            $field_name:tt : $field_ty:ty,
            $optionality:ident,
            $field_name_param:ty $( , )*
        )
    )=>{
        unsafe impl<$($typarams)*>
            $crate::pmr::IntoVariantFieldImpl<$variant_name_str,$field_name_param>
        for $self_
        where
            $($where_)*
        {}

        impl<$($typarams)*>
            $crate::pmr::IntoFieldImpl<
                $crate::pmr::VariantField<$variant_name_str,$field_name_param>,
                $crate::pmr::UncheckedVariantField<$variant_name_str,$field_name_param>,
            >
        for $self_
        where
            $($where_)*
        {
            #[inline(always)]
            fn into_field_(
                self,
                _:$crate::pmr::VariantField<$variant_name_str,$field_name_param>,
                _:$crate::pmr::UncheckedVariantField<$variant_name_str,$field_name_param>,
            )->Result<$field_ty,$crate::err_from_opt!($optionality)>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        $crate::handle_optionality!($optionality,move,field)
                    }
                    #[allow(unreachable_patterns)]
                    _=>unsafe{
                        // The proxies for each variant ought only be constructible
                        // when the enum is that particular variant.
                        $crate::pmr::unreachable_unchecked()
                    }
                }
            }

            $crate::z_impl_box_into_field_method!{
                $crate::pmr::VariantField<$variant_name_str,$field_name_param>,
                $crate::pmr::UncheckedVariantField<$variant_name_str,$field_name_param>,
                $field_ty,
                $crate::err_from_opt!($optionality),
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
        $crate::_private_impl_getter_enum!{
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
        $crate::_private_impl_getter_enum!{
            GetFieldImpl
            shared $shared
            $kind($($field_params)*)
        }
        $crate::_private_impl_getter_enum!{
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
        $crate::_private_impl_getter_enum!{
            GetFieldImpl
            shared $shared
            $kind($($field_params)*)
        }
        $crate::_private_impl_getter_enum!{
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
        $crate::_private_impl_getter_enum!{
            GetFieldImpl
            shared $shared
            $kind($($field_params)*)
        }
        $crate::_private_impl_getter_enum!{
            GetFieldMutImpl
            shared $shared
            $kind($($field_params)*)
        }
        $crate::_private_impl_getter_enum!{
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
                variant=$variant:ident
                variant_name($variant_name_str:ty)
            )

            $field_name:tt : $field_ty:ty ,
            $optionality:ident
            $(,$field_name_param:ty)? $( , )*
        )
    )=>{
        $crate::_private_impl_getter_enum!{
            $trait_
            shared $shared
            newtype(
                $field_name : $field_ty,$optionality $(,$field_name_param)?
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
                $field_name:tt : $field_ty:ty ,
                $optionality:ident
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

                $field_name : $field_ty,$optionality $(,$field_name_param)?
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
macro_rules! private_delegate_to_variant_proxy {
    (is_variant;
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
            variant=$variant:ident
            variant_name($variant_name_str:ty)
        )
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
    (
        shared(
            impl[$($typarams:tt)*] $self_:ty
            where[$($where_:tt)*]

            enum=$enum_:ident
        )
    )=>{
        impl<$($typarams)* _V,_F,_O>
            $crate::GetFieldImpl<$crate::pmr::VariantField<_V,_F>>
        for $self_
        where
            Self: $crate::pmr::GetVariantFieldImpl<_V, _F, Ty=_O>,
            _V: $crate::pmr::IsTStr,
            $($where_)*
        {
            type Err=$crate::pmr::OptionalField;

            #[inline(always)]
            fn get_field_(
                &self,
                name:$crate::pmr::VariantField<_V,_F>,
                _:(),
            )->Result<&_O,$crate::pmr::OptionalField>{
                let vari_name=name.variant;
                if $crate::pmr::IsVariant::is_variant_(self,vari_name) {
                    unsafe{
                        $crate::map_of!(
                            $crate::pmr::GetFieldImpl::get_field_(
                                self,
                                name,
                                $crate::pmr::UncheckedVariantField::<_V,_F>::new(),
                            )
                        )
                    }
                }else{
                    Err($crate::pmr::OptionalField)
                }
            }
        }

        unsafe impl<$($typarams)* _V,_F,_O>
            $crate::GetFieldMutImpl<$crate::pmr::VariantField<_V,_F>>
        for $self_
        where
            Self: $crate::pmr::GetVariantFieldMutImpl<_V, _F, Ty=_O>,
            _V: $crate::pmr::IsTStr,
            $($where_)*
        {
            fn get_field_mut_(
                &mut self,
                name:$crate::pmr::VariantField<_V,_F>,
                _:(),
            )->Result<&mut _O,$crate::pmr::OptionalField>{
                let vari_name=name.variant;
                if $crate::pmr::IsVariant::is_variant_(self,vari_name) {
                    unsafe{
                        $crate::map_of!(
                            $crate::pmr::GetFieldMutImpl::get_field_mut_(
                                self,
                                name,
                                $crate::pmr::UncheckedVariantField::<_V,_F>::new(),
                            )
                        )
                    }
                }else{
                    Err($crate::pmr::OptionalField)
                }
            }

            unsafe fn get_field_raw_mut(
                this:*mut *mut (),
                name:$crate::pmr::VariantField<_V,_F>,
                _:(),
            )->Result<
                *mut _O,
                $crate::pmr::OptionalField
            >{
                let vari_name=name.variant;
                let this_s=*(this as *mut *mut Self);
                if $crate::pmr::IsVariant::is_variant_(&*this_s,vari_name) {
                    $crate::map_of!(
                        <$self_ as $crate::pmr::GetFieldMutImpl<_,_>>::get_field_raw_mut(
                            this,
                            name,
                            $crate::pmr::UncheckedVariantField::<_V,_F>::new()
                        )
                    )
                }else{
                    Err($crate::pmr::OptionalField)
                }
            }

            fn get_field_raw_mut_func(
                &self
            )->$crate::field_traits::GetFieldRawMutFn<
                $crate::pmr::VariantField<_V,_F>,
                (),
                _O,
                $crate::pmr::OptionalField
            >{
                <Self as
                    $crate::GetFieldMutImpl<
                        $crate::pmr::VariantField<_V,_F>
                    >
                >::get_field_raw_mut
            }
        }

        impl<$($typarams)* _V,_F,_O>
            $crate::IntoFieldImpl<$crate::pmr::VariantField<_V,_F>>
        for $self_
        where
            Self: $crate::pmr::IntoVariantFieldImpl<_V, _F, Ty=_O>,
            _V: $crate::pmr::IsTStr,
            $($where_)*
        {
            #[inline(always)]
            fn into_field_(
                self,
                name:$crate::pmr::VariantField<_V,_F>,
                _:(),
            )->Result<_O,$crate::pmr::OptionalField>{
                let vari_name=name.variant;
                if $crate::pmr::IsVariant::is_variant_(&self,vari_name) {
                    unsafe{
                        $crate::map_of!(
                            $crate::pmr::IntoFieldImpl::into_field_(
                                self,
                                name,
                                $crate::pmr::UncheckedVariantField::<
                                    _V,
                                    _F,
                                >::new(),
                            )
                        )
                    }
                }else{
                    Err($crate::pmr::OptionalField)
                }
            }

            $crate::z_impl_box_into_field_method!{
                $crate::pmr::VariantField<_V,_F>,
                (),
                _O,
                $crate::pmr::OptionalField,
            }
        }
    }
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

            $crate::private_delegate_to_variant_proxy!{
                is_variant;
                shared(
                    impl $typarams $self_
                    where $where_preds

                    enum=$enum_
                    variant=$variant
                    variant_name($variant_tstr)
                )
            }

        )*

        $crate::private_delegate_to_variant_proxy!{
            shared(
                impl $typarams $self_
                where $where_preds

                enum=$enum_
            )
        }

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
}
