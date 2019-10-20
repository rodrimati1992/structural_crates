use super::*;

use std::{
    ops::{},
};


impl_getter!{
    impl[T] IntoField< start:T ,U0 > for Range<T>
}
impl_getter!{
    impl[T] IntoField< end:T ,U1 > for Range<T>
}


impl_getter!{
    impl[T] IntoField< start:T ,U0 > for RangeFrom<T>
}

impl_getter!{
    impl[T] IntoField< end:T ,U0 > for RangeTo<T>
}


impl_getter!{
    impl[T] IntoField< end:T ,U0 > for RangeToInclusive<T>
}


impl<T> GetField<field_name!(start)> for RangeInclusive<T>{
    type FieldTy=T;
    type Index=U1;

    fn get_field_(&self)->&Self::FieldTy{
        self.start()
    }
}
impl<T> GetField<field_name!(end)> for RangeInclusive<T>{
    type FieldTy=T;
    type Index=U1;

    fn get_field_(&self)->&Self::FieldTy{
        self.end()
    }
}


impl<T> IntoField<field_name!(start)> for RangeInclusive<T>{
    fn into_field_(self)->Self::FieldTy{
        self.into_inne().0
    }
}
impl<T> IntoField<field_name!(end)> for RangeInclusive<T>{
    fn into_field_(self)->Self::FieldTy{
        self.into_inne().0
    }
}
