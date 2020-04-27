mod param_ret {
    use structural::{fp, impl_struct, make_struct, StructuralExt};

    fn hi(blah: impl_struct! {a:u32,b:u32}) -> impl_struct! {a:u32,b:u64} {
        make_struct! {
            a:*blah.field_(fp!(a))/2,
            b:*blah.field_(fp!(b)) as u64 * 2,
        }
    }

    #[test]
    fn param_test() {
        let ret = hi(make_struct! {a:25,b:100});
        assert_eq!(ret.fields(fp!(a, b)), (&12, &200));
    }
}

mod mutabilities {
    use structural::{fp, impl_struct, StructuralExt};

    #[derive(structural::Structural, Clone)]
    struct Mutabilities0 {
        #[struc(access = "ref")]
        a: u32,
        #[struc(access = "mut")]
        b: u32,
        #[struc(access = "move")]
        c: u32,
        #[struc(access = "mut move")]
        d: u32,
        #[struc(public)]
        e: u32,
    }

    fn hi(mut this: impl_struct! {Clone;ref a:u32,mut b:u32,move c:u32,mut move d:u32,e:u32}) {
        assert_eq!(this.field_(fp!(a)), &0);

        assert_eq!(this.field_mut(fp!(b)), &1);

        assert_eq!(this.clone().into_field(fp!(c)), 2);

        assert_eq!(this.clone().into_field(fp!(d)), 3);
        assert_eq!(this.clone().field_mut(fp!(d)), &mut 3);

        assert_eq!(this.clone().into_field(fp!(e)), 4);
        assert_eq!(this.clone().field_mut(fp!(e)), &mut 4);
    }

    #[test]
    fn mutabilities() {
        hi(Mutabilities0 {
            a: 0,
            b: 1,
            c: 2,
            d: 3,
            e: 4,
        });
    }
}
