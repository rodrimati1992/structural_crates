#![allow(non_camel_case_types)]

use super::*;

use crate::field_traits::NonOptField;
#[allow(unused_imports)]
use crate::GetFieldExt;

use core::{
    //marker::Unpin,
    mem::ManuallyDrop,
    ops::Deref,
    ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive},
    pin::Pin,
};

type Start_STR = FP!(start);
type End_STR = FP!(end);

///////////////////////////////////////////////////////

_private_impl_getters_for_derive_struct! {
    impl[T] Range<T>
    where[]
    {
        (IntoFieldMut < start : T,Start_STR,opt=nonopt,"start",> )
        (IntoFieldMut < end : T,End_STR,opt=nonopt,"end",> )
    }
}

_private_impl_getters_for_derive_struct! {
    impl[T] RangeFrom<T>
    where[]
    {
        (IntoFieldMut < start : T,Start_STR,opt=nonopt,"start",> )
    }
}

_private_impl_getters_for_derive_struct! {
    impl[T] RangeTo<T>
    where[]
    {
        (IntoFieldMut < end : T,End_STR,opt=nonopt,"end",> )
    }
}

_private_impl_getters_for_derive_struct! {
    impl[T] RangeToInclusive<T>
    where[]
    {
        (IntoFieldMut < end : T,End_STR,opt=nonopt,"end",> )
    }
}

///////////////////////////////////////////////////////

impl<T> Structural for RangeInclusive<T> {}

impl<T> FieldType<Start_STR> for RangeInclusive<T> {
    type Ty = T;
}
impl<T> FieldType<End_STR> for RangeInclusive<T> {
    type Ty = T;
}

impl<T> GetFieldImpl<Start_STR> for RangeInclusive<T> {
    type Err = NonOptField;

    fn get_field_(&self, _: Start_STR, _: ()) -> Result<&Self::Ty, NonOptField> {
        Ok(self.start())
    }
}
impl<T> GetFieldImpl<End_STR> for RangeInclusive<T> {
    type Err = NonOptField;

    fn get_field_(&self, _: End_STR, _: ()) -> Result<&Self::Ty, NonOptField> {
        Ok(self.end())
    }
}

impl<T> IntoFieldImpl<Start_STR> for RangeInclusive<T> {
    fn into_field_(self, _: Start_STR, _: ()) -> Result<Self::Ty, NonOptField> {
        Ok(self.into_inner().0)
    }
    z_impl_box_into_field_method! {Start_STR}
}
impl<T> IntoFieldImpl<End_STR> for RangeInclusive<T> {
    fn into_field_(self, _: End_STR, _: ()) -> Result<Self::Ty, NonOptField> {
        Ok(self.into_inner().0)
    }
    z_impl_box_into_field_method! {End_STR}
}

///////////////////////////////////////////////////////

// This allows using all the field accessors in T from `ManuallyDrop<T>`
unsafe_delegate_structural_with! {
    impl[T,] ManuallyDrop<T>
    where[]
    self_ident=this;
    specialization_params(Sized);
    delegating_to_type=T;
    field_name_param=( fname : FnameTy );

    GetFieldImpl { this }

    unsafe GetFieldMutImpl { this }
    as_delegating_raw{ this as *mut ManuallyDrop<T> as *mut T }

    IntoFieldImpl { ManuallyDrop::into_inner(this) }
}

#[test]
fn delegated_mdrop() {
    let tup = (2, 3, 5, 8);
    let mut mdrop = ManuallyDrop::new(tup);
    assert_eq!(mdrop.fields(fp!(0, 1, 2, 3)), (&2, &3, &5, &8));

    assert_eq!(
        mdrop.fields_mut(fp!(0, 1, 2, 3)),
        (&mut 2, &mut 3, &mut 5, &mut 8)
    );

    assert_eq!(mdrop.clone().into_field(fp!(0)), 2);
    assert_eq!(mdrop.clone().into_field(fp!(1)), 3);
    assert_eq!(mdrop.clone().into_field(fp!(2)), 5);
    assert_eq!(mdrop.clone().into_field(fp!(3)), 8);
}

///////////////////////////////////////////////////////

unsafe_delegate_structural_with! {
    impl[P,] Pin<P>
    where[
        P:Deref,
        P::Target:Sized,
    ]
    self_ident=this;
    delegating_to_type=P::Target;
    field_name_param=( fname : FnameTy );

    GetFieldImpl { &*this }
}

#[test]
fn delegated_pin() {
    let tup = (2, 3, 5, 8);
    let pin = Pin::new(&tup);
    assert_eq!(pin.fields(fp!(0, 1, 2, 3)), (&2, &3, &5, &8));
    //assert_eq!( pin.fields_mut(fp!(0,1,2,3)), (&mut 2,&mut 3,&mut 5,&mut 8) );
}

///////////////////////////////////////////////////////

unsafe_delegate_structural_with! {
    impl['a,T,] &'a T
    where [T:?Sized,]
    self_ident=this;
    delegating_to_type=T;
    field_name_param=( fname_var : FnameTy );

    GetFieldImpl { &**this }

}

///////////////////////////////////////////////////////

unsafe_delegate_structural_with! {
    impl['a,T,] &'a mut T
    where [T:?Sized,]
    self_ident=this;
    delegating_to_type=T;
    field_name_param=( fname_var : FnameTy );

    GetFieldImpl { &**this }

}

unsafe impl<'a, T: 'a, __FieldName, P> GetFieldMutImpl<__FieldName, P> for &'a mut T
where
    T: ?Sized + GetFieldMutImpl<__FieldName, P>,
{
    fn get_field_mut_(&mut self, name: __FieldName, param: P) -> Result<&mut Self::Ty, Self::Err> {
        <T as GetFieldMutImpl<__FieldName, P>>::get_field_mut_(self, name, param)
    }

    #[inline(always)]
    unsafe fn get_field_raw_mut(
        this: *mut *mut (),
        name: __FieldName,
        param: P,
    ) -> Result<*mut Self::Ty, Self::Err> {
        <Self as SpecGetFieldMut<__FieldName, P>>::get_field_raw_mut_inner(this, name, param)
    }

    fn get_field_raw_mut_func(&self) -> GetFieldRawMutFn<__FieldName, P, Self::Ty, Self::Err> {
        <Self as GetFieldMutImpl<__FieldName, P>>::get_field_raw_mut
    }
}

unsafe impl<'a, T: 'a, __FieldName, P> SpecGetFieldMut<__FieldName, P> for &'a mut T
where
    T: ?Sized + GetFieldMutImpl<__FieldName, P>,
{
    default_if! {
        #[inline(always)]
        cfg(feature="specialization")
        unsafe fn get_field_raw_mut_inner(
            this:*mut *mut (),
            name:__FieldName,
            param:P,
        )->Result<*mut Self::Ty,Self::Err> {
            let func={
                // this is a triple pointer on purpose
                let this:*mut T=**(this as *mut *mut *mut T);
                T::get_field_raw_mut_func(&*this)
            };

            let this: *mut *mut () = *(this as *mut *mut *mut ());
            func( this, name, param )
        }
    }
}

#[cfg(feature = "specialization")]
unsafe impl<'a, T: 'a, __FieldName, P> SpecGetFieldMut<__FieldName, P> for &'a mut T
where
    T: Sized + GetFieldMutImpl<__FieldName, P>,
{
    unsafe fn get_field_raw_mut_inner(
        this: *mut *mut (),
        name: __FieldName,
        param: P,
    ) -> Result<*mut Self::Ty, Self::Err> {
        let this: *mut *mut () = *(this as *mut *mut *mut ());
        T::get_field_raw_mut(this, name, param)
    }
}
