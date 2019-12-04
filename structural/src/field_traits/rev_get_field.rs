/*!
Contains traits implemented on field paths,taking Structural types as parameters.
*/

use crate::{
    mut_ref::MutRef,
    type_level::{FieldPath,FieldPath1},
    GetField,GetFieldMut,IntoField,
};

#[cfg(feature="alloc")]
use crate::pmr::Box;

use std_::marker::PhantomData;

/////////////////////////////////////////////////////////////////////////////


/// Gets the type used to access the `FieldName` field(s) in `This` by reference.
pub type RevFieldRefType<'a,FieldName,This>=
    <FieldName as RevGetField<'a,This>>::Field;

/// Gets the type used to access the `FieldName` field(s) in `This` by mutable reference.
pub type RevFieldMutType<'a,FieldName,This>=
    <FieldName as RevGetFieldMut<'a,This>>::Field;

/// Gets the type used to access the `FieldName` field(s) in `This` by `MutRef`.
pub type RevFieldMutRefType<'a,FieldName,This>=
    <FieldName as RevGetFieldMut<'a,This>>::FieldMutRef;

/// Gets the type used to access the `FieldName` field(s) in `This` by value.
pub type RevIntoFieldType<'a,FieldName,This>=
    <FieldName as RevIntoField<'a,This>>::Field;


/////////////////////////////////////////////////////////////////////////////

/// Allows querying the type of a nested field in This,
/// what field is queried is determined by `FieldName`,
/// 
/// # Example
/// 
/// ```
/// use structural::{
///     GetFieldType3,GetFieldExt,RevGetFieldType,Structural,
///     field_path_aliases,
/// };
/// 
/// field_path_aliases!{
///     foo_bar_baz= foo.bar.baz,
///     foo_bar_strand= foo.bar.strand,
/// }
/// 
/// fn main(){
///     let this=TopLevel::default();
///     
///     let baz: &RevGetFieldType<foo_bar_baz, TopLevel>=
///         this.field_(foo_bar_baz);
///     assert_eq!( *baz, Vec::new() );
///     
///     let strand: &RevGetFieldType<foo_bar_strand, TopLevel>= 
///         this.field_(foo_bar_strand);
///     assert_eq!( *strand, String::new() );
/// }
/// 
/// #[derive(Debug,Default,Structural)]
/// struct TopLevel{
///     pub foo:Foo,
/// }
/// 
/// #[derive(Debug,Default,Structural)]
/// struct Foo{
///     pub bar:Bar,
/// }
/// 
/// #[derive(Debug,Default,Structural)]
/// struct Bar{
///     pub baz:Vec<()>,
///     pub strand:String,
/// }
/// ```
pub type RevGetFieldType<FieldName,This>=
    <FieldName as RevGetFieldType_<This>>::Output;

/// Allows querying the type of nested field in This,
/// what field is queried is determined by `Self`,
pub trait RevGetFieldType_<This:?Sized>{
    type Output;
}


/////////////////////////////////////////////////////////////////////////////

/// Like GetField,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
pub trait RevGetField<'a,This:?Sized>{
    /// The reference-containing type this returns.
    type Field:'a;

    /// Accesses the field(s) that `self` represents inside of `this`,by reference.
    fn rev_get_field(self,this:&'a This)->Self::Field;
}


/////////////////////////////////////////////////////////////////////////////

/// Like GetFieldMut,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// # Safety
///
/// The implementation must assign virtually the same type to both
/// the `Field` and `FieldMutRef` associated types.
///
/// - If `Field` is a single mutable references,
/// `FieldMutRef` must be a `MutRef` pointing to the same type.
///
/// - If `Field` is a collection of mutable references,
/// `FieldMutRef` must be the same collection with the 
/// mutable references replaced with the `MutRef` type.
pub unsafe trait RevGetFieldMut<'a,This:?Sized>{
    /// The mutable-reference-containing type this returns.
    type Field:'a;
    /// The `MUtRef`-containing type this returns.
    type FieldMutRef:'a;

    /// Accesses the field(s) that `self` represents inside of `this`,by mutable reference.
    fn rev_get_field_mut(self,this:&'a mut This)->Self::Field;

    /// Accesses the field(s) that `self` represents inside of `this`,by `MutRef`.
    unsafe fn rev_get_field_raw_mut(
        self,
        field:MutRef<'a,This>,
    )->Self::FieldMutRef;
}


/////////////////////////////////////////////////////////////////////////////

/// Like IntoField,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
pub trait RevIntoField<'a,This:?Sized>{
    /// The type this returns.
    type Field:'a;

    /// Accesses the field(s) that `self` represents inside of `this`,by value.
    fn rev_into_field(self,this:This)->Self::Field
    where This:Sized;

    /// Accesses the field(s) that `self` represents inside of `this`,by value.
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
        impl<$($fname,)* $($all_tys,)* This> 
            RevGetFieldType_<This>
        for FieldPath<($($fname,)*)> 
        where
            This:?Sized,
            $(
                $prefix:GetField<FieldPath1<$fname>,Ty=$all_tys>,
            )*
            $lastty:Sized,
        {
            type Output=$lastty;
        }


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
                fn rev_box_into_field(self,field:$crate::pmr::Box<This>)->Self::Field{
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
