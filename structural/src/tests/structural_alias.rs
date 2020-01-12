use crate::enum_traits::IsVariant;
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

    impl<This> AssertImplies for This where This: Copy + IntoFieldMut<FP!(a), Ty = u8, Err = NonOptField>
    {}

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
        This: Copy + IntoFieldMut<FP!(a), Ty = T, Err = NonOptField>,
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

    tstring_aliases! {
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
