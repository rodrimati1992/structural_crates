use crate::enum_traits::{IsVariant, VariantCount};
use crate::field_traits::variant_field::*;
use crate::field_traits::NonOptField;
use crate::*;

use std_::fmt::Debug;

mod with_super_traits {
    use super::*;

    structural_alias! {
        trait Trait:Copy{
            a:u8,
        }
    }
    trait AssertImplies: Trait {}

    impl<This> AssertImplies for This where
        This: Copy + IntoFieldMut<FP!(a), Ty = u8, Err = NonOptField> + IsStructural
    {
    }

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
        This: Copy + IntoFieldMut<FP!(a), Ty = T, Err = NonOptField> + IsStructural,
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
            This: IntoFieldMut<FP!(a), Ty = u32>
                + GetFieldImpl<FP!(b), Ty = T>
                + GetFieldMutImpl<FP!(c), Ty = i64>
                + IntoFieldImpl<FP!(d), Ty = &'static str>,
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
            IsVariant<names::B>+
            IsStructural
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
                a:?(u8,u8),
                ref b:?(u8,u16),
                mut c:?(u8,u32),
                move d:?(u8,u64),
                mut move e:?(u16,u8),
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
            IsStructural+
            IsVariant<paths::A>+
            IsVariant<paths::B>+
            IsVariant<paths::C>+
            IntoVariantFieldMut< strings::A, strings::a, Ty= (u8,u8) >+
            GetVariantField< strings::A, strings::b, Ty= (u8,u16) >+
            GetVariantFieldMut< strings::A, strings::c, Ty= (u8,u32) >+
            IntoVariantField< strings::A, strings::d, Ty= (u8,u64) >+
            IntoVariantFieldMut< strings::A, strings::e, Ty= (u16,u8) >+

            OptIntoVariantFieldMut< strings::AOpt, strings::a, Ty= (u8,u8) >+
            OptGetVariantField< strings::AOpt, strings::b, Ty= (u8,u16) >+
            OptGetVariantFieldMut< strings::AOpt, strings::c, Ty= (u8,u32) >+
            OptIntoVariantField< strings::AOpt, strings::d, Ty= (u8,u64) >+
            OptIntoVariantFieldMut< strings::AOpt, strings::e, Ty= (u16,u8) >+

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
                        IsStructural+
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
                        IsStructural+
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
        trait Tuple0{
            A()
        }

        trait Tuple1{
            ref A(u8)
        }

        trait Tuple2{
            mut A(u8,ref ?u16)
        }
        trait Tuple5{
            A(
                u8,
                ref u16,
                mut u32,
                move u64,
                mut move i8,
                mut move ?i16,
            ),
        }

        trait Unit1{
            A,
        }


    }

    assert_equal_bounds! {
        trait Dummy0,
        (Tuple0),
        (
            IsStructural+
            IsVariant<paths::A>+
        )
    }

    assert_equal_bounds! {
        trait Dummy1,
        (Tuple1),
        (
            IsStructural+
            IsVariant<paths::A>+
            GetVariantField<strings::A,strings::n0, Ty=u8>
        )
    }

    assert_equal_bounds! {
        trait Dummy2,
        (Tuple2),
        (
            IsStructural+
            IsVariant<paths::A>+
            GetVariantFieldMut<strings::A,strings::n0, Ty=u8>+
            OptGetVariantField<strings::A,strings::n1, Ty=u16>+
        )
    }
    assert_equal_bounds! {
        trait Dummy5,
        (Tuple5),
        (
            IsStructural+
            IsVariant<paths::A>+
            IntoVariantFieldMut<strings::A,strings::n0, Ty=u8>+
            GetVariantField<strings::A,strings::n1, Ty=u16>+
            GetVariantFieldMut<strings::A,strings::n2, Ty=u32>+
            IntoVariantField<strings::A,strings::n3, Ty=u64>+
            IntoVariantFieldMut<strings::A,strings::n4, Ty=i8>+
            OptIntoVariantFieldMut<strings::A,strings::n5, Ty=i16>+
        )
    }
    assert_equal_bounds! {
        trait UnitDummy5,
        (Unit1),
        (
            IsStructural+
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

    impl FieldType<FP!(a)> for G {
        type Ty = u32;
    }
    impl GetFieldImpl<FP!(a)> for G {
        type Err = NonOptField;

        fn get_field_(&self, _: FP!(a), _: ()) -> Result<&u32, NonOptField> {
            Ok(&404)
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
