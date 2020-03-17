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
            GetField <$field_name:tt : $field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        impl<$($typarams)*> $crate::FieldType<$name_param> for $self_
        $( where $($where_)* )?
        {
            type Ty=$field_ty;
        }

        impl<$($typarams)*> $crate::GetField<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn get_field_(&self,_:$name_param)->&Self::Ty{
                &self.$field_name
            }
        }
    };
    (
        unsafe impl[$($typarams:tt)*]
            GetFieldMut <$field_name:tt : $field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::_private_impl_getter!{
            impl[$($typarams)*] GetField<$field_name:$field_ty,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }

        unsafe impl<$($typarams)*> $crate::GetFieldMut<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn get_field_mut_(&mut self,_:$name_param)->&mut Self::Ty{
                &mut self.$field_name
            }

            $crate::z_unsafe_impl_get_field_raw_mut_method!{
                Self,
                field_tstr=$field_name,
                name_generic=$name_param,
            }
        }
    };
    (
        $(unsafe)?
        impl[$($typarams:tt)*]
            IntoField <$field_name:tt : $field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::_private_impl_getter!{
            impl[$($typarams)*]
                GetField<$field_name:$field_ty,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }

        impl<$($typarams)*> $crate::IntoField<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn into_field_(self,_:$name_param)->Self::Ty{
                self.$field_name
            }
            $crate::z_impl_box_into_field_method!{field_tstr=$name_param}
        }
    };
    (
        unsafe impl[$($typarams:tt)*]
            IntoFieldMut <$field_name:tt : $field_ty:ty,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::_private_impl_getter!{
            unsafe impl[$($typarams)*]
                GetFieldMut<$field_name:$field_ty,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }

        impl<$($typarams)*> $crate::IntoField<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn into_field_(
                self,
                _:$name_param,
            )->Self::Ty{
                self.$field_name
            }
            $crate::z_impl_box_into_field_method!{field_tstr=$name_param}
        }
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
/// [GetFieldMut](./field_traits/trait.GetFieldMut.html)
/// trait for structs.
///
/// This implements the [GetFieldMut::get_field_raw_mut]
/// by returning a mutable pointer to a field,
/// and [GetFieldMut::get_field_raw_mut_fn] by returning
/// `get_field_raw_mut` as a function pointer.
///
/// # Safety
///
/// This is an unsafe macro,
/// because GetFieldMut requires no impl to borrow the same field mutably as any other,
/// otherwise this would cause undefined behavior because it would
/// create multiple mutable borrows to the same field.
///
/// # Example
///
/// For an example where this macro is used,
/// you can look at the
/// [manual implementation example of the GetFieldMut trait
/// ](./field_traits/trait.GetFieldMut.html#manual-implementation-example)
#[macro_export]
macro_rules! z_unsafe_impl_get_field_raw_mut_method {
    (
        $Self:ident,
        field_tstr=$field_name:tt,
        name_generic=$name_param:ty,
    ) => {
        unsafe fn get_field_raw_mut(
            this: *mut (),
            _: $name_param,
        ) -> *mut $crate::GetFieldType<$Self, $name_param> {
            &mut (*(this as *mut $Self)).$field_name
                as *mut $crate::GetFieldType<$Self, $name_param>
        }

        fn get_field_raw_mut_fn(
            &self,
        ) -> $crate::field_traits::GetFieldRawMutFn<
            $name_param,
            $crate::GetFieldType<$Self, $name_param>,
        > {
            <$Self as $crate::field_traits::GetFieldMut<$name_param>>::get_field_raw_mut
        }
    };
}

/// For borrowing an enum field as either `Some(NonNull<_>)` or `None`.
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
/// ](./field_traits/variant_field/trait.GetVariantFieldMut.html#manual-impl-example)
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

/// Implements the `get_vfield_raw_mut_fn` and `get_vfield_raw_mut_unchecked_fn`
/// methods from the `GetVariantFieldMut` trait.
///
/// # Safety
///
/// The `$Self` argument must be the `Self` type in the impl block.
///
/// # Example
///
/// For an example of using this macro look at
/// [the manual implementation example for `GetVariantFieldMut`
/// ](./field_traits/variant_field/trait.GetVariantFieldMut.html#manual-impl-example)
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

/// For use in manual implementations of the IntoField trait.
///
/// Implements the [`IntoField::box_into_field_`] method
/// by delegatign to the [`IntoField::into_field_`] method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// # Example
///
/// For an example of using this macro look at
/// [the documentation for IntoField
/// ](./field_traits/trait.IntoField.html#manual-implementation-example)
///
/// [`IntoField::box_into_field_`]: ./field_traits/trait.IntoField.html#tymethod.box_into_field_
/// [`IntoField::into_field_`]: ./field_traits/trait.IntoField.html#tymethod.into_field_
#[macro_export]
#[cfg(not(feature = "alloc"))]
macro_rules! z_impl_box_into_field_method {
    ($($anything:tt)*) => {};
}

/// For use in manual implementations of the IntoField trait.
///
/// Implements the [`IntoField::box_into_field_`] method
/// by delegatign to the [`IntoField::into_field_`] method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// # Example
///
/// For an example of using this macro look at
/// [the documentation for IntoField
/// ](./field_traits/trait.IntoField.html#manual-implementation-example)
///
/// [`IntoField::box_into_field_`]: ./field_traits/trait.IntoField.html#tymethod.box_into_field_
/// [`IntoField::into_field_`]: ./field_traits/trait.IntoField.html#tymethod.into_field_
#[macro_export]
#[cfg(feature = "alloc")]
macro_rules! z_impl_box_into_field_method {
    (field_tstr= $field_name:ty) => {
        $crate::z_impl_box_into_field_method! {
            field_tstr= $field_name,
            field_type= $crate::GetFieldType<Self,$field_name>,
        }
    };
    (field_tstr= $field_name:ty, field_type= $field_ty:ty $(,)*) => {
        #[inline(always)]
        fn box_into_field_(self: $crate::pmr::Box<Self>, name: $field_name) -> $field_ty {
            <Self as $crate::IntoField<$field_name>>::into_field_(*self, name)
        }
    };
}

#[macro_export]
#[cfg(not(feature = "alloc"))]
macro_rules! z_impl_box_into_variant_field_method {
    ($($anything:tt)*) => {};
}

/// For use in manual implementations of the IntoVariantField trait.
///
/// Implements the [`IntoVariantField::box_into_vfield_`] method
/// by delegatign to the [`IntoVariantField::into_vfield_`] method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// # Example
///
/// For an example of using this macro look at
/// [the documentation for IntoVariantField
/// ](./field_traits/variant_field/trait.IntoVariantField.html#manual-impl-example)
///
/// [`IntoField::box_into_vfield_`]:
/// ./field_traits/variant_field/trait.IntoVariantField.html#tymethod.box_into_vfield_
/// [`IntoField::into_vfield_`]:
/// ./field_traits/variant_field/trait.IntoVariantField.html#tymethod.into_vfield_
#[macro_export]
#[cfg(feature = "alloc")]
macro_rules! z_impl_box_into_variant_field_method {
    (
        variant_tstr= $variant_name:ty,
        field_tstr= $field_name:ty,
        field_type=$field_ty:ty $(,)*
    ) => {
        fn box_into_vfield_(
            self: $crate::alloc::boxed::Box<Self>,
            vname: $variant_name,
            fname: $field_name,
        ) -> Option<Self::Ty> {
            <Self as $crate::IntoVariantField<$variant_name, $field_name>>::into_vfield_(
                *self, vname, fname,
            )
        }
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
                        ),
                    )*
                ]
            }
        }

        $(
            $crate::_private_impl_getter!{
                unsafe impl $typarams
                    $getter_trait<$field_name : $field_ty,$name_param_ty>
                for $self_
                where $where_preds
            }
        )*
    }
}

/// Asserts that the `$left` bounds are the same as the `$right` bounds
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
macro_rules! map_of {
    ( $expr:expr ) => {
        match $expr {
            Ok(x) => Ok(x),
            Err(_) => Err($crate::field_traits::FailedAccess),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! ok_or_of {
    ( $expr:expr ) => {
        match $expr {
            Some(x) => Ok(x),
            None => Err($crate::field_traits::FailedAccess),
        }
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
