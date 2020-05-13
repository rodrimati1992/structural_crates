use structural::{
    for_examples::NewtypeEnum, fp, reexports::ConstDefault, structural_alias, FieldCloner,
    StructuralExt,
};

use std::{cmp::PartialEq, fmt::Debug};

structural_alias! {
    trait Array4Ref<T>{
        ref 0: T,
        ref 1: T,
        ref 2: T,
        ref 3: T,
    }
    trait Array4Mut<T>{
        mut 0: T,
        mut 1: T,
        mut 2: T,
        mut 3: T,
    }
}

fn for_access_test_ref<T>(params: [FieldCloner<impl Array4Ref<T>>; 2])
where
    T: From<u8> + Clone + PartialEq + Debug,
{
    let [this, other] = params;

    let n5 = T::from(5);
    let n8 = T::from(8);

    assert_eq!(this.field_(fp!(2)), &n8);
    assert_eq!(this.as_ref().into_field(fp!(2)), n8.clone());
    assert_eq!(other.into_field(fp!(2)), n8.clone());

    assert_eq!(this.fields(fp!(1, 2)), (&n5, &n8));
    assert_eq!(this.cloned_fields(fp!(1, 2)), (n5.clone(), n8.clone()));
    assert_eq!(
        this.as_ref().into_fields(fp!(1, 2)),
        (n5.clone(), n8.clone())
    );
    assert_eq!(this.into_fields(fp!(1, 2)), (n5.clone(), n8.clone()));
}

fn for_access_test_mut<T>(param: [FieldCloner<impl Array4Mut<T>>; 3])
where
    T: From<u8> + Clone + PartialEq + Debug,
{
    let n5 = T::from(5);
    let n8 = T::from(8);

    let [mut this, other, third] = param;
    assert_eq!(this.field_mut(fp!(2)), &mut n8.clone());
    assert_eq!(
        this.fields_mut(fp!(1, 2)),
        (&mut n5.clone(), &mut n8.clone())
    );

    for_access_test_ref([other, third]);
}

#[test]
fn field_cloner_access() {
    {
        let this = FieldCloner((3, 5, 8, 13));
        for_access_test_ref([this.clone().as_ref(); 2]);
        for_access_test_mut([this.clone(); 3]);
        for_access_test_mut([
            this.clone().as_mut(),
            this.clone().as_mut(),
            this.clone().as_mut(),
        ]);
    }
    {
        let this = FieldCloner(NewtypeEnum::Some((3, 5, 8, 13)));
        assert!(this.is_variant(fp!(Some)));
        assert!(!this.is_variant(fp!(None)));
        let proxy = this.then(|x| x.into_field(fp!(::Some)).unwrap());
        for_access_test_ref([proxy.as_ref(); 2]);
        for_access_test_mut([proxy.clone(); 3]);
        for_access_test_mut([
            proxy.clone().as_mut(),
            proxy.clone().as_mut(),
            proxy.clone().as_mut(),
        ]);
    }
}

#[test]
fn field_cloner_mapping() {
    let param = (3, 5, 8, (13, 21));
    let this = FieldCloner(param.clone());
    {
        assert_eq!(this.map(|x| x.2), FieldCloner(8));
        assert_eq!(this.map(|x| x.3), FieldCloner((13, 21)));
    }
    {
        assert_eq!(this.then(|x| x.into_field(fp!(0))), FieldCloner(3));
        assert_eq!(this.then(|x| x.into_field(fp!(1))), FieldCloner(5));
        assert_eq!(this.then(|x| x.into_field(fp!(2))), FieldCloner(8));
        assert_eq!(this.then(|x| x.into_field(fp!(3))), FieldCloner((13, 21)));
    }
    {
        let this = this.as_ref();
        assert_eq!(this, FieldCloner(&param.clone()));

        assert_eq!(this.field_(fp!(2)), &8);
        assert_eq!(this.into_field(fp!(2)), 8.clone());

        assert_eq!(this.fields(fp!(1, 2)), (&5, &8));
        assert_eq!(this.cloned_fields(fp!(1, 2)), (5, 8));
        assert_eq!(this.into_fields(fp!(1, 2)), (5, 8));
    }
    {
        let mut this = this.clone();
        assert_eq!(this.as_mut(), FieldCloner(&mut param.clone()));
    }
}

#[test]
fn field_cloner_impls() {
    {
        assert_eq!(
            [3, 5, 8, 13, 21, 34, 55, 89].into_struc::<FieldCloner<[u32; 4]>>(),
            FieldCloner([3, 5, 8, 13]),
        );
        assert_eq!(
            [3, 5, 8, 13, 21, 34, 55, 89].try_into_struc::<FieldCloner<[u32; 4]>>(),
            Ok(FieldCloner([3, 5, 8, 13])),
        );
    }
    {
        assert_eq!(
            <FieldCloner<Option<u32>> as ConstDefault>::DEFAULT,
            FieldCloner(None)
        );
        assert_eq!(
            <FieldCloner<[u32; 4]> as ConstDefault>::DEFAULT,
            FieldCloner([0u32; 4])
        );
    }
}
