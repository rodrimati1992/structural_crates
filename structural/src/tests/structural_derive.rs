use crate::{
    structural_trait::accessor_names,
    GetField,GetFieldMut,IntoField,IntoFieldMut,
    GetFieldExt,
    GetFieldType,
    Structural,
};

////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
fn object_safety(){
    let _:dyn GetField<TStr!(a b),Ty=()>;
    let _:dyn GetFieldMut<TStr!(a b),Ty=()>;
}


////////////////////////////////////////////////////////////////////////////////


structural_alias!{
    trait HuhInterface{
        a:u32,
        b:u32,
    }
}

#[derive(Structural)]
struct Huh{
    pub a:u32,
    pub b:u32,
    pub c:u32,
    pub d:u32,
}

#[derive(Structural)]
struct Whoah{
    pub nah:u32,
    pub yep:u32,
    pub b:u32,
    pub a:u32,
}

fn huh_printer<This>(this:This)
where
    This:HuhInterface
{
    let (a,b)=this.fields(tstr!("a","b"));
    assert_eq!(a, &10);
    assert_eq!(b, &33);
}

#[test]
fn huh_printing(){
    huh_printer(Huh{
        a:10,
        b:33,
        c:44,
        d:66,
    });

    huh_printer(Whoah{
        nah:0x000F,
        yep:!0,
        b:33,
        a:10,
    });
}

////////////////////////////////////////////////////////////////////////////////


#[derive(Structural,Clone)]
#[struc(public)]
struct Privacies0{
    a:u32,
    b:u32,
    #[struc(private)]
    c:u32,
}

#[derive(Structural,Clone)]
//#[struc(debug_print)]
struct Privacies1{
    pub a:u32,
    pub b:u32,
    #[struc(not_public)]
    pub c:u32,
    d:u32,
    #[struc(public)]
    e:u32,
    #[struc(access="ref")]
    f:u32,
    #[struc(access="mut")]
    g:u32,
    #[struc(access="mut move")]
    hello:u32,
    #[struc(access="move")]
    world:u32,
}


trait Privacies1Test:Privacies1_SI{
    type Dummy;
}

// Using this trait to test that `Privacies1_SI` has the bounds from bellow as supertraits.
// Because these bounds might be more constrained than `Privacies1_SI` itself
// I'm testing that `Privacies1` implements those traits inside the `privacies` test.
impl<L> Privacies1Test for L
where
    L:GetField<TStr!(a),Ty=u32>+
        GetField<TStr!(b),Ty=u32>+
        GetField<TStr!(e),Ty=u32>+
        GetField<TStr!(f),Ty=u32>+
        GetFieldMut<TStr!(g),Ty=u32>+
        IntoFieldMut<TStr!(h e l l o),Ty=u32>+
        IntoField<TStr!(w o r l d),Ty=u32>+
        Sized,
{
    type Dummy=();
}


#[test]
fn privacies(){
    let _:<Privacies1 as Privacies1Test>::Dummy;

    let _=|this:Privacies0|{
        let _=this.fields(tstr!("a","b"));
    };
    fn generic_1<T>(mut this:T)
    where
        T:Privacies1_SI+Clone
    {
        let _=this.fields(tstr!("a","b","e","f","g","hello"));
        let _=this.fields_mut(tstr!("g","hello"));
        let _=this.clone().into_field(tstr!("hello"));
        let _=this.clone().into_field(tstr!("world"));
    }
    let _=generic_1::<Privacies1>;

    assert!(
        accessor_names::<Privacies0>()
        .eq(["a","b"].iter().cloned()),
    );

    assert!(
        accessor_names::<Privacies1>()
        .eq(["a","b","e","f","g","hello","world"].iter().cloned()),
    );
}


////////////////////////////////////////////////////////////////////////////////


#[derive(Structural,Clone)]
#[struc(public)]
struct Renamed{
    pub a:u32,
    pub b:u32,
    #[struc(rename="e")]
    pub c:u32,
}


#[test]
fn renamed(){
    assert!(
        accessor_names::<Renamed>()
            .eq(["a","b","e"].iter().cloned())
    );

    let _:GetFieldType<Renamed,TStr!(a)>;
    let _:GetFieldType<Renamed,TStr!(b)>;
    let _:GetFieldType<Renamed,TStr!(e)>;

}