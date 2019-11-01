use super::*;

use crate::structural_trait::{FieldInfo,StructuralDyn,TField};

use core::{
    ops::{Range,RangeFrom,RangeTo,RangeInclusive,RangeToInclusive},
};


type Start_STR=TStr!(s t a r t);
type End_STR=TStr!(e n d);

///////////////////////////////////////////////////////

impl_getters_for_derive!{
    impl[T] Range<T>
    where[]
    {
        (IntoFieldMut< start : T,Start_STR,"start",  > )
        (IntoFieldMut< end : T,End_STR,"end",  > )
    }
}

impl_getters_for_derive!{
    impl[T] RangeFrom<T>
    where[]
    {
        (IntoFieldMut< start : T,Start_STR,"start",  > )
    }
}

impl_getters_for_derive!{
    impl[T] RangeTo<T>
    where[]
    {
        (IntoFieldMut< end : T,End_STR,"end",  > )
    }
}

impl_getters_for_derive!{
    impl[T] RangeToInclusive<T>
    where[]
    {
        (IntoFieldMut< end : T,End_STR,"end",  > )
    }
}


///////////////////////////////////////////////////////


impl<T> Structural for RangeInclusive<T>{
    const FIELDS:&'static[FieldInfo]=&[
        FieldInfo::not_renamed("start"),
        FieldInfo::not_renamed("end"),
    ];

    type Fields=TList![
        TField<Start_STR,T>,
        TField<End_STR,T>,
    ];
}

impl<T> StructuralDyn for RangeInclusive<T>{
    fn fields_info(&self)->&'static[FieldInfo]{
        <Self as crate::Structural>::FIELDS
    }
}


impl<T> GetField<Start_STR> for RangeInclusive<T>{
    type Ty=T;

    fn get_field_(&self)->&Self::Ty{
        self.start()
    }
}
impl<T> GetField<End_STR> for RangeInclusive<T>{
    type Ty=T;

    fn get_field_(&self)->&Self::Ty{
        self.end()
    }
}


impl<T> IntoField<Start_STR> for RangeInclusive<T>{
    fn into_field_(self)->Self::Ty{
        self.into_inner().0
    }
    impl_box_into_field_method!{Start_STR}
}
impl<T> IntoField<End_STR> for RangeInclusive<T>{
    fn into_field_(self)->Self::Ty{
        self.into_inner().0
    }
    impl_box_into_field_method!{End_STR}
}


///////////////////////////////////////////////////////


