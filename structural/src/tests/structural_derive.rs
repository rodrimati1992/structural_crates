use crate::{
    field_traits::NonOptField, GetFieldExt, GetFieldImpl, GetFieldMutImpl, IntoFieldImpl,
    IntoFieldMut, Structural,
};

#[cfg(feature = "alloc")]
use crate::alloc::{boxed::Box, rc::Rc, sync::Arc};

////////////////////////////////////////////////////////////////////////////////

field_path_aliases! {
    mod paths{
        a,b
    }
}
tstr_aliases! {
    mod strings{
        A,B,C,a,b,n0=0,
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn derive_inside_function() {
    #[allow(dead_code)]
    #[derive(Structural)]
    pub struct Bar {
        pub b: u32,
    }
}

////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
fn object_safety() {
    #[cfg(feature = "alloc")]
    type AllocPtrs<'a, T> = (crate::alloc::boxed::Box<T>, crate::alloc::sync::Arc<T>);

    #[cfg(not(feature = "alloc"))]
    type AllocPtrs<'a, T> = (T,);

    type TraitObjects<'a, T> = (&'a T, &'a mut T, AllocPtrs<'a, T>);

    let _: TraitObjects<'_, dyn GetFieldImpl<FP!(ab), Ty = (), Err = NonOptField>>;
    let _: TraitObjects<'_, dyn GetFieldMutImpl<FP!(ab), Ty = (), Err = NonOptField>>;
    let _: TraitObjects<'_, dyn Huh_SI>;
    let _: TraitObjects<'_, dyn Whoah_SI>;
    let _: TraitObjects<'_, dyn Renamed_SI>;
    let _: TraitObjects<'_, dyn Privacies1_SI>;
}

////////////////////////////////////////////////////////////////////////////////

structural_alias! {
    trait HuhInterface{
        a:u32,
        b:u32,
    }
}

#[derive(Structural)]
struct Huh {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
}

#[derive(Structural)]
struct Whoah {
    pub nah: u32,
    pub yep: u32,
    pub b: u32,
    pub a: u32,
}

fn huh_printer<This>(this: This)
where
    This: HuhInterface,
{
    let (a, b) = this.fields(fp!(a, b));
    assert_eq!(a, &10);
    assert_eq!(b, &33);
}

#[test]
fn huh_printing() {
    huh_printer(Huh {
        a: 10,
        b: 33,
        c: 44,
        d: 66,
    });

    huh_printer(Whoah {
        nah: 0x000F,
        yep: !0,
        b: 33,
        a: 10,
    });
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Structural, Clone)]
#[struc(public)]
struct Privacies0 {
    a: u32,
    b: u32,
    #[struc(private)]
    c: u32,
}

#[derive(Structural, Clone)]
//#[struc(debug_print)]
struct Privacies1 {
    pub a: u32,
    pub b: u32,
    #[struc(not_public)]
    pub c: u32,
    d: u32,
    #[struc(public)]
    e: u32,
    #[struc(access = "ref")]
    f: u32,
    #[struc(access = "mut")]
    g: u32,
    #[struc(access = "mut move")]
    hello: u32,
    #[struc(access = "move")]
    world: u32,
}

assert_equal_bounds! {
    trait Privacies1Test,
    ( Privacies1_SI ),
    (
        IntoFieldMut<FP!(a), Ty = u32, Err = NonOptField>
        + IntoFieldMut<FP!(b), Ty = u32, Err = NonOptField>
        + IntoFieldMut<FP!(e), Ty = u32, Err = NonOptField>
        + GetFieldImpl<FP!(f), Ty = u32, Err = NonOptField>
        + GetFieldMutImpl<FP!(g), Ty = u32, Err = NonOptField>
        + IntoFieldMut<FP!(hello), Ty = u32, Err = NonOptField>
        + IntoFieldImpl<FP!(world), Ty = u32, Err = NonOptField>
    ),
}

#[test]
fn privacies() {
    let _ = <Privacies1 as Privacies1Test>::DUMMY;

    let _ = |mut this: Privacies0| {
        let _ = this.fields_mut(fp!(a, b));
        let _ = this.clone().into_field(fp!(a));
        let _ = this.clone().into_field(fp!(b));
    };
    let _ = generic_1::<Privacies1>;
}

// This tests that boxed trait objects can still call GetFieldExt methods.
#[cfg(feature = "alloc")]
#[test]
fn ptr_dyn_methods() {
    let _ = |this: Privacies1| {
        generic_1_dyn(|| Box::new(this.clone()));
    };

    let _ = |this: Privacies0| {
        generic_0_dyn(|| Arc::new(this.clone()));
        let _: Rc<dyn Privacies0_SI> = Rc::new(this.clone());
    };
}

fn generic_1<T>(mut this: T)
where
    T: Privacies1_SI + Clone,
{
    let _ = this.fields(fp!(a, b, e, f, g, hello));
    let _ = this.fields_mut(fp!(a, b, e, g, hello));
    let _ = this.clone().into_field(fp!(a));
    let _ = this.clone().into_field(fp!(b));
    let _ = this.clone().into_field(fp!(e));
    let _ = this.clone().into_field(fp!(hello));
    let _ = this.clone().into_field(fp!(world));
    #[cfg(feature = "alloc")]
    {
        let _ = Box::new(this.clone()).box_into_field(fp!(a));
        let _ = Box::new(this.clone()).box_into_field(fp!(b));
        let _ = Box::new(this.clone()).box_into_field(fp!(e));
        let _ = Box::new(this.clone()).box_into_field(fp!(hello));
        let _ = Box::new(this.clone()).box_into_field(fp!(world));
    }
}

#[cfg(feature = "alloc")]
fn generic_0_dyn(mut ctor: impl FnMut() -> Arc<dyn Privacies0_SI>) {
    let this = ctor();
    let _ = this.fields(fp!(a, b));
}

#[cfg(feature = "alloc")]
fn generic_1_dyn(mut ctor: impl FnMut() -> Box<dyn Privacies1_SI>) {
    let mut this = ctor();
    let _ = this.fields(fp!(a, b, e, f, g, hello));
    let _ = this.fields_mut(fp!(g, hello));
    let _ = this.field_mut(fp!(g));
    let _ = this.field_mut(fp!(hello));
    #[cfg(feature = "alloc")]
    {
        let _ = ctor().box_into_field(fp!(hello));
        let _ = ctor().box_into_field(fp!(world));
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Structural, Clone)]
#[struc(public)]
struct Renamed {
    pub a: u32,
    pub b: u32,
    #[struc(rename = "e")]
    pub c: u32,
}

#[derive(Structural)]
#[struc(no_trait)]
enum Vegetable {
    #[struc(rename = "foo")]
    Potato {
        #[struc(rename = "bar")]
        volume_cm: u32,
    },
    #[struc(rename = "baz")]
    Letuce {
        #[struc(rename = "qux")]
        leaves: u32,
    },
}

#[test]
fn renamed() {
    // struct
    {
        let mut this = Renamed { a: 3, b: 5, c: 8 };

        assert_eq!(this.field_(fp!(a)), &3);
        assert_eq!(this.field_mut(fp!(a)), &mut 3);
        assert_eq!(this.clone().into_field(fp!(a)), 3);

        assert_eq!(this.field_(fp!(b)), &5);
        assert_eq!(this.field_mut(fp!(b)), &mut 5);
        assert_eq!(this.clone().into_field(fp!(b)), 5);

        assert_eq!(this.field_(fp!(e)), &8);
        assert_eq!(this.field_mut(fp!(e)), &mut 8);
        assert_eq!(this.clone().into_field(fp!(e)), 8);
    }

    // enum
    //
    // Copied this from a documentation example so that it doesn't get deleted if
    // the example gets deleted.
    {
        let mut potato = Vegetable::Potato { volume_cm: 13 };
        let mut letuce = Vegetable::Letuce { leaves: 21 };

        assert_eq!(potato.field_(fp!(::foo.bar)), Some(&13));
        assert_eq!(potato.field_(fp!(::baz.qux)), None);

        assert_eq!(letuce.field_(fp!(::foo.bar)), None);
        assert_eq!(letuce.field_(fp!(::baz.qux)), Some(&21));

        assert_eq!(potato.field_mut(fp!(::foo.bar)), Some(&mut 13));
        assert_eq!(potato.field_mut(fp!(::baz.qux)), None);

        assert_eq!(letuce.field_mut(fp!(::foo.bar)), None);
        assert_eq!(letuce.field_mut(fp!(::baz.qux)), Some(&mut 21));

        assert_eq!(potato.is_variant(ts!(foo)), true);
        assert_eq!(potato.is_variant(ts!(baz)), false);

        assert_eq!(letuce.is_variant(ts!(foo)), false);
        assert_eq!(letuce.is_variant(ts!(baz)), true);
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Structural, Clone)]
#[struc(no_trait)]
struct Foo<T> {
    #[struc(delegate_to)]
    value: T,
}

#[test]
fn delegate_to_test() {
    let mut this = Foo {
        value: (3, 5, 8, 13, 21),
    };

    assert_eq!(this.fields(fp!(1, 3, 0, 2, 4)), (&5, &13, &3, &8, &21),);
    assert_eq!(
        this.fields_mut(fp!(1, 3, 0, 2, 4)),
        (&mut 5, &mut 13, &mut 3, &mut 8, &mut 21),
    );

    assert_eq!(this.clone().into_field(fp!(0)), 3);
    assert_eq!(this.clone().into_field(fp!(1)), 5);
    assert_eq!(this.clone().into_field(fp!(2)), 8);
    assert_eq!(this.clone().into_field(fp!(3)), 13);
    assert_eq!(this.clone().into_field(fp!(4)), 21);
}

////////////////////////////////////////////////////////////////////////////////

/// Tests that `#[struc(no_trait)]` has an effect on structs.
trait Foo_SI {}
trait Foo_VSI {}
trait Foo_ESI {}

/// Tests that `#[struc(no_trait)]` has an effect on enums.
trait Vegetable_SI {}
trait Vegetable_VSI {}
trait Vegetable_ESI {}

////////////////////////////////////////////////////////////////////////////////

mod struct_with_constraints {
    use super::*;

    #[allow(dead_code)]
    #[derive(Structural, Copy, Clone)]
    // #[struc(debug_print)]
    pub struct WhereClause0<T>
    where
        T: Copy,
    {
        pub b: T,
    }

    assert_equal_bounds! {
        trait WhereClause0Dummy[T,],
        (WhereClause0_SI<T>),
        (IntoFieldMut<paths::b,Ty=T>),
        where[ T:Copy, ]
    }

    #[allow(dead_code)]
    #[derive(Structural, Copy, Clone)]
    pub struct Bounds0<T, U: Clone> {
        pub a: T,
        pub b: U,
    }

    assert_equal_bounds! {
        trait Bounds0Dummy[T,U,],
        (Bounds0_SI<T,U>),
        (
            IntoFieldMut<paths::a,Ty=T>+
            IntoFieldMut<paths::b,Ty=U>
        ),
        where[ U:Clone, ]
    }

    #[allow(dead_code)]
    #[derive(Structural, Copy, Clone)]
    #[struc(bound = "T:'a")]
    pub struct Bounds1<'a, T, U: 'a> {
        pub a: &'a T,
        pub b: U,
    }

    assert_equal_bounds! {
        trait Bounds1Dummy['a,T,U,],
        (Bounds1_SI<'a,T,U>),
        (
            IntoFieldMut<paths::a,Ty=&'a T>+
            IntoFieldMut<paths::b,Ty=U>
        ),
        where[ T:'a,U:'a, ]
    }
}

mod struct_delegated_with_constraints {
    use super::*;

    #[allow(dead_code)]
    #[derive(Structural, Copy, Clone)]
    pub struct WhereClause0<T>
    where
        T: Copy,
    {
        #[struc(delegate_to)]
        pub b: T,
    }

    #[allow(dead_code)]
    #[derive(Structural, Copy, Clone)]
    pub struct Bounds0<T, U: Clone> {
        #[struc(delegate_to)]
        pub a: T,
        pub b: U,
    }

    #[allow(dead_code)]
    #[derive(Structural, Copy, Clone)]
    #[struc(bound = "T:'a")]
    pub struct Bounds1<'a, T, U: 'a> {
        #[struc(delegate_to)]
        pub a: T,
        pub b: U,
        pub c: &'a (),
    }
}

mod enum_with_constraints {
    use super::*;
    use crate::field_traits::IntoVariantFieldMut;

    #[allow(dead_code)]
    #[derive(Structural, Copy, Clone)]
    //#[struc(debug_print)]
    pub enum WhereClause0<T>
    where
        T: Copy,
    {
        A(T),
    }

    assert_equal_bounds! {
        trait WhereClause0Dummy[T,],
        (WhereClause0_SI<T>),
        (IntoVariantFieldMut<strings::A,strings::n0,Ty=T>),
        where[ T:Copy, ]
    }

    #[allow(dead_code)]
    #[derive(Structural, Copy, Clone)]
    pub enum Bounds0<T, U: Clone> {
        A(T),
        B(U),
    }

    assert_equal_bounds! {
        trait Bounds0Dummy[T,U,],
        (Bounds0_SI<T,U>),
        (
            IntoVariantFieldMut<strings::A,strings::n0,Ty=T>+
            IntoVariantFieldMut<strings::B,strings::n0,Ty=U>
        ),
        where[ U:Clone, ]
    }

    #[allow(dead_code)]
    #[derive(Structural, Copy, Clone)]
    #[struc(bound = "T:'a")]
    pub enum Bounds1<'a, T, U: 'a> {
        A(T),
        B(U),
        C(&'a ()),
    }

    assert_equal_bounds! {
        trait Bounds1Dummy['a,T,U,],
        (Bounds1_SI<'a,T,U>),
        (
            IntoVariantFieldMut<strings::A,strings::n0,Ty=T>+
            IntoVariantFieldMut<strings::B,strings::n0,Ty=U>+
            IntoVariantFieldMut<strings::C,strings::n0,Ty=&'a ()>+
        ),
        where[ T:'a,U:'a, ]
    }
}
