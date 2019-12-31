use crate::*;

use std_::mem;

declare_variant_proxy! {
    BuiltinProxy
}

#[test]
fn option_test() {
    {
        let mut tup = (0, Some((1, "hello", 3, Some((19, 30)))), 2);
        assert_eq!(tup.field_(fp!(1.0)), Some(&1));
        assert_eq!(tup.field_(fp!(1.1)), Some(&"hello"));
        assert_eq!(tup.field_(fp!(1.2)), Some(&3));
        assert_eq!(tup.field_(fp!(1.3.0)), Some(&19));
        assert_eq!(tup.field_(fp!(1.3.1)), Some(&30));
        assert_eq!(
            tup.fields_mut(fp!(1.0, 1.1, 1.2)),
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

#[derive(Debug, Clone)]
enum Pair<T, U> {
    AllCorrect(T),
    Pair { left: T, right: U },
    Unit,
}

tstring_aliases_module! {
    mod pair_strs {
        AllCorrect,
        Pair,
        left,
        right,
        Unit,
    }
}

impl_getters_for_derive_enum! {
    impl[T,U,] Pair<T,U>
    where[]
    {
        enum=Pair
        proxy=BuiltinProxy
        (
            AllCorrect,
            pair_strs::AllCorrect,
            kind=newtype,
            fields((IntoFieldMut,0:T,nonopt))
        )
        (
            Pair,
            pair_strs::Pair,
            kind=regular,
            fields(
                (IntoFieldMut,left:T ,nonopt,pair_strs::left )
                (IntoFieldMut,right:U,nonopt,pair_strs::right)
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

#[derive(Structural, Debug, Clone)]
enum DerivingPair<T, U> {
    AllCorrect(T),
    Pair { left: T, right: U },
    Unit,
}

///////////////////////////////////////////////////////////////////////////////

macro_rules! pair_accessors {
    ( $type_:ident ) => {{
        {
            let mut this = $type_::<(i32, i32), ()>::AllCorrect((11, 22));
            assert_eq!(this.field_(fp!(AllCorrect)), Some(&(11, 22)));
            assert_eq!(this.field_(fp!(::AllCorrect.0)), Some(&11));
            assert_eq!(this.field_(fp!(::AllCorrect.1)), Some(&22));
            assert_eq!(this.field_(fp!(::Pair.left)), None);
            assert_eq!(this.field_(fp!(::Pair.right)), None);

            assert_eq!(this.field_mut(fp!(AllCorrect)), Some(&mut (11, 22)));
            assert_eq!(this.field_mut(fp!(::AllCorrect.0)), Some(&mut 11));
            assert_eq!(this.field_mut(fp!(::AllCorrect.1)), Some(&mut 22));
            assert_eq!(this.field_mut(fp!(::Pair.left)), None);
            assert_eq!(this.field_mut(fp!(::Pair.right)), None);

            assert_eq!(this.clone().into_field(fp!(AllCorrect)), Some((11, 22)));
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
            assert_eq!(this.field_(fp!(AllCorrect)), None);
            assert_eq!(this.field_(fp!(::Pair.left)), Some(&false));
            assert_eq!(this.field_(fp!(::Pair.right)), Some(&100));

            assert_eq!(this.field_mut(fp!(AllCorrect)), None);
            assert_eq!(this.field_mut(fp!(::Pair.left)), Some(&mut false));
            assert_eq!(this.field_mut(fp!(::Pair.right)), Some(&mut 100));

            assert_eq!(this.clone().into_field(fp!(AllCorrect)), None);
            assert_eq!(this.clone().into_field(fp!(::Pair.left)), Some(false));
            assert_eq!(this.clone().into_field(fp!(::Pair.right)), Some(100));
        }
        {
            let mut this = $type_::<u32, u32>::Pair {
                left: 100,
                right: 200,
            };
            let pair = this.field_mut(fp!(Pair)).unwrap();
            let (left, right) = pair.fields_mut(fp!(left, right));
            assert_eq!(left, &mut 100);
            assert_eq!(right, &mut 200);
            mem::swap(left, right);
            assert_eq!(left, &mut 200);
            assert_eq!(right, &mut 100);
        }
        {
            let mut this = $type_::<bool, u32>::Unit;
            assert_eq!(this.field_(fp!(AllCorrect)), None);
            assert_eq!(this.field_(fp!(Unit)).map(drop), Some(()));

            assert_eq!(this.field_mut(fp!(AllCorrect)), None);
            assert_eq!(this.field_mut(fp!(Unit)).map(drop), Some(()));

            assert_eq!(this.clone().into_field(fp!(AllCorrect)), None);
            assert_eq!(this.clone().into_field(fp!(Unit)).map(drop), Some(()));
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
