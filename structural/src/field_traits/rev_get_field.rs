/*!
Contains traits implemented on field paths,taking Structural types as parameters.
*/

use crate::{
    field_traits::{
        errors::{CombineErrs, FieldErr, IntoFieldErr},
        NormalizeFields,
    },
    type_level::{FieldPath, FieldPath1},
    GetFieldImpl, GetFieldMutImpl, IntoFieldImpl,
};

#[cfg(feature = "alloc")]
use crate::pmr::Box;

use std_::marker::PhantomData;

/////////////////////////////////////////////////////////////////////////////

/// Gets the type used to access the `FieldName` field(s) in `This` by reference.
pub type RevFieldRefType<'a, FieldName, This> = <FieldName as RevGetField<'a, This>>::Field;

/// Gets the type used to access the `FieldName` field(s) in `This` by mutable reference.
pub type RevFieldMutType<'a, FieldName, This> = <FieldName as RevGetFieldMut<'a, This>>::Field;

/// Gets the type used to access the `FieldName` field(s) in `This` by `MutRef`.
pub type RevFieldRawMutType<'a, FieldName, This> =
    <FieldName as RevGetFieldMut<'a, This>>::FieldRawMut;

/// Gets the type used to access the `FieldName` field(s) in `This` by value.
pub type RevIntoFieldType<'a, FieldName, This> = <FieldName as RevIntoField<'a, This>>::Field;

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
pub type RevGetFieldType<FieldName, This> = <FieldName as RevGetFieldType_<This>>::Output;

/// Allows querying the type of nested field in This,
/// what field is queried is determined by `Self`,
pub trait RevGetFieldType_<This: ?Sized> {
    type Output;
}

/////////////////////////////////////////////////////////////////////////////

pub trait RevGetFieldErr_<This: ?Sized> {
    type Output;
}

pub type RevGetFieldErr<FieldName, This> = <FieldName as RevGetFieldErr_<This>>::Output;

/////////////////////////////////////////////////////////////////////////////

/// Like GetFieldImpl,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
pub trait RevGetField<'a, This: ?Sized> {
    /// The reference-containing type this returns.
    type Field: 'a + NormalizeFields;

    /// Accesses the field(s) that `self` represents inside of `this`,by reference.
    fn rev_get_field(self, this: &'a This) -> Self::Field;
}

/////////////////////////////////////////////////////////////////////////////

/// Like GetFieldMutImpl,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
///
/// # Safety
///
/// The implementation must assign virtually the same type to both
/// the `Field` and `FieldRawMut` associated types.
///
/// - If `Field` is a single mutable references,
/// `FieldRawMut` must be a `MutRef` pointing to the same type.
///
/// - If `Field` is a collection of mutable references,
/// `FieldRawMut` must be the same collection with the
/// mutable references replaced with the `MutRef` type.
pub unsafe trait RevGetFieldMut<'a, This: ?Sized> {
    /// The mutable-reference-containing type this returns.
    type Field: 'a + NormalizeFields;
    /// The `MUtRef`-containing type this returns.
    type FieldRawMut: NormalizeFields;

    /// Accesses the field(s) that `self` represents inside of `this`,by mutable reference.
    fn rev_get_field_mut(self, this: &'a mut This) -> Self::Field;

    /// Accesses the field(s) that `self` represents inside of `this`,by raw pointer.
    unsafe fn rev_get_field_raw_mut(self, field: *mut This) -> Self::FieldRawMut;
}

/////////////////////////////////////////////////////////////////////////////

/// Like IntoFieldImpl,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
pub trait RevIntoField<'a, This: ?Sized> {
    /// The type this returns.
    type Field: 'a + NormalizeFields;

    /// Accesses the field(s) that `self` represents inside of `this`,by value.
    fn rev_into_field(self, this: This) -> Self::Field
    where
        This: Sized;

    /// Accesses the field(s) that `self` represents inside of `this`,by value.
    #[cfg(feature = "alloc")]
    fn rev_box_into_field(self, this: Box<This>) -> Self::Field;
}

/////////////////////////////////////////////////////////////////////////////

macro_rules! impl_get_nested_field_inner {
    (inner;
        all( $($fname:ident)*)
        prefix_field_tys($($prefix:ident)* )
        last=$lastty:ident
        last_err=$last_ferr:ident
        all_field_tys( $($all_tys:ident)*)
        all_field_errs($($ferr:ident)*)
        $( into_box_field_tys( $($ibf_prefix:ident)* ) )?
        $( into_box_impl( $($into_box_impl:tt)* ) )?
        enable_specialization($($enable_specialization:tt)*)
    )=>{
        impl<$($fname,)* $($all_tys,)* This,CombErrs>
            RevGetFieldErr_<This>
        for FieldPath<($($fname,)*)>
        where
            This:?Sized,
            $(
                $prefix:GetFieldImpl<FieldPath1<$fname>,Ty=$all_tys>,
            )*
            ($(<$prefix as GetFieldImpl<FieldPath1<$fname>>>::Err,)*):
                CombineErrs<Combined=CombErrs>,
            CombErrs:FieldErr,
        {
            type Output=CombErrs;
        }

        impl<$($fname,)* $($all_tys,)* This>
            RevGetFieldType_<This>
        for FieldPath<($($fname,)*)>
        where
            This:?Sized,
            $(
                $prefix:GetFieldImpl<FieldPath1<$fname>,Ty=$all_tys>,
            )*
            $lastty:Sized,
        {
            type Output=$lastty;
        }

        impl<'a,$($fname,)* $($all_tys,$ferr,)* This,CombErr>
            RevGetField<'a,This>
        for FieldPath<($($fname,)*)>
        where
            This:?Sized+'a,
            Self:RevGetFieldErr_<This,Output=CombErr>,
            CombErr:FieldErr+'a,
            $(
                $prefix:GetFieldImpl<FieldPath1<$fname>,Ty=$all_tys,Err=$ferr>+'a,
                $ferr:IntoFieldErr< CombErr >+'a,
            )*
            $lastty:Sized+'a,
            Result<&'a $lastty,CombErr>:NormalizeFields,
        {
            type Field=Result<&'a $lastty, CombErr >;

            #[inline(always)]
            fn rev_get_field(self,field:&'a This)->Self::Field{
                $( let field=try_fe!(GetFieldImpl::<FieldPath1<$fname>>::get_field_(field)); )*
                Ok(field)
            }
        }

        unsafe impl<'a,$($fname,)* $($all_tys,$ferr,)* This,CombErr>
            RevGetFieldMut<'a,This>
        for FieldPath<($($fname,)*)>
        where
            This:?Sized+'a,
            Self:RevGetFieldErr_<This,Output=CombErr>,
            CombErr:FieldErr+'a,
            $(
                $prefix:GetFieldMutImpl<FieldPath1<$fname>,Ty=$all_tys,Err=$ferr>+'a,
                $ferr:IntoFieldErr< CombErr >+'a,
            )*
            $lastty:Sized+'a,
            Result<&'a mut $lastty,CombErr>:NormalizeFields,
            Result<*mut $lastty,CombErr>:NormalizeFields,
        {
            type Field=Result<&'a mut $lastty,CombErr >;
            type FieldRawMut=Result<*mut $lastty,CombErr>;

            #[inline(always)]
            fn rev_get_field_mut(self,field:&'a mut This)->Self::Field{
                $( let field=try_fe!(GetFieldMutImpl::<FieldPath1<$fname>>::get_field_mut_(field)); )*
                Ok(field)
            }

            default_if!{
                #[inline(always)]
                cfg(all(feature="specialization", $($enable_specialization)*))

                unsafe fn rev_get_field_raw_mut( self, field:*mut This )->Self::FieldRawMut{
                    $(
                        let field={
                            let _:PhantomData<$all_tys>;
                            let func=(*field).get_field_raw_mut_func();
                            try_fe!(func(field as *mut (),PhantomData))
                        };
                    )*
                    Ok(field)
                }
            }
        }

        #[cfg(all(feature="specialization", $($enable_specialization)*))]
        unsafe impl<'a,$($fname,)* $($all_tys,$ferr,)* This,CombErr>
            RevGetFieldMut<'a,This>
        for FieldPath<($($fname,)*)>
        where
            This:'a,
            Self:RevGetFieldErr_<This,Output=CombErr>,
            CombErr:FieldErr+'a,
            $(
                $prefix:GetFieldMutImpl<FieldPath1<$fname>,Ty=$all_tys,Err=$ferr>+'a,
                $ferr:IntoFieldErr< CombErr >+'a,
            )*
            $lastty:Sized+'a,
            Result<&'a mut $lastty,CombErr>:NormalizeFields,
            Result<*mut $lastty,CombErr>:NormalizeFields,
        {
            #[inline(always)]
            unsafe fn rev_get_field_raw_mut( self, field:*mut This )->Self::FieldRawMut{
                $(
                    let field=try_fe!($prefix::get_field_raw_mut(field as *mut (),PhantomData));
                )*
                Ok(field)
            }
        }



        impl<'a,$($fname,)* $($all_tys,$ferr,)* This,CombErr>
            RevIntoField<'a,This>
        for FieldPath<($($fname,)*)>
        where
            This:?Sized+'a,
            Self:RevGetFieldErr_<This,Output=CombErr>,
            CombErr:FieldErr+'a,
            $(
                $prefix:IntoFieldImpl<FieldPath1<$fname>,Ty=$all_tys,Err=$ferr>+'a,
                $ferr:IntoFieldErr< CombErr >+'a,
            )*
            $lastty:Sized+'a,
            Result<$lastty,CombErr>:NormalizeFields,
        {
            type Field=Result<$lastty,CombErr >;

            #[inline(always)]
            fn rev_into_field(self,field:This)->Self::Field
            where
                This:Sized
            {
                $( let field=try_fe!(IntoFieldImpl::<FieldPath1<$fname>>::into_field_(field)); )*
                Ok(field)
            }

            $(
                #[cfg(feature="alloc")]
                #[inline(always)]
                fn rev_box_into_field(self,field:$crate::pmr::Box<This>)->Self::Field{
                    let field=try_fe!(IntoFieldImpl::box_into_field_(field));
                    $(
                        let field=try_fe!($ibf_prefix::into_field_(field));
                    )*
                    Ok(field)
                }
            )?

            $(
                $($into_box_impl)*
            )?
        }

    };
    (
        $(($fname:ident $ferr:ident $fty:ident))*
        ;last=($last_fname:ident $last_ferr:ident $last_fty:ident)
    ) => {
        impl_get_nested_field_inner!{
            inner;

            all($($fname)* $last_fname)
            prefix_field_tys(This $($fty)*)
            last=$last_fty
            last_err=$last_ferr
            all_field_tys($($fty)* $last_fty)
            all_field_errs($($ferr)* $last_ferr)
            into_box_field_tys($($fty)*)
            enable_specialization()
        }
    }
}

impl_get_nested_field_inner! {
    inner;

    all()
    prefix_field_tys()
    last=This
    last_err=NonOptField
    all_field_tys()
    all_field_errs()

    into_box_impl(
        #[cfg(feature="alloc")]
        #[inline(always)]
        fn rev_box_into_field(self,field:Box<This>)->Result<This,CombErr>{
            Ok(*field)
        }
    )

    enable_specialization(FALSE)
}

impl_get_nested_field_inner! {
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    (F4 E4 T4)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    (F4 E4 T4)
    (F5 E5 T5)
    ;last=(FL EL TL)
}
impl_get_nested_field_inner! {
    (F0 E0 T0)
    (F1 E1 T1)
    (F2 E2 T2)
    (F3 E3 T3)
    (F4 E4 T4)
    (F5 E5 T5)
    (F6 E6 T6)
    ;last=(FL EL TL)
}
