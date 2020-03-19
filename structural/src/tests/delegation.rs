use crate::{
    field_path_aliases,
    field_traits::Array3,
    for_examples::{EnumOptFlying, EnumOptFlying_SI},
    structural_alias, GetField, GetFieldExt, GetFieldMut, IntoField, Structural,
};

use std_::{fmt::Debug, marker::PhantomData, mem};

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

    trait EnumOptFlying_Mut_SI {
        mut Limbs {
            legs: Option<usize>,
            hands: Option<usize>,
            noodles: usize,
        },
    }
}

fn struct_testing<T>(mut this: T)
where
    T: MutArray3<u32>,
{
    let (a, b, c) = this.fields_mut(fp!(0, 1, 2));
    mem::swap(a, c);
    assert_eq!(a, &mut 5);
    assert_eq!(b, &mut 3);
    assert_eq!(c, &mut 2);
}

fn struct_val_testing<T>(mut this: T)
where
    T: Array3<u32> + Clone,
{
    assert_eq!(this.clone().into_field(fp!(0)), 2);
    assert_eq!(this.clone().into_field(fp!(1)), 3);
    assert_eq!(this.clone().into_field(fp!(2)), 5);

    #[cfg(feature = "alloc")]
    {
        use crate::reexports::Box;
        let erase = |v: &T| Box::new(v.clone()) as Box<dyn Array3<u32>>;
        assert_eq!(erase(&this).into_field(fp!(0)), 2);
        assert_eq!(erase(&this).into_field(fp!(1)), 3);
        assert_eq!(erase(&this).into_field(fp!(2)), 5);
    }
}

fn enum_testing<T>(mut this: T)
where
    T: EnumOptFlying_Mut_SI,
{
    let (a, b, c) = this.fields_mut(fp!(::Limbs=>legs,hands,noodles)).unwrap();
    assert_eq!(a, &mut Some(3));
    assert_eq!(b, &mut Some(5));
    assert_eq!(c, &mut 8);

    mem::swap(a, b);
    *c += 20;
    assert_eq!(a, &mut Some(5));
    assert_eq!(b, &mut Some(3));
    assert_eq!(c, &mut 28);
}

fn enum_val_testing<T>(mut this: T)
where
    T: EnumOptFlying_SI + Clone,
{
    assert_eq!(this.clone().into_field(fp!(::Limbs.legs?)), Some(3));
    assert_eq!(this.clone().into_field(fp!(::Limbs.hands?)), Some(5));
    assert_eq!(this.clone().into_field(fp!(::Limbs.noodles)), Some(8));

    #[cfg(feature = "alloc")]
    {
        use crate::reexports::Box;
        let erase = |v: &T| Box::new(v.clone()) as Box<dyn EnumOptFlying_SI>;
        assert_eq!(erase(&this).into_field(fp!(::Limbs.legs?)), Some(3));
        assert_eq!(erase(&this).into_field(fp!(::Limbs.hands?)), Some(5));
        assert_eq!(erase(&this).into_field(fp!(::Limbs.noodles)), Some(8));
    }
}

//////////////////////////////////////////////////

#[derive(Structural, Copy, Clone)]
#[repr(transparent)]
struct SizedFoo<T> {
    #[struc(delegate_to)]
    value: T,
}

#[test]
fn delegate_sized() {
    let struct_ = SizedFoo {
        value: [2, 3, 5, 8, 13, 21, 34],
    };
    struct_testing(struct_);
    struct_val_testing(struct_);

    let mut value = EnumOptFlying::Limbs {
        legs: Some(3),
        hands: Some(5),
        noodles: 8,
    };
    enum_testing(SizedFoo { value });
    enum_val_testing(SizedFoo { value });
    enum_testing(SizedFoo { value: &mut value });
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
    specialization_params(?Sized);
    delegating_to_type=T;

    GetField { &*this.value }

    GetFieldMut { &mut *this.value }
    as_delegating_raw{
        (*(this as *mut MaybeSizedFoo<'a,T>)).value as *mut T
    }
}

#[test]
fn delegate_unsized() {
    let array: &mut dyn MutArray3<u32> = &mut [2, 3, 5, 8, 13, 21, 34];
    let this: MaybeSizedFoo<'_, dyn MutArray3<u32>> = MaybeSizedFoo { value: array };
    struct_testing(this);

    let mut value = EnumOptFlying::Limbs {
        legs: Some(3),
        hands: Some(5),
        noodles: 8,
    };
    let this: MaybeSizedFoo<'_, dyn EnumOptFlying_SI> = MaybeSizedFoo { value: &mut value };
    enum_testing(this);
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
    specialization_params(specialize_cfg(feature="specialization"));
    delegating_to_type=T;

    GetField { &*this.value }

    GetFieldMut { &mut *this.value }
    as_delegating_raw{
        (*(this as *mut SpecializedFoo<'a,T>)).value as *mut T
    }
}

#[test]
fn delegate_cfg() {
    {
        let value: &mut dyn MutArray3<u32> = &mut [2, 3, 5, 8, 13, 21, 34];
        let this: SpecializedFoo<'_, dyn MutArray3<u32>> = SpecializedFoo { value };

        struct_testing(this);

        struct_testing(SizedFoo {
            value: &mut [2, 3, 5, 8, 13, 21, 34],
        });
    }

    {
        let value = EnumOptFlying::Limbs {
            legs: Some(3),
            hands: Some(5),
            noodles: 8,
        };
        {
            let mut value = value.clone();

            let this: SpecializedFoo<'_, dyn EnumOptFlying_SI> =
                SpecializedFoo { value: &mut value };

            enum_testing(this);
        }
        enum_testing(SizedFoo {
            value: &mut value.clone(),
        });
    }
}
