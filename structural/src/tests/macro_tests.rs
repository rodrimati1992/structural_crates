/// Tests that the ti and TI macros are correct
#[allow(non_camel_case_types)]
#[cfg(feature="better_ti")]
#[test]
fn identifier_macros_equality(){
    use crate::chars;

    fn assert_ident<T,U>(_ident:T)
    where
        T:core_extensions::TypeIdentity<Type=U>
    {}

    {
        type TheStr=chars::TString<(chars::_a,chars::_b,chars::_c,chars::_d)>;
        assert_ident::<TheStr,TI!("abcd")>(ti!(abcd));
        assert_ident::<TheStr,TI!(a b c d)>(ti!(abcd));
        assert_ident::<TheStr,TI!(abcd)>(ti!(abcd));
    }
    {
        type TheStr=chars::TString<(chars::_0,)>;
        assert_ident::<TheStr,TI!(0)>(ti!(0));
    }
    {
        type TheStr=chars::TString<(chars::_2,chars::_1)>;
        assert_ident::<TheStr,TI!(21)>(ti!(21));
        assert_ident::<TheStr,TI!(2 1)>(ti!(21));
    }
    {
        type TheStr=chars::TString<(chars::_a,chars::_b,chars::_0)>;
        assert_ident::<TheStr ,TI!(ab0)>(ti!(ab0));
    }
    

}






mod make_struct_tests{
    use crate::{
        type_level::TString,
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
            assert_eq!(hi.field_::<TI!(a)>(TString::NEW),&0);
            assert_eq!(hi.field_::<TI!(b)>(TString::NEW).unwrap(), "hello");
            assert_eq!(hi.field_::<TI!(c)>(TString::NEW), &"");
        }

        {
            let hi:&dyn Hi<&'static str>=&returns_hi();
            assert_eq!(hi.field_(ti!(a)),&0);
            assert_eq!(hi.field_(ti!(b)).unwrap(), "hello");
            assert_eq!(hi.field_(ti!(c)), &"");
        }
    }
}


mod names_module_tests{
    declare_names_module!{
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
            k=("p"),
        }
    }
    #[test]
    fn names_module_a(){
        let _:names_a::_a=ti!(_a);
        let _:names_a::_b=ti!(_b);
        let _:names_a::_0=ti!(_0);
        let _:names_a::c=ti!(c);
        let _:names_a::d=ti!(0);
        let _:names_a::e=ti!(10);
        let _:names_a::g=ti!(abcd);
        let _:names_a::h=ti!(a,b,c);
        let _:names_a::i=ti!(0,3,5);
        let _:names_a::j=ti!(p);
        let _:names_a::k=ti!(p);
    }
}








