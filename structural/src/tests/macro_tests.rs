/// Tests that the fp and FP macros are correct
#[allow(non_camel_case_types)]
#[cfg(feature="better_macros")]
#[test]
fn identifier_macros_equality(){
    use crate::{chars,type_level::{TString,FieldPath}};

    fn assert_ident<T,U>(_ident:U)
    where
        FieldPath<(T,)>:core_extensions::TypeIdentity<Type=U>
    {}

    {
        type TheStr=TString<(chars::_a,chars::_b,chars::_c,chars::_d)>;
        assert_ident::<TheStr,FP!(abcd)>(fp!(abcd));
    }
    {
        type TheStr=TString<(chars::_0,)>;
        assert_ident::<TheStr,FP!(0)>(fp!(0));
    }
    {
        type TheStr=TString<(chars::_2,chars::_1)>;
        assert_ident::<TheStr,FP!(21)>(fp!(21));
    }
    {
        type TheStr=TString<(chars::_a,chars::_b,chars::_0)>;
        assert_ident::<TheStr ,FP!(ab0)>(fp!(ab0));
    }
    

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








