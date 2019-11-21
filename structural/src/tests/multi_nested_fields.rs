use crate::*;
use crate::type_level::{AliasedPaths,UniquePaths,FieldPath,FieldPathSet};

use core_extensions::SelfOps;


#[test]
fn boxed_fields() {
    let mut f = Box::new((0, 1, Box::new((20, 21)), 3));
    let (f_0, f_1, f_2_0, f_2_1, f_3) = f.fields_mut(fp!(0, 1, 2.0, 2.1, 3));

    *f_0 = 0;
    *f_1 = 0;
    *f_2_0 = 0;
    *f_2_1 = 0;
    *f_3 = 0;

    *f_0 = 5;
    *f_1 = 6;
    *f_2_0 = 7;
    *f_2_1 = 8;
    *f_3 = 9;

    assert_eq!(f.0, 5);
    assert_eq!(f.1, 6);
    assert_eq!((f.2).0, 7);
    assert_eq!((f.2).1, 8);
    assert_eq!(f.3, 9);
}

fn wrap_single<T>(value:T)->(T,){
    (value,) 
}

#[test]
fn deeply_nested(){

    {
        let mut f=make_struct!{
            a:make_struct!{
                aa:(101,103),
                ab:"hello",
            },
            b:false,
        };

        let (f_aa_0,f_aa_1,f_ab,f_b)=f.fields_mut(fp!( a.aa.0, a.aa.1, a.ab, b ));
        assert_eq!(f_aa_0 , &mut 101);
        assert_eq!(f_aa_1, &mut 103);
        assert_eq!(f_ab, &mut "hello");
        *f_aa_0*=3;
        *f_aa_1*=2;
        *f_ab="shoot";
        *f_b=true;
        assert_eq!(f_aa_0 , &mut 303);
        assert_eq!(f_aa_1, &mut 206);
        assert_eq!(f_ab, &mut "shoot");
        assert_eq!(f_b, &mut true);
        
        assert_eq!(f.a.aa.0, 303);
        assert_eq!(f.a.aa.1, 206);
        assert_eq!(f.a.ab, "shoot");
        assert_eq!(f.b, true);
    }

    {
        let mut this=10
            .piped(wrap_single)
            .piped(wrap_single)
            .piped(wrap_single)
            .piped(wrap_single);

        assert_eq!( (((this.0).0).0).0, 10 );
        let num=this.field_mut(fp!(0.0.0.0));
        *num*=2;
        assert_eq!( (((this.0).0).0).0, 20 );
    }


}

#[test]
fn identity_getters(){
    let mut this=Box::new((0,1));
    let ()=this.fields_mut(fp!());
    /*
    {
        let other=this.field_mut(fp!(()));
        *other=Default::default();
        assert_eq!(this, Default::default());
    }

    {
        let _:FieldPathSet<(),UniquePaths>=
            fp!();

        let _:FieldPath<()>=
            fp!(());

        let _:FieldPathSet<(FieldPath<()>,FieldPath<()>),AliasedPaths>=
            fp!((),());

        let _:FieldPathSet<(FieldPath<()>,FieldPath<()>,FieldPath<()>),AliasedPaths>=
            fp!((),(),());
    }
    */
}

