/// For semi-manual implementors of the
/// [GetFieldMut](./field/trait.GetFieldMut.html)
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
/// ](./field/trait.GetFieldMut.html#manual-implementation-example)
#[macro_export]
macro_rules! z_unsafe_impl_get_field_raw_mut {
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
        ) -> $crate::field::GetFieldRawMutFn<$name_param, $crate::GetFieldType<$Self, $name_param>>
        {
            <$Self as $crate::field::GetFieldMut<$name_param>>::get_field_raw_mut
        }
    };
}

/// Implements an infallible getter
#[doc(hidden)]
#[macro_export]
macro_rules! _private_impl_getter{
    (
        $(unsafe)?
        impl[$($typarams:tt)*]
            GetField <$field_name:tt : $field_ty:ty, $field_index:expr,$name_param:ty>
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
            GetFieldMut <$field_name:tt : $field_ty:ty, $field_index:expr,$name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::_private_impl_getter!{
            impl[$($typarams)*] GetField<$field_name:$field_ty, $field_index,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }

        unsafe impl<$($typarams)*> $crate::GetFieldMut<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn get_field_mut_(&mut self,_:$name_param)->&mut Self::Ty{
                &mut self.$field_name
            }

            $crate::z_unsafe_impl_get_field_raw_mut!{
                Self,
                field_tstr=$field_name,
                name_generic=$name_param,
            }
        }
    };
    (@just_into_field
        unsafe impl[$($typarams:tt)*]
            IntoField <$field_name:tt : $field_ty:ty, $field_index:expr, $name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        unsafe impl<$($typarams)*> $crate::IntoField<$name_param> for $self_
        $( where $($where_)* )?
        {
            fn into_field_(
                self,
                _:$name_param,
            )->Self::Ty{
                self.$field_name
            }

            unsafe fn move_out_field_(
                &mut self,
                _: $name_param,
                dropped_fields: &mut $crate::pmr::DroppedFields,
            ) -> Self::Ty {
                {
                    use $crate::pmr::DropBit;
                    const BIT: DropBit = DropBit::new($field_index);
                    dropped_fields.set_dropped(BIT);
                }
                (&mut self.$field_name as *mut $field_ty).read()
            }
        }
    };
    (
        unsafe impl[$($typarams:tt)*]
            IntoField <$field_name:tt : $field_ty:ty, $field_index:expr, $name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::_private_impl_getter!{
            impl[$($typarams)*]
                GetField<$field_name:$field_ty,$field_index,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }

        $crate::_private_impl_getter!{
            @just_into_field
            unsafe impl[$($typarams)*]
                IntoField<$field_name:$field_ty,$field_index,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }
    };
    (
        unsafe impl[$($typarams:tt)*]
            IntoFieldMut <$field_name:tt : $field_ty:ty, $field_index:expr, $name_param:ty>
        for $self_:ty
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::_private_impl_getter!{
            unsafe impl[$($typarams)*]
                GetFieldMut<$field_name:$field_ty,$field_index,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }

        $crate::_private_impl_getter!{
            @just_into_field
            unsafe impl[$($typarams)*]
                IntoField<$field_name:$field_ty,$field_index,$name_param>
            for $self_
            $( where[$($where_)*] )?
        }
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
            DropFields{
                drop_fields=$($drop_kind:ident)?,
                $(not_public($($drop_fields:tt)*))?
            }

            $((
                $getter_trait:ident<
                    $field_name:tt : $field_ty:ty,
                    $field_index:expr,
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
        }

        $crate::_private_impl_drop_fields!{
            struct_or_enum(struct),

            drop_kind=$($drop_kind)?,

            impl $typarams DropFields for $self_
            where $where_preds
            {
                not_public( $($($drop_fields)*)? ),
                field_names(
                    $(
                        (
                            $field_name,
                            $field_index,
                        ),
                    )*
                ),
            }
        }

        $(
            $crate::_private_impl_getter!{
                unsafe impl $typarams
                    $getter_trait<$field_name : $field_ty,$field_index,$name_param_ty>
                for $self_
                where $where_preds
            }
        )*
    }
}
