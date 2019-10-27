use super::*;

use core::{
    ops::{Range,RangeFrom,RangeTo,RangeInclusive,RangeToInclusive},
};


impl_getter!{
    unsafe impl[T] IntoFieldMut< start:T ,TStr!(s t a r t) > for Range<T>
}
impl_getter!{
    unsafe impl[T] IntoFieldMut< end:T ,TStr!(e n d) > for Range<T>
}


impl_getter!{
    unsafe impl[T] IntoFieldMut< start:T ,TStr!(s t a r t) > for RangeFrom<T>
}

impl_getter!{
    unsafe impl[T] IntoFieldMut< end:T ,TStr!(e n d) > for RangeTo<T>
}


impl_getter!{
    unsafe impl[T] IntoFieldMut< end:T ,TStr!(e n d) > for RangeToInclusive<T>
}


impl<T> GetField<TStr!(s t a r t)> for RangeInclusive<T>{
    type Ty=T;

    fn get_field_(&self)->&Self::Ty{
        self.start()
    }
}
impl<T> GetField<TStr!(e n d)> for RangeInclusive<T>{
    type Ty=T;

    fn get_field_(&self)->&Self::Ty{
        self.end()
    }
}


impl<T> IntoField<TStr!(s t a r t)> for RangeInclusive<T>{
    fn into_field_(self)->Self::Ty{
        self.into_inner().0
    }
}
impl<T> IntoField<TStr!(e n d)> for RangeInclusive<T>{
    fn into_field_(self)->Self::Ty{
        self.into_inner().0
    }
}
