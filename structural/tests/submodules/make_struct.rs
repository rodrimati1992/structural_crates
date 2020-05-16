use structural::{fp, make_struct, path::TStr, StructuralExt, FP};

structural::structural_alias! {
    trait Hi<T>{
        mut move a:u32,
        mut move b:Option<&'static str>,
        mut move c:T,
    }
}

fn returns_hi() -> impl Hi<&'static str> {
    make_struct! {
        a:0,
        b:"hello".into(),
        c:Default::default(),
    }
}

#[test]
fn make_struct_test() {
    {
        let hi = returns_hi();

        // I had to write it like this due to a rustc bug.
        // https://github.com/rust-lang/rust/issues/66057
        assert_eq!(hi.field_::<FP!(a)>(TStr::NEW), &0);
        assert_eq!(hi.field_::<FP!(b)>(TStr::NEW).unwrap(), "hello");
        assert_eq!(hi.field_::<FP!(c)>(TStr::NEW), &"");
    }

    {
        let hi: &dyn Hi<&'static str> = &returns_hi();
        assert_eq!(hi.field_(fp!(a)), &0);
        assert_eq!(hi.field_(fp!(b)).unwrap(), "hello");
        assert_eq!(hi.field_(fp!(c)), &"");
    }
}
