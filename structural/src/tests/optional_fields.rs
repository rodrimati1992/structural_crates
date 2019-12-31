use crate::GetFieldExt;

field_path_aliases_module! {
    mod names{
        a,b,c,d
    }
}

#[derive(Copy, Clone)]
struct StructManual {
    a: Option<u32>,
    b: Option<u64>,
    c: Option<&'static str>,
    d: Option<bool>,
}

impl_getters_for_derive_struct! {
    impl[] StructManual
    where[]
    {
        (IntoFieldMut < a : u32,names::a,opt=opt,"a",> )
        (IntoFieldMut < b : u64,names::b,opt=opt,"b",> )
        (IntoFieldMut < c : &'static str,names::c,opt=opt,"c",> )
        (IntoFieldMut < d : Option<bool>,names::d,opt=nonopt,"d",> )
    }
}

#[test]
fn with_struct_manual() {
    {
        let mut this = StructManual {
            a: Some(0),
            b: Some(10),
            c: Some("200"),
            d: Some(true),
        };

        assert_eq!(this.field_(fp!(a)), Some(&0));
        assert_eq!(this.field_(fp!(b)), Some(&10));
        assert_eq!(this.field_(fp!(c)), Some(&"200"));
        assert_eq!(this.field_(fp!(d)), &Some(true));

        assert_eq!(this.field_mut(fp!(a)), Some(&mut 0));
        assert_eq!(this.field_mut(fp!(b)), Some(&mut 10));
        assert_eq!(this.field_mut(fp!(c)), Some(&mut "200"));
        assert_eq!(this.field_mut(fp!(d)), &mut Some(true));

        assert_eq!(this.into_field(fp!(a)), Some(0));
        assert_eq!(this.into_field(fp!(b)), Some(10));
        assert_eq!(this.into_field(fp!(c)), Some("200"));
        assert_eq!(this.into_field(fp!(d)), Some(true));
    }
    {
        let mut this = StructManual {
            a: None,
            b: Some(10),
            c: None,
            d: None::<bool>,
        };

        assert_eq!(this.field_(fp!(a)), None);
        assert_eq!(this.field_(fp!(b)), Some(&10));
        assert_eq!(this.field_(fp!(c)), None);
        assert_eq!(this.field_(fp!(d)), &None::<bool>);

        assert_eq!(this.field_mut(fp!(a)), None);
        assert_eq!(this.field_mut(fp!(b)), Some(&mut 10));
        assert_eq!(this.field_mut(fp!(c)), None);
        assert_eq!(this.field_mut(fp!(d)), &mut None::<bool>);

        assert_eq!(this.into_field(fp!(a)), None);
        assert_eq!(this.into_field(fp!(b)), Some(10));
        assert_eq!(this.into_field(fp!(c)), None);
        assert_eq!(this.into_field(fp!(d)), None::<bool>);
    }
}

/////////////////////////////////////////////////////

declare_variant_proxy! {
    EnumProxy
}

tstring_aliases_module! {
    mod strings{
        A,B,C,a,b,c,d
    }
}

#[derive(Copy, Clone)]
enum EnumManual {
    A {
        a: Option<u32>,
        b: Option<u64>,
        c: Option<&'static str>,
        d: Option<bool>,
    },
    B(Option<(u32, u64)>),
    C,
}

impl_getters_for_derive_enum! {
    impl[] EnumManual
    where[]
    {
        enum=EnumManual
        proxy=EnumProxy
        (
            A,
            strings::A,
            kind=regular,
            fields(
                (IntoFieldMut,a:u32,opt,strings::a)
                (IntoFieldMut,b:u64,opt,strings::b)
                (IntoFieldMut,c:&'static str,opt,strings::c)
                (IntoFieldMut,d:Option<bool>,nonopt,strings::d)
            )
        )
        (
            B,
            strings::B,
            kind=newtype,
            fields((IntoFieldMut,0:(u32,u64),opt))
        )
        (
            C,
            strings::C,
            kind=regular,
            fields()
        )
    }
}

fn drop_ref<T>(_: &T) {}

fn drop_mut<T>(_: &mut T) {}

#[test]
fn with_enum_manual() {
    {
        let mut this = EnumManual::A {
            a: Some(0),
            b: None,
            c: Some("200"),
            d: Some(false),
        };

        assert_eq!(this.field_(fp!(::A.a)), Some(&0));
        assert_eq!(this.field_(fp!(::A.b)), None);
        assert_eq!(this.field_(fp!(::A.c)), Some(&"200"));
        assert_eq!(this.field_(fp!(::A.d)), Some(&Some(false)));

        assert_eq!(this.field_mut(fp!(::A.a)), Some(&mut 0));
        assert_eq!(this.field_mut(fp!(::A.b)), None);
        assert_eq!(this.field_mut(fp!(::A.c)), Some(&mut "200"));
        assert_eq!(this.field_mut(fp!(::A.d)), Some(&mut Some(false)));

        assert_eq!(this.into_field(fp!(::A.a)), Some(0));
        assert_eq!(this.into_field(fp!(::A.b)), None);
        assert_eq!(this.into_field(fp!(::A.c)), Some("200"));
        assert_eq!(this.into_field(fp!(::A.d)), Some(Some(false)));

        assert_eq!(this.field_(fp!(::B.0)), None);
        assert_eq!(this.field_(fp!(::B.1)), None);

        assert_eq!(this.field_(fp!(A)).map(drop_ref), Some(()));
        assert_eq!(this.field_(fp!(B)).map(drop_ref), None);
        assert_eq!(this.field_(fp!(C)).map(drop_ref), None);

        assert_eq!(this.field_mut(fp!(A)).map(drop_mut), Some(()));
        assert_eq!(this.field_mut(fp!(B)).map(drop_mut), None);
        assert_eq!(this.field_mut(fp!(C)).map(drop_mut), None);

        assert_eq!(this.into_field(fp!(A)).map(drop), Some(()));
        assert_eq!(this.into_field(fp!(B)).map(drop), None);
        assert_eq!(this.into_field(fp!(C)).map(drop), None);
    }
    {
        let mut this = EnumManual::A {
            a: Some(0),
            b: None,
            c: Some("200"),
            d: None,
        };

        assert_eq!(this.field_(fp!(::A.d)), Some(&None));
        assert_eq!(this.field_mut(fp!(::A.d)), Some(&mut None));
        assert_eq!(this.into_field(fp!(::A.d)), Some(None));
    }
    {
        let mut this = EnumManual::B(Some((33, 44)));

        assert_eq!(this.field_(fp!(::A.a)), None);
        assert_eq!(this.field_(fp!(::A.b)), None);
        assert_eq!(this.field_(fp!(::A.c)), None);
        assert_eq!(this.field_(fp!(::A.d)), None::<&Option<bool>>);

        assert_eq!(this.field_(fp!(::B.0)), Some(&33));
        assert_eq!(this.field_(fp!(::B.1)), Some(&44));
        assert_eq!(this.field_mut(fp!(::B.0)), Some(&mut 33));
        assert_eq!(this.field_mut(fp!(::B.1)), Some(&mut 44));
        assert_eq!(this.into_field(fp!(::B.0)), Some(33));
        assert_eq!(this.into_field(fp!(::B.1)), Some(44));

        assert_eq!(this.field_(fp!(A)).map(drop_ref), None);
        assert_eq!(this.field_(fp!(B)).map(drop_ref), Some(()));
        assert_eq!(this.field_(fp!(C)).map(drop_ref), None);
    }
    {
        let mut this = EnumManual::B(None);

        assert_eq!(this.field_(fp!(::A.a)), None);
        assert_eq!(this.field_(fp!(::A.b)), None);
        assert_eq!(this.field_(fp!(::A.c)), None);
        assert_eq!(this.field_(fp!(::A.d)), None::<&Option<bool>>);

        assert_eq!(this.field_(fp!(::B.0)), None);
        assert_eq!(this.field_(fp!(::B.1)), None);
    }
    {
        let mut this = EnumManual::C;

        assert_eq!(this.field_(fp!(::A.a)), None);
        assert_eq!(this.field_(fp!(::A.b)), None);
        assert_eq!(this.field_(fp!(::A.c)), None);
        assert_eq!(this.field_(fp!(::A.d)), None::<&Option<bool>>);

        assert_eq!(this.field_(fp!(::B.0)), None);
        assert_eq!(this.field_(fp!(::B.1)), None);

        assert_eq!(this.field_(fp!(A)).map(drop_ref), None);
        assert_eq!(this.field_(fp!(B)).map(drop_ref), None);
        assert_eq!(this.field_(fp!(C)).map(drop_ref), Some(()));
    }
}
