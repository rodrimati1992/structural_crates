/*!
Contains traits implemented on field paths,taking Structural types as parameters.
*/

use crate::{
    enum_traits::{EnumExt, IsVariant, VariantProxy},
    field_traits::{
        errors::{CombinedErrs, CombinedErrsOut, IntoFieldErr, IsFieldErr},
        GetFieldErr, NonOptField, OptionalField,
    },
    type_level::_private::TString,
    type_level::{FieldPath, FieldPath1, VariantField, VariantName},
    GetFieldImpl, GetFieldMutImpl, GetFieldType, IntoFieldImpl,
};

#[cfg(feature = "alloc")]
use crate::pmr::Box;

use core_extensions::MarkerType;

use std_::marker::PhantomData;

/////////////////////////////////////////////////////////////////////////////

/// Allows querying the type of a nested field in This,
/// what field is queried is determined by `FieldName`,
///
/// # Example
///
/// ```
/// use structural::{
///     field_traits::RevGetFieldType,
///     GetFieldType3,GetFieldExt,Structural,
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
pub type RevGetFieldType<'a, FieldName, This> = <FieldName as RevGetField<'a, This>>::Ty;

pub type RevIntoBoxedFieldType<'a, FieldName, This> =
    <FieldName as RevIntoField<'a, This>>::BoxedTy;

pub type RevGetFieldErr<'a, FieldName, This> = <FieldName as RevGetField<'a, This>>::Err;

/////////////////////////////////////////////////////////////////////////////

/// Like GetFieldImpl,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
pub trait RevGetField<'a, This: ?Sized> {
    /// The reference-containing type this returns.
    type Ty: ?Sized + 'a;
    type Err;

    /// Accesses the field(s) that `self` represents inside of `this`,by reference.
    fn rev_get_field(self, this: &'a This) -> Result<&'a Self::Ty, Self::Err>;
}

/////////////////////////////////////////////////////////////////////////////

/// Like GetFieldMutImpl,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
pub trait RevGetFieldMut<'a, This: ?Sized>: RevGetField<'a, This> {
    /// Accesses the field(s) that `self` represents inside of `this`,by mutable reference.
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut Self::Ty, Self::Err>;

    /// Accesses the field(s) that `self` represents inside of `this`,by raw pointer.
    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut Self::Ty, Self::Err>;
}

/////////////////////////////////////////////////////////////////////////////

/// Like IntoFieldImpl,except that the parameters are reversed,
/// `This` is the type we are accessing,and `Self` is a field path.
pub trait RevIntoField<'a, This: ?Sized>: RevGetField<'a, This> {
    type BoxedTy: 'a;

    /// Accesses the field(s) that `self` represents inside of `this`,by value.
    fn rev_into_field(self, this: This) -> Result<Self::Ty, Self::Err>
    where
        This: Sized,
        Self::Ty: Sized;

    /// Accesses the field(s) that `self` represents inside of `this`,by value.
    #[cfg(feature = "alloc")]
    fn rev_box_into_field(self, this: Box<This>) -> Result<Self::BoxedTy, Self::Err>;
}

/////////////////////////////////////////////////////////////////////////////

macro_rules! get_last_ident {
    ($first:ty ) => ( $first );
    ($first:ty , $last:ty ) => ( $last );
    ($first:ty , $last:ty $(,$rem:ty)+ ) => (
        get_last_ident!{ $($rem),* }
    );
}

macro_rules! impl_get_nested_field_inner {
    (inner;
        receivers( $($receiver:ident)* )
        first($fname0:ident $ferr0:ident $fty0:ident)
        second(
            ($fname1:ident $ferr1:ident $fty1:ident)
            $($rem_000:tt)*
        )
        middle(
            $(($fname_m:ident $ferr_m:ident $fty_m:ident))*
        )
        suffix(
            $(($fname_s:ident $ferr_s:ident $fty_s:ident))*
        )
        all(
            $(($fname_a:ident $ferr_a:ident $fty_a:ident))*
        )
        last($fname_l:ident $ferr_l:ident $fty_l:ident)
    )=>{
        impl<'a,$($fname_a,$fty_a, $ferr_a,)* This,CombErr>
            RevGetField<'a,This>
        for FieldPath<($($fname_a,)*)>
        where
            This:?Sized+'a,
            $(
                $fname_a: MarkerType + RevGetField<'a,$receiver, Ty=$fty_a, Err=$ferr_a>,
                $fty_a:?Sized+'a,
                $ferr_a:IntoFieldErr< CombErr >,
            )*
            ( $($ferr_a,)* ): CombinedErrs<Combined= CombErr >,
            CombErr:IsFieldErr,
        {
            type Ty=$fty_l;
            type Err=CombErr;

            #[inline(always)]
            fn rev_get_field(self,field:&'a This)->Result<&'a $fty_l,CombErr>{
                $(
                    let name=<$fname_a as MarkerType>::MTVAL;
                    let field=try_fe!( name.rev_get_field(field) );
                )*
                Ok(field)
            }
        }


        impl<'a,$($fname_a,$fty_a, $ferr_a,)* This,CombErr>
            RevGetFieldMut<'a,This>
        for FieldPath<($($fname_a,)*)>
        where
            This:?Sized+'a,
            $(
                $fname_a: MarkerType + RevGetFieldMut<'a,$receiver, Ty=$fty_a, Err=$ferr_a>,
                $fty_a:'a,
                $ferr_a:IntoFieldErr< CombErr >,
            )*
            Self:RevGetField<'a,This,Ty=$fty_l,Err=CombErr>,
            CombErr:IsFieldErr,
        {
            #[inline(always)]
            fn rev_get_field_mut(self,field:&'a mut This)->Result<&'a mut $fty_l,CombErr >{
                $(
                    let name=<$fname_a as MarkerType>::MTVAL;
                    let field=try_fe!( name.rev_get_field_mut(field) );
                )*
                Ok(field)
            }

            unsafe fn rev_get_field_raw_mut(
                self,
                field:*mut This
            )->Result<*mut $fty_l,CombErr>{
                $(
                    let field={
                        let name=<$fname_a as MarkerType>::MTVAL;
                        try_fe!(name.rev_get_field_raw_mut(field))
                    };
                )*
                Ok(field)
            }
        }


        impl<'a,$($fname_a, $fty_a:'a, $ferr_a,)* This,BoxedTy0:'a,CombErr>
            RevIntoField<'a,This>
        for FieldPath<($($fname_a,)*)>
        where
            Self:RevGetField<'a,This,Ty=$fty_l,Err=CombErr>,
            CombErr:IsFieldErr,

            This:?Sized+'a,
            $fname0: MarkerType,
            $fname0: RevIntoField<'a, This, Ty=$fty0, BoxedTy=BoxedTy0, Err=$ferr0>,

            $fname1: RevIntoField<
                'a,
                BoxedTy0,
                Ty= RevGetFieldType<'a,$fname1,$fty0>,
                Err= RevGetFieldErr<'a,$fname1,$fty0>,
            >,

            $(
                $fname_s: MarkerType,
                $fname_s: RevIntoField<'a, $fty_m, Ty=$fty_s, Err=$ferr_s>,
            )*

            $( $ferr_a:IntoFieldErr< CombErr >, )*
        {
            type BoxedTy=$fty_l;

            #[inline(always)]
            fn rev_into_field(self,field:This)->Result<$fty_l,CombErr>
            where
                This:Sized
            {
                $(
                    let name=<$fname_a as MarkerType>::MTVAL;
                    let field=try_fe!( name.rev_into_field(field) );
                )*
                Ok(field)
            }

            #[cfg(feature="alloc")]
            #[inline(always)]
            fn rev_box_into_field(
                self,
                field:crate::pmr::Box<This>,
            )->Result<$fty_l,CombErr>{
                let field=try_fe!(
                    <$fname0 as MarkerType>::MTVAL.rev_box_into_field(field)
                );
                $(
                    let field=try_fe!(
                        <$fname_s as MarkerType>::MTVAL.rev_into_field(field)
                    );
                )*
                Ok(field)
            }
        }

    };
    (
        ($fname0:ident $ferr0:ident $fty0:ident)
        $(($fname:ident $ferr:ident $fty:ident))*
        ;last=($fname_l:ident $ferr_l:ident $fty_l:ident)
    ) => {
        impl_get_nested_field_inner!{
            inner;

            receivers( This $fty0 $($fty)* )
            first ($fname0 $ferr0 $fty0)
            second (
                $(($fname $ferr $fty))*
                ($fname_l $ferr_l $fty_l)
            )
            middle(
                ($fname0 $ferr0 $fty0)
                $(($fname $ferr $fty))*
            )
            suffix(
                $(($fname $ferr $fty))*
                ($fname_l $ferr_l $fty_l)
            )
            all(
                ($fname0 $ferr0 $fty0)
                $(($fname $ferr $fty))*
                ($fname_l $ferr_l $fty_l)
            )
            last($fname_l $ferr_l $fty_l)
        }
    }
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

////////////////////////////////////////////////////////////////////////////////
/////           Implementations for FP!() (The empty field path)
////////////////////////////////////////////////////////////////////////////////

impl<'a, This> RevGetField<'a, This> for FieldPath<()>
where
    This: ?Sized + 'a,
{
    type Ty = This;
    type Err = NonOptField;

    fn rev_get_field(self, this: &'a This) -> Result<&'a Self::Ty, Self::Err> {
        Ok(this)
    }
}

impl<'a, This> RevGetFieldMut<'a, This> for FieldPath<()>
where
    This: ?Sized + 'a,
{
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut Self::Ty, Self::Err> {
        Ok(this)
    }

    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut Self::Ty, Self::Err> {
        Ok(this)
    }
}

impl<'a, This> RevIntoField<'a, This> for FieldPath<()>
where
    This: Sized + 'a,
{
    type BoxedTy = This;

    fn rev_into_field(self, this: This) -> Result<Self::Ty, Self::Err> {
        Ok(this)
    }

    #[cfg(feature = "alloc")]
    fn rev_box_into_field(self, this: Box<This>) -> Result<Self::BoxedTy, Self::Err> {
        Ok(*this)
    }
}

////////////////////////////////////////////////////////////////////////////////
/////           Implementations for single path field paths
////////////////////////////////////////////////////////////////////////////////

impl<'a, This, F0> RevGetField<'a, This> for FieldPath1<F0>
where
    This: ?Sized + 'a,
    F0: MarkerType + RevGetField<'a, This>,
{
    type Ty = F0::Ty;
    type Err = F0::Err;

    fn rev_get_field(self, this: &'a This) -> Result<&'a F0::Ty, F0::Err> {
        F0::MTVAL.rev_get_field(this)
    }
}

impl<'a, This, F0> RevGetFieldMut<'a, This> for FieldPath1<F0>
where
    This: ?Sized + 'a,
    F0: MarkerType + RevGetFieldMut<'a, This>,
{
    fn rev_get_field_mut(self, this: &'a mut This) -> Result<&'a mut F0::Ty, F0::Err> {
        F0::MTVAL.rev_get_field_mut(this)
    }

    unsafe fn rev_get_field_raw_mut(self, this: *mut This) -> Result<*mut F0::Ty, F0::Err> {
        F0::MTVAL.rev_get_field_raw_mut(this)
    }
}

impl<'a, This, F0> RevIntoField<'a, This> for FieldPath1<F0>
where
    This: ?Sized + 'a,
    F0: MarkerType + RevIntoField<'a, This>,
{
    type BoxedTy = F0::BoxedTy;

    fn rev_into_field(self, this: This) -> Result<F0::Ty, F0::Err>
    where
        This: Sized,
        F0::Ty: Sized,
    {
        F0::MTVAL.rev_into_field(this)
    }

    #[cfg(feature = "alloc")]
    fn rev_box_into_field(self, this: Box<This>) -> Result<F0::BoxedTy, F0::Err> {
        F0::MTVAL.rev_box_into_field(this)
    }
}

////////////////////////////////////////////////////////////////////////////////
/////           Implementations for field path components
////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_rev_traits {
    (
        impl[ $($typarams:tt)*] $self_:ty
        where[ $($where_:tt)* ]
    ) => (
        impl<'a,This,_Ty,_Err,$($typarams)*> RevGetField<'a,This> for $self_
        where
            This: ?Sized + 'a + GetFieldImpl<FieldPath1<Self>, Ty=_Ty, Err=_Err>,
            _Ty: 'a,
            _Err: IsFieldErr,
            $($where_)*
        {
            type Ty =_Ty;
            type Err=_Err;

            #[inline(always)]
            fn rev_get_field(self, this: &'a This) -> Result<&'a _Ty,_Err>{
                GetFieldImpl::get_field_( this, FieldPath1::<Self>::NEW,() )
            }
        }


        impl<'a,This,_Ty,_Err,$($typarams)*> RevGetFieldMut<'a,This> for $self_
        where
            This: ?Sized + 'a + GetFieldMutImpl<FieldPath1<Self>, Ty=_Ty, Err=_Err>,
            _Ty: 'a,
            _Err: IsFieldErr,
            $($where_)*
        {
            #[inline(always)]
            fn rev_get_field_mut(self,this:&'a mut This)->Result<&'a mut _Ty,_Err >{
                map_fe!(
                    GetFieldMutImpl::get_field_mut_( this, FieldPath1::<Self>::NEW, () )
                )
            }

            default_if!{
                #[inline(always)]
                cfg(feature="specialization")
                unsafe fn rev_get_field_raw_mut(self,this:*mut This)->Result<*mut _Ty,_Err>{
                    let func=(*this).get_field_raw_mut_func();
                    func(
                        this as *mut (),
                        FieldPath1::<Self>::NEW,
                        (),
                    )
                }
            }
        }

        #[cfg(feature="specialization")]
        impl<'a,This,_Ty,_Err,$($typarams)*> RevGetFieldMut<'a,This> for $self_
        where
            This: 'a + GetFieldMutImpl<FieldPath1<Self>, Ty=_Ty, Err=_Err>,
            _Ty: 'a,
            _Err: IsFieldErr,
            $($where_)*
        {
            #[inline(always)]
            unsafe fn rev_get_field_raw_mut(self,this:*mut This)->Result<*mut _Ty,_Err>{
                let name=FieldPath1::<Self>::NEW;
                <This as
                    GetFieldMutImpl<FieldPath1<Self>>
                >::get_field_raw_mut(this as *mut (), name, ())
            }
        }

        impl<'a,This,_Ty,_Err,$($typarams)*> RevIntoField<'a,This> for $self_
        where
            This: ?Sized + 'a + IntoFieldImpl<FieldPath1<Self>, Ty=_Ty, Err=_Err>,
            _Ty: 'a,
            _Err: IsFieldErr,
            $($where_)*
        {
            type BoxedTy=_Ty;

            #[inline(always)]
            fn rev_into_field(self,this:This)->Result<_Ty,_Err>
            where
                This:Sized
            {
                this.into_field_(FieldPath1::<Self>::NEW,())
            }

            #[cfg(feature="alloc")]
            #[inline(always)]
            fn rev_box_into_field(self,this:crate::pmr::Box<This>)->Result<_Ty,_Err>{
                this.box_into_field_(FieldPath1::<Self>::NEW,())
            }
        }
    )
}

impl_rev_traits! {
    impl[T] TString<T>
    where[]
}

impl_rev_traits! {
    impl[V,T] VariantField<V,T>
    where[]
}

////////////////////////////////////////////

impl<'a, This, V> RevGetField<'a, This> for VariantName<V>
where
    This: ?Sized + 'a + IsVariant<FieldPath1<V>>,
    V: 'static,
{
    type Ty = VariantProxy<This, FieldPath1<V>>;
    type Err = OptionalField;

    #[inline(always)]
    fn rev_get_field(
        self,
        this: &'a This,
    ) -> Result<&'a VariantProxy<This, FieldPath1<V>>, OptionalField> {
        map_of!( this.as_variant(FieldPath1::<V>::NEW) )
    }
}

impl<'a, This, V> RevGetFieldMut<'a, This> for VariantName<V>
where
    This: ?Sized + 'a + IsVariant<FieldPath1<V>>,
    V: 'static,
{
    #[inline(always)]
    fn rev_get_field_mut(
        self,
        this: &'a mut This,
    ) -> Result<&'a mut VariantProxy<This, FieldPath1<V>>, OptionalField> {
        map_of!( this.as_mut_variant(FieldPath1::<V>::NEW) )
    }

    #[inline(always)]
    unsafe fn rev_get_field_raw_mut(
        self,
        this: *mut This,
    ) -> Result<*mut VariantProxy<This, FieldPath1<V>>, OptionalField> {
        map_of!( EnumExt::as_raw_mut_variant(this,FieldPath1::<V>::NEW) )
    }
}

impl<'a, This, V> RevIntoField<'a, This> for VariantName<V>
where
    This: ?Sized + 'a + IsVariant<FieldPath1<V>>,
    V: 'static,
{
    type BoxedTy = Box<VariantProxy<This, FieldPath1<V>>>;

    #[inline(always)]
    fn rev_into_field(self, this: This) -> Result<VariantProxy<This, FieldPath1<V>>, OptionalField>
    where
        This: Sized,
    {
        map_of!( this.into_variant(FieldPath1::<V>::NEW) )
    }

    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn rev_box_into_field(
        self,
        this: crate::pmr::Box<This>,
    ) -> Result<Box<VariantProxy<This, FieldPath1<V>>>, OptionalField> {
        map_of!( this.box_into_variant(FieldPath1::<V>::NEW) )
    }
}
