use crate::{
    field_path_aliases, structural_alias, GetField, GetFieldExt, GetFieldMut, IntoField, Structural,
};

use std_::{fmt::Debug, marker::PhantomData};

use core_extensions::type_level_bool::{False, True};

field_path_aliases! {
    mod paths{
        p0=0,
    }
}

structural_alias! {
    trait MutArray3<T>{
        mut 0:T,
        mut 1:T,
        mut 2:T,
    }
}

fn huh<T>(mut this: T)
where
    T: MutArray3<u32>,
{
    assert_eq!(this.fields_mut(fp!(0, 1, 2)), (&mut 1, &mut 1, &mut 1));
}

//////////////////////////////////////////////////

#[derive(Structural)]
#[repr(transparent)]
struct SizedFoo<T> {
    #[struc(delegate_to)]
    value: T,
}

#[test]
fn delegate_sized() {
    huh(SizedFoo { value: [1; 10] });
}

//////////////////////////////////////////////////

#[derive(Structural, Debug, Copy, Clone)]
#[repr(transparent)]
struct BoundedFoo<T> {
    #[struc(delegate_to(bound = "T:Clone", mut_bound = "T:Copy", into_bound = "T:Debug"))]
    value: T,
}

declare_querying_trait! {
    trait ImplsGetField
    implements [GetField<paths::p0,Ty=()>]
    fn impls_get_field;
}

declare_querying_trait! {
    trait ImplsGetFieldMut
    implements [GetFieldMut<paths::p0,Ty=()>]
    fn impls_get_field_mut;
}

declare_querying_trait! {
    trait ImplsIntoField
    implements [IntoField<paths::p0,Ty=()>]
    fn impls_into_field;
}

declare_querying_trait! {
    trait ImplsClone
    implements[Clone]
    fn impls_clone;
}

declare_querying_trait! {
    trait ImplsCopy
    implements[Copy]
    fn impls_copy;
}

declare_querying_trait! {
    trait ImplsDebug
    implements[Debug]
    fn impls_debug;
}

// This tests that `BoundedFoo`'s delegation attribute adds those bounds to
// the generated impls.
#[test]
fn bounded_delegation_tests() {
    struct NotClone;

    #[derive(Clone)]
    struct CloneNotCopy;

    #[derive(Copy, Clone)]
    struct CopyNotDebug;

    #[derive(Debug, Clone)]
    struct CloneDebug;

    #[derive(Debug, Clone, Copy)]
    struct CopyDebug;

    {
        let this = PhantomData::<BoundedFoo<((), NotClone)>>;
        let _: False = this.impls_clone();
        let _: False = this.impls_copy();
        let _: False = this.impls_debug();
        let _: False = this.impls_get_field();
        let _: False = this.impls_get_field_mut();
        let _: False = this.impls_into_field();
    }
    {
        let this = PhantomData::<BoundedFoo<((), CloneNotCopy)>>;
        let _: True = this.impls_clone();
        let _: False = this.impls_copy();
        let _: False = this.impls_debug();
        let _: True = this.impls_get_field();
        let _: False = this.impls_get_field_mut();
        let _: False = this.impls_into_field();
    }
    {
        let this = PhantomData::<BoundedFoo<((), CopyNotDebug)>>;
        let _: True = this.impls_clone();
        let _: True = this.impls_copy();
        let _: False = this.impls_debug();
        let _: True = this.impls_get_field();
        let _: True = this.impls_get_field_mut();
        let _: False = this.impls_into_field();
    }
    {
        let this = PhantomData::<BoundedFoo<((), CopyDebug)>>;
        let _: True = this.impls_clone();
        let _: True = this.impls_copy();
        let _: True = this.impls_debug();
        let _: True = this.impls_get_field();
        let _: True = this.impls_get_field_mut();
        let _: True = this.impls_into_field();
    }
}

//////////////////////////////////////////////////

#[repr(transparent)]
struct MaybeSizedFoo<'a, T: ?Sized> {
    value: &'a mut T,
}

unsafe_delegate_structural_with! {
    impl['a,T:'a+?Sized,] MaybeSizedFoo<'a,T>
    where[]
    self_ident=this;
    delegating_to_type=T;
    field_name_param=( fname : FnameTy );

    GetFieldImpl { &*this.value }

    unsafe GetFieldMutImpl { &mut *this.value }
    as_delegating_raw{
        (*(this as *mut MaybeSizedFoo<'a,T>)).value as *mut T
    }
    raw_mut_impl(?Sized)
}

#[test]
fn delegate_unsized() {
    let array: &mut dyn MutArray3<u32> = &mut [1; 10];
    let this: MaybeSizedFoo<'_, dyn MutArray3<u32>> = MaybeSizedFoo { value: array };

    huh(this);
}

//////////////////////////////////////////////////

#[repr(transparent)]
struct SpecializedFoo<'a, T: ?Sized> {
    value: &'a mut T,
}

unsafe_delegate_structural_with! {
    impl['a,T:'a+?Sized,] SpecializedFoo<'a,T>
    where[]
    self_ident=this;
    delegating_to_type=T;
    field_name_param=( fname : FnameTy );

    GetFieldImpl { &*this.value }

    unsafe GetFieldMutImpl { &mut *this.value }
    as_delegating_raw{
        (*(this as *mut SpecializedFoo<'a,T>)).value as *mut T
    }
    raw_mut_impl(specialize_cfg(feature="specialization"))
}

#[test]
fn delegate_cfg() {
    let array: &mut dyn MutArray3<u32> = &mut [1; 10];
    let this: SpecializedFoo<'_, dyn MutArray3<u32>> = SpecializedFoo { value: array };

    huh(this);

    huh(SizedFoo {
        value: &mut [1; 10],
    });
}
