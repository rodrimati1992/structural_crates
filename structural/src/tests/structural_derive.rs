use crate::{
    structural_trait::accessor_names,
    GetField,GetFieldMut,IntoField,IntoFieldMut,
    GetFieldExt,
    GetFieldType,
    Structural,
};

#[cfg(feature="alloc")]
use crate::alloc::{
    rc::Rc,
    sync::Arc,
};

////////////////////////////////////////////////////////////////////////////////


#[test]
fn derive_inside_function(){
    #[derive(Structural)]
    pub struct Bar{
        pub b:u32,
    }
}


////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
fn object_safety(){
    #[cfg(feature="alloc")]
    type AllocPtrs<'a,T>=(
        crate::alloc::boxed::Box<T>,
        crate::alloc::sync::Arc<T>,
    );

    #[cfg(not(feature="alloc"))]
    type AllocPtrs<'a,T>=(T,);


    type TraitObjects<'a,T>=(
        &'a T,
        &'a mut T,
        AllocPtrs<'a,T>,
    );

    let _:TraitObjects<'_,dyn GetField<TI!(a b),Ty=()>>;
    let _:TraitObjects<'_,dyn GetFieldMut<TI!(a b),Ty=()>>;
    let _:TraitObjects<'_,dyn Huh_SI>;
    let _:TraitObjects<'_,dyn Whoah_SI>;
    let _:TraitObjects<'_,dyn Renamed_SI>;
    let _:TraitObjects<'_,dyn Privacies1_SI>;
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
    let (a,b)=this.fields(ti!(a,b));
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
    L:GetField<TI!(a),Ty=u32>+
        GetField<TI!(b),Ty=u32>+
        GetField<TI!(e),Ty=u32>+
        GetField<TI!(f),Ty=u32>+
        GetFieldMut<TI!(g),Ty=u32>+
        IntoFieldMut<TI!(h e l l o),Ty=u32>+
        IntoField<TI!(w o r l d),Ty=u32>+
        Sized,
{
    type Dummy=();
}


#[test]
fn privacies(){
    let _:<Privacies1 as Privacies1Test>::Dummy;

    let _=|this:Privacies0|{
        let _=this.fields(ti!(a,b));
    };
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


// This tests that boxed trait objects can still call GetFieldExt methods.
#[cfg(feature="alloc")]
#[test]
fn ptr_dyn_methods(){
    let _=|this:Privacies1|{
        #[cfg(feature="alloc")]
        {
            generic_1_dyn(||Box::new(this.clone()));
        }
    };

    let _=|this:Privacies0|{
        #[cfg(feature="alloc")]
        {
            generic_0_dyn(||Arc::new(this.clone()));
            let _:Rc<dyn Privacies0_SI>=Rc::new(this.clone());
        }
    };
}


fn generic_1<T>(mut this:T)
where
    T:Privacies1_SI+Clone
{
    let _=this.fields(ti!(a,b,e,f,g,hello));
    let _=this.fields_mut(ti!(g,hello));
    let _=this.clone().into_field(ti!(hello));
    let _=this.clone().into_field(ti!(world));
    #[cfg(feature="alloc")]
    {
        let _=Box::new(this.clone()).box_into_field(ti!(hello));
        let _=Box::new(this.clone()).box_into_field(ti!(world));
    }
}

#[cfg(feature="alloc")]
fn generic_0_dyn(mut ctor:impl FnMut()->Arc<dyn Privacies0_SI> ){
    let this=ctor();
    let _=this.fields(ti!(a,b));
}

#[cfg(feature="alloc")]
fn generic_1_dyn(mut ctor:impl FnMut()->Box<dyn Privacies1_SI> ){
    let mut this=ctor();
    let _=this.fields(ti!(a,b,e,f,g,hello));
    let _=this.fields_mut(ti!(g,hello));
    let _=this.field_mut(ti!(g));
    let _=this.field_mut(ti!(hello));
    #[cfg(feature="alloc")]
    {
        let _=ctor().box_into_field(ti!(hello));
        let _=ctor().box_into_field(ti!(world));
    }
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

    let _:GetFieldType<Renamed,TI!(a)>;
    let _:GetFieldType<Renamed,TI!(b)>;
    let _:GetFieldType<Renamed,TI!(e)>;

}