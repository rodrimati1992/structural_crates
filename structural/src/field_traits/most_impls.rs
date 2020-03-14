#![allow(non_camel_case_types)]

use super::*;

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
        (IntoFieldMut < start : T,Start_STR,"start",> )
        (IntoFieldMut < end : T,End_STR,"end",> )
    }
}

_private_impl_getters_for_derive_struct! {
    impl[T] RangeFrom<T>
    where[]
    {
        (IntoFieldMut < start : T,Start_STR,"start",> )
    }
}

_private_impl_getters_for_derive_struct! {
    impl[T] RangeTo<T>
    where[]
    {
        (IntoFieldMut < end : T,End_STR,"end",> )
    }
}

_private_impl_getters_for_derive_struct! {
    impl[T] RangeToInclusive<T>
    where[]
    {
        (IntoFieldMut < end : T,End_STR,"end",> )
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

impl<T> GetField<Start_STR> for RangeInclusive<T> {
    fn get_field_(&self, _: Start_STR) -> &Self::Ty {
        self.start()
    }
}
impl<T> GetField<End_STR> for RangeInclusive<T> {
    fn get_field_(&self, _: End_STR) -> &Self::Ty {
        self.end()
    }
}

impl<T> IntoField<Start_STR> for RangeInclusive<T> {
    fn into_field_(self, _: Start_STR) -> Self::Ty {
        self.into_inner().0
    }
    z_impl_box_into_field_method! {Start_STR}
}
impl<T> IntoField<End_STR> for RangeInclusive<T> {
    fn into_field_(self, _: End_STR) -> Self::Ty {
        self.into_inner().0
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

    GetField { this }

    unsafe GetFieldMut { this }
    as_delegating_raw{ this as *mut ManuallyDrop<T> as *mut T }

    IntoField { ManuallyDrop::into_inner(this) }
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

    GetField { &*this }
}

#[test]
fn delegated_pin() {
    let tup = (2, 3, 5, 8);
    let pin = Pin::new(&tup);
    assert_eq!(pin.fields(fp!(0, 1, 2, 3)), (&2, &3, &5, &8));
    //assert_eq!( pin.fields_mut(fp!(0,1,2,3)), (&mut 2,&mut 3,&mut 5,&mut 8) );
}

///////////////////////////////////////////////////////

#[cfg(feature = "alloc")]
mod alloc_impls {
    use crate::alloc::{boxed::Box, rc::Rc, sync::Arc};

    macro_rules! impl_shared_ptr_accessors {
        ( $this:ident ) => {
            unsafe_delegate_structural_with! {
                impl[T,] $this<T>
                where[T:?Sized,]

                self_ident=this;
                delegating_to_type=T;

                GetField {
                    &*this
                }
            }
        };
    }
    impl_shared_ptr_accessors! {Arc}
    impl_shared_ptr_accessors! {Rc}

    unsafe_delegate_structural_with! {
        impl[T,] Box<T>
        where[T:?Sized,]

        self_ident=this;
        specialization_params(specialize_cfg(feature="specialization"));
        delegating_to_type=T;

        GetField {
            &*this
        }

        unsafe GetFieldMut{
            &mut **this
        }
        as_delegating_raw{
            *(this as *mut Box<T> as *mut *mut T)
        }


        IntoField{
            *this
        }
    }
}

///////////////////////////////////////////////////////

unsafe_delegate_structural_with! {
    impl['a,T,] &'a T
    where [T:?Sized,]
    self_ident=this;
    delegating_to_type=T;

    GetField { &**this }

}

///////////////////////////////////////////////////////

unsafe_delegate_structural_with! {
    impl['a,T:'a,] &'a mut T
    where [T:?Sized,]
    self_ident=this;
    specialization_params(specialize_cfg(feature="specialization"));
    delegating_to_type=T;

    GetField { &**this }

    unsafe GetFieldMut{
        *this
    }
    as_delegating_raw{
        *(this as *mut *mut T)
    }
}
