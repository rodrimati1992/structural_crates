/*!
Contains traits for accessing multiple fields at once.
*/
#![allow(non_snake_case)]

use super::*;

/*
pub trait RevGetField<'a,This:?Sized>{
    type Field:'a;

    fn rev_get_field(self,this:&'a This)->&'a Self::Field;
}

pub unsafe trait RevGetFieldMut<'a,This:?Sized>{
    type Field:'a;

    fn rev_get_field_mut(self,this:&'a mut This)->&'a mut Self::Field;

    unsafe fn rev_get_field_raw_mut(
        self,
        field:MutRef<'a,This>,
    )->MutRef<'a,Self::Field>;
}
*/


macro_rules! impl_get_multi_field {
    ( $(($fpath:ident $fty:ident))* ) => (
        impl<'a,This:?Sized,$($fpath,)* $($fty,)* U> 
            RevGetField<'a,This> 
        for FieldPathSet<($(FieldPath<$fpath>,)*),U>
        where
            $(
                FieldPath<$fpath>:RevGetField<'a,This,Field=&'a $fty>,
                $fty:'a,
            )*
        {
            type Field=(
                $(
                    &'a $fty,
                )*
            );

            fn rev_get_field(self,this:&'a This)->Self::Field{
                (
                    $(
                        FieldPath::<$fpath>::NEW.rev_get_field(this),
                    )*
                )
            }
        }        

        unsafe impl<'a,This:?Sized,$($fpath,)* $($fty,)*>
            RevGetFieldMut<'a,This> 
        for FieldPathSet<($(FieldPath<$fpath>,)*),UniquePaths> 
        where
            $(
                FieldPath<$fpath>:RevGetFieldMut<
                    'a,
                    This,
                    Field=&'a mut $fty,
                    FieldMutRef=MutRef<'a,$fty>,
                >,
                $fty:'a,
                // RevFieldMutType<'a,FieldPath<$fpath>,This>:'a,
            )*
        {
            type Field=(
                $(
                    &'a mut $fty,
                )*
            );
            type FieldMutRef=(
                $(
                    MutRef<'a,$fty>,
                )*
            );

            fn rev_get_field_mut(self,this:&'a mut This)->Self::Field{
                unsafe{
                    let ($($fpath,)*)={
                        let this=MutRef::new(this);

                        self.rev_get_field_raw_mut(this)
                    };

                    (
                        $(
                            &mut *$fpath.ptr,
                        )*
                    )
                }
            }

            unsafe fn rev_get_field_raw_mut(
                self,
                this:MutRef<'a,This>,
            )->Self::FieldMutRef{
                (
                    $(
                        FieldPath::<$fpath>::NEW.rev_get_field_raw_mut(this),
                    )*
                )
            }
        }
    )
}


impl_get_multi_field!{}
impl_get_multi_field!{
    (F0 T0)
}
impl_get_multi_field!{
    (F0 T0) (F1 T1)
}
impl_get_multi_field!{
    (F0 T0) (F1 T1) (F2 T2)
}
impl_get_multi_field!{
    (F0 T0) (F1 T1) (F2 T2) (F3 T3)
}
impl_get_multi_field!{
    (F0 T0) (F1 T1) (F2 T2) (F3 T3) (F4 T4)
}
impl_get_multi_field!{
    (F0 T0) (F1 T1) (F2 T2) (F3 T3) (F4 T4) (F5 T5)
}
impl_get_multi_field!{
    (F0 T0) (F1 T1) (F2 T2) (F3 T3) (F4 T4) (F5 T5) (F6 T6)
}
impl_get_multi_field!{
    (F0 T0) (F1 T1) (F2 T2) (F3 T3) (F4 T4) (F5 T5) (F6 T6) (F7 T7)
}

