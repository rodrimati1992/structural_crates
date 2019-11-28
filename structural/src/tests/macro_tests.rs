#[cfg(feature="better_macros")]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[macro_use]
mod for_better_macros{
    use crate::{chars,type_level::_private::TString};

    pub type S_foo=TString<(chars::_f,chars::_o,chars::_o)>;
    pub type S_bar=TString<(chars::_b,chars::_a,chars::_r)>;
    pub type S_baz=TString<(chars::_b,chars::_a,chars::_z)>;
    pub type S_a=TString<(chars::_a,)>;
    pub type S_b=TString<(chars::_b,)>;
    pub type S_c=TString<(chars::_c,)>;
    pub type S_d=TString<(chars::_d,)>;
    pub type S_0=TString<(chars::_0,)>;
    pub type S_1=TString<(chars::_1,)>;
    pub type S_2=TString<(chars::_2,)>;
    pub type S_3=TString<(chars::_3,)>;
    pub type S_4=TString<(chars::_4,)>;

    pub fn assert_ty<T,U>(_ident:U)
    where
        T:core_extensions::TypeIdentity<Type=U>
    {}

    macro_rules! path_assertion {
        (fp!( $($fp_params:tt)* ),$ty:ty $(,)?) => (
            assert_ty::<
                $ty,
                FP!($($fp_params)*),
            >(fp!($($fp_params)*));
        )
    }

}

/// Tests that the fp and FP macros are correct
#[allow(non_camel_case_types)]
#[cfg(feature="better_macros")]
#[test]
fn identifier_macros_equality(){
    use crate::chars;
    use crate::type_level::{_private::TString,FieldPath};
    use self::for_better_macros::*;

    type S_abcd=TString<(chars::_a,chars::_b,chars::_c,chars::_d)>;
    type S_21=TString<(chars::_2,chars::_1)>;
    type S_ab0=TString<(chars::_a,chars::_b,chars::_0)>;

    path_assertion!(fp!(abcd),FieldPath<(S_abcd,)>);
    path_assertion!(fp!(0),FieldPath<(S_0,)>);
    path_assertion!(fp!(21),FieldPath<(S_21,)>);
    path_assertion!(fp!(ab0),FieldPath<(S_ab0,)>);
    path_assertion!(fp!(0.1),FieldPath<(S_0,S_1)>);
    path_assertion!(fp!(0.1.2),FieldPath<(S_0,S_1,S_2)>);
    path_assertion!(fp!(0.1.2.3),FieldPath<(S_0,S_1,S_2,S_3)>);
    path_assertion!(fp!(0.1.2.3.4),FieldPath<(S_0,S_1,S_2,S_3,S_4)>);
    path_assertion!(fp!(0.foo),FieldPath<(S_0,S_foo)>);
    path_assertion!(fp!(0.foo.1),FieldPath<(S_0,S_foo,S_1)>);
    path_assertion!(fp!(0.foo.1.bar),FieldPath<(S_0,S_foo,S_1,S_bar)>);

    /*
    path_assertion!(fp!(0[FP!(0)]),FieldPath<(S_0,S_0)>);
    path_assertion!(fp!(0.[FP!(0)]),FieldPath<(S_0,S_0)>);
    path_assertion!(fp!(0[FP!(1)].2),FieldPath<(S_0,S_1,S_2)>);
    path_assertion!(fp!(0.[FP!(1)].2),FieldPath<(S_0,S_1,S_2)>);
    path_assertion!(fp!([FP!(0)].2),FieldPath<(S_0,S_2)>);
    path_assertion!(fp!([FP!(0)]),FieldPath<(S_0,)>);

    path_assertion!(fp!(0.(FP!(0.1))),FieldPath<(S_0,S_0,S_1)>);
    path_assertion!(fp!((FP!(0.1)).2),FieldPath<(S_0,S_1,S_2)>);
    path_assertion!(fp!(0.(FP!(0.1)).2),FieldPath<(S_0,S_0,S_1,S_2)>);
    path_assertion!(fp!((FP!(0.1))),FieldPath<(S_0,S_1)>);
    
    path_assertion!(fp!((FP!(0.1))[S_3]),FieldPath<(S_0,S_1,S_3)>);
    path_assertion!(fp!([S_3].(FP!(0.1))),FieldPath<(S_3,S_0,S_1)>);
    path_assertion!(fp!([S_3].(FP!(0.1)).[S_a]),FieldPath<(S_3,S_0,S_1,S_a)>);
    */
}

#[allow(non_camel_case_types)]
#[cfg(feature="better_macros")]
#[test]
fn field_paths_equality(){
    use crate::type_level::{FieldPath,FieldPathSet,UniquePaths};

    use self::for_better_macros::*;

    path_assertion!{
        fp!(foo,bar),
        FieldPathSet<(FieldPath<(S_foo,)>,FieldPath<(S_bar,)>,),UniquePaths>
    }

    path_assertion!{
        fp!(foo.bar,baz),
        FieldPathSet<(FieldPath<(S_foo,S_bar)>,FieldPath<(S_baz,)>,),UniquePaths>,
    }

    path_assertion!{
        fp!(foo.bar,a.b),
        FieldPathSet<(FieldPath<(S_foo,S_bar)>,FieldPath<(S_a,S_b)>,),UniquePaths>,
    }

    path_assertion!{
        fp!(0,foo),
        FieldPathSet<(FieldPath<(S_0,)>,FieldPath<(S_foo,)>,),UniquePaths>,
    }

    path_assertion!{
        fp!(0.1,foo),
        FieldPathSet<(FieldPath<(S_0,S_1)>,FieldPath<(S_foo,)>,),UniquePaths>,
    }
    path_assertion!{
        fp!(0.1.2,foo),
        FieldPathSet<(FieldPath<(S_0,S_1,S_2)>,FieldPath<(S_foo,)>,),UniquePaths>,
    }
    path_assertion!{
        fp!(foo,0.1.2.3),
        FieldPathSet<(FieldPath<(S_foo,)>,FieldPath<(S_0,S_1,S_2,S_3)>),UniquePaths>,
    }
    path_assertion!{
        fp!(0.1.2.3.4,foo),
        FieldPathSet<(FieldPath<(S_0,S_1,S_2,S_3,S_4)>,FieldPath<(S_foo,)>,),UniquePaths>,
    }
    
    /*
    path_assertion!{
        fp!((FP!(a.b)).(FP!(c.d)),0),
        FieldPathSet<(FieldPath<(S_a,S_b,S_c,S_d)>,FieldPath<(S_0,)>,),AliasedPaths>,
    }
    */

}






mod make_struct_tests{
    use crate::{
        type_level::FieldPath,
        GetFieldExt,
    };

    crate::structural_alias!{
        trait Hi<T>{
            mut move a:u32,
            mut move b:Option<&'static str>,
            mut move c:T,
        }
    }

    fn returns_hi()->impl Hi<&'static str>{
        make_struct!{
            a:0,
            b:"hello".into(),
            c:Default::default(),
        }
    }

    #[test]
    fn make_struct_test(){
        {
            let hi=returns_hi();
            
            // I had to write it like this due to a rustc bug.
            // https://github.com/rust-lang/rust/issues/66057
            assert_eq!(hi.field_::<FP!(a)>(FieldPath::NEW),&0);
            assert_eq!(hi.field_::<FP!(b)>(FieldPath::NEW).unwrap(), "hello");
            assert_eq!(hi.field_::<FP!(c)>(FieldPath::NEW), &"");
        }

        {
            let hi:&dyn Hi<&'static str>=&returns_hi();
            assert_eq!(hi.field_(fp!(a)),&0);
            assert_eq!(hi.field_(fp!(b)).unwrap(), "hello");
            assert_eq!(hi.field_(fp!(c)), &"");
        }
    }
}


mod names_module_tests{
    field_path_aliases_module!{
        mod names_a{
            _a,
            _b,
            _0,
            c,
            d=0,
            e=10,
            g=abcd,
            h=(a,b,c),
            i=(0,3,5),
            j=(p), 
        }
    }
    #[test]
    fn names_module_a(){
        let _:names_a::_a=fp!(_a);
        let _:names_a::_b=fp!(_b);
        let _:names_a::_0=fp!(_0);
        let _:names_a::c=fp!(c);
        let _:names_a::d=fp!(0);
        let _:names_a::e=fp!(10);
        let _:names_a::g=fp!(abcd);
        let _:names_a::h=fp!(a,b,c);
        let _:names_a::i=fp!(0,3,5);
        let _:names_a::j=fp!(p);
    }
}








