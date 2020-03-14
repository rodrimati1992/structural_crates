use crate::GetFieldExt;

#[cfg(feature = "alloc")]
use crate::pmr::Box;

use core_extensions::type_asserts::AssertEq;
use core_extensions::{SelfOps, Void};

#[cfg(feature = "alloc")]
#[test]
fn boxed_fields() {
    let mut f = Box::new((0, 1, Box::new((20, 21)), 3));
    let (f_0, f_1, f_2_0, f_2_1, f_3) = f.fields_mut(fp!(0, 1, 2.0, 2.1, 3));

    *f_0 = 0;
    *f_1 = 0;
    *f_2_0 = 0;
    *f_2_1 = 0;
    *f_3 = 0;

    *f_0 = 5;
    *f_1 = 6;
    *f_2_0 = 7;
    *f_2_1 = 8;
    *f_3 = 9;

    assert_eq!(f.0, 5);
    assert_eq!(f.1, 6);
    assert_eq!((f.2).0, 7);
    assert_eq!((f.2).1, 8);
    assert_eq!(f.3, 9);
}

fn wrap_single<T>(value: T) -> (T,) {
    (value,)
}

#[test]
fn deeply_nested() {
    {
        let mut f = make_struct! {
            a:make_struct!{
                aa:(101,103),
                ab:"hello",
            },
            b:false,
        };

        let (f_aa_0, f_aa_1, f_ab, f_b) = f.fields_mut(fp!(a.aa.0, a.aa.1, a.ab, b));
        assert_eq!(f_aa_0, &mut 101);
        assert_eq!(f_aa_1, &mut 103);
        assert_eq!(f_ab, &mut "hello");
        *f_aa_0 *= 3;
        *f_aa_1 *= 2;
        *f_ab = "shoot";
        *f_b = true;
        assert_eq!(f_aa_0, &mut 303);
        assert_eq!(f_aa_1, &mut 206);
        assert_eq!(f_ab, &mut "shoot");
        assert_eq!(f_b, &mut true);

        assert_eq!(f.a.aa.0, 303);
        assert_eq!(f.a.aa.1, 206);
        assert_eq!(f.a.ab, "shoot");
        assert_eq!(f.b, true);
    }

    {
        let mut this = 10
            .piped(wrap_single)
            .piped(wrap_single)
            .piped(wrap_single)
            .piped(wrap_single);

        assert_eq!((((this.0).0).0).0, 10);
        let num = this.field_mut(fp!(0.0.0.0));
        *num *= 2;
        assert_eq!((((this.0).0).0).0, 20);
    }
}

#[test]
fn identity_getters() {
    #[cfg(feature = "alloc")]
    {
        let mut this = Box::new((0, 1));
        let () = this.fields_mut(fp!());
    }
    /*
    {
        let other=this.field_mut(fp!(()));
        *other=Default::default();
        assert_eq!(this, Default::default());
    }

    {
        let _:FieldPathSet<(),UniquePaths>=
            fp!();

        let _:NestedFieldPath<()>=
            fp!(());

        let _:FieldPathSet<(NestedFieldPath<()>,NestedFieldPath<()>),AliasedPaths>=
            fp!((),());

        let _:FieldPathSet<(NestedFieldPath<()>,NestedFieldPath<()>,NestedFieldPath<()>),AliasedPaths>=
            fp!((),(),());
    }
    */
}

field_path_aliases! {
    FP_0_0=0.0,
    FP_0_1=0.1,
    FP_0_2_0=0.2.0,
    FP_0_2_1=0.2.1,
    FP_0_2_1_0=0.2.1.0,
    FP_0_2_1_1=0.2.1.1,
}

#[test]
fn get_nested_field_types() {
    use crate::field_traits::RevGetFieldType;
    use crate::{GetFieldType, GetFieldType2, GetFieldType3, GetFieldType4};

    type FP0 = FP!(0);
    type FP1 = FP!(1);
    type FP2 = FP!(2);

    type Unary<T> = (T,);

    type SStr = &'static str;
    type VVec = &'static [()];

    {
        type Tuple = Unary<SStr>;
        let _: AssertEq<GetFieldType<Tuple, FP0>, SStr>;
    }
    {
        type Tuple = Unary<(Void, SStr)>;
        let _: AssertEq<GetFieldType2<Tuple, FP0, FP0>, Void>;
        let _: AssertEq<GetFieldType2<Tuple, FP0, FP1>, SStr>;

        let _: AssertEq<RevGetFieldType<FP_0_0, Tuple>, Void>;
        let _: AssertEq<RevGetFieldType<FP_0_1, Tuple>, SStr>;
    }
    {
        type Tuple = Unary<((), (), (u64, SStr))>;
        let _: AssertEq<GetFieldType3<Tuple, FP0, FP2, FP0>, u64>;
        let _: AssertEq<GetFieldType3<Tuple, FP0, FP2, FP1>, SStr>;

        let _: AssertEq<RevGetFieldType<FP_0_2_0, Tuple>, u64>;
        let _: AssertEq<RevGetFieldType<FP_0_2_1, Tuple>, SStr>;
    }
    {
        type Tuple = Unary<((), (), ((), (SStr, VVec)))>;
        let _: AssertEq<GetFieldType4<Tuple, FP0, FP2, FP1, FP0>, SStr>;
        let _: AssertEq<GetFieldType4<Tuple, FP0, FP2, FP1, FP1>, VVec>;

        let _: AssertEq<RevGetFieldType<FP_0_2_1_0, Tuple>, SStr>;
        let _: AssertEq<RevGetFieldType<FP_0_2_1_1, Tuple>, VVec>;
    }
}
