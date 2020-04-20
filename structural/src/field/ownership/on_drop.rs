use super::{DropFields, DroppedFields, PrePostDropFields};

use std_::mem::ManuallyDrop;

/////////////////////////////////////////////////////////////////////////////////

pub struct AndDroppedFields<T: DropFields> {
    value: ManuallyDrop<T>,
    dropped: DroppedFields,
}

impl<T: DropFields> AndDroppedFields<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: ManuallyDrop::new(value),
            dropped: DroppedFields::new(),
        }
    }

    pub unsafe fn inner_and_dropped_mut(&mut self) -> (&mut T, &mut DroppedFields) {
        (&mut self.value, &mut self.dropped)
    }

    pub unsafe fn inner_and_dropped_raw(&mut self) -> (*mut T, *mut DroppedFields) {
        (&mut *self.value as *mut T, &mut self.dropped as *mut _)
    }
}

impl<T: DropFields> Drop for AndDroppedFields<T> {
    fn drop(&mut self) {
        unsafe {
            DropFields::drop_fields(&mut *self.value, self.dropped);
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////

macro_rules! declare_run_on_drop {
    (
        $(#[$meta:meta])*
        struct $struct:ident
        $(where[$($where_preds:tt)*])?
        {$($extra_var:ident : $extra_ty:ty),* $(,)?}
        this=$this:ident,
        drop={$($drop:tt)*}
    ) => (
        $(#[$meta])*
        pub struct $struct<'a,T>
        $(where $($where_preds)*)?
        {
            mutref:&'a mut T,
            $($extra_var : $extra_ty,)*
        }

        impl<'a,T> $struct<'a,T>
        $(where $($where_preds)*)?
        {
            pub unsafe fn new(mutref:&'a mut T $(,$extra_var : $extra_ty)*)->Self{
                Self{
                    mutref,
                    $($extra_var,)*
                }
            }

            pub fn get_mut(&mut self)->&mut T{
                self.mutref
            }
        }

        impl<'a,T> Drop for  $struct<'a,T>
        $(where $($where_preds)*)?
        {
            #[inline(always)]
            fn drop(&mut self){
                let $this=self;
                $($drop)*
            }
        }

    )
}

declare_run_on_drop! {
    struct RunDrop{}
    this=this,
    drop={
        unsafe{
            std_::ptr::drop_in_place(this.mutref)
        }
    }
}

declare_run_on_drop! {
    struct RunPreDrop
    where[ T: PrePostDropFields ]
    {}
    this=this,
    drop={
        unsafe{
            PrePostDropFields::pre_drop(this.mutref)
        }
    }
}

declare_run_on_drop! {
    struct RunPostDrop
    where[ T: PrePostDropFields ]
    {}
    this=this,
    drop={
        unsafe{
            PrePostDropFields::post_drop(this.mutref)
        }
    }
}

declare_run_on_drop! {
    struct RunDropFields
    where[ T: DropFields ]
    {
        dropped: DroppedFields,
    }
    this=this,
    drop={
        unsafe{
            this.mutref.drop_fields(this.dropped)
        }
    }
}

impl<'a, T> RunDropFields<'a, T>
where
    T: DropFields,
{
    pub fn get_mut_and_dropped_fields(&mut self) -> (&mut T, &mut DroppedFields) {
        (&mut self.mutref, &mut self.dropped)
    }
}
