use core_extensions::SelfOps;

use structural::{
    for_examples::NewtypeEnum, fp, reexports::const_default, structural_aliases::Array4,
    StrucWrapper, StructuralExt,
};

use std::mem;

fn access(this: impl Array4<u8> + Copy) {
    assert_eq!(this.field_(fp!(0)), &3);
    assert_eq!(this.field_(fp!(1)), &5);
    assert_eq!(this.field_(fp!(2)), &8);
    assert_eq!(this.field_(fp!(3)), &13);
    assert_eq!(this.fields(fp!(0, 1, 2, 3)), (&3, &5, &8, &13));

    {
        let mut this = this;

        assert_eq!(this.field_mut(fp!(0)), &mut 3);
        assert_eq!(this.field_mut(fp!(1)), &mut 5);
        assert_eq!(this.field_mut(fp!(2)), &mut 8);
        assert_eq!(this.field_mut(fp!(3)), &mut 13);

        assert_eq!(
            this.fields_mut(fp!(0, 1, 2, 3)),
            (&mut 3, &mut 5, &mut 8, &mut 13)
        );

        let (f0, f1, f2, f3) = this.fields_mut(fp!(0, 1, 2, 3));
        mem::swap(f0, f1);
        mem::swap(f2, f3);
        assert_eq!((f0, f1, f2, f3), (&mut 5, &mut 3, &mut 13, &mut 8));
    }

    assert_eq!(this.into_field(fp!(0)), 3);
    assert_eq!(this.into_field(fp!(1)), 5);
    assert_eq!(this.into_field(fp!(2)), 8);
    assert_eq!(this.into_field(fp!(3)), 13);
    assert_eq!(this.into_fields(fp!(0, 1, 2, 3)), (3, 5, 8, 13));

    assert_eq!(this.cloned_fields(fp!(0, 1, 2, 3)), (3, 5, 8, 13));
}

#[test]
fn wrapper_access() {
    access((3, 5, 8, 13));

    {
        let this = StrucWrapper(NewtypeEnum::Some((3, 5, 8, 13)));
        assert!(this.is_variant(fp!(Some)));
        assert!(!this.is_variant(fp!(None)));
        this.then(|x| x.into_field(fp!(::Some)).unwrap())
            .piped(access);
    }
}

#[test]
fn struc_wrapper_mapping() {
    let param = (3, 5, 8, (13, 21));
    let this = StrucWrapper(param.clone());
    {
        assert_eq!(this.map(|x| x.2), StrucWrapper(8));
        assert_eq!(this.map(|x| x.3), StrucWrapper((13, 21)));
    }
    {
        assert_eq!(this.then(|x| x.into_field(fp!(0))), StrucWrapper(3));
        assert_eq!(this.then(|x| x.into_field(fp!(1))), StrucWrapper(5));
        assert_eq!(this.then(|x| x.into_field(fp!(2))), StrucWrapper(8));
        assert_eq!(this.then(|x| x.into_field(fp!(3))), StrucWrapper((13, 21)));
    }
    {
        let this = this.as_ref();
        assert_eq!(this, StrucWrapper(&param.clone()));
        assert_eq!(this.cloned(), StrucWrapper(param.clone()));
        assert_eq!(this.reref(), &StrucWrapper(param.clone()));
    }
    {
        let mut this = this.clone();
        assert_eq!(this.as_mut(), StrucWrapper(&mut param.clone()));
        assert_eq!(this.as_mut().cloned(), StrucWrapper(param.clone()));
        assert_eq!(this.as_mut().remut(), &mut StrucWrapper(param.clone()));
    }
}

#[test]
fn struc_wrapper_impls() {
    // FromStructural tests in from_structural module.

    {
        let mut this = StrucWrapper((3, 5, 8, 13));
        assert_eq!(&this[fp!(0)], &3);
        assert_eq!(&this[fp!(1)], &5);
        assert_eq!(&this[fp!(2)], &8);
        assert_eq!(&this[fp!(3)], &13);
        assert_eq!(&mut this[fp!(0)], &mut 3);
        assert_eq!(&mut this[fp!(1)], &mut 5);
        assert_eq!(&mut this[fp!(2)], &mut 8);
        assert_eq!(&mut this[fp!(3)], &mut 13);
    }

    {
        assert_eq!(
            const_default!(StrucWrapper<Option<u32>>),
            StrucWrapper(None)
        );
        assert_eq!(
            const_default!(StrucWrapper<[u32; 4]>),
            StrucWrapper([0u32; 4])
        );
    }
}
