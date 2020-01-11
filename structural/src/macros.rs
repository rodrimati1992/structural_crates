#[macro_use]
mod delegate_structural;

#[macro_use]
mod list;

#[macro_use]
mod ident;

#[macro_use]
mod impl_struct;

#[macro_use]
mod make_struct;

#[macro_use]
mod structural_alias;

#[macro_use]
mod enum_derivation;

/// Implements an infallible getter
#[doc(hidden)]
#[macro_export]
macro_rules! impl_getter{
    (
        $(unsafe)?
        impl[$($typarams:tt)*]
            GetFieldImpl <$field_name:tt : $field_ty:ty,$optionality:ident,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        impl<$($typarams)*> $crate::FieldType<$name_param> for $self_
        $( where $($where_)* )?
        {
            type Ty=$field_ty;
        }

        impl<$($typarams)*> $crate::GetFieldImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            type Err=$crate::optionality_ty!($optionality);

            fn get_field_(
                &self,
                _:$name_param,
                _:(),
            )->Result<&Self::Ty,$crate::optionality_ty!($optionality)>{
                $crate::handle_optionality!($optionality,ref,&self.$field_name)
            }
        }
    };
    (
        unsafe impl[$($typarams:tt)*]
            GetFieldMutImpl <$field_name:tt : $field_ty:ty,$optionality:ident,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            impl[$($typarams)*] GetFieldImpl<$field_name:$field_ty,$optionality,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }

        unsafe impl<$($typarams)*> $crate::GetFieldMutImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn get_field_mut_(
                &mut self,
                _:$name_param,
                _:(),
            )->Result<&mut Self::Ty,$crate::optionality_ty!($optionality)>{
                $crate::handle_optionality!($optionality,mut,&mut self.$field_name)
            }

            $crate::z_unsafe_impl_get_field_raw_mut_method!{
                Self,
                field_name=$field_name,
                name_generic=$name_param,
                optionality=$optionality,
            }
        }
    };
    (
        $(unsafe)?
        impl[$($typarams:tt)*]
            IntoFieldImpl <$field_name:tt : $field_ty:ty,$optionality:ident,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            impl[$($typarams)*]
                GetFieldImpl<$field_name:$field_ty,$optionality,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }

        impl<$($typarams)*> $crate::IntoFieldImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn into_field_(
                self,
                _:$name_param,
                _:(),
            )->Result<Self::Ty,$crate::optionality_ty!($optionality)>{
                $crate::handle_optionality!($optionality,move,self.$field_name)
            }
            $crate::z_impl_box_into_field_method!{$name_param}
        }
    };
    (
        unsafe impl[$($typarams:tt)*]
            IntoFieldMut <$field_name:tt : $field_ty:ty,$optionality:ident,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            unsafe impl[$($typarams)*]
                GetFieldMutImpl<$field_name:$field_ty,$optionality,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }

        impl<$($typarams)*> $crate::IntoFieldImpl<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn into_field_(
                self,
                _:$name_param,
                _:(),
            )->Result<Self::Ty,$crate::optionality_ty!($optionality)>{
                $crate::handle_optionality!($optionality,move,self.$field_name)
            }
            $crate::z_impl_box_into_field_method!{$name_param}
        }
    };
}

macro_rules! unsafe_delegate_variant_field {
    (
        impl[$($typarams:tt)*] GetVariantField for $self_:ty
        where[$($where_:tt)*]
        delegate_to=$delegating_to:ty,
    )=>{
        unsafe impl<$($typarams)* _V,_F>
            $crate::pmr::GetVariantFieldImpl<_V,_F>
        for $self_
        where
            $delegating_to: $crate::pmr::GetVariantFieldImpl<_V,_F>
            $($where_)*
        {}
    };
    (
        impl[$($typarams:tt)*] GetVariantFieldMut for $self_:ty
        where[$($where_:tt)*]
        delegate_to=$delegating_to:ty,
    )=>{
        unsafe_delegate_variant_field!{
            impl[$($typarams)*] GetVariantField for $self_
            where[$($where_)*]
            delegate_to=$delegating_to,
        }

        unsafe impl<$($typarams)* _V,_F>
            $crate::pmr::GetVariantFieldMutImpl<_V,_F>
        for $self_
        where
            $delegating_to: $crate::pmr::GetVariantFieldMutImpl<_V,_F>
            $($where_)*
        {}
    };
    (
        impl[$($typarams:tt)*] IntoVariantField for $self_:ty
        where[$($where_:tt)*]
        delegate_to=$delegating_to:ty,
    )=>{
        unsafe_delegate_variant_field!{
            impl[$($typarams)*] GetVariantField for $self_
            where[$($where_)*]
            delegate_to=$delegating_to,
        }


        unsafe impl<$($typarams)* _V,_F>
            $crate::pmr::IntoVariantFieldImpl<_V,_F>
        for $self_
        where
            $delegating_to: $crate::pmr::IntoVariantFieldImpl<_V,_F>
            $($where_)*
        {}
    };
    (
        impl[$($typarams:tt)*] IntoVariantFieldMut for $self_:ty
        where[$($where_:tt)*]
        delegate_to=$delegating_to:ty,
    )=>{
        unsafe_delegate_variant_field!{
            impl[$($typarams)*] GetVariantFieldMut for $self_
            where[$($where_)*]
            delegate_to=$delegating_to,
        }


        unsafe impl<$($typarams)* _V,_F>
            $crate::pmr::IntoVariantFieldImpl<_V,_F>
        for $self_
        where
            $delegating_to: $crate::pmr::IntoVariantFieldImpl<_V,_F>
            $($where_)*
        {}
    };
}

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
/// # Parapmeters
///
/// - `optionality`: whether the field is optional.
/// The value of this can be either `opt` for an optional field,
/// or `nonopt` for a regular (non-optional) field.
///
/// # Example
///
/// For an example where this macro is used,
/// you can look at the
/// [manual implementation example of the GetFieldMutImpl trait
/// ](./field_traits/trait.GetFieldMutImpl.html#manual-implementation-example)
#[macro_export]
macro_rules! z_unsafe_impl_get_field_raw_mut_method {
    (
        $Self:ident,
        field_name=$field_name:tt,
        name_generic=$name_param:ty,
        optionality=$optionality:ident,
    ) => (
        unsafe fn get_field_raw_mut(
            this:*mut (),
            _:$name_param,
            _:(),
        )->Result<
            *mut $crate::GetFieldType<$Self,$name_param>,
            $crate::pmr::GetFieldErr<$Self,$name_param>,
        >{
            $crate::handle_optionality!(
                $optionality,
                raw,
                &mut (*(this as *mut $Self)).$field_name
                    as *mut $crate::option_or_value_ty!(
                        $optionality,
                        $crate::GetFieldType<$Self,$name_param>
                    )
            )
        }

        fn get_field_raw_mut_func(
            &self
        )->$crate::field_traits::GetFieldRawMutFn<
            $name_param,
            (),
            $crate::GetFieldType<$Self,$name_param>,
            $crate::pmr::GetFieldErr<$Self,$name_param>,
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
        $crate::z_impl_box_into_field_method!{
            $field_name,
            (),
            $crate::GetFieldType<Self,$field_name>,
            $crate::pmr::GetFieldErr<Self,$field_name>,
        }
    );
    ($field_name:ty, $field_ty:ty, $field_err:ty $(,)*) => (
        $crate::z_impl_box_into_field_method!{
            $field_name,
            (),
            $field_ty,
            $field_err,
        }
    );
    ($field_name:ty, $param_ty:ty, $field_ty:ty, $field_err:ty $(,)*) => (
        #[inline(always)]
        fn box_into_field_(
            self:$crate::pmr::Box<Self>,
            name:$field_name,
            param:$param_ty,
        )->Result<$field_ty,$field_err>{
            <Self as
                $crate::IntoFieldImpl::<$field_name,$param_ty>
            >::into_field_(*self,name,param)
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! optionality_from {
    (opt) => {
        $crate::structural_trait::IsOptional::Yes
    };
    (nonopt) => {
        $crate::structural_trait::IsOptional::No
    };
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
                    opt=$optionality:ident,
                ),
            )*]
        }
    )=>{
        impl<$($typarams)*> $crate::IsStructural for $self_
        where $($where_)*
        {}

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
                            optionality:$crate::optionality_from!($optionality),
                        },
                    )*
                ])
            };
        }
    };
    (
        impl[$($typarams:tt)*] Structural for $self_:ty
        where[$($where_:tt)*]
        {
            variants=[ $( $variant:ident ),* $(,)*]
        }
    )=>{
        impl<$($typarams)*> $crate::IsStructural for $self_
        where $($where_)*
        {}

        impl<$($typarams)*> $crate::Structural for $self_
        where $($where_)*
        {
            const FIELDS: &'static $crate::structural_trait::FieldInfos= {
                use $crate::structural_trait::{FieldInfos,VariantInfo};

                &FieldInfos::Enum(&[
                    $(
                        VariantInfo::not_renamed(stringify!($variant)),
                    )*
                ])
            };
        }
    };
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
                    opt=$optionality:ident,
                    $name_param_str:expr,
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
                            opt=$optionality,
                        ),
                    )*
                ]
            }
        }

        $(
            $crate::impl_getter!{
                unsafe impl $typarams
                    $getter_trait<$field_name : $field_ty,$optionality,$name_param_ty>
                for $self_
                where $where_preds
            }
        )*
    }
}

macro_rules! assert_equal_bounds {
    (
        trait $trait_:ident,
        ( $($left:tt)* ),
        ( $($right:tt)* )$(,)*
    ) => (
        trait $trait_: $($left)* {
            fn foo<T>()
            where
                T: ?Sized+$($left)*;
        }

        impl<_This> $trait_ for _This
        where
            _This: ?Sized+$($right)*
        {
            fn foo<T>()
            where
                T:?Sized+$($right)*
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
            Err(e) => return Err($crate::field_traits::IntoFieldErr::into_field_err(e)),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! map_fe {
    ( $expr:expr ) => {
        match $expr {
            Ok(x) => Ok(x),
            Err(e) => Err($crate::field_traits::IntoFieldErr::into_field_err(e)),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! try_of {
    ( $expr:expr ) => {
        match $expr {
            Ok(x) => x,
            Err(_) => return Err($crate::field_traits::OptionalField),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! map_of {
    ( $expr:expr ) => {
        match $expr {
            Ok(x) => Ok(x),
            Err(_) => Err($crate::field_traits::OptionalField),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! handle_optionality {
    ( nonopt,$pointerness:ident,$value:expr $(,)* ) => {
        Ok($value)
    };
    ( opt,raw,$value:expr $(,)* ) => {
        $crate::utils::option_as_mut_result($value)
    };
    ( opt,$remaining_pointerness:ident,$value:expr $(,)* ) => {
        match $value {
            Some(x) => Ok(x),
            None => Err($crate::pmr::OptionalField),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! try_optionality {
    ( nonopt,$pointerness:ident,$value:expr $(,)* ) => {
        $value
    };
    ( opt,raw,$value:expr $(,)* ) => {{
        let value: *mut Option<_> = $value;
        match *$value {
            Some(ref mut x) => x as *mut _,
            None => return Err($crate::pmr::OptionalField),
        }
    }};
    ( opt,$pointerness:ident,$value:expr $(,)* ) => {
        match $value {
            Some(x) => x,
            None => return Err($crate::pmr::OptionalField),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! map_optionality {
    ( nonopt,$value:expr $(,)* ) => {
        $value
    };
    ( opt,$value:expr $(,)* ) => {
        match $value {
            Ok(x) => Ok(x),
            Err(e) => Err($crate::pmr::OptionalField),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! err_from_opt {
    ( nonopt ) => {
        $crate::pmr::NonOptField
    };
    ( opt ) => {
        $crate::pmr::OptionalField
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! vf_err {
    ( nonopt,$field:ty,$field_name_string:ty ) => {
        $crate::pmr::GetFieldErr<$field,$crate::pmr::FieldPath1<$field_name_string>>
    };
    ( opt,$field:ty,$field_name:ty ) => {
        $crate::pmr::OptionalField
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! option_or_value_ty {
    (opt,$ty:ty) => ( $crate::pmr::Option<$ty> );
    (nonopt,$ty:ty) => ( $ty );
}

#[doc(hidden)]
#[macro_export]
macro_rules! optionality_ty {
    (opt) => {
        $crate::pmr::OptionalField
    };
    (nonopt) => {
        $crate::pmr::NonOptField
    };
}
