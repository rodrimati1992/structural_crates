// Every one of these modules is independent
#![allow(clippy::wildcard_imports)]

use crate::{
    enums::{IsVariant, VariantCount},
    FieldType, GetField, GetFieldMut, GetVariantField, GetVariantFieldMut, IntoField, IntoFieldMut,
    IntoVariantField, IntoVariantFieldMut, Structural, StructuralExt,
};

use std_::fmt::Debug;

mod with_super_traits {
    use super::*;

    structural_alias! {
        trait Trait:Copy{
            a:u8,
        }
    }
    trait AssertImplies: Trait {}

    impl<This> AssertImplies for This where This: Copy + IntoFieldMut<TS!(a), Ty = u8> {}

    /// This function ensures that the supertraits and field accessors in Trait
    /// are implied by `T:Trait`.
    #[allow(dead_code)]
    fn func<T: Trait>(v: T) {
        let _copy = v;
        let _: &u8 = v.field_(fp!(a));
    }
}

/////////////////////////////////////////////

mod with_where_clause {
    use super::*;

    structural_alias! {
        trait WithWhereClause<T:Clone>:Copy
        where
            T:Debug
        {
            a:T,
        }
    }

    trait AssertImplies<T>: WithWhereClause<T>
    where
        T: Clone + Debug,
    {
    }

    impl<This, T> AssertImplies<T> for This
    where
        T: Clone + Debug,
        This: Copy + IntoFieldMut<TS!(a), Ty = T>,
    {
    }
}

/////////////////////////////////////////////

mod all_access {
    use super::*;

    structural_alias! {
        trait Foo<T>{
                 a:u32,
            ref  b:T,
            mut  c:i64,
            move d:&'static str,
        }
    }

    trait Dummy {
        fn well<This, T>()
        where
            This: Foo<T>;
    }

    impl Dummy for () {
        fn well<This, T>()
        where
            This: IntoFieldMut<TS!(a), Ty = u32>
                + GetField<FP!(b), Ty = T>
                + GetFieldMut<FP!(c), Ty = i64>
                + IntoField<FP!(d), Ty = &'static str>,
        {
        }
    }
}

// Testing the presence of `IsVariant`
mod with_variants {
    use super::*;

    structural_alias! {
        pub trait Foo{
            A{},
            B{},
        }
    }

    field_path_aliases! {
        mod names{
            A,
            B,
        }
    }

    assert_equal_bounds! {
        trait Dummy,
        (Foo),
        (
            IsVariant<names::A>+
            IsVariant<names::B>
        )
    }
}

// Testing the presence of `IsVariant`
mod variants_with_accesses {
    use super::*;

    field_path_aliases! {
        mod paths{
            a,
            A,B,C,
        }
    }

    tstr_aliases! {
        mod strings{
            a,b,c,d,e,
            A,AOpt,B,C,
        }
    }

    structural_alias! {
        pub trait Foo{
            A{
                a:(u8,u8),
                ref b:(u8,u16),
                mut c:(u8,u32),
                move d:(u8,u64),
                mut move e:(u16,u8),
            },
            AOpt{
                a:Option<(u8,u8)>,
                ref b:Option<(u8,u16)>,
                mut c:Option<(u8,u32)>,
                move d:Option<(u8,u64)>,
                mut move e:Option<(u16,u8)>,
            },
            mut move B{
                a:i8,
                ref b:i16,
            },
            ref C{
                a:u8,
                move b:u16,
            },
            a:(),
        }
    }

    assert_equal_bounds! {
        trait Dummy,
        (Foo),
        (
            IsVariant<paths::A>+
            IsVariant<paths::B>+
            IsVariant<paths::C>+
            IntoVariantFieldMut< strings::A, strings::a, Ty= (u8,u8) >+
            GetVariantField< strings::A, strings::b, Ty= (u8,u16) >+
            GetVariantFieldMut< strings::A, strings::c, Ty= (u8,u32) >+
            IntoVariantField< strings::A, strings::d, Ty= (u8,u64) >+
            IntoVariantFieldMut< strings::A, strings::e, Ty= (u16,u8) >+

            IntoVariantFieldMut< strings::AOpt, strings::a, Ty= Option<(u8,u8)> >+
            GetVariantField< strings::AOpt, strings::b, Ty= Option<(u8,u16)> >+
            GetVariantFieldMut< strings::AOpt, strings::c, Ty= Option<(u8,u32)> >+
            IntoVariantField< strings::AOpt, strings::d, Ty= Option<(u8,u64)> >+
            IntoVariantFieldMut< strings::AOpt, strings::e, Ty= Option<(u16,u8)> >+

            IntoVariantFieldMut< strings::B, strings::a, Ty= i8 >+
            GetVariantField< strings::B, strings::b, Ty= i16 >+
            GetVariantField< strings::C, strings::a, Ty= u8 >+
            IntoVariantField< strings::C, strings::b, Ty= u16 >+
            IntoFieldMut< paths::a, Ty= () >
        )
    }
}

mod exhaustive_enums {
    use super::*;

    field_path_aliases! {
        mod paths{
            A,B,C,
        }
    }

    tstr_aliases! {
        mod strings{
            n0="0",
            n1="1",
            n2="2",
            n3="3",
        }
    }

    macro_rules! exhaustive_enum_no_args_test {
        (
            mod $module:ident
            variants( $($variant:ident),* )
            variant_count_str=$variant_count_str:ident
        ) => {
            mod $module{
                use super::*;

                structural_alias!{
                    #[struc(exhaustive_enum)]
                    trait Foo{
                        $($variant{},)*
                    }
                }

                assert_equal_bounds! {
                    trait Dummy0,
                    (Foo),
                    (
                                    $( IsVariant<paths::$variant>+ )*
                        VariantCount<Count=strings::$variant_count_str>
                    )
                }
            }
        };
    }

    exhaustive_enum_no_args_test! {
        mod exhaustive_0
        variants()
        variant_count_str=n0
    }
    exhaustive_enum_no_args_test! {
        mod exhaustive_1
        variants(A)
        variant_count_str=n1
    }
    exhaustive_enum_no_args_test! {
        mod exhaustive_2
        variants(A,B)
        variant_count_str=n2
    }
    exhaustive_enum_no_args_test! {
        mod exhaustive_3
        variants(A,B,C)
        variant_count_str=n3
    }

    macro_rules! exhaustive_enum_with_args_test {
        (
            mod $module:ident
            $( exhaustive_enum_args $ee_arguments:tt )?
            trait $trait_:ident
            trait $nonex_trait:ident
            variants( $($variant:ident),* )
            variant_count_str=$variant_count_str:ident
        ) => {
            mod $module{
                use super::*;

                structural_alias!{
                    #[struc(and_exhaustive_enum $( $ee_arguments )? )]
                    trait $trait_{
                        $($variant{},)*
                    }
                }

                assert_equal_bounds! {
                    trait Dummy0,
                    ($trait_),
                    (
                                    $( IsVariant<paths::$variant>+ )*
                    )
                }

                assert_equal_bounds! {
                    trait Dummy1,
                    ($nonex_trait),
                    (
                        $trait_+
                        VariantCount<Count=strings::$variant_count_str>
                    )
                }
            }
        };
    }

    exhaustive_enum_with_args_test! {
        mod argless_and_exhaustive_2
        trait Foo
        trait Foo_Exhaustive
        variants( A,B )
        variant_count_str=n2
    }

    exhaustive_enum_with_args_test! {
        mod empty_args_and_exhaustive_2
        exhaustive_enum_args()
        trait Foo
        trait Foo_Exhaustive
        variants( A,B )
        variant_count_str=n2
    }

    exhaustive_enum_with_args_test! {
        mod suffix_arg_and_exhaustive_1
        exhaustive_enum_args( suffix="_Wha" )
        trait Foo
        trait Foo_Wha
        variants( A )
        variant_count_str=n1
    }

    exhaustive_enum_with_args_test! {
        mod name_arg_and_exhaustive_1
        exhaustive_enum_args( name="Bar" )
        trait Foo
        trait Bar
        variants( A )
        variant_count_str=n1
    }
}

mod tuple_and_unit_variants {
    use super::*;

    field_path_aliases! {
        mod paths{
            A,
        }
    }

    tstr_aliases! {
        mod strings{
            n0="0",
            n1="1",
            n2="2",
            n3="3",
            n4="4",
            n5="5",
            n6="6",
            A,
        }
    }

    structural_alias! {
        pub trait Tuple0{
            A()
        }

        pub trait Tuple1{
            ref A(u8)
        }

        pub trait Tuple2{
            mut A(u8,ref Option<u16>)
        }
        pub trait Tuple5{
            A(
                u8,
                ref u16,
                mut u32,
                move u64,
                mut move i8,
                mut move Option<i16>,
            ),
        }

        pub trait Unit1{
            A,
        }


    }

    assert_equal_bounds! {
        trait Dummy0,
        (Tuple0),
        (
            IsVariant<paths::A>+
        )
    }

    assert_equal_bounds! {
        trait Dummy1,
        (Tuple1),
        (
            IsVariant<paths::A>+
            GetVariantField<strings::A,strings::n0, Ty=u8>
        )
    }

    assert_equal_bounds! {
        trait Dummy2,
        (Tuple2),
        (
            IsVariant<paths::A>+
            GetVariantFieldMut<strings::A,strings::n0, Ty=u8>+
            GetVariantField<strings::A,strings::n1, Ty=Option<u16>>+
        )
    }
    assert_equal_bounds! {
        trait Dummy5,
        (Tuple5),
        (
            IsVariant<paths::A>+
            IntoVariantFieldMut<strings::A,strings::n0, Ty=u8>+
            GetVariantField<strings::A,strings::n1, Ty=u16>+
            GetVariantFieldMut<strings::A,strings::n2, Ty=u32>+
            IntoVariantField<strings::A,strings::n3, Ty=u64>+
            IntoVariantFieldMut<strings::A,strings::n4, Ty=i8>+
            IntoVariantFieldMut<strings::A,strings::n5, Ty=Option<i16>>+
        )
    }
    assert_equal_bounds! {
        trait UnitDummy5,
        (Unit1),
        (
            IsVariant<paths::A>
        )
    }
}

mod with_defaulted_items {
    use super::*;

    structural_alias! {
        pub trait Foo{
            fn hi()->u32{
                101
            }

            A{},

            const FOO:&'static str="what";

            ref a:u32,
        }
    }

    #[derive(Structural)]
    enum G {
        A,
    }

    impl FieldType<TS!(a)> for G {
        type Ty = u32;
    }
    impl GetField<TS!(a)> for G {
        fn get_field_(&self, _: TS!(a)) -> &u32 {
            &404
        }
    }

    fn with_foo<T>(this: &T)
    where
        T: Foo,
    {
        assert_eq!(G::hi(), 101);
        assert_eq!(G::FOO, "what");

        assert!(this.is_variant(fp!(A)));
        assert_eq!(this.field_(fp!(a)), &404);
    }

    #[test]
    fn defaulted_items() {
        with_foo(&G::A);
    }
}

mod non_ident_variant_names {
    use super::*;
    use crate::path::string_aliases as tstrs;

    structural_alias! {
        trait NonIdentVariantNames{
            0,
            1(u8),
            ref 2{"a":u16},
            mut 3{"b":u32},
            mut move 4{"c":u64},
            move 5{"d":u128},
            "ö",
            "á"(i8),
            ref "é"{h:i16},
            mut "í"{i:i32},
            mut move "ó"{j:i64},
            move "ú"{k: i128},
            r#move,
            r#async{a: Option<i8>},
            ref r#fn{r#impl: Option<i16>},
        }
    }

    assert_equal_bounds! {
        trait Dummy0,
        (NonIdentVariantNames),
        (
            IsVariant<tstrs::str_0>+
            IntoVariantFieldMut<tstrs::str_1, TS!(0), Ty=u8>+
            GetVariantField<tstrs::str_2, TS!(a), Ty=u16>+
            GetVariantFieldMut<tstrs::str_3, TS!(b), Ty=u32>+
            IntoVariantFieldMut<tstrs::str_4, TS!(c), Ty=u64>+
            IntoVariantField<tstrs::str_5, TS!(d), Ty=u128>+

            IsVariant<TS!("ö")>+
            IntoVariantFieldMut<TS!("á"), TS!(0), Ty=i8>+
            GetVariantField<TS!("é"), TS!(h), Ty=i16>+
            GetVariantFieldMut<TS!("í"), TS!(i), Ty=i32>+
            IntoVariantFieldMut<TS!("ó"), TS!(j), Ty=i64>+
            IntoVariantField<TS!("ú"), TS!(k), Ty=i128>+

            IsVariant<TS!("move")>+
            IntoVariantFieldMut<TS!("async"), TS!(a), Ty=Option<i8>>+
            GetVariantField<TS!("fn"), TS!("impl"), Ty=Option<i16>>+
        )
    }
}

mod with_generic_names {
    use super::*;

    tstr_aliases! {
        mod tstrs{
            Ooh,
            Qux,
            Foo,
            Bar,
            Baz,
        }
    }

    structural_alias! {
        pub trait GenericNames<B,C,D>{
            <TS!(a)>:u32,
            <B>:i32,
            Foo{
                c:(),
                ref <FP!("what the")>:i8,
                mut move <C>:&'static str,
                <tstrs::Ooh>:&'static str,
            },
            Bar(i16,u16),
            ref <TS!(Baz)>(u32),
            mut <tstrs::Qux>(u64),
            mut move<D>(u128),
        }
    }

    assert_equal_bounds! {
        trait Dummy0[B,C,D,],
        (GenericNames<B,C,D>),
        (
            IntoFieldMut<TS!(a),Ty=u32>+
            IntoFieldMut<B,Ty=i32>+

            IntoVariantFieldMut<tstrs::Foo, TS!(c),Ty=()>+
            GetVariantField<tstrs::Foo, TS!("what the"),Ty=i8>+
            IntoVariantFieldMut<tstrs::Foo, C,Ty=&'static str>+
            IntoVariantFieldMut<tstrs::Foo, tstrs::Ooh,Ty=&'static str>+

            IntoVariantFieldMut<tstrs::Bar, TS!(0),Ty=i16>+
            IntoVariantFieldMut<tstrs::Bar, TS!(1),Ty=u16>+

            GetVariantField<tstrs::Baz, TS!(0),Ty=u32>+

            GetVariantFieldMut<tstrs::Qux, TS!(0),Ty=u64>+

            IntoVariantFieldMut<D, TS!(0),Ty=u128>+

        )
    }
}
