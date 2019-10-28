use super::*;

use crate::structural_trait::FieldInfo;

use core::{
    ops::{Range,RangeFrom,RangeTo,RangeInclusive,RangeToInclusive},
};


impl_getters_for_derive!{
    impl[T] Range<T>
    where[]
    {
        (IntoFieldMut< start : T,TStr!(s t a r t),"start",  > )
        (IntoFieldMut< end : T,TStr!(e n d),"end",  > )
    }
}

impl_getters_for_derive!{
    impl[T] RangeFrom<T>
    where[]
    {
        (IntoFieldMut< start : T,TStr!(s t a r t),"start",  > )
    }
}

impl_getters_for_derive!{
    impl[T] RangeTo<T>
    where[]
    {
        (IntoFieldMut< end : T,TStr!(e n d),"end",  > )
    }
}

impl_getters_for_derive!{
    impl[T] RangeToInclusive<T>
    where[]
    {
        (IntoFieldMut< end : T,TStr!(e n d),"end",  > )
    }
}


///////////////////////////////////////////////////////


impl<T> Structural for RangeInclusive<T>{
    const FIELDS:&'static[FieldInfo]=&[
        FieldInfo::not_renamed("start"),
        FieldInfo::not_renamed("end"),
    ];
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


///////////////////////////////////////////////////////


