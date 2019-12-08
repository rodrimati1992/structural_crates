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
            This: GetFieldImpl<FP!(a), Ty = u32>
                + GetFieldImpl<FP!(b), Ty = T>
                + GetFieldMutImpl<FP!(c), Ty = i64>
                + IntoFieldImpl<FP!(d), Ty = &'static str>,
        {
        }
    }
}
