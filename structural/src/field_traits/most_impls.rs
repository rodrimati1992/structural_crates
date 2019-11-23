#![allow(non_camel_case_types)]

use super::*;

use crate::structural_trait::{FieldInfo,StructuralDyn};
#[allow(unused_imports)]
use crate::GetFieldExt;

use core::{
    ops::{Range,RangeFrom,RangeTo,RangeInclusive,RangeToInclusive},
    //marker::Unpin,
    mem::ManuallyDrop,
    ops::Deref,
    pin::Pin,
};


type Start_STR=FP!(s t a r t);
type End_STR=FP!(e n d);

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
    z_impl_box_into_field_method!{Start_STR}
}
impl<T> IntoField<End_STR> for RangeInclusive<T>{
    fn into_field_(self)->Self::Ty{
        self.into_inner().0
    }
    z_impl_box_into_field_method!{End_STR}
}


///////////////////////////////////////////////////////


// This allows using all the field accessors in T from `ManuallyDrop<T>`
z_delegate_structural_with!{
    impl[T] ManuallyDrop<T>
    where[]
    self_ident=this;
    delegating_to_type=T;
    field_name_param=( fname : fname_ty );

    GetField { this }
    
    unsafe GetFieldMut { this }
    as_delegating_raw{ this as *mut ManuallyDrop<T> as *mut T }
    
    IntoField { ManuallyDrop::into_inner(this) }
}

#[test]
fn delegated_mdrop(){
    let tup=(2,3,5,8);
    let mut mdrop=ManuallyDrop::new(tup);
    assert_eq!( mdrop.fields(fp!(0,1,2,3)), (&2,&3,&5,&8) );
    
    assert_eq!( mdrop.fields_mut(fp!(0,1,2,3)), (&mut 2,&mut 3,&mut 5,&mut 8) );
    
    assert_eq!( mdrop.clone().into_field(fp!(0)), 2 );
    assert_eq!( mdrop.clone().into_field(fp!(1)), 3 );
    assert_eq!( mdrop.clone().into_field(fp!(2)), 5 );
    assert_eq!( mdrop.clone().into_field(fp!(3)), 8 );
}



///////////////////////////////////////////////////////



z_delegate_structural_with!{
    impl[P] Pin<P>
    where[
        P:Deref,
        P::Target:Sized,
    ]
    self_ident=this;
    delegating_to_type=P::Target;
    field_name_param=( fname : fname_ty );

    GetField { &*this }
}


#[test]
fn delegated_pin(){
    let tup=(2,3,5,8);
    let pin=Pin::new(&tup);
    assert_eq!( pin.fields(fp!(0,1,2,3)), (&2,&3,&5,&8) );
    //assert_eq!( pin.fields_mut(fp!(0,1,2,3)), (&mut 2,&mut 3,&mut 5,&mut 8) );
}


///////////////////////////////////////////////////////


z_delegate_structural_with!{
    impl['a,T] &'a T
    where [T:?Sized,]
    self_ident=this;
    delegating_to_type=T;
    field_name_param=( fname_var : fname_ty );

    GetField { &**this }

}

///////////////////////////////////////////////////////


z_delegate_structural_with!{
    impl['a,T] &'a mut T
    where [T:?Sized,]
    self_ident=this;
    delegating_to_type=T;
    field_name_param=( fname_var : fname_ty );

    GetField { &**this }

}

unsafe impl<T,__FieldName> GetFieldMut<__FieldName> for &'_ mut T
where
    T:?Sized+GetFieldMut<__FieldName>,
{
    fn get_field_mut_(&mut self)->&mut Self::Ty{
        <T as GetFieldMut<__FieldName>>::get_field_mut_(self)
    }

    default_if!{
        cfg(feature="specialization")
        
        unsafe fn get_field_raw_mut(
            this:*mut (),
            fname:PhantomData<__FieldName>
        )->*mut Self::Ty {
            let this:*mut T=*(this as *mut *mut T);
            let func=T::get_field_raw_mut_func(&*this);
            
            func( this as *mut (), fname )
        }
    }

    fn get_field_raw_mut_func(
        &self
    )->GetFieldMutRefFn<__FieldName,Self::Ty>{
        <Self as GetFieldMut<__FieldName>>::get_field_raw_mut
    }
}


#[cfg(feature="specialization")]
unsafe impl<T,__FieldName> GetFieldMut<__FieldName> for &'_ mut T
where
    T:GetFieldMut<__FieldName>,
{
    unsafe fn get_field_raw_mut(
        this:*mut (),
        fname:PhantomData<__FieldName>
    )->*mut Self::Ty {
        T::get_field_raw_mut( *(this as *mut *mut ()), fname )
    }
}
