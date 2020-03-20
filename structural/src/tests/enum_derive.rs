use crate::enums::{IsVariant, VariantCount};
use crate::field_traits::variant_field::IntoVariantFieldMut;
use crate::*;

use std_::mem;

#[test]
fn option_test() {
    {
        let mut tup = (0, Some((1, "hello", 3, Some((19, 30)))), 2);
        assert_eq!(tup.field_(fp!(1::Some.0.0)), Some(&1));
        assert_eq!(tup.field_(fp!(1::Some.0.1)), Some(&"hello"));
        assert_eq!(tup.field_(fp!(1::Some.0.2)), Some(&3));
        assert_eq!(tup.field_(fp!(1::Some.0.3::Some.0.0)), Some(&19));
        assert_eq!(tup.field_(fp!(1::Some.0.3::Some.0.1)), Some(&30));
        assert_eq!(
            tup.fields_mut(fp!(1::Some.0.0,1::Some.0.1,1::Some.0.2)),
            (Some(&mut 1), Some(&mut "hello"), Some(&mut 3)),
        );
    }
    // {
    //     let mut none=None::<(u32,&str,u32,Option<(u32,u32)>)>;

    //     assert_eq!( none.field_(fp!(1.0)), None );
    //     assert_eq!( none.field_(fp!(1.1)), None );
    //     assert_eq!( none.field_(fp!(1.2)), None );
    //     assert_eq!( none.field_(fp!(1.3)), None );
    //     assert_eq!( none.field_(fp!(1.3.0)), None );
    //     assert_eq!( none.field_(fp!(1.3.1)), None );
    //     assert_eq!( none.fields_mut(fp!(1.0, 1.1, 1.2)), (None, None, None) );
    // }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
enum Pair<T, U> {
    AllCorrect(T),
    Pair { left: T, right: U },
    Unit,
}

tstr_aliases! {
    mod pair_strs {
        AllCorrect,
        Pair,
        left,
        right,
        Unit,
    }
}

_private_impl_getters_for_derive_enum! {
    impl[T,U,] Pair<T,U>
    where[]
    {
        enum=Pair
        variant_count=TS!(3),
        (
            AllCorrect,
            pair_strs::AllCorrect,
            kind=newtype,
            fields((IntoVariantFieldMut,0:T))
        )
        (
            Pair,
            pair_strs::Pair,
            kind=regular,
            fields(
                (IntoVariantFieldMut,left:T ,pair_strs::left )
                (IntoVariantFieldMut,right:U,pair_strs::right)
            )
        )
        (
            Unit,
            pair_strs::Unit,
            kind=regular,
            fields()
        )
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Structural, Debug, Clone, PartialEq)]
// #[struc(debug_print)]
enum DerivingPair<T, U> {
    #[struc(newtype)]
    AllCorrect(T),
    Pair {
        left: T,
        right: U,
    },
    Unit,
}

tstr_aliases! {
    mod dp_strs{
        AllCorrect,
        Pair,
        left,
        right,
        Unit,
        vc=3,
    }
}

assert_equal_bounds! {
    trait AssertDP[T,U,],
    (DerivingPair_ESI<T,U>),
    (
          IsVariant<dp_strs::AllCorrect>
        + IntoVariantFieldMut<dp_strs::Pair, dp_strs::left, Ty = T>
        + IntoVariantFieldMut<dp_strs::Pair, dp_strs::right, Ty = U>
        + IsVariant<dp_strs::Unit>
        + VariantCount<Count = dp_strs::vc>
    ),
}

///////////////////////////////////////////////////////////////////////////////

macro_rules! pair_accessors {
    ( $type_:ident ) => {{
        {
            let mut this = $type_::<(i32, i32), ()>::AllCorrect((11, 22));
            assert_eq!(this.field_(fp!(::AllCorrect)).map(|_|()), Some(()));
            assert_eq!(this.field_(fp!(::AllCorrect.0)), Some(&11));
            assert_eq!(this.field_(fp!(::AllCorrect.1)), Some(&22));
            assert_eq!(this.fields(fp!(::AllCorrect=>0,1)), Some((&11,&22)));
            assert_eq!(this.field_(fp!(::Pair.left)), None);
            assert_eq!(this.field_(fp!(::Pair.right)), None);
            assert_eq!(this.fields(fp!(::Pair=>left,right)), None);

            assert_eq!(this.field_mut(fp!(::AllCorrect)).map(|_|()), Some(()));
            assert_eq!(this.field_mut(fp!(::AllCorrect.0)), Some(&mut 11));
            assert_eq!(this.field_mut(fp!(::AllCorrect.1)), Some(&mut 22));
            assert_eq!(this.fields_mut(fp!(::AllCorrect=>0,1)), Some((&mut 11,&mut 22)));
            assert_eq!(this.field_mut(fp!(::Pair.left)), None);
            assert_eq!(this.field_mut(fp!(::Pair.right)), None);
            assert_eq!(this.fields_mut(fp!(::Pair=>left,right)), None);

            assert_eq!(this.clone().into_field(fp!(::AllCorrect)).map(drop), Some(()));
            assert_eq!(this.clone().into_field(fp!(::AllCorrect.0)), Some(11));
            assert_eq!(this.clone().into_field(fp!(::AllCorrect.1)), Some(22));
            assert_eq!(this.clone().into_field(fp!(::Pair.left)), None);
            assert_eq!(this.clone().into_field(fp!(::Pair.right)), None);
        }
        {
            let mut this = $type_::<bool, u32>::Pair {
                left: false,
                right: 100,
            };
            assert_eq!(this.field_(fp!(::AllCorrect)).map(drop), None);
            assert_eq!(this.field_(fp!(::Pair.left)), Some(&false));
            assert_eq!(this.field_(fp!(::Pair.right)), Some(&100));
            assert_eq!(this.fields(fp!(::Pair=>left,right)), Some((&false,&100)));

            assert_eq!(this.field_mut(fp!(::AllCorrect)).map(|_|()), None);
            assert_eq!(this.field_mut(fp!(::Pair.left)), Some(&mut false));
            assert_eq!(this.field_mut(fp!(::Pair.right)), Some(&mut 100));
            assert_eq!(this.fields_mut(fp!(::Pair=>left,right)), Some((&mut false,&mut 100)));

            assert_eq!(this.clone().into_field(fp!(::AllCorrect)), None);
            assert_eq!(this.clone().into_field(fp!(::Pair.left)), Some(false));
            assert_eq!(this.clone().into_field(fp!(::Pair.right)), Some(100));
        }
        {
            let mut this = $type_::<u32, u32>::Pair {
                left: 100,
                right: 200,
            };
            let pair = this.field_mut(fp!(::Pair)).unwrap();
            let (left, right) = pair.fields_mut(fp!(left, right));
            assert_eq!(left, &mut 100);
            assert_eq!(right, &mut 200);
            mem::swap(left, right);
            assert_eq!(left, &mut 200);
            assert_eq!(right, &mut 100);
        }
        {
            let mut this = $type_::<bool, u32>::Unit;
            assert_eq!(this.field_(fp!(::AllCorrect)).map(|_|()), None);
            assert_eq!(this.field_(fp!(::Unit)).map(|_|()), Some(()));

            assert_eq!(this.field_mut(fp!(::AllCorrect)).map(|_|()), None);
            assert_eq!(this.field_mut(fp!(::Unit)).map(|_|()), Some(()));

            assert_eq!(this.clone().into_field(fp!(::AllCorrect)).map(drop), None);
            assert_eq!(this.clone().into_field(fp!(::Unit)).map(drop), Some(()));
        }
    }};
}

#[test]
fn pair_accessors() {
    pair_accessors!(Pair)
}

#[test]
fn deriving_pair_accessors() {
    pair_accessors!(DerivingPair)
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Structural, Debug)]
// #[struc(debug_print)]
enum HuhNT {
    #[struc(newtype(bounds = "WhatNT_VSI< @variant >"))]
    U(WhatNT),
    V {
        a: &'static str,
        b: u32,
    },
}

#[derive(Structural, Debug, Clone, PartialEq)]
struct WhatNT {
    pub a: u64,
    pub b: u32,
    pub c: u16,
    pub d: u8,
}

tstr_aliases! {
    mod rb_strs{
        U,V,a,b,c,d,
        n0="0",n1="1",
    }
}

assert_equal_bounds! {
    trait AssertNT,
    (HuhNT_SI),
    (
          IntoVariantFieldMut<rb_strs::U, rb_strs::a, Ty = u64>
        + IntoVariantFieldMut<rb_strs::U, rb_strs::b, Ty = u32>
        + IntoVariantFieldMut<rb_strs::U, rb_strs::c, Ty = u16>
        + IntoVariantFieldMut<rb_strs::U, rb_strs::d, Ty = u8>
        + IntoVariantFieldMut<rb_strs::V, rb_strs::a, Ty = &'static str>
        + IntoVariantFieldMut<rb_strs::V, rb_strs::b, Ty = u32>
    ),
}

#[test]
fn test_replace_newtype_trait_object() {
    fn hi(wha_u: &mut dyn HuhNT_SI, wha_v: &mut dyn HuhNT_SI) {
        {
            assert_eq!(wha_u.field_(fp!(::U.a)), Some(&11));
            assert_eq!(wha_u.field_(fp!(::U.b)), Some(&22));
            assert_eq!(wha_u.field_(fp!(::U.c)), Some(&33));
            assert_eq!(wha_u.field_(fp!(::U.d)), Some(&44));
            assert_eq!(wha_u.fields(fp!(::U=>a,b,c,d)), Some((&11, &22, &33, &44)));
            assert_eq!(
                wha_u.fields_mut(fp!(::U=>a,b,c,d)),
                Some((&mut 11, &mut 22, &mut 33, &mut 44)),
            );

            let proxy = wha_u.field_mut(fp!(::U)).unwrap();
            assert_eq!(proxy.field_(fp!(a)), &11);
            assert_eq!(proxy.field_(fp!(b)), &22);
            assert_eq!(proxy.field_(fp!(c)), &33);
            assert_eq!(proxy.field_(fp!(d)), &44);

            assert_eq!(proxy.field_mut(fp!(a)), &mut 11);
            assert_eq!(proxy.field_mut(fp!(b)), &mut 22);
            assert_eq!(proxy.field_mut(fp!(c)), &mut 33);
            assert_eq!(proxy.field_mut(fp!(d)), &mut 44);

            assert_eq!(
                proxy.fields_mut(fp!(a, b, c, d)),
                (&mut 11, &mut 22, &mut 33, &mut 44)
            );
        }

        {
            assert_eq!(wha_v.field_(fp!(::V.a)), Some(&"55"));
            assert_eq!(wha_v.field_(fp!(::V.b)), Some(&66));
            assert_eq!(wha_v.fields(fp!(::V=>a,b)), Some((&"55", &66)));
            assert_eq!(wha_v.fields_mut(fp!(::V=>a,b)), Some((&mut "55", &mut 66)));

            let proxy = wha_v.field_mut(fp!(::V)).unwrap();
            assert_eq!(proxy.field_(fp!(a)), &"55");
            assert_eq!(proxy.field_(fp!(b)), &66);

            assert_eq!(proxy.field_mut(fp!(a)), &"55");
            assert_eq!(proxy.field_mut(fp!(b)), &66);

            assert_eq!(proxy.fields_mut(fp!(a, b)), (&mut "55", &mut 66));
        }
    }

    let what_nt = WhatNT {
        a: 11,
        b: 22,
        c: 33,
        d: 44,
    };
    let mut this = HuhNT::U(what_nt.clone());
    assert_eq!(this.field_(fp!(::U)).map(|_| ()), Some(()));

    hi(&mut this, &mut HuhNT::V { a: "55", b: 66 })
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Structural, Debug)]
// #[struc(debug_print)]
enum HuhRB {
    #[struc(replace_bounds = "WhatRB_VSI< @variant >")]
    U(u8, u16, u32, u64),
    V {
        a: &'static str,
        b: u32,
    },
}

#[derive(Structural, Debug)]
#[struc(public)]
struct WhatRB(u8, u16);

assert_equal_bounds! {
    trait AssertRB,
    (HuhRB_SI),
    (
          IntoVariantFieldMut<rb_strs::U, rb_strs::n0, Ty = u8>
        + IntoVariantFieldMut<rb_strs::U, rb_strs::n1, Ty = u16>
        + IntoVariantFieldMut<rb_strs::V, rb_strs::a, Ty = &'static str>
        + IntoVariantFieldMut<rb_strs::V, rb_strs::b, Ty = u32>
    ),
}

#[test]
fn test_replace_bounds_trait_object() {
    fn hi(wha_u: &mut dyn HuhRB_SI, wha_v: &mut dyn HuhRB_SI) {
        {
            assert_eq!(wha_u.field_(fp!(::U.0)), Some(&11));
            assert_eq!(wha_u.field_(fp!(::U.1)), Some(&22));
            assert_eq!(wha_u.fields(fp!(::U=>0,1)), Some((&11, &22)));
            assert_eq!(wha_u.fields_mut(fp!(::U=>0,1)), Some((&mut 11, &mut 22)));

            let proxy = wha_u.field_mut(fp!(::U)).unwrap();
            assert_eq!(proxy.field_(fp!(0)), &11);
            assert_eq!(proxy.field_(fp!(1)), &22);
            assert_eq!(proxy.field_mut(fp!(0)), &mut 11);
            assert_eq!(proxy.field_mut(fp!(1)), &mut 22);

            assert_eq!(proxy.fields_mut(fp!(0, 1)), (&mut 11, &mut 22));
        }

        {
            assert_eq!(wha_v.field_(fp!(::V.a)), Some(&"55"));
            assert_eq!(wha_v.field_(fp!(::V.b)), Some(&66));
            assert_eq!(wha_v.fields(fp!(::V=>a,b)), Some((&"55", &66)));
            assert_eq!(wha_v.fields_mut(fp!(::V=>a,b)), Some((&mut "55", &mut 66)));

            let proxy = wha_v.field_mut(fp!(::V)).unwrap();
            assert_eq!(proxy.field_(fp!(a)), &"55");
            assert_eq!(proxy.field_(fp!(b)), &66);

            assert_eq!(proxy.field_mut(fp!(a)), &"55");
            assert_eq!(proxy.field_mut(fp!(b)), &66);

            assert_eq!(proxy.fields_mut(fp!(a, b)), (&mut "55", &mut 66));
        }
    }

    hi(
        &mut HuhRB::U(11, 22, 33, 44),
        &mut HuhRB::V { a: "55", b: 66 },
    )
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Structural, Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum Accesses {
    #[struc(access = "ref")]
    RefVar(u8),

    #[struc(access = "mut")]
    MutVar(u16),

    #[struc(access = "move")]
    MoveVar(u32),

    #[struc(access = "mut move")]
    MutMoveVar(u64),

    #[struc(access = "ref")]
    MixedVar(
        (),
        #[struc(access = "ref")] i8,
        #[struc(access = "mut")] i16,
        #[struc(access = "move")] i32,
        #[struc(access = "mut move")] i64,
    ),
}

assert_equal_bounds! {
    trait AssertA[],
    (Accesses_ESI),
    (
        GetVariantField<TS!(RefVar), TS!(0), Ty = u8> +
        GetVariantFieldMut<TS!(MutVar), TS!(0), Ty = u16> +
        IntoVariantField<TS!(MoveVar), TS!(0), Ty = u32> +
        IntoVariantFieldMut<TS!(MutMoveVar), TS!(0), Ty = u64> +
        GetVariantField<TS!(MixedVar), TS!(0), Ty = ()> +
        GetVariantField<TS!(MixedVar), TS!(1), Ty = i8> +
        GetVariantFieldMut<TS!(MixedVar), TS!(2), Ty = i16> +
        IntoVariantField<TS!(MixedVar), TS!(3), Ty = i32> +
        IntoVariantFieldMut<TS!(MixedVar), TS!(4), Ty = i64> +
        VariantCount<Count = TS!(5)>
    ),
}

///////////////////////////////////////////////////////////////////////////////

mod with_variant_count_attr_1 {
    use crate::enums::VariantCountOut;
    use crate::{Structural, TS};

    #[derive(Structural)]
    #[struc(variant_count_alias)]
    #[allow(dead_code)]
    enum Enum {
        A,
    }

    #[test]
    fn variant_count_1() {
        let _: TS!(1) = Enum_VC::NEW;
        let _: TS!(1) = VariantCountOut::<Enum>::NEW;
    }
}

mod with_variant_count_attr_4 {
    use crate::enums::VariantCountOut;
    use crate::{Structural, TS};

    #[derive(Structural)]
    #[struc(variant_count_alias)]
    #[allow(dead_code)]
    pub enum Enum {
        A,
        B,
        C,
        D,
    }

    #[test]
    fn variant_count_4() {
        let _: TS!(4) = Enum_VC::NEW;
        let _: TS!(4) = VariantCountOut::<Enum>::NEW;
    }
}

#[test]
#[allow(unused_imports)]
fn publicness_of_variant_count_alias() {
    pub use with_variant_count_attr_4::Enum_VC;
}

mod without_variant_count_attr {
    use crate::Structural;

    #[derive(Structural)]
    #[allow(dead_code)]
    enum Enum {
        A,
    }

    type Enum_VC = ();

    #[test]
    fn no_variant_count() {
        let _: Enum_VC = ();
    }
}

mod nonexhaustive_enum {
    use crate::Structural;

    #[derive(Structural)]
    #[struc(non_exhaustive)]
    #[allow(dead_code)]
    pub enum NonExhaustiveEnum {
        A,
        B,
    }

    trait NonExhaustiveEnum_ESI {}
    fn impls_trait_assertion(ne: NonExhaustiveEnum) -> impl NonExhaustiveEnum_SI {
        ne
    }

    #[test]
    fn nonexhaustive_impls_trait() {
        impls_trait_assertion(NonExhaustiveEnum::A);
    }
}
