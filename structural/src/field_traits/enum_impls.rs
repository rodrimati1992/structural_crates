use crate::{
    enum_traits::{IsVariant, VariantProxy},
    field_traits::{
        FieldType, GetFieldImpl, GetFieldMutImpl, GetFieldRawMutFn, IntoFieldImpl, OptionalField,
    },
    structural_trait::{FieldInfos, IsStructural, Structural},
    type_level::FieldPath1,
};

use std_::marker::PhantomData;

tstring_aliases_module! {
    mod strings {
        Ok,
        Err,
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<T> IsStructural for Option<T> {}

impl<T, Name> FieldType<Name> for Option<T>
where
    T: FieldType<Name>,
{
    type Ty = T::Ty;
}

impl<T, Name, P> GetFieldImpl<Name, P> for Option<T>
where
    T: GetFieldImpl<Name, P>,
{
    type Err = OptionalField;

    fn get_field_(&self, name: Name, param: P) -> Result<&Self::Ty, Self::Err> {
        match self {
            Some(expr) => map_of!(expr.get_field_(name, param)),
            None => Err(OptionalField),
        }
    }
}

unsafe impl<T, Name, P> GetFieldMutImpl<Name, P> for Option<T>
where
    T: GetFieldMutImpl<Name, P>,
{
    fn get_field_mut_(&mut self, name: Name, param: P) -> Result<&mut Self::Ty, Self::Err> {
        match self {
            Some(expr) => map_of!(expr.get_field_mut_(name, param)),
            None => Err(OptionalField),
        }
    }

    unsafe fn get_field_raw_mut(
        ptr: *mut (),
        name: Name,
        param: P,
    ) -> Result<*mut Self::Ty, Self::Err>
    where
        Self: Sized,
    {
        match *(ptr as *mut Self) {
            Some(ref mut expr) => {
                let ptr = expr as *mut T as *mut ();
                map_of!(T::get_field_raw_mut(ptr, name, param))
            }
            None => Err(OptionalField),
        }
    }

    fn get_field_raw_mut_func(&self) -> GetFieldRawMutFn<Name, P, Self::Ty, Self::Err> {
        <Self as GetFieldMutImpl<Name, P>>::get_field_raw_mut
    }
}

impl<T, Name, P> IntoFieldImpl<Name, P> for Option<T>
where
    T: IntoFieldImpl<Name, P>,
{
    fn into_field_(self, name: Name, param: P) -> Result<Self::Ty, Self::Err> {
        match self {
            Some(expr) => map_of!(expr.into_field_(name, param)),
            None => Err(OptionalField),
        }
    }

    z_impl_box_into_field_method! { Name, P, Self::Ty, OptionalField }
}

unsafe impl<T, V> IsVariant<FieldPath1<V>> for Option<T>
where
    T: IsVariant<FieldPath1<V>>,
{
    fn is_variant_(&self, name: FieldPath1<V>) -> bool {
        match self {
            Some(x) => x.is_variant_(name),
            None => false,
        }
    }
}

unsafe_delegate_variant_field! {
    impl[T,] IntoVariantFieldMut for Option<T>
    where[]
    delegate_to=T,
}

impl<T> Structural for Option<T>
where
    T: Structural,
{
    const FIELDS: &'static FieldInfos = { &FieldInfos::Option(T::FIELDS) };
}

///////////////////////////////////////////////////////////////////////////////

impl_getters_for_derive_enum! {
    impl[T,E,] Result<T,E>
    where[]
    {
        enum=Result
        proxy=VariantProxy
        (Ok,strings::Ok,kind=newtype,fields((IntoFieldMut,0:T,nonopt)))
        (Err,strings::Err,kind=newtype,fields((IntoFieldMut,0:E,nonopt)))
    }
}

///////////////////////////////////////////////////////////////////////////////
