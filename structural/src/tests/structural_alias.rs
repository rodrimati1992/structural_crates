use crate::*;

use core::fmt::Debug;


mod with_super_traits{
    use super::*;

    structural_alias!{
        trait Trait:Copy{
            a:u8,
        }
    }
    trait AssertImplies:Trait{}

    impl<This> AssertImplies for This
    where
        This:Copy+GetField<TStr!(a),Ty=u8>
    {}

    fn func<T:Trait>(v:T){
        let _copy=v;
        let _:&u8=v.field_(tstr!("a"));
    }
}




/////////////////////////////////////////////


mod with_where_clause{
    use super::*;

    structural_alias!{
        trait WithWhereClause<T:Clone>:Copy
        where
            T:Debug
        {
            a:T,
        }
    }


    trait AssertImplies<T>:WithWhereClause<T>
    where
        T:Clone+Debug
    {}

    impl<This,T> AssertImplies<T> for This
    where
        T:Clone+Debug,
        This:Copy+GetField<TStr!(a),Ty=T>
    {}
}


/////////////////////////////////////////////



mod all_access{
    use super::*;

    structural_alias!{
        trait Foo<T>{
                 a:u32,
            ref  b:T,
            mut  c:i64,
            move d:&'static str,
        }
    }

    trait Dummy{
        fn well<This,T>()
        where
            This:Foo<T>;
    }

    impl Dummy for () {
        fn well<This,T>()
        where
            This:
                GetField<TStr!(a), Ty=u32>+
                GetField<TStr!(b), Ty=T>+
                GetFieldMut<TStr!(c), Ty=i64>+
                IntoField<TStr!(d), Ty=&'static str>,
        {}
    }


}
