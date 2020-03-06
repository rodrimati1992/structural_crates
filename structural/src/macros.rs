#[macro_use]
mod delegate_structural;

#[macro_use]
mod list;

#[macro_use]
mod field_paths;

#[macro_use]
mod tstr_macros;

#[macro_use]
mod impl_struct;

#[macro_use]
mod make_struct;

#[macro_use]
mod structural_alias;

#[macro_use]
mod enum_derivation;

#[macro_use]
mod switch;

#[macro_use]
mod type_level_internal;

/// Implements an infallible getter
#[doc(hidden)]
#[macro_export]
macro_rules! _private_impl_getter{
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
        $crate::_private_impl_getter!{
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
        $crate::_private_impl_getter!{
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
        $crate::_private_impl_getter!{
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

/// Delegates the enum traits that don't have required methods.
///
/// # Safety
///
/// The `Self` parameter of this macro must delegate accessor trait impls
/// parameterized by `<VariantField<V,F>,UncheckedVariantField<V,F>>`
/// to the `delegated_to` type,
/// with the option to return `OptionalField` when the delegated to type does not.
///
/// Example:in `impl<V,F> GetFieldImpl<VariantField<V,F>,UncheckedVariantField<V,F>> for Foo`
/// it must gelegate that trait to the same trait impl of the delegated-to type.
#[macro_export]
#[doc(hidden)]
macro_rules! unsafe_delegate_variant_field {
    (
        impl[$($typarams:tt)*] GetVariantFieldImpl for $self_:ty
        where[$($where_:tt)*]
        delegate_to=$delegating_to:ty,
    )=>{
        unsafe impl<$($typarams)*> $crate::pmr::VariantCount for $self_
        where
            $delegating_to: $crate::pmr::VariantCount,
            $($where_)*
        {
            type Count=$crate::pmr::VariantCountOut<$delegating_to>;
        }

        unsafe impl<$($typarams)* _V,_F>
            $crate::pmr::GetVariantFieldImpl<_V,_F>
        for $self_
        where
            $delegating_to: $crate::pmr::GetVariantFieldImpl<_V,_F>,
            $($where_)*
        {}
    };
    (
        impl[$($typarams:tt)*] GetVariantFieldMutImpl for $self_:ty
        where[$($where_:tt)*]
        delegate_to=$delegating_to:ty,
    )=>{
        unsafe_delegate_variant_field!{
            impl[$($typarams)*] GetVariantFieldImpl for $self_
            where[$($where_)*]
            delegate_to=$delegating_to,
        }

        unsafe impl<$($typarams)* _V,_F>
            $crate::pmr::GetVariantFieldMutImpl<_V,_F>
        for $self_
        where
            $delegating_to: $crate::pmr::GetVariantFieldMutImpl<_V,_F>,
            $($where_)*
        {}
    };
    (
        impl[$($typarams:tt)*] IntoVariantFieldImpl for $self_:ty
        where[$($where_:tt)*]
        delegate_to=$delegating_to:ty,
    )=>{
        unsafe_delegate_variant_field!{
            impl[$($typarams)*] GetVariantFieldImpl for $self_
            where[$($where_)*]
            delegate_to=$delegating_to,
        }


        unsafe impl<$($typarams)* _V,_F>
            $crate::pmr::IntoVariantFieldImpl<_V,_F>
        for $self_
        where
            $delegating_to: $crate::pmr::IntoVariantFieldImpl<_V,_F>,
            $($where_)*
        {}
    };
    (
        impl[$($typarams:tt)*] IntoVariantFieldMut for $self_:ty
        where[$($where_:tt)*]
        delegate_to=$delegating_to:ty,
    )=>{
        unsafe_delegate_variant_field!{
            impl[$($typarams)*] GetVariantFieldMutImpl for $self_
            where[$($where_)*]
            delegate_to=$delegating_to,
        }


        unsafe impl<$($typarams)* _V,_F>
            $crate::pmr::IntoVariantFieldImpl<_V,_F>
        for $self_
        where
            $delegating_to: $crate::pmr::IntoVariantFieldImpl<_V,_F>,
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

/// For semi-manual implementors of the
/// [GetFieldMutImpl](./field_traits/trait.GetFieldMutImpl.html)
/// trait for structs.
///
/// This implements the [GetFieldMutImpl::get_field_raw_mut]
/// by returning a mutable pointer to a field,
/// and [GetFieldMutImpl::get_field_raw_mut_func] by returning
/// `get_field_raw_mut` as a function pointer.
///
/// # Safety
///
/// This is an unsafe macro,
/// because GetFieldMutImpl requires no impl to borrow the same field mutably as any other,
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
            this:*mut *mut (),
            _:$name_param,
            _:(),
        )->Result<
            *mut $crate::GetFieldType<$Self,$name_param>,
            $crate::pmr::GetFieldErr<$Self,$name_param>,
        >{
            $crate::handle_optionality!(
                $optionality,
                raw,
                &mut (**(this as *mut *mut $Self)).$field_name
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
/// Implements the `IntoFieldImpl::box_into_field_` method
/// by delegatign to the [IntoFieldImpl::into_field_] method,
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

/// For use in semi-manual implementations of the IntoFieldImpl trait.
///
/// Implements the [IntoFieldImpl::box_into_field_] method
/// by delegatign to the [IntoFieldImpl::into_field_] method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// For an example of using this macro look at
/// [the documentation for IntoFieldImpl
/// ](./field_traits/trait.IntoFieldImpl.html#manual-implementation-example)
#[macro_export]
#[cfg(feature = "alloc")]
macro_rules! z_impl_box_into_field_method {
    ($field_name:ty) => {
        $crate::z_impl_box_into_field_method! {
            $field_name,
            (),
            $crate::GetFieldType<Self,$field_name>,
            $crate::pmr::GetFieldErr<Self,$field_name>,
        }
    };
    ($field_name:ty, $field_ty:ty, $field_err:ty $(,)*) => {
        $crate::z_impl_box_into_field_method! {
            $field_name,
            (),
            $field_ty,
            $field_err,
        }
    };
    ($field_name:ty, $param_ty:ty, $field_ty:ty, $field_err:ty $(,)*) => {
        #[inline(always)]
        fn box_into_field_(
            self: $crate::pmr::Box<Self>,
            name: $field_name,
            param: $param_ty,
        ) -> Result<$field_ty, $field_err> {
            <Self as $crate::IntoFieldImpl<$field_name, $param_ty>>::into_field_(*self, name, param)
        }
    };
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
macro_rules! _private_impl_structural{
    (
        $(#[doc=$docs:literal])*
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
        $(#[doc=$docs])*
        impl<$($typarams)*> $crate::Structural for $self_
        where $($where_)*
        {}
    };
    (
        $(#[doc=$docs:literal])*
        impl[$($typarams:tt)*] Structural for $self_:ty
        where[$($where_:tt)*]
        {
            variants=[ $( $variant:ident ),* $(,)*]
        }
    )=>{
        $(#[doc=$docs])*
        impl<$($typarams)*> $crate::Structural for $self_
        where $($where_)*
        {}
    };
}

// Implements the Structural and accessor traits for a struct
#[doc(hidden)]
#[macro_export]
macro_rules! _private_impl_getters_for_derive_struct{
    (
        $(#[doc=$docs:literal])*
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

        $crate::_private_impl_structural!{
            $(#[doc=$docs])*
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
            $crate::_private_impl_getter!{
                unsafe impl $typarams
                    $getter_trait<$field_name : $field_ty,$optionality,$name_param_ty>
                for $self_
                where $where_preds
            }
        )*
    }
}

#[cfg(test)]
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
        match *value {
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
            Err(_) => Err($crate::pmr::OptionalField),
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
        $crate::pmr::GetFieldErr<$field,$field_name_string>
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

/// Using this to test implemented traits.
#[cfg(test)]
macro_rules! declare_querying_trait {
    (
        trait $trait_name:ident $([$($params:tt)*])?
        $( implements[ $($supertraits:tt)* ] )?
        $( where[ $($where_:tt)* ] )?
        fn $impls_fn:ident;
    ) => (
        trait $trait_name<$($($params)*)?>:Sized{
            type Impls:crate::pmr::Boolean;
            fn $impls_fn(self)->Self::Impls{
                <Self::Impls as crate::pmr::MarkerType>::MTVAL
            }
        }

        impl<$($($params)*)? __This> $trait_name<$($($params)*)?>
        for crate::pmr::PhantomData<__This>
        where
            $( __This:$($supertraits)*, )?
            $( $($where_)* )?
        {
            type Impls=crate::pmr::True;
        }

        impl<$($($params)*)? __This> $trait_name<$($($params)*)?>
        for &'_ crate::pmr::PhantomData<__This>
        {
            type Impls=crate::pmr::False;
        }
    )
}
