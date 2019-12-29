use crate::enum_traits::IsVariant;
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

    field_path_aliases_module! {
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

    field_path_aliases_module! {
        mod names{
            a,
            A,B,C,
            A_a=::A.a,
            A_b=::A.b,
            A_c=::A.c,
            A_d=::A.d,
            A_e=::A.e,
            B_a=::B.a,
            B_b=::B.b,
            C_a=::C.a,
            C_b=::C.b,
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
            IsVariant<names::A>+
            IsVariant<names::B>+
            IsVariant<names::C>+
            OptIntoFieldMut< names::A_a, Ty= (u8,u8) >+
            OptGetField< names::A_b, Ty= (u8,u16) >+
            OptGetFieldMut< names::A_c, Ty= (u8,u32) >+
            OptIntoField< names::A_d, Ty= (u8,u64) >+
            OptIntoFieldMut< names::A_e, Ty= (u16,u8) >+
            OptIntoFieldMut< names::B_a, Ty= i8 >+
            OptGetField< names::B_b, Ty= i16 >+
            OptGetField< names::C_a, Ty= u8 >+
            OptIntoField< names::C_b, Ty= u16 >+
            IntoFieldMut< names::a, Ty= () >
        )
    }
}
