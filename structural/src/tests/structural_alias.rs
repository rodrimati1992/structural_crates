use crate::GetField;

use std::fmt::Debug;


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


