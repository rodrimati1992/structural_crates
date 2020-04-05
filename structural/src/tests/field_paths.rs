#[cfg(all(feature = "use_const_str", not(feature = "disable_const_str")))]
macro_rules! cond_tstr_alias {
    ( $name:ident=($with_type:ty, $with_const:literal) ) => {
        pub type $name = crate::__TStrPriv<$with_const>;
    };
}
#[cfg(any(not(feature = "use_const_str"), feature = "disable_const_str"))]
macro_rules! cond_tstr_alias {
    ( $name:ident=($with_type:ty, $with_const:literal) ) => {
        pub type $name = crate::__TStrPriv<$with_type>;
    };
}

#[cfg(all(feature = "use_const_str", not(feature = "disable_const_str")))]
macro_rules! tstr_asserts{
    ( $(($with_type:ty, $with_const:literal)=($($found:expr),*);)* ) => {
        $(
            $(
                let _: crate::__TStrPriv<$with_const> = $found;
            )*
        )*
    };
}
#[cfg(any(not(feature = "use_const_str"), feature = "disable_const_str"))]
macro_rules! tstr_asserts{
    ( $(($with_type:ty, $with_const:literal)=($($found:expr),*);)* ) => {
        $(
            $(
                let _: crate::__TStrPriv<$with_type> = $found;
            )*
        )*
    };
}

#[allow(clippy::wildcard_imports)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[macro_use]
mod for_string_tests {
    #[cfg(any(not(feature = "use_const_str"), feature = "disable_const_str"))]
    use crate::chars::*;

    cond_tstr_alias!(S_foo = ((__f, __o, __o), "foo"));
    cond_tstr_alias!(S_bar = ((__b, __a, __r), "bar"));
    cond_tstr_alias!(S_baz = ((__b, __a, __z), "baz"));
    cond_tstr_alias!(S_qux = ((__q, __u, __x), "qux"));
    cond_tstr_alias!(S_Some = ((__S, __o, __m, __e), "Some"));
    cond_tstr_alias!(S_a = ((__a,), "a"));
    cond_tstr_alias!(S_b = ((__b,), "b"));
    cond_tstr_alias!(S_c = ((__c,), "c"));
    cond_tstr_alias!(S_d = ((__d,), "d"));
    cond_tstr_alias!(S_0 = ((__0,), "0"));
    cond_tstr_alias!(S_1 = ((__1,), "1"));
    cond_tstr_alias!(S_2 = ((__2,), "2"));
    cond_tstr_alias!(S_3 = ((__3,), "3"));
    cond_tstr_alias!(S_4 = ((__4,), "4"));

    pub fn assert_ty<T, U>(_ident: U)
    where
        T: core_extensions::TypeIdentity<Type = U>,
    {
    }

    macro_rules! path_assertion {
        (
            fp!$fp_params_tt:tt,
            fp!$fp_params_rhs:tt
            $(,$($rest:tt)*)?
        )=>{
            assert_ty::< FP!$fp_params_tt, FP!$fp_params_rhs >(fp!$fp_params_tt);
            let _:FP!$fp_params_rhs=fp!$fp_params_rhs;

            $( path_assertion!{fp!$fp_params_tt,  $($rest)*} )?
        };
        ( fp!$fp_params_tt:tt ,$ty:ty $(,$($rest:tt)*)? )=>{
            assert_ty::<$ty,FP!$fp_params_tt>(fp!$fp_params_tt);

            $( path_assertion!{fp!$fp_params_tt,  $($rest)*} )?
        };
        (fp! $fp_params_tt:tt $(,)* )=>{};
    }
}

/// Tests that the fp and FP macros produce the correct TStr and NestedFieldPath types
#[allow(clippy::wildcard_imports)]
#[allow(non_camel_case_types)]
#[test]
fn field_path_nested() {
    use self::for_string_tests::{assert_ty, S_bar, S_foo, S_0, S_1, S_2, S_3, S_4};
    #[cfg(any(not(feature = "use_const_str"), feature = "disable_const_str"))]
    use crate::chars::*;
    use crate::NestedFieldPath;

    cond_tstr_alias!(S_abcd = ((__a, __b, __c, __d), "abcd"));
    cond_tstr_alias!(S_21 = ((__2, __1), "21"));
    cond_tstr_alias!(S_ab0 = ((__a, __b, __0), "ab0"));

    path_assertion!(fp!(abcd), S_abcd, fp!("abcd"));
    path_assertion!(fp!(0), S_0, fp!("0"));
    path_assertion!(fp!(21), S_21, fp!("21"));
    path_assertion!(fp!(ab0), S_ab0, fp!("ab0"));
    path_assertion!(fp!(0.1), NestedFieldPath<(S_0, S_1)>, fp!("0"."1"));
    // 0.1 and 0 .1 are tokenized differently
    path_assertion!(
        fp!(0.1),
        NestedFieldPath<(S_0, S_1)>,
        fp!("0"."1"),
        fp!(0.1)
    );
    // `0.1.2`,`0.1 .2`,`0 .1.2` and `0 .1 .2` are tokenized differently
    #[rustfmt::skip]
    path_assertion!(
        fp!(0.1.2),
        NestedFieldPath<(S_0, S_1, S_2)>,
        fp!(0 .1 .2),
        fp!(0 .1.2),
        fp!(0.1 .2),
        fp!("0"."1"."2")
    );
    path_assertion!(fp!(0.1.2.3), NestedFieldPath<(S_0, S_1, S_2, S_3)>);
    path_assertion!(fp!(0.1.2.3.4), NestedFieldPath<(S_0, S_1, S_2, S_3, S_4)>);
    path_assertion!(fp!(0.foo), NestedFieldPath<(S_0, S_foo)>, fp!("0"."foo"));
    path_assertion!(
        fp!(0.foo.1),
        NestedFieldPath<(S_0, S_foo, S_1)>,
        fp!("0"."foo"."1")
    );
    path_assertion!(
        fp!(0.foo.1.bar),
        NestedFieldPath<(S_0, S_foo, S_1, S_bar)>,
        fp!("0"."foo"."1"."bar"),
    );
}

#[allow(non_camel_case_types)]
#[test]
fn field_paths_more() {
    use crate::path::{AliasedPaths, UniquePaths};
    use crate::{FieldPathSet, NestedFieldPath, VariantField, VariantName};

    use self::for_string_tests::{
        assert_ty, S_Some, S_a, S_b, S_bar, S_baz, S_foo, S_qux, S_0, S_1, S_2, S_3, S_4,
    };

    path_assertion! {
        fp!(foo,bar),
        FieldPathSet<(S_foo,S_bar,),UniquePaths>
    }

    path_assertion! {
        fp!(foo.bar,baz),
        FieldPathSet<(NestedFieldPath<(S_foo,S_bar)>,S_baz,),UniquePaths>,
    }

    path_assertion! {
        fp!(foo.bar,a.b),
        FieldPathSet<(NestedFieldPath<(S_foo,S_bar)>,NestedFieldPath<(S_a,S_b)>,),UniquePaths>,
    }

    path_assertion! {
        fp!(0,foo),
        FieldPathSet<(S_0,S_foo),UniquePaths>,
    }

    path_assertion! {
        fp!(0.1,foo),
        FieldPathSet<(NestedFieldPath<(S_0,S_1)>,S_foo),UniquePaths>,
    }
    path_assertion! {
        fp!(0.1.2,foo),
        FieldPathSet<(NestedFieldPath<(S_0,S_1,S_2)>,S_foo),UniquePaths>,
    }
    path_assertion! {
        fp!(foo,0.1.2.3),
        FieldPathSet<(S_foo,NestedFieldPath<(S_0,S_1,S_2,S_3)>),UniquePaths>,
    }
    path_assertion! {
        fp!(0.1.2.3.4,foo),
        FieldPathSet<(NestedFieldPath<(S_0,S_1,S_2,S_3,S_4)>,S_foo),UniquePaths>,
    }

    path_assertion! {
        fp!(0,::0),
        FieldPathSet<(S_0, VariantName<S_0>),AliasedPaths>,
    }

    path_assertion! {
        fp!(0.1, ::0.1),
        FieldPathSet<(NestedFieldPath<(S_0,S_1)>, VariantField<S_0,S_1>),AliasedPaths>,
    }
    path_assertion! {
        fp!(::0.1, 0.1),
        FieldPathSet<(VariantField<S_0,S_1>, NestedFieldPath<(S_0,S_1)>),AliasedPaths>,
    }

    {
        type NFP_0_1 = NestedFieldPath<(S_0, S_1)>;
        type NFP_0_1_2 = NestedFieldPath<(S_0, S_1, S_2)>;
        type NFP_0_1_2_3 = NestedFieldPath<(S_0, S_1, S_2, S_3)>;
        type VF_0_1_2 = NestedFieldPath<(VariantField<S_0, S_1>, S_2)>;

        path_assertion! { fp!(::0.1.2, 0.1), FieldPathSet<(VF_0_1_2, NFP_0_1), AliasedPaths> }
        path_assertion! { fp!(0.1, ::0.1.2), FieldPathSet<(NFP_0_1, VF_0_1_2), AliasedPaths> }
        path_assertion! {
            fp!(0.1.2, ::0.1.2),
            FieldPathSet<(NFP_0_1_2, VF_0_1_2), AliasedPaths>
        }
        path_assertion! {
            fp!(0.1.2.3, ::0.1.2),
            FieldPathSet<(NFP_0_1_2_3, VF_0_1_2), AliasedPaths>
        }
    }
    type VF_foo_bar = VariantField<S_foo, S_bar>;
    type VF_foo_bar_baz = NestedFieldPath<(VariantField<S_foo, S_bar>, S_baz)>;

    type NFP_foo_qux = NestedFieldPath<(S_foo, S_qux)>;
    type NFP_foo_bar = NestedFieldPath<(S_foo, S_bar)>;
    type NFP_foo_bar_baz = NestedFieldPath<(S_foo, S_bar, S_baz)>;
    type NFP_foo_bar_baz_qux = NestedFieldPath<(S_foo, S_bar, S_baz, S_qux)>;
    // pairs of single-field field paths
    {
        // FieldPathSet<_,UniquePaths> go here
        path_assertion! {
            fp!(foo.qux, ::foo.bar),
            FieldPathSet<(NFP_foo_qux, VF_foo_bar), UniquePaths>,
        }

        // FieldPathSet<_,AliasedPaths> go here
        path_assertion! {
            fp!(foo, ::foo),
            fp!(foo, ::"foo"),
            fp!("foo", ::foo),
            FieldPathSet<(S_foo, VariantName<S_foo>), AliasedPaths>,
        }
        path_assertion! {
            fp!(::foo, foo),
            fp!(::foo, "foo"),
            fp!(::"foo", foo),
            FieldPathSet<(VariantName<S_foo>, S_foo), AliasedPaths>,
        }
        path_assertion! {
            fp!(::foo.bar, foo),
            fp!(::foo.bar, "foo"),
            fp!(::"foo".bar, foo),
            FieldPathSet<(VF_foo_bar, S_foo), AliasedPaths>,
        }
        path_assertion! {
            fp!(foo, ::foo.bar),
            fp!(foo, ::"foo".bar),
            fp!("foo", ::foo.bar),
            FieldPathSet<(S_foo, VF_foo_bar), AliasedPaths>,
        }
        path_assertion! {
            fp!(foo.bar, ::foo.bar),
            FieldPathSet<(NFP_foo_bar, VF_foo_bar), AliasedPaths>,
        }
        path_assertion! {
            fp!(::foo.bar, foo.bar),
            FieldPathSet<(VF_foo_bar, NFP_foo_bar), AliasedPaths>,
        }
        path_assertion! {
            fp!(::foo.bar.baz, foo.bar),
            FieldPathSet<(VF_foo_bar_baz, NFP_foo_bar), AliasedPaths>,
        }
        path_assertion! {
            fp!(foo.bar, ::foo.bar.baz),
            FieldPathSet<(NFP_foo_bar, VF_foo_bar_baz), AliasedPaths>,
        }
        path_assertion! {
            fp!(foo.bar.baz, ::foo.bar.baz),
            FieldPathSet<(NFP_foo_bar_baz, VF_foo_bar_baz), AliasedPaths>,
        }
        path_assertion! {
            fp!(foo.bar.baz.qux, ::foo.bar.baz),
            FieldPathSet<(NFP_foo_bar_baz_qux, VF_foo_bar_baz), AliasedPaths>,
        }
    }
    {
        path_assertion! {
            fp!(::foo, foo, ::foo.bar),
            fp!(::foo, foo, ::"foo".bar),
            fp!(::foo, "foo", ::foo.bar),
            fp!(::"foo", foo, ::foo.bar),
            FieldPathSet<(VariantName<S_foo>, S_foo, VF_foo_bar), AliasedPaths>,
        }
        path_assertion! {
            fp!(::foo.bar, ::foo, foo),
            FieldPathSet<(VF_foo_bar, VariantName<S_foo>, S_foo), AliasedPaths>,
        }
    }
    {
        type VF_Some_0 = VariantField<S_Some, S_0>;
        type FP_a_Some_0 = NestedFieldPath<(S_a, VariantField<S_Some, S_0>)>;

        path_assertion! {
            fp!(::Some.0, ?),
            FieldPathSet<(VF_Some_0, VF_Some_0), AliasedPaths>,
        }

        path_assertion! {
            fp!(::Some.0, a?),
            FieldPathSet<(VF_Some_0, FP_a_Some_0), UniquePaths>,
        }
    }

    path_assertion! {
        fp!(0.1,::0.2),
        FieldPathSet<(NestedFieldPath<(S_0,S_1)>, VariantField<S_0,S_2>),UniquePaths>,
    }
}

mod names_module_tests {
    use core_extensions::type_asserts::AssertEq3;

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

#[allow(clippy::wildcard_imports)]
mod tstr_aliases_tests {
    #[cfg(any(not(feature = "use_const_str"), feature = "disable_const_str"))]
    use crate::chars::*;

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
            ((__a,),"a") = (strs::a::NEW,ts!("a"),ts!(a),ts!(r#a));
            ((__b,),"b") = (strs::b::NEW,ts!("b"),ts!(b),ts!(r#b));
            ((__w, __o, __r, __d),"word") =
                (strs::word::NEW,ts!("word"),ts!(word),ts!(r#word));
            ((__d, __d),"dd") = (strs::d::NEW,ts!("dd"),ts!(dd),ts!(r#dd));
            ((__c, __c),"cc") = (strs::c::NEW,ts!("cc"),ts!(cc),ts!(r#cc));
            ((__0,),"0") = (strs::p0::NEW,ts!("0"),ts!(0));
            ((__1, __0),"10") = (strs::p10::NEW,ts!("10"),ts!(10));
            ((__1, __0, __0),"100") = (strs::p100::NEW,ts!("100"),ts!(100));
        }
    }

    #[test]
    fn escaped() {
        tstr_asserts! {
            (
                (__0x00, __0x5C, __0xF0, __0x9F, __0x99, __0x82),
                "\0\\🙂"
            ) = (ts!("\0\\🙂"));
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
            ((__0,),"0") = (strs::m0::__TString_Aliases_Count::NEW,ts!("0"),ts!(0));

            ((__1, __0),"10") = (strs::a0::NEW,ts!("10"),ts!(10));

            ((__2,),"2") = (strs::m1::__TString_Aliases_Count::NEW,ts!("2"),ts!(2));
            ((__1, __1),"11") = (strs::m1::a0::NEW,ts!("11"),ts!(11));
            ((__a, __1),"a1") = (strs::m1::a1::NEW,ts!("a1"),ts!(a1));

            ((__a, __1),"a1") = (strs::a1::NEW,ts!("a1"),ts!(a1));

            ((__f, __o, __o),"foo") = (strs::m2::foo::NEW,ts!("foo"),ts!(foo));
            ((__0,),"0") = (strs::m2::bar::NEW,ts!("0"),ts!(0));
            ((__b, __a, __a, __a, __a),"baaaa") =
                (strs::m2::baz::NEW,ts!("baaaa"),ts!(baaaa));

            ((__0,),"0") = (strs::m3::__TString_Aliases_Count::NEW,ts!("0"),ts!(0));
            ((__2,),"2") = (strs::m3::m3m0::__TString_Aliases_Count::NEW,ts!("2"),ts!(2));
            ((__a, __a, __a),"aaa") =
                (strs::m3::m3m0::aaa::NEW,ts!("aaa"),ts!(aaa));

            ((__b, __b, __b),"bbb") =
                (strs::m3::m3m0::bbb::NEW,ts!("bbb"),ts!(bbb));
        }
    }
}
