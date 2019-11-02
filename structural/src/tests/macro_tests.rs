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
