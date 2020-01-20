use crate::{
    field_traits::NonOptField,
    structural_trait::{accessor_names, FieldInfo, FieldInfos},
    GetFieldExt, GetFieldImpl, GetFieldMutImpl, GetFieldType, IntoFieldImpl, IntoFieldMut,
    IsStructural, Structural,
};

#[cfg(feature = "alloc")]
use crate::alloc::{boxed::Box, rc::Rc, sync::Arc};

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

    let _: TraitObjects<'_, dyn GetFieldImpl<FP!(a b), Ty = (), Err = NonOptField>>;
    let _: TraitObjects<'_, dyn GetFieldMutImpl<FP!(a b), Ty = (), Err = NonOptField>>;
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
        + IntoFieldMut<FP!(h e l l o), Ty = u32, Err = NonOptField>
        + IntoFieldImpl<FP!(w o r l d), Ty = u32, Err = NonOptField>
        + IsStructural
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

    assert!(accessor_names::<Privacies0>().eq(["a", "b"].iter().cloned()),);

    assert!(accessor_names::<Privacies1>()
        .eq(["a", "b", "e", "f", "g", "hello", "world"].iter().cloned()),);
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

#[test]
fn renamed() {
    assert!(accessor_names::<Renamed>().eq(["a", "b", "e"].iter().cloned()));

    let _: GetFieldType<Renamed, FP!(a)>;
    let _: GetFieldType<Renamed, FP!(b)>;
    let _: GetFieldType<Renamed, FP!(e)>;
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Structural, Clone)]
struct Foo<T> {
    #[struc(delegate_to)]
    value: T,
}

fn get_fields_assoc_const<T>(_: &T) -> &'static [FieldInfo]
where
    T: Structural,
{
    match T::FIELDS {
        FieldInfos::Struct(x) => x,
        x => panic!("{:?} wasn't expected here", x),
    }
}

#[test]
fn delegate_to_test() {
    let mut this = Foo {
        value: (3, 5, 8, 13, 21),
    };

    let fields = &[
        FieldInfo::not_renamed("0"),
        FieldInfo::not_renamed("1"),
        FieldInfo::not_renamed("2"),
        FieldInfo::not_renamed("3"),
        FieldInfo::not_renamed("4"),
    ];

    assert_eq!(get_fields_assoc_const(&this), &fields[..]);

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
