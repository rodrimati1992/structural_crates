#[macro_use]
mod delegate_structural;

#[macro_use]
mod list;

#[macro_use]
mod ident;

#[macro_use]
mod make_struct;

#[macro_use]
mod structural_alias;

/// Implements an infallible getter
#[doc(hidden)]
#[macro_export]
macro_rules! impl_getter{
    (
        $(unsafe)?
        impl[$($typarams:tt)*]
            GetFieldImpl <$field_name:tt : $field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        impl<$($typarams)*> $crate::GetFieldImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            type Ty=$field_ty;
            type Err=$crate::field_traits::NonOptField;

            fn get_field_(&self)->Result<&Self::Ty,Self::Err>{
                Ok(&self.$field_name)
            }
        }
    };
    (
        unsafe impl[$($typarams:tt)*]
            GetFieldMutImpl <$field_name:tt : $field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            impl[$($typarams)*] GetFieldImpl<$field_name:$field_ty,$name_param> for $self_
            $( where[$($where_)*] )?
        }

        unsafe impl<$($typarams)*> $crate::GetFieldMutImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn get_field_mut_(&mut self)->Result<&mut Self::Ty,Self::Err>{
                Ok(&mut self.$field_name)
            }

            $crate::z_unsafe_impl_get_field_raw_mut_method!{
                Self,
                field_name=$field_name,
                name_generic=$name_param
            }
        }
    };
    (
        $(unsafe)?
        impl[$($typarams:tt)*]
            IntoFieldImpl <$field_name:tt : $field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            impl[$($typarams)*]
                GetFieldImpl<$field_name:$field_ty,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }

        impl<$($typarams)*> $crate::IntoFieldImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn into_field_(self)->Result<Self::Ty,Self::Err>{
                Ok(self.$field_name)
            }
            $crate::z_impl_box_into_field_method!{$name_param}
        }
    };
    (
        unsafe impl[$($typarams:tt)*]
            IntoFieldMut <$field_name:tt : $field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            unsafe impl[$($typarams)*]
                GetFieldMutImpl<$field_name:$field_ty,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }

        impl<$($typarams)*> $crate::IntoFieldImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn into_field_(self)->Result<Self::Ty,Self::Err>{
                Ok(self.$field_name)
            }
            $crate::z_impl_box_into_field_method!{$name_param}
        }
    };
}

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

/// For manual implementors of the GetFieldMutImpl trait,
/// implementing the methods used for accession multiple mutable fields.
///
/// # Safety
///
/// This is an unsafe macro,
/// because it requires each invocation of it to borrow a different field for the type
/// (the `field_name=` argument),
/// otherwise this would cause undefined behavior because it would
/// create multiple mutable borrows to the same field.
///
/// # Example
///
/// For an example where this macro is used,
/// you can look at the
/// [manual implementation example of the GetFieldMutImpl trait
/// ](./field_traits/trait.GetFieldMutImpl.html#manual-implementation-example)
#[macro_export]
macro_rules! z_unsafe_impl_get_field_raw_mut_method {
    ( $Self:ident,field_name=$field_name:tt,name_generic=$name_param:ty ) => (
        unsafe fn get_field_raw_mut(
            this:*mut (),
            _:$crate::pmr::PhantomData<$name_param>,
        )->Result<*mut $Self::Ty,Self::Err>{
            Ok(&mut (*(this as *mut $Self)).$field_name as *mut $Self::Ty)
        }

        fn get_field_raw_mut_func(
            &self
        )->$crate::field_traits::GetFieldMutRefFn<$name_param,$Self::Ty,$Self::Err>{
            <$Self as $crate::field_traits::GetFieldMutImpl<$name_param>>::get_field_raw_mut
        }
    )
}

#[macro_export]
macro_rules! z_unsafe_impl_get_field_raw_mut_method_enum {
    (inner;
        $this:ident $Self:ident
        transparency($enum_:ident, $variant:ident, ())
    )=>{
        match *($this as *mut $Self) {
            $enum_::$variant{..}=>{
                Ok( $crate::utils::MakeUnit::UNIT )
            }
            _=>{
                Err( $crate::field_traits::OptionalField )
            }
        }
    };
    (inner;
        $this:ident $Self:ident
        transparency($enum_:ident, $variant:ident, $field_name:tt)
    )=>{
        match *($this as *mut $Self) {
            $enum_::$variant{$field_name:ref mut this,..    }=>{
                Ok( this as *mut $Self::Ty )
            }
            _=>{
                Err( $crate::field_traits::OptionalField )
            }
        }
    };
    (
        $Self:ident,
        transparency $transparency_params:tt,
        name_generic=$name_param:ty,
    ) => (
        unsafe fn get_field_raw_mut(
            this:*mut (),
            _:$crate::pmr::PhantomData<$name_param>,
        )->Result<*mut $Self::Ty,$crate::field_traits::OptionalField>{
            z_unsafe_impl_get_field_raw_mut_method_enum!{
                inner;
                this $Self
                transparency $transparency_params
            }
        }

        fn get_field_raw_mut_func(
            &self
        )->$crate::field_traits::GetFieldMutRefFn<
            $name_param,
            $Self::Ty,
            $crate::field_traits::OptionalField
        >{
            <$Self as $crate::field_traits::GetFieldMutImpl<$name_param>>::get_field_raw_mut
        }
    )
}

/// For use in manual implementations of the IntoFieldImpl trait.
///
/// Implements the `IntoFieldImpl::box_into_field_` method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// For an example of using this macro look at
/// [the documentation for IntoFieldImpl
/// ](./field_traits/trait.IntoFieldImpl.html#manual-implementation-example)
#[macro_export]
#[cfg(not(feature = "alloc"))]
macro_rules! z_impl_box_into_field_method {
    ($($anything:tt)*) => {};
}

/// For use in manual implementations of the IntoFieldImpl trait.
///
/// Implements the `IntoFieldImpl::box_into_field_` method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// For an example of using this macro look at
/// [the documentation for IntoFieldImpl
/// ](./field_traits/trait.IntoFieldImpl.html#manual-implementation-example)
#[macro_export]
#[cfg(feature = "alloc")]
macro_rules! z_impl_box_into_field_method {
    ($field_name:ty) => (
        fn box_into_field_(
            self:$crate::pmr::Box<Self>
        )->Result<Self::Ty,<Self as $crate::GetFieldImpl<$field_name>>::Err>{
            $crate::IntoFieldImpl::<$field_name>::into_field_(*self)
        }
    )
}

// Implements the Structural traits
#[doc(hidden)]
#[macro_export]
macro_rules! impl_structural{
    (
        impl[$($typarams:tt)*] Structural for $self_:ty
        where[$($where_:tt)*]
        {
            field_names=[$(
                (
                    $field_name:tt,
                    $name_param_str:expr,
                    opt=$is_optional:expr,
                ),
            )*]
        }
    )=>{
        impl<$($typarams)*> $crate::Structural for $self_
        where $($where_)*
        {
            const FIELDS: &'static $crate::structural_trait::FieldInfos= {
                use $crate::structural_trait::{FieldInfo,FieldInfos,IsOptional};

                &FieldInfos::Struct(&[
                    $(
                        FieldInfo{
                            name: $crate::structural_trait::Name{
                                original:stringify!($field_name),
                                accessor:$name_param_str,
                            },
                            optionality:IsOptional::new($is_optional),
                        },
                    )*
                ])
            };
        }
    }
}

// Implements the Structural and accessor traits for a struct
#[doc(hidden)]
#[macro_export]
macro_rules! impl_getters_for_derive_struct{
    (
        impl $typarams:tt $self_:ty
        where $where_preds:tt
        {
            $((
                $getter_trait:ident<
                    $field_name:tt : $field_ty:ty,
                    $name_param_ty:ty,
                    $name_param_str:expr,
                    opt=$is_optional:expr,
                >
            ))*
        }
    )=>{

        $crate::impl_structural!{
            impl $typarams Structural for $self_
            where $where_preds
            {
                field_names=[
                    $(
                        (
                            $field_name,
                            $name_param_str,
                            opt=$is_optional,
                        ),
                    )*
                ]
            }
        }

        $(
            $crate::impl_getter!{
                unsafe impl $typarams
                    $getter_trait<$field_name : $field_ty,$name_param_ty>
                for $self_
                where $where_preds
            }
        )*
    }
}

////////////////////////////////////////////////////////////////////////////////
////                    Macros for enums
////////////////////////////////////////////////////////////////////////////////

/// Implements an infallible getter
#[doc(hidden)]
#[macro_export]
macro_rules! impl_getter_enum{
    // For unit fields
    (safe_access_field;
        GetFieldMutImpl, $this:ident, $Self:ident,
        transparency( $enum_:ident, $variant:ident, () )
    )=>{
        match $this {
            $enum_::$variant{..}=>Ok($crate::utils::unit_mut_ref()),
            _=>Err($crate::field_traits::OptionalField),
        }
    };
    (safe_access_field;
        $trait_:ident, $this:ident, $Self:ident,
        transparency( $enum_:ident, $variant:ident, () )
    )=>{
        match $this {
            $enum_::$variant{..}=>Ok($crate::utils::MakeUnit::UNIT),
            _=>Err($crate::field_traits::OptionalField),
        }
    };
    // For single field variants
    (safe_access_field;
        $trait_:ident, $this:ident, $Self:ident,
        transparency( $enum_:ident, $variant:ident, $field_name:tt )
    )=>{
        match $this {
            $enum_::$variant{$field_name:field,..}=>Ok(field),
            _=>Err($crate::field_traits::OptionalField),
        }
    };
    (
        $(unsafe)?
        impl[$($typarams:tt)*]
            Structural
        $($rem:tt)*
    )=>{
    };
    (
        $(unsafe)?
        impl[$($typarams:tt)*]
            GetFieldImpl <$field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?

        ;transparency $transparency_params:tt
    )=>{
        impl<$($typarams)*> $crate::GetFieldImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            type Ty=$field_ty;
            type Err=$crate::field_traits::OptionalField;

            fn get_field_(
                &self
            )->Result<&Self::Ty,$crate::field_traits::OptionalField>{
                let this=self;
                impl_getter_enum!{
                    safe_access_field;
                    GetFieldImpl,this,Self,
                    transparency $transparency_params
                }
            }
        }
    };
    (
        unsafe impl[$($typarams:tt)*]
            GetFieldMutImpl <$field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?

        ;transparency $transparency_params:tt
    )=>{
        $crate::impl_getter_enum!{
            impl[$($typarams)*] GetFieldImpl<$field_ty,$name_param> for $self_
            $( where[$($where_)*] )?
            ;transparency $transparency_params
        }

        unsafe impl<$($typarams)*> $crate::GetFieldMutImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn get_field_mut_(
                &mut self
            )->Result<&mut Self::Ty,$crate::field_traits::OptionalField>{
                let this=self;
                impl_getter_enum!{
                    safe_access_field;
                    GetFieldMutImpl,this,Self,
                    transparency $transparency_params
                }
            }

            $crate::z_unsafe_impl_get_field_raw_mut_method_enum!{
                Self,
                transparency $transparency_params,
                name_generic=$name_param,
            }
        }
    };
    (
        $(unsafe)?
        impl[$($typarams:tt)*]
            IntoFieldImpl <$field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?

        ;transparency $transparency_params:tt
    )=>{
        $crate::impl_getter_enum!{
            impl[$($typarams)*]
                GetFieldImpl<$field_ty,$name_param>
            for $self_
            $( where[$($where_)*] )?
            ;transparency $transparency_params
        }

        impl<$($typarams)*> $crate::IntoFieldImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn into_field_(self)->Result<Self::Ty,$crate::field_traits::OptionalField>{
                let this=self;
                impl_getter_enum!{
                    safe_access_field;
                    IntoFieldImpl,this,Self,
                    transparency $transparency_params
                }
            }
            $crate::z_impl_box_into_field_method!{$name_param}
        }
    };
    (
        unsafe impl[$($typarams:tt)*]
            IntoFieldMut <$field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?

        ;transparency $transparency_params:tt
    )=>{
        $crate::impl_getter_enum!{
            unsafe impl[$($typarams)*]
                GetFieldMutImpl<$field_ty,$name_param>
            for $self_
            $( where[$($where_)*] )?
            ;transparency $transparency_params
        }

        impl<$($typarams)*> $crate::IntoFieldImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn into_field_(self)->Result<Self::Ty,$crate::field_traits::OptionalField>{
                let this=self;
                impl_getter_enum!{
                    safe_access_field;
                    IntoFieldImpl,this,Self,
                    transparency $transparency_params
                }
            }
            $crate::z_impl_box_into_field_method!{$name_param}
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
            $((
                $getter_trait:ident,
                $variant_name:tt:$field_ty:ty,
                $name_param_ty:ty,
                $name_param_str:expr,
                transparency $transparency_params:tt
            ))*
        }

    )=>{

        $crate::impl_structural!{
            impl $typarams Structural for $self_
            where $where_preds
            {
                field_names=[
                    $(
                        (
                            $variant_name,
                            $name_param_str,
                            opt=true,
                        ),
                    )*
                ]
            }
        }

        $(
            $crate::impl_getter_enum!{
                unsafe impl $typarams
                    $getter_trait<$field_ty,$name_param_ty>
                for $self_
                where $where_preds

                ;transparency $transparency_params
            }
        )*
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! try_fe {
    ( $expr:expr ) => {
        match $expr {
            Ok(x) => x,
            Err(e) => return Err($crate::field_traits::IntoFieldErr::into_field_err(e)),
        }
    };
}

