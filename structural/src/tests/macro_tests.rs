use crate::p as chars;

#[cfg(feature = "use_const_str")]
macro_rules! cond_tstr_alias {
    ( $name:ident=($with_type:ty, $with_const:literal) ) => {
        pub type $name = crate::p::TStrPriv<$with_const>;
    };
}
#[cfg(not(feature = "use_const_str"))]
macro_rules! cond_tstr_alias {
    ( $name:ident=($with_type:ty, $with_const:literal) ) => {
        pub type $name = crate::p::TStrPriv<$with_type>;
    };
}

#[cfg(feature = "use_const_str")]
macro_rules! tstr_asserts{
    ( $(($with_type:ty, $with_const:literal)=($($found:expr),*);)* ) => {
        $(
            $(
                let _: crate::p::TStrPriv<$with_const> = $found;
            )*
        )*
    };
}
#[cfg(not(feature = "use_const_str"))]
macro_rules! tstr_asserts{
    ( $(($with_type:ty, $with_const:literal)=($($found:expr),*);)* ) => {
        $(
            $(
                let _: crate::p::TStrPriv<$with_type> = $found;
            )*
        )*
    };
}

#[cfg(feature = "better_macros")]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[macro_use]
mod for_better_macros {
    use crate::{p as chars, p::TStrPriv};

    cond_tstr_alias!(S_foo = ((chars::_f, chars::_o, chars::_o), "foo"));
    cond_tstr_alias!(S_bar = ((chars::_b, chars::_a, chars::_r), "bar"));
    cond_tstr_alias!(S_baz = ((chars::_b, chars::_a, chars::_z), "baz"));
    cond_tstr_alias!(S_a = ((chars::_a,), "a"));
    cond_tstr_alias!(S_b = ((chars::_b,), "b"));
    cond_tstr_alias!(S_c = ((chars::_c,), "c"));
    cond_tstr_alias!(S_d = ((chars::_d,), "d"));
    cond_tstr_alias!(S_0 = ((chars::_0,), "0"));
    cond_tstr_alias!(S_1 = ((chars::_1,), "1"));
    cond_tstr_alias!(S_2 = ((chars::_2,), "2"));
    cond_tstr_alias!(S_3 = ((chars::_3,), "3"));
    cond_tstr_alias!(S_4 = ((chars::_4,), "4"));

    pub fn assert_ty<T, U>(_ident: U)
    where
        T: core_extensions::TypeIdentity<Type = U>,
    {
    }

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
#[cfg(feature = "better_macros")]
#[test]
fn identifier_macros_equality() {
    use self::for_better_macros::*;
    use crate::field_path::FieldPath;
    use crate::p::TStrPriv;

    cond_tstr_alias!(S_abcd = ((chars::_a, chars::_b, chars::_c, chars::_d), "abcd"));
    cond_tstr_alias!(S_21 = ((chars::_2, chars::_1), "21"));
    cond_tstr_alias!(S_ab0 = ((chars::_a, chars::_b, chars::_0), "ab0"));

    path_assertion!(fp!(abcd), S_abcd);
    path_assertion!(fp!(0), S_0);
    path_assertion!(fp!(21), S_21);
    path_assertion!(fp!(ab0), S_ab0);
    path_assertion!(fp!(0.1), FieldPath<(S_0, S_1)>);
    path_assertion!(fp!(0.1.2), FieldPath<(S_0, S_1, S_2)>);
    path_assertion!(fp!(0.1.2.3), FieldPath<(S_0, S_1, S_2, S_3)>);
    path_assertion!(fp!(0.1.2.3.4), FieldPath<(S_0, S_1, S_2, S_3, S_4)>);
    path_assertion!(fp!(0.foo), FieldPath<(S_0, S_foo)>);
    path_assertion!(fp!(0.foo.1), FieldPath<(S_0, S_foo, S_1)>);
    path_assertion!(fp!(0.foo.1.bar), FieldPath<(S_0, S_foo, S_1, S_bar)>);
}

#[allow(non_camel_case_types)]
#[cfg(feature = "better_macros")]
#[test]
fn field_paths_equality() {
    use crate::field_path::{FieldPath, FieldPathSet, UniquePaths};

    use self::for_better_macros::*;

    path_assertion! {
        fp!(foo,bar),
        FieldPathSet<(S_foo,S_bar,),UniquePaths>
    }

    path_assertion! {
        fp!(foo.bar,baz),
        FieldPathSet<(FieldPath<(S_foo,S_bar)>,S_baz,),UniquePaths>,
    }

    path_assertion! {
        fp!(foo.bar,a.b),
        FieldPathSet<(FieldPath<(S_foo,S_bar)>,FieldPath<(S_a,S_b)>,),UniquePaths>,
    }

    path_assertion! {
        fp!(0,foo),
        FieldPathSet<(S_0,S_foo),UniquePaths>,
    }

    path_assertion! {
        fp!(0.1,foo),
        FieldPathSet<(FieldPath<(S_0,S_1)>,S_foo),UniquePaths>,
    }
    path_assertion! {
        fp!(0.1.2,foo),
        FieldPathSet<(FieldPath<(S_0,S_1,S_2)>,S_foo),UniquePaths>,
    }
    path_assertion! {
        fp!(foo,0.1.2.3),
        FieldPathSet<(S_foo,FieldPath<(S_0,S_1,S_2,S_3)>),UniquePaths>,
    }
    path_assertion! {
        fp!(0.1.2.3.4,foo),
        FieldPathSet<(FieldPath<(S_0,S_1,S_2,S_3,S_4)>,S_foo),UniquePaths>,
    }

    /*
    path_assertion!{
        fp!((FP!(a.b)).(FP!(c.d)),0),
        FieldPathSet<(FieldPath<(S_a,S_b,S_c,S_d)>,FieldPath<(S_0,)>,),AliasedPaths>,
    }
    */
}

mod make_struct_tests {
    use crate::{field_path::TStr, GetFieldExt};

    crate::structural_alias! {
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
}

mod names_module_tests {
    use core_extensions::type_asserts::*;

    field_path_aliases! {
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
            k0=(::p),
            k1=(::"p"),
            l0=(a,b,c,d,e),
            l1=(a,"b",c,"d",e),
            m0=(::a.b),
            m1=(::"a"."b"),
            n0=(0=>"1","2"),
            n1=("0"=>1,2),

            o0=(::0.1),
            o1=(::"0".1),
            o2=(::0."1"),
            o3=(::"0"."1"),
        }
    }
    #[test]
    fn names_module_a() {
        let _: names_a::_a = fp!(_a);
        let _: names_a::_b = fp!(_b);
        let _: names_a::_0 = fp!(_0);
        let _: names_a::c = fp!(c);
        let _: names_a::d = fp!(0);
        let _: names_a::e = fp!(10);
        let _: names_a::g = fp!(abcd);
        let _: names_a::h = fp!(a, b, c);
        let _: names_a::i = fp!(0, 3, 5);
        let _: names_a::j = fp!(p);

        let _: AssertEq3<names_a::k0, _, _> = AssertEq3::new(names_a::k0, names_a::k1, fp!(::p));

        let _: AssertEq3<names_a::l0, _, _> =
            AssertEq3::new(names_a::l0, names_a::l1, fp!(a, b, c, d, e));

        let _: AssertEq3<names_a::n0, _, _> = AssertEq3::new(names_a::n0, names_a::n1, fp!(0=>1,2));

        let _: AssertEq3<names_a::o0, _, _> =
            AssertEq3::new(names_a::o0, names_a::o1, fp!(::0."1"));
        let _: AssertEq3<names_a::o0, _, _> =
            AssertEq3::new(names_a::o0, names_a::o2, fp!(::"0".1));
        let _: AssertEq3<names_a::o0, _, _> =
            AssertEq3::new(names_a::o0, names_a::o3, fp!(::0."1"));
        let _: AssertEq3<names_a::o0, _, _> =
            AssertEq3::new(names_a::o0, names_a::o3, fp!(::"0"."1"));
    }
}

mod tstr_aliases_tests {
    use crate::{p as chars, p::TStrPriv};

    #[test]
    fn just_aliases() {
        mod strs {
            tstr_aliases! {
                a,
                b,
                word,
                c="cc",
                d=dd,
                p0=0,
                p10=10,
                p100=100,
            }
        }

        tstr_asserts! {
            ((chars::_a,),"a") = (strs::a::NEW,ts!("a"),ts!(a));
            ((chars::_b,),"b") = (strs::b::NEW,ts!("b"),ts!(b));
            ((chars::_w, chars::_o, chars::_r, chars::_d),"word") =
                (strs::word::NEW,ts!("word"),ts!(word));
            ((chars::_d, chars::_d),"dd") = (strs::d::NEW,ts!("dd"),ts!(dd));
            ((chars::_c, chars::_c),"cc") = (strs::c::NEW,ts!("cc"),ts!(cc));
            ((chars::_0,),"0") = (strs::p0::NEW,ts!("0"),ts!(0));
            ((chars::_1, chars::_0),"10") = (strs::p10::NEW,ts!("10"),ts!(10));
            ((chars::_1, chars::_0, chars::_0),"100") = (strs::p100::NEW,ts!("100"),ts!(100));
        }
    }

    #[test]
    fn with_submodules() {
        tstr_aliases! {
            mod strs{
                mod m0{
                    @count
                }
                a0="10",
                mod m1{
                    @count
                    a0="11",
                    a1,
                }
                a1,
                mod m2{
                    foo,
                    bar=0,
                    baz=baaaa,
                }
                mod m3{
                    @count
                    mod m3m0{
                        aaa,
                        bbb,
                    }
                }
            }
        }

        tstr_asserts! {
            ((chars::_0,),"0") = (strs::m0::__TString_Aliases_Count::NEW,ts!("0"),ts!(0));

            ((chars::_1, chars::_0),"10") = (strs::a0::NEW,ts!("10"),ts!(10));

            ((chars::_2,),"2") = (strs::m1::__TString_Aliases_Count::NEW,ts!("2"),ts!(2));
            ((chars::_1, chars::_1),"11") = (strs::m1::a0::NEW,ts!("11"),ts!(11));
            ((chars::_a, chars::_1),"a1") = (strs::m1::a1::NEW,ts!("a1"),ts!(a1));

            ((chars::_a, chars::_1),"a1") = (strs::a1::NEW,ts!("a1"),ts!(a1));

            ((chars::_f, chars::_o, chars::_o),"foo") = (strs::m2::foo::NEW,ts!("foo"),ts!(foo));
            ((chars::_0,),"0") = (strs::m2::bar::NEW,ts!("0"),ts!(0));
            ((chars::_b, chars::_a, chars::_a, chars::_a, chars::_a),"baaaa") =
                (strs::m2::baz::NEW,ts!("baaaa"),ts!(baaaa));

            ((chars::_0,),"0") = (strs::m3::__TString_Aliases_Count::NEW,ts!("0"),ts!(0));
            ((chars::_2,),"2") = (strs::m3::m3m0::__TString_Aliases_Count::NEW,ts!("2"),ts!(2));
            ((chars::_a, chars::_a, chars::_a),"aaa") = (strs::m3::m3m0::aaa::NEW,ts!("aaa"),ts!(aaa));
            ((chars::_b, chars::_b, chars::_b),"bbb") = (strs::m3::m3m0::bbb::NEW,ts!("bbb"),ts!(bbb));
        }
    }
}
