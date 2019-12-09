use crate::{
    field_traits::{
        GetFieldImpl,
        GetFieldMutImpl,
        IntoFieldImpl,
        OptionalField,
        GetFieldMutRefFn,
    },
    structural_trait::{FieldInfos,Structural},
};

use std_::marker::PhantomData;


field_path_aliases_module! {
    mod names{
        Ok,
        Err,
    }
}

///////////////////////////////////////////////////////////////////////////////


macro_rules! map_fe {
    ( $expr:expr ) => {
        match $expr {
            Ok(x) => Ok(x),
            Err(_) => Err(OptionalField),
        }
    };
}


impl<T,Name> GetFieldImpl<Name> for Option<T>
where
    T: GetFieldImpl<Name>
{
    type Ty=T::Ty;
    type Err=OptionalField;

    fn get_field_(&self) -> Result<&Self::Ty, Self::Err>{
        match self {
            Some(expr) => map_fe!(expr.get_field_()),
            None => Err(OptionalField),
        }
    }
}


unsafe impl<T,Name> GetFieldMutImpl<Name> for Option<T> 
where
    T: GetFieldMutImpl<Name>
{
    fn get_field_mut_(&mut self) -> Result<&mut Self::Ty, Self::Err>{
        match self {
            Some(expr) => map_fe!(expr.get_field_mut_()),
            None => Err(OptionalField),
        }
    }

    unsafe fn get_field_raw_mut(
        ptr: *mut (),
        name: PhantomData<Name>,
    ) -> Result<*mut Self::Ty, Self::Err>
    where
        Self: Sized
    {
        match *(ptr as *mut Self) {
            Some(ref mut expr) => {
                let ptr=expr as *mut T as *mut ();
                map_fe!(T::get_field_raw_mut(ptr,name))
            }
            None => Err(OptionalField),
        }
    }

    fn get_field_raw_mut_func(&self) -> GetFieldMutRefFn<Name, Self::Ty, Self::Err>{
        <Self as GetFieldMutImpl<Name>>::get_field_raw_mut
    }
}

impl<T,Name> IntoFieldImpl<Name> for Option<T>
where
    T: IntoFieldImpl<Name>
{
    fn into_field_(self) -> Result<Self::Ty, Self::Err>{
        match self {
            Some(expr) => map_fe!(expr.into_field_()),
            None => Err(OptionalField),
        }
    }

    z_impl_box_into_field_method!{Name}
}


impl<T> Structural for Option<T>
where
    T: Structural
{
    const FIELDS: &'static FieldInfos={
        &FieldInfos::Option(T::FIELDS)
    };
}

///////////////////////////////////////////////////////////////////////////////


impl_getters_for_derive_enum!{
    impl[T,E] Result<T,E>
    where[]
    {
        (IntoFieldMut,Ok:T,names::Ok,"Ok",transparency(Result,Ok,0))
        (IntoFieldMut,Err:E,names::Err,"Err",transparency(Result,Err,0))
    }
}
