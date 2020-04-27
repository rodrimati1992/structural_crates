#![allow(non_camel_case_types)]

use crate::{FieldType, GetField, Structural};

#[allow(unused_imports)]
use crate::StructuralExt;

use std_::{
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
        DropFields{ drop_fields={just_fields,} }

        (IntoFieldMut < start : T,0,Start_STR,"start",> )
        (IntoFieldMut < end : T,1,End_STR,"end",> )
    }

}

_private_impl_getters_for_derive_struct! {
    impl[T] RangeFrom<T>
    where[]
    {
        DropFields{ drop_fields={just_fields,} }

        (IntoFieldMut < start : T,0,Start_STR,"start",> )
    }
}

_private_impl_getters_for_derive_struct! {
    impl[T] RangeTo<T>
    where[]
    {
        DropFields{ drop_fields={just_fields,} }

        (IntoFieldMut < end : T,0,End_STR,"end",> )
    }
}

_private_impl_getters_for_derive_struct! {
    impl[T] RangeToInclusive<T>
    where[]
    {
        DropFields{ drop_fields={just_fields,} }

        (IntoFieldMut < end : T,0,End_STR,"end",> )
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

///////////////////////////////////////////////////////

// This allows using all the field accessors in T from `ManuallyDrop<T>`
unsafe_delegate_structural_with! {
    impl[T,] ManuallyDrop<T>
    where[]
    self_ident=this;
    specialization_params(Sized);
    delegating_to_type=T;

    GetField { this }

    GetFieldMut { this }
    as_delegating_raw{ this as *mut ManuallyDrop<T> as *mut T }

    IntoField { ManuallyDrop::into_inner(this) }
    move_out_field { &mut *this }

    DropFields = {
        dropped_fields[]
        drop_delegated_to_variable=false;
    }
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
    use crate::{
        alloc::{boxed::Box, rc::Rc, sync::Arc},
        field::{
            ownership::{DropFields, IntoFieldsWrapper, MovedOutFields},
            IntoField, IntoVariantField,
        },
        TStr,
    };
    use std_::mem::ManuallyDrop;

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

        GetFieldMut{
            &mut **this
        }
        as_delegating_raw{
            *(this as *mut Box<T> as *mut *mut T)
        }
    }

    unsafe impl<T, P> IntoField<P> for Box<T>
    where
        T: ?Sized + IntoField<P>,
    {
        #[inline(always)]
        fn into_field_(self, path: P) -> Self::Ty {
            unsafe {
                let mut this = IntoFieldsWrapper::new(self);
                let (this, moved) = this.inner_and_moved_mut();
                let this: &mut T = this;
                this.move_out_field_(path, moved)
            }
        }

        #[inline(always)]
        unsafe fn move_out_field_(&mut self, path: P, moved: &mut MovedOutFields) -> Self::Ty {
            let this: &mut T = self;
            this.move_out_field_(path, moved)
        }
    }

    unsafe impl<T, V, F, Ty> IntoVariantField<TStr<V>, F> for Box<T>
    where
        T: ?Sized + IntoVariantField<TStr<V>, F, Ty = Ty>,
    {
        #[inline(always)]
        fn into_vfield_(self, vname: TStr<V>, fname: F) -> Option<Ty> {
            unsafe {
                let mut this = IntoFieldsWrapper::new(self);
                let (this, moved) = this.inner_and_moved_mut();
                let this: &mut T = this;
                this.move_out_vfield_(vname, fname, moved)
            }
        }

        #[inline(always)]
        unsafe fn move_out_vfield_(
            &mut self,
            vname: TStr<V>,
            fname: F,
            moved: &mut MovedOutFields,
        ) -> Option<Ty> {
            let this: &mut T = self;
            this.move_out_vfield_(vname, fname, moved)
        }
    }

    unsafe impl<T> DropFields for Box<T>
    where
        T: ?Sized + DropFields,
    {
        #[inline(always)]
        fn pre_move(&mut self) {
            <T as DropFields>::pre_move(&mut **self);
        }

        #[inline(always)]
        unsafe fn drop_fields(&mut self, moved: MovedOutFields) {
            let mut this = Box::<ManuallyDrop<T>>::from_raw(
                &mut **({ self } as *mut Box<T> as *mut Box<ManuallyDrop<T>>),
            );
            let this: &mut T = &mut **this;
            <T as DropFields>::drop_fields(this, moved)
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

    GetFieldMut{
        *this
    }
    as_delegating_raw{
        *(this as *mut *mut T)
    }
}
