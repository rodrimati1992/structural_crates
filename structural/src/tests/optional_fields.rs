use crate::{
    enums::IsVariant,
    field::{IntoFieldMut, IntoVariantFieldMut},
    Structural, StructuralExt,
};

field_path_aliases! {
    mod names{
        a,b,c,d,
    }
}

tstr_aliases! {
    mod strings{
        A,B,C,a,b,c,d,
        f0=0,f1=1,
    }
}

#[derive(Copy, Clone)]
struct StructManual {
    a: Option<u32>,
    b: Option<u64>,
    c: Option<&'static str>,
    d: Option<bool>,
}

_private_impl_getters_for_derive_struct! {
    impl[] StructManual
    where[]
    {
        (IntoFieldMut < a : Option<u32>,names::a,"a",> )
        (IntoFieldMut < b : Option<u64>,names::b,"b",> )
        (IntoFieldMut < c : Option<&'static str>,names::c,"c",> )
        (IntoFieldMut < d : Option<bool>,names::d,"d",> )
    }
}

#[derive(Structural, Copy, Clone)]
#[struc(public)]
struct StructDerivedExplicit {
    a: Option<u32>,

    b: Option<u64>,

    c: Option<&'static str>,

    d: Option<bool>,
}

assert_equal_bounds! {
    trait StructDerivedExplicit_SI_Dummy[],
    (StructDerivedExplicit_SI),
    (
        IntoFieldMut<strings::a,Ty=Option<u32>>+
        IntoFieldMut<strings::b,Ty=Option<u64>>+
        IntoFieldMut<strings::c,Ty=Option<&'static str>>+
        IntoFieldMut<strings::d,Ty=Option<bool>>+
    ),
}

assert_equal_bounds! {
    trait StructDerivedExplicit_VSI_Dummy[Vari,],
    (StructDerivedExplicit_VSI<Vari>),
    (
        IntoVariantFieldMut<Vari,strings::a,Ty=Option<u32>>+
        IntoVariantFieldMut<Vari,strings::b,Ty=Option<u64>>+
        IntoVariantFieldMut<Vari,strings::c,Ty=Option<&'static str>>+
        IntoVariantFieldMut<Vari,strings::d,Ty=Option<bool>>+
    ),
}

macro_rules! declare_struct_tests {
    (
        type=$ty:ident
        test=$test_name:ident
    ) => {
        #[test]
        fn $test_name() {
            {
                let mut this = $ty {
                    a: Some(0),
                    b: Some(10),
                    c: Some("200"),
                    d: Some(true),
                };

                assert_eq!(this.field_(fp!(a?)), Some(&0));
                assert_eq!(this.field_(fp!(b?)), Some(&10));
                assert_eq!(this.field_(fp!(c?)), Some(&"200"));
                assert_eq!(this.field_(fp!(d)), &Some(true));

                assert_eq!(this.field_mut(fp!(a?)), Some(&mut 0));
                assert_eq!(this.field_mut(fp!(b?)), Some(&mut 10));
                assert_eq!(this.field_mut(fp!(c?)), Some(&mut "200"));
                assert_eq!(this.field_mut(fp!(d)), &mut Some(true));

                assert_eq!(this.into_field(fp!(a)), Some(0));
                assert_eq!(this.into_field(fp!(b)), Some(10));
                assert_eq!(this.into_field(fp!(c)), Some("200"));
                assert_eq!(this.into_field(fp!(d)), Some(true));
            }
            {
                let mut this = $ty {
                    a: None,
                    b: Some(10),
                    c: None,
                    d: None::<bool>,
                };

                assert_eq!(this.field_(fp!(a?)), None);
                assert_eq!(this.field_(fp!(b?)), Some(&10));
                assert_eq!(this.field_(fp!(c?)), None);
                assert_eq!(this.field_(fp!(d)), &None::<bool>);

                assert_eq!(this.field_mut(fp!(a?)), None);
                assert_eq!(this.field_mut(fp!(b?)), Some(&mut 10));
                assert_eq!(this.field_mut(fp!(c?)), None);
                assert_eq!(this.field_mut(fp!(d)), &mut None::<bool>);

                assert_eq!(this.into_field(fp!(a)), None);
                assert_eq!(this.into_field(fp!(b)), Some(10));
                assert_eq!(this.into_field(fp!(c)), None);
                assert_eq!(this.into_field(fp!(d)), None::<bool>);
            }
        }
    };
}

declare_struct_tests! {
    type=StructManual
    test=with_struct_manual
}

declare_struct_tests! {
    type=StructDerivedExplicit
    test=with_struct_derive_explicit
}

/////////////////////////////////////////////////////

#[derive(Copy, Clone)]
enum EnumManual {
    A {
        a: Option<u32>,
        b: Option<u64>,
        c: Option<&'static str>,
        d: Option<bool>,
    },
    C,
}

_private_impl_getters_for_derive_enum! {
    impl[] EnumManual
    where[]
    {
        enum=EnumManual
        variant_count=TS!(3),
        (
            A,
            strings::A,
            kind=regular,
            fields(
                (IntoVariantFieldMut,a:Option<u32>,strings::a)
                (IntoVariantFieldMut,b:Option<u64>,strings::b)
                (IntoVariantFieldMut,c:Option<&'static str>,strings::c)
                (IntoVariantFieldMut,d:Option<bool>,strings::d)
            )
        )
        (
            C,
            strings::C,
            kind=regular,
            fields()
        )
    }
}

#[derive(Structural, Copy, Clone)]
#[struc(public)]
enum EnumDerivedImplicit {
    A {
        a: Option<u32>,
        b: Option<u64>,
        c: Option<&'static str>,
        d: Option<bool>,
    },
    C,
}

#[derive(Structural, Copy, Clone)]
#[struc(public)]
enum EnumDerivedExplicit {
    A {
        a: Option<u32>,

        b: Option<u64>,

        c: Option<&'static str>,

        d: Option<bool>,
    },
    C,
}

assert_equal_bounds! {
    trait EnumDerived_SI_Dummy[],
    (EnumDerivedImplicit_SI),
    (
        IntoVariantFieldMut<strings::A,strings::a,Ty=Option<u32>>+
        IntoVariantFieldMut<strings::A,strings::b,Ty=Option<u64>>+
        IntoVariantFieldMut<strings::A,strings::c,Ty=Option<&'static str>>+
        IntoVariantFieldMut<strings::A,strings::d,Ty=Option<bool>>+
        IsVariant<strings::C>+
    ),
}

assert_equal_bounds! {
    trait EnumDerived_SI_DummyB[],
    (EnumDerivedImplicit_SI),
    (EnumDerivedExplicit_SI),
}

fn drop_ref<T>(_: &T) {}

fn drop_mut<T>(_: &mut T) {}

macro_rules! declare_enum_tests {
    (
        type=$ty:ident
        test=$test_name:ident
    ) => {
        #[test]
        fn $test_name() {
            {
                let mut this = $ty::A {
                    a: Some(0),
                    b: None,
                    c: Some("200"),
                    d: Some(false),
                };

                assert_eq!(this.field_(fp!(::A.a?)), Some(&0));
                assert_eq!(this.field_(fp!(::A.b?)), None);
                assert_eq!(this.field_(fp!(::A.c?)), Some(&"200"));
                assert_eq!(this.field_(fp!(::A.d)), Some(&Some(false)));

                assert_eq!(this.field_mut(fp!(::A.a?)), Some(&mut 0));
                assert_eq!(this.field_mut(fp!(::A.b?)), None);
                assert_eq!(this.field_mut(fp!(::A.c?)), Some(&mut "200"));
                assert_eq!(this.field_mut(fp!(::A.d)), Some(&mut Some(false)));

                assert_eq!(this.into_field(fp!(::A.a?)), Some(0));
                assert_eq!(this.into_field(fp!(::A.b?)), None);
                assert_eq!(this.into_field(fp!(::A.c?)), Some("200"));
                assert_eq!(this.into_field(fp!(::A.d)), Some(Some(false)));

                assert_eq!(this.field_(fp!(::A)).map(drop_ref), Some(()));
                assert_eq!(this.field_(fp!(::C)).map(drop_ref), None);

                assert_eq!(this.field_mut(fp!(::A)).map(drop_mut), Some(()));
                assert_eq!(this.field_mut(fp!(::C)).map(drop_mut), None);

                assert_eq!(this.into_field(fp!(::A)).map(drop), Some(()));
                assert_eq!(this.into_field(fp!(::C)).map(drop), None);
            }
            {
                let mut this = $ty::A {
                    a: Some(0),
                    b: None,
                    c: Some("200"),
                    d: None,
                };

                assert_eq!(this.field_(fp!(::A.d)), Some(&None));
                assert_eq!(this.field_mut(fp!(::A.d)), Some(&mut None));
                assert_eq!(this.into_field(fp!(::A.d)), Some(None));

                assert_eq!(this.field_(fp!(::A.d?)), None);
                assert_eq!(this.field_mut(fp!(::A.d?)), None);
            }
            {
                let this = $ty::C;

                assert_eq!(this.field_(fp!(::A.a?)), None);
                assert_eq!(this.field_(fp!(::A.b?)), None);
                assert_eq!(this.field_(fp!(::A.c?)), None);
                assert_eq!(this.field_(fp!(::A.d)), None::<&Option<bool>>);

                assert_eq!(this.field_(fp!(::A)).map(drop_ref), None);
                assert_eq!(this.field_(fp!(::C)).map(drop_ref), Some(()));
            }
        }
    };
}

declare_enum_tests! {
    type=EnumManual
    test=with_enum_manual
}

declare_enum_tests! {
    type=EnumDerivedImplicit
    test=with_enum_derive_implicit
}

declare_enum_tests! {
    type=EnumDerivedExplicit
    test=with_enum_derive_explicit
}
