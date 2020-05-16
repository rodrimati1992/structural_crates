/// For creating a raw pointer of an enum field,as either `Some(NonNull<_>)` or `None`.
///
/// # Safety
///
/// The `$pointer` must be raw pointer to a valid(and fully initialized)
/// instance of the `$enum` type.
///
/// # Example
///
/// For an example of using this macro look at
/// [the manual implementation example for `GetVariantFieldMut`
/// ](./field/trait.GetVariantFieldMut.html#manual-impl-example)
#[macro_export]
macro_rules! z_raw_borrow_enum_field {
    (
        $pointer:expr,
        $enum:ident::$variant:ident.$field:tt : $field_ty:ty
    ) => {{
        match *$pointer {
            $enum::$variant {
                $field: ref mut field,
                ..
            } => {
                if false {
                    let _: *mut $field_ty = field;
                }
                let ptr = field as *mut $field_ty;
                Some($crate::pmr::NonNull::new_unchecked(ptr))
            }
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }};
}

/// Implements the [`get_vfield_raw_mut_fn`] and [`get_vfield_raw_mut_unchecked_fn`]
/// methods from the [`GetVariantFieldMut`] trait.
///
/// # Safety
///
/// The `$Self` argument must be the `Self` type in the impl block.
///
/// # Example
///
/// For an example of using this macro look at
/// [the manual implementation example
/// ](./field/trait.GetVariantFieldMut.html#manual-impl-example)
/// for [`GetVariantFieldMut`]
///
/// [`GetVariantFieldMut`]: ./field/trait.GetVariantFieldMut.html
/// [`get_vfield_raw_mut_fn`]:
/// ./field/trait.GetVariantFieldMut.html#tymethod.get_vfield_raw_mut_fn
/// [`get_vfield_raw_mut_unchecked_fn`]:
/// ./field/trait.GetVariantFieldMut.html#tymethod.get_vfield_raw_mut_unchecked_fn
#[macro_export]
macro_rules! z_unsafe_impl_get_vfield_raw_mut_fn {
    (
        Self=$Self:ty,
        variant_tstr=$variant_name:ty,
        field_tstr=$field_name:ty,
        field_type=$field_ty:ty,
    ) => {
        fn get_vfield_raw_mut_fn(
            &self,
        ) -> $crate::pmr::GetVFieldRawMutFn<$variant_name, $field_name, $field_ty> {
            <$Self as
                        $crate::pmr::GetVariantFieldMut<$variant_name,$field_name>
                    >::get_vfield_raw_mut_
        }

        fn get_vfield_raw_mut_unchecked_fn(
            &self,
        ) -> $crate::pmr::GetFieldRawMutFn<$field_name, $field_ty> {
            <$Self as
                                        $crate::pmr::GetVariantFieldMut<$variant_name, $field_name>
                                    >::get_vfield_raw_mut_unchecked
        }
    };
}

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
            $field_name:tt : $field_ty:ty,
            dropping $dropping:tt
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
            dropping $dropping:tt,
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
            $field_name:tt : $field_ty:ty,
            dropping $dropping:tt
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
            dropping $dropping:tt,
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
            $field_name:tt : $field_ty:ty,
            dropping($field_var:ident, $field_index:expr)
            $(,$field_name_param:ty)? $( , )*
        )
    )=>{
        unsafe impl<$($typarams)* __F,__Ty>
            $crate::pmr::IntoVariantField<$variant_name_str,__F>
        for $self_
        where
            $field_ty: $crate::IntoField<__F,Ty=__Ty>,
            Self: $crate::pmr::DropFields,
            $($where_)*
        {
            #[inline(always)]
            fn into_vfield_(self, _:$variant_name_str, name:__F) -> Option<Self::Ty> {
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        Some($crate::IntoField::into_field_(field,name))
                    }
                    #[allow(unreachable_patterns)]
                    _=>None
                }
            }

            #[inline(always)]
            unsafe fn move_out_vfield_(
                &mut self,
                _:$variant_name_str,
                name:__F,
                moved_fields: &mut $crate::pmr::MovedOutFields,
            ) -> Option<Self::Ty> {
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        Some($crate::IntoField::move_out_field_(field,name,moved_fields))
                    }
                    #[allow(unreachable_patterns)]
                    _=>None
                }
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
            dropping($field_var:ident, $field_index:expr),
            $field_name_param:ty $( , )*
        )
    )=>{
        unsafe impl<$($typarams)*>
            $crate::pmr::IntoVariantField<$variant_name_str,$field_name_param>
        for $self_
        where
            Self: $crate::pmr::DropFields,
            $($where_)*
        {
            #[inline(always)]
            fn into_vfield_(self, _:$variant_name_str, _:$field_name_param)->Option<$field_ty>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>Some(field),
                    #[allow(unreachable_patterns)]
                    _=>None
                }
            }

            #[inline(always)]
            unsafe fn move_out_vfield_(
                &mut self,
                _:$variant_name_str,
                _:$field_name_param,
                moved_fields: &mut $crate::pmr::MovedOutFields,
            )->Option<$field_ty>{
                match self {
                    $enum_::$variant{$field_name:field,..}=>{
                        {
                            use $crate::pmr::FieldBit;
                            const BIT: FieldBit = FieldBit::new($field_index);
                            moved_fields.set_moved_out(BIT);
                        }
                        Some((field as *mut $field_ty).read())
                    },
                    #[allow(unreachable_patterns)]
                    _=>None
                }
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

            $($field_params:tt)*
        )
    )=>{
        $crate::_private_impl_getter_enum!{
            $trait_
            shared $shared
            newtype(
                $($field_params)*
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
                $trait_:ident,
                $field_name:tt : $field_ty:ty,
                dropping $dropping:tt
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

                $field_name : $field_ty, dropping $dropping $(,$field_name_param)?
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
            drop_fields=$drop_kind:tt
            $(variant_count=$variant_count:ty,)?
            $((
                $variant:ident,
                $variant_tstr:ty,
                kind=$variant_kind:ident,
                not_public( $(($priv_field:tt = $priv_field_var:ident))* ),
                fields($( $field:tt )*)
            ))*
        }

    )=>{

        $crate::_private_impl_structural!{
            $(#[doc=$docs])*
            impl $typarams Structural for $self_
            where $where_preds
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

        $crate::_private_impl_drop_fields!{
            struct_or_enum(enum),
            for_drop=$drop_kind

            impl $typarams DropFields for $self_
            where $where_preds
            {
                $(
                    $variant(
                        kind=$variant_kind,

                        not_public( $(($priv_field = $priv_field_var))* ),

                        fields( $( $field )* ),
                    )
                )*
            }
        }


        $(
            $crate::_private_impl_getters_for_derive_enum!{
                @inner
                impl $typarams $self_
                where $where_preds

                variant_count=$variant_count,
            }
        )?
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
