use crate::{
    enums::VariantProxy,
    field_path_aliases,
    test_utils::{GetRefKind, RefKind},
    Structural, StructuralExt,
};

use core_extensions::SelfOps;

use std_::fmt::Debug;

#[derive(Debug, Structural, Copy, Clone)]
enum Foo {
    Var0(u32, u64),
    Var1 { order: u8, is_true: i8 },
    Var2,
}

field_path_aliases! {
    mod paths{
        f0=0,
        f1=1,
        order,
        is_true,
        f_var0=(0,1),
        f_var1=(order,is_true),
        Var0,
        Var1,
        Var2,
    }
}

macro_rules! ne_move_test_0 {
    (
        this=$this:ident,
        $($access:ident)*
    ) => ({
        #[allow(unused_variables)]
        let variant_index=switch!{ $($access)* self_=$this;
            Var0=>{
                assert_eq!(self_.fields(paths::f_var0), (&5,&8));
                assert_eq!(self_.fields_mut(paths::f_var0), (&mut 5,&mut 8));
                assert_eq!(self_.cloned_fields(paths::f_var0), (5,8));
                assert_eq!(self_.into_field(paths::f0), 5);
                assert_eq!(self_.into_field(paths::f1), 8);
                0
            }
            Var1=>self_
                .cloned_fields(paths::f_var1)
                .piped(|(order,is_true)|{
                    assert_eq!((order,is_true), (13,21));
                    1
                }),
            Var2{}=>{2},
            _=>{3}
        };

        assert!(
            $this.is_variant(paths::Var0)&&variant_index==0||
            $this.is_variant(paths::Var1)&&variant_index==1||
            $this.is_variant(paths::Var2)&&variant_index==2||
            variant_index==3,
            "variant_index:{} $this:{:?}",
            variant_index,$this
        );
    })
}

#[test]
fn exhaustive_switch() {
    fn destructuring<T>(this: T)
    where
        T: Foo_ESI + Copy + Debug,
    {
        #[allow(unused_parens)]
        let variant_index = switch! { ref this;
            Var0((mut x0),&x1)=>{
                let _:u32=*x0;
                let _:u64=x1;
                assert_eq!(*x0, 5);
                assert_eq!(x1, 8);

                x0=&99; //Replaces the `x0:&u32` binding
                assert_eq!(*x0, 99);

                0
            }
            Var1{order,&is_true}=>({
                assert_eq!((order,is_true), (&13,21));
                1
            }),
            Var2{}=>{2},
            _=>{
                unreachable!()
            }
        };

        assert!(
            this.is_variant(paths::Var0) && variant_index == 0
                || this.is_variant(paths::Var1) && variant_index == 1
                || this.is_variant(paths::Var2) && variant_index == 2,
            "variant_index:{} $this:{:?}",
            variant_index,
            this
        );
    }

    fn proxy_getters<T>(mut this: T)
    where
        T: Foo_ESI + Copy + Debug,
    {
        ne_move_test_0! { this=this, }
        ne_move_test_0! { this=this,move }

        switch! { this;
            ref Var0=>{ let _:&VariantProxy<T,_>=this; }
            Var1=>{}
            Var2=>{}
        }

        #[allow(unused_variables)]
        #[allow(unused_parens)]
        {
            switch! { ref self_ = &this;
                Var0=>{
                    let _:&VariantProxy<T,_>=self_;
                    assert_eq!(self_.field_(paths::f0), &5);
                    assert_eq!(self_.field_(paths::f1), &8);
                }
                Var1=>{}
                Var2=>{}
            }
            switch! { self_ = this;
                ref Var0=>{ let _:&VariantProxy<T,_>=self_; }
                Var1=>{}
                Var2=>{}
            }
            switch! { ref mut self_ = this;
                Var0=>{
                    let _:&mut VariantProxy<T,paths::Var0>=self_;
                    assert_eq!(self_.field_mut(paths::f0), &mut 5);
                    assert_eq!(self_.field_mut(paths::f1), &mut 8);
                }
                Var1=>{
                    let _:&mut VariantProxy<T,paths::Var1>=self_;
                }
                Var2=>{
                    let _:&mut VariantProxy<T,paths::Var2>=self_;
                }
            }
            switch! { self_ = this;
                ref mut Var0=>{ let _:&mut VariantProxy<T,_>=self_; }
                Var1=>{}
                Var2=>{}
            }
        }
        switch! { this;
            ref mut Var0=>{ let _:&mut VariantProxy<T,paths::Var0>=this; }
            Var1=>{
                let _:VariantProxy<T,paths::Var1>=this;
            }
            Var2=>{
                let _:VariantProxy<T,paths::Var2>=this;
            }
        }
    }

    let run_both = |this: Foo| {
        destructuring(this);
        proxy_getters(this);
    };

    run_both(Foo::Var0(5, 8));
    run_both(Foo::Var1 {
        order: 13,
        is_true: 21,
    });
    run_both(Foo::Var2);
}

#[test]
fn nonexhaustive_switch() {
    fn generic<T>(this: T)
    where
        T: Foo_SI + Copy + Debug,
    {
        switch! { this ;
            Var0=>{
                let _:VariantProxy<T,_>=this;
                assert_eq!(this.into_field(paths::f0), 5);
                assert_eq!(this.into_field(paths::f1), 8);
            }
            Var1=>{
                let _:VariantProxy<T,_>=this;
                assert_eq!(this.into_field(paths::order), 13);
                assert_eq!(this.into_field(paths::is_true), 21);
            }
            Var2=>{
                let _:VariantProxy<T,_>=this;
            }
            _=>{
                let _:T=this;
            }
        }
    }
    let run_all = |this: Foo| {
        generic(this);
    };

    run_all(Foo::Var0(5, 8));
    run_all(Foo::Var1 {
        order: 13,
        is_true: 21,
    });
    run_all(Foo::Var2);
}

#[derive(Structural, Copy, Clone)]
enum WithT<T> {
    Var0(T, ()),
}

#[test]
fn irrefutable_patterns() {
    switch! { ref WithT::Var0((0,1,2,3),());
        Var0((a,..,d),())=>{
            assert_eq!(a, &0);
            assert_eq!(d, &3);
        }
    }
    switch! { ref WithT::Var0([0,1,2,3],());
        Var0([a,b,c,d],())=>{
            assert_eq!(a, &0);
            assert_eq!(b, &1);
            assert_eq!(c, &2);
            assert_eq!(d, &3);
        }
    }
    switch! { ref mut WithT::Var0([0,1,2,3],());
        Var0([a,b,c,d],())=>{
            assert_eq!(a, &mut 0);
            assert_eq!(b, &mut 1);
            assert_eq!(c, &mut 2);
            assert_eq!(d, &mut 3);
        }
    }
}

#[test]
fn access_types() {
    {
        let mut this = Foo::Var0(5, 8);
        let x = switch! { this;
            ref Var0=>{
                assert_eq!(this.get_ref_kind(),RefKind::Shared);
                *this.field_(paths::f0) as u64
            },
            ref mut Var1=>{
                assert_eq!(this.get_ref_kind(),RefKind::Mutable);
                *this.field_mut(paths::is_true) as u64
            },
            move Var2=>unreachable!(),
        };
        assert_eq!(x, 5);
    }
    {
        let mut this = Foo::Var1 {
            order: 13,
            is_true: 21,
        };
        let x = switch! { ref this;
            ref Var0=>{
                assert_eq!(this.get_ref_kind(),RefKind::Shared);
                *this.field_(paths::f0) as u64
            },
            ref mut Var1=>{
                assert_eq!(this.get_ref_kind(),RefKind::Mutable);
                *this.field_mut(paths::is_true) as u64
            },
            move Var2=>unreachable!(),
        };
        assert_eq!(x, 21);
    }
    {
        let mut this = Foo::Var1 {
            order: 13,
            is_true: 21,
        };
        let x = switch! { ref mut this;
            ref mut Var0=>{
                assert_eq!(this.get_ref_kind(),RefKind::Mutable);
                *this.field_mut(paths::f0) as u64
            },
            move Var1=>this.into_field(paths::order) as u64,
            ref Var2=>unreachable!(),
        };
        assert_eq!(x, 13);
    }
}

// I know that these have redundant pattern matches
#[allow(clippy::redundant_pattern_matching)]
#[test]
fn extra_branch_types() {
    {
        let x = switch! {();
            if false=>{unreachable!()}
            if true=>11,
            _=>unreachable!()
        };
        assert_eq!(x, 11);
    }
    {
        let x = switch! {();
            _ if false=>{unreachable!()}
            _ if true=>22,
            _=>unreachable!()
        };
        assert_eq!(x, 22);
    }
    {
        let a = switch! {();
            if let None=Some(33) =>{unreachable!()}
            if let Some(x)=Some(33) =>x,
            _=>{unreachable!()}
        };
        assert_eq!(a, 33);
    }
    {
        let a = switch! {();
            _ if let Some(0xAAAA)=Some(44) =>44,
            _ if let None=Some(44) =>{unreachable!()}
            _=>44
        };
        assert_eq!(a, 44);
    }
}
