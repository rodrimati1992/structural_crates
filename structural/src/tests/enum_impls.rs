use crate::*;


#[test]
fn option_test() {
    {
        let mut tup = (0, Some((1, "hello", 3, Some((19,30)))), 2);
        assert_eq!( tup.field_(fp!(1.0)), Some(&1) );
        assert_eq!( tup.field_(fp!(1.1)), Some(&"hello") );
        assert_eq!( tup.field_(fp!(1.2)), Some(&3) );
        assert_eq!( tup.field_(fp!(1.3.0)), Some(&19) );
        assert_eq!( tup.field_(fp!(1.3.1)), Some(&30) );
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
