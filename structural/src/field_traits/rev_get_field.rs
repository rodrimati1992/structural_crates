use crate::{
    mut_ref::MutRef,
    type_level::{FieldPath,FieldPath1},
    GetField,GetFieldMut,IntoField,
};

use std_::marker::PhantomData;

/////////////////////////////////////////////////////////////////////////////


pub type RevFieldType<'a,FieldName,This>=
    <FieldName as RevGetField<'a,This>>::Field;

pub type RevFieldMutType<'a,FieldName,This>=
    <FieldName as RevGetFieldMut<'a,This>>::Field;

pub type RevFieldMutRefType<'a,FieldName,This>=
    <FieldName as RevGetFieldMut<'a,This>>::FieldMutRef;

pub type RevIntoFieldType<'a,FieldName,This>=
    <FieldName as RevIntoField<'a,This>>::Field;


/////////////////////////////////////////////////////////////////////////////

/// Like GetField,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
pub trait RevGetField<'a,This:?Sized>{
    type Field:'a;

    fn rev_get_field(self,this:&'a This)->Self::Field;
}


/////////////////////////////////////////////////////////////////////////////

/// Like GetFieldMut,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// # Safety
///
/// TODO
pub unsafe trait RevGetFieldMut<'a,This:?Sized>{
    type Field:'a;
    type FieldMutRef:'a;

    fn rev_get_field_mut(self,this:&'a mut This)->Self::Field;

    unsafe fn rev_get_field_raw_mut(
        self,
        field:MutRef<'a,This>,
    )->Self::FieldMutRef;
}


/////////////////////////////////////////////////////////////////////////////

/// Like IntoField,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
pub trait RevIntoField<'a,This:?Sized>{
    type Field:'a;

    fn rev_into_field(self,this:This)->Self::Field
    where This:Sized;

    #[cfg(feature="alloc")]
    fn rev_box_into_field(self,this:Box<This>)->Self::Field;
}

/////////////////////////////////////////////////////////////////////////////


macro_rules! impl_get_multi_field {
    (inner;
        all( $($fname:ident)*)
        prefix_field_tys($($prefix:ident)* )
        last=$lastty:ident,
        all_field_tys( $($all_tys:ident)*)
        $( into_box_field_tys( $($ibf_prefix:ident)* ) )?
        $( into_box_impl( $($into_box_impl:tt)* ) )?
        enable_specialization($($enable_specialization:tt)*)
    )=>{
        impl<'a,$($fname,)* $($all_tys,)* This> 
            RevGetField<'a,This>
        for FieldPath<($($fname,)*)> 
        where
            This:?Sized+'a,
            $(
                $prefix:GetField<FieldPath1<$fname>,Ty=$all_tys>+'a,
            )*
            $lastty:Sized+'a,
        {
            type Field=&'a $lastty;

            #[inline(always)]
            fn rev_get_field(self,field:&'a This)->Self::Field{
                $( let field=GetField::<FieldPath1<$fname>>::get_field_(field); )*
                field
            }
        }
        //TODO:justify why this is safe
        unsafe impl<'a,$($fname,)* $($all_tys,)* This>
            RevGetFieldMut<'a,This>
        for FieldPath<($($fname,)*)> 
        where
            This:?Sized+'a,
            $(
                $prefix:GetFieldMut<FieldPath1<$fname>,Ty=$all_tys>+'a,
            )*
            $lastty:Sized+'a,
        {
            type Field=&'a mut $lastty;
            type FieldMutRef=MutRef<'a,$lastty>;

            fn rev_get_field_mut(self,field:&'a mut This)->Self::Field{
                $( let field=GetFieldMut::<FieldPath1<$fname>>::get_field_mut_(field); )*
                field
            }

            default_if!{
                #[inline]
                cfg(all(feature="specialization", $($enable_specialization)*))

                unsafe fn rev_get_field_raw_mut(
                    self,
                    field:MutRef<'a,This>,
                )->Self::FieldMutRef{
                    let field=field.ptr;
                    $(
                        let field={
                            let _:PhantomData<$all_tys>;
                            let func=(*field).get_field_raw_mut_func();
                            func(field as *mut (),PhantomData)
                        };
                    )*
                    MutRef::from_ptr(field)
                }
            }
        }

        //TODO:justify why this is safe
        #[cfg(all(feature="specialization", $($enable_specialization)*))]
        unsafe impl<'a,$($fname,)* $($all_tys,)* This>
            RevGetFieldMut<'a,This>
        for FieldPath<($($fname,)*)> 
        where
            This:'a,
            $(
                $prefix:GetFieldMut<FieldPath1<$fname>,Ty=$all_tys>+'a,
            )*
            $lastty:Sized+'a,
        {
            #[inline]
            unsafe fn rev_get_field_raw_mut(
                self,
                field:MutRef<'a,This>,
            )->Self::FieldMutRef{
                let field=field.ptr;
                $( let field=$prefix::get_field_raw_mut(field as *mut (),PhantomData); )*
                MutRef::from_ptr(field)
            }
        }



        impl<'a,$($fname,)* $($all_tys,)* This>
            RevIntoField<'a,This>
        for FieldPath<($($fname,)*)> 
        where
            This:?Sized+'a,
            $(
                $prefix:IntoField<FieldPath1<$fname>,Ty=$all_tys>+'a,
            )*
            $lastty:Sized+'a,
        {
            type Field=$lastty;

            #[inline(always)]
            fn rev_into_field(self,field:This)->Self::Field
            where
                This:Sized
            {
                $( let field=IntoField::<FieldPath1<$fname>>::into_field_(field); )*
                field
            }

            $(
                #[cfg(feature="alloc")]
                fn rev_box_into_field(self,field:Box<This>)->Self::Field{
                    let field=IntoField::box_into_field_(field);
                    $(
                        let field=$ibf_prefix::into_field_(field); 
                    )*
                    field
                }
            )?

            $( 
                $($into_box_impl)*
            )?
        }

    };
    ( 
        $(($fname:ident $fty:ident))*
        ;last=($last_fname:ident $last_fty:ident)
    ) => {
        impl_get_multi_field!{
            inner;

            all($($fname)* $last_fname)
            prefix_field_tys(This $($fty)*)
            last=$last_fty,
            all_field_tys($($fty)* $last_fty)
            into_box_field_tys($($fty)*)
            enable_specialization()
        }
    }
}

impl_get_multi_field!{
    inner;

    all()
    prefix_field_tys()
    last=This,
    all_field_tys()

    into_box_impl(
        #[cfg(feature="alloc")]
        fn rev_box_into_field(self,field:Box<This>)->This{
            *field
        }
    )

    enable_specialization(FALSE)
}

impl_get_multi_field!{
    ;last=(FL TL)
}
impl_get_multi_field!{
    (F0 T0)
    ;last=(FL TL)
}
impl_get_multi_field!{
    (F0 T0)
    (F1 T1)
    ;last=(FL TL)
}
impl_get_multi_field!{
    (F0 T0)
    (F1 T1)
    (F2 T2)
    ;last=(FL TL)
}
impl_get_multi_field!{
    (F0 T0)
    (F1 T1)
    (F2 T2)
    (F3 T3)
    ;last=(FL TL)
}
impl_get_multi_field!{
    (F0 T0)
    (F1 T1)
    (F2 T2)
    (F3 T3)
    (F4 T4)
    ;last=(FL TL)
}
impl_get_multi_field!{
    (F0 T0)
    (F1 T1)
    (F2 T2)
    (F3 T3)
    (F4 T4)
    (F5 T5)
    ;last=(FL TL)
}
impl_get_multi_field!{
    (F0 T0)
    (F1 T1)
    (F2 T2)
    (F3 T3)
    (F4 T4)
    (F5 T5)
    (F6 T6)
    ;last=(FL TL)
}
