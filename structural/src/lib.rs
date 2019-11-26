/*!

This library provides field accessor traits,and emulation of structural types.

# Features

These are the features this library provides:

- [Derivation of the 3 accessor traits for every public field](./docs/structural_macro/index.html)
(GetField/GetFieldMut/IntoField).

- [Declaration of trait aliases for accessor trait bounds,using field-in-trait syntax.
](./macro.structural_alias.html).

- [Construction of anonymous structs with make_struct](./macro.make_struct.html)

# Examples


### Structural Derive

This demonstrates how you can use any type with a superset of the
fields of another one in a function.

For details on the [Structural derive macro look here](./docs/structural_macro/index.html).

```rust
use structural::{GetFieldExt,Structural,fp};


fn reads_point4<S>(point:&S)
where
    // The `Structural` derive generated the `Point3D_SI` trait for `Point3D`,
    // aliasing the accessor traits for it.
    S:Point3D_SI<u32>
{
    let (a,b,c)=point.fields(fp!( x, y, z ));
    
    assert_eq!(a,&0);
    assert_eq!(b,&11);
    assert_eq!(c,&33);
}

fn main(){
    reads_point4(&Point3D { x: 0, y: 11, z: 33 });

    reads_point4(&Point4D {
        x: 0,
        y: 11,
        z: 33,
        a: 0xDEAD,
    });

    reads_point4(&Point5D {
        x: 0,
        y: 11,
        z: 33,
        a: 0xDEAD,
        b: 0xBEEF,
    });
}


#[derive(Structural)]
// Using the `#[struc(public)]` attribute tells the derive macro to 
// generate the accessor trait impls for non-`pub` fields.
#[struc(public)]
struct Point3D<T>{
    x:T,
    y:T,
    z:T,
}

#[derive(Structural)]
// By default only public fields get accessor trait impls,
// using `#[struc(public)]` you can have impls to access private fields.
#[struc(public)]
struct Point4D<T>{
    x:T,
    y:T,
    z:T,
    a:T,
}

#[derive(Structural)]
struct Point5D<T>{
    pub x:T,
    pub y:T,
    pub z:T,
    pub a:T,
    pub b:T,
}





```

### Structural alias

This demonstrates how you can define a trait aliasing field accessors,
using a fields-in-traits syntax.

For more details you can look at the docs for the 
[`structural_alias`](./macro.structural_alias.html) macro.

```rust

use structural::{GetFieldExt,Structural,structural_alias,fp};

use std::borrow::Borrow;

structural_alias!{
    trait Person<H:House>{
        name:String,
        house:H,
    }

    trait House{
        dim:Dimension3D,
    }
}


fn print_name<T,H>(this:&T)
where
    T:?Sized+Person<H>,
    H:House,
{
    let (name,house_dim)=this.fields(fp!( name, house.dim ));
    println!("Hello, {}!", name);

    let (w,h,d)=house_dim.fields(fp!( width, height, depth ));

    if w*h*d >= 1_000_000 {
        println!("Your house is enormous.");
    }else{
        println!("Your house is normal sized.");
    }
}

// most structural aliases are object safe
fn print_name_dyn<H>(this:&dyn Person<H>)
where
    H:House,
{
    print_name(this)
}



#[derive(Structural)]
#[struc(public)]
struct Dimension3D{
    width:u32,
    height:u32,
    depth:u32,
}

//////////////////////////////////////////////////////////////////////////
////          The stuff here could be defined in a separate crate


fn main(){
    let worker=Worker{
        name:"John Doe".into(),
        salary:Cents(1_000_000_000_000_000),
        house:Mansion{
            dim:Dimension3D{
                width:300,
                height:300,
                depth:300,
            },
            money_vault_location:"In the basement".into(),
        }
    };

    let student=Student{
        name:"Jake English".into(),
        birth_year:1995,
        house:SmallHouse{
            dim:Dimension3D{
                width:30,
                height:30,
                depth:30,
            },
            residents:10,
        }
    };

    print_name(&worker);
    print_name(&student);

    print_name_dyn(&worker);
    print_name_dyn(&student);
}


#[derive(Structural)]
// Using the `#[struc(public)]` attribute tells the derive macro to 
// generate the accessor trait impls for non-`pub` fields.
#[struc(public)]
struct Worker{
    name:String,
    salary:Cents,
    house:Mansion,
}

#[derive(Structural)]
#[struc(public)]
struct Student{
    name:String,
    birth_year:u32,
    house:SmallHouse,
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Cents(u64);

#[derive(Structural)]
#[struc(public)]
struct Mansion{
    dim:Dimension3D,
    money_vault_location:String,
}

#[derive(Structural)]
#[struc(public)]
struct SmallHouse{
    dim:Dimension3D,
    residents:u32,
}

```

### Anonymous structs (`make_struct` macro)

This demonstrates how you can construct an anonymous struct.

For more details you can look at the docs for the 
[`make_struct`](./macro.make_struct.html) macro.

```rust

use structural::{GetFieldExt,make_struct,structural_alias,fp};

structural_alias!{
    trait Person<T>{
        // We only have shared access (`&String`) to the field.
        ref name:String,

        // We have shared,mutable,and by value access to the field.
        // Not specifying any of `mut`/`ref`/`move` is equivalent to `mut move value:T,`
        value:T,
    }
}


fn make_person(name:String)->impl Person<()> {
    make_struct!{
        name,
        value: (),
    }
}


fn print_name<T>(mut this:T)
where
    T:Person<Vec<String>>,
{
    println!("Hello, {}!",this.field_(fp!(name)) );

    let list=vec!["what".into()];
    *this.field_mut(fp!(value))=list.clone();
    assert_eq!( this.field_(fp!(value)), &list );
    assert_eq!( this.into_field(fp!(value)), list );
}


// most structural aliases are object safe
fn print_name_dyn(this:&mut dyn Person<Vec<String>>){
    println!("Hello, {}!",this.field_(fp!(name)) );

    let list=vec!["what".into()];
    *this.field_mut(fp!(value))=list.clone();
    assert_eq!( this.field_(fp!(value)), &list );
}

//////////////////////////////////////////////////////////////////////////
////          The stuff here could be defined in a separate crate

fn main(){
    let worker=make_struct!{
        // This derives clone for the anonymous struct
        #![derive(Clone)]
        name:"John Doe".into(),
        salary:Cents(1_000_000_000_000_000),
        value:vec![],
    };

    let student=make_struct!{
        // This derives clone for the anonymous struct
        #![derive(Clone)] 
        name:"Jake English".into(),
        birth_year:1995,
        value:vec![],
    };

    print_name(worker.clone());
    print_name(student.clone());

    print_name_dyn(&mut worker.clone());
    print_name_dyn(&mut student.clone());

    let person=make_person("Louis".into());

    assert_eq!( person.field_(fp!(name)), "Louis" );
    assert_eq!( person.field_(fp!(value)), &() );
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Cents(u64);

```



*/
#![cfg_attr(feature="nightly_impl_fields",feature(associated_type_bounds))]
#![cfg_attr(feature="nightly_specialization",feature(specialization))]
#![cfg_attr(feature="nightly_better_macros",feature(proc_macro_hygiene))]

#![cfg_attr(not(feature="alloc"),no_std)]

#[doc(hidden)]
pub extern crate core as std_;

#[doc(hidden)]
#[cfg(all(feature="alloc",feature="rust_1_36"))]
pub extern crate alloc as alloc_;

#[doc(hidden)]
#[cfg(all(feature="alloc",feature="rust_1_36"))]
pub use alloc_ as alloc;

#[doc(hidden)]
#[cfg(all(feature="alloc",not(feature="rust_1_36")))]
pub use std as alloc;


extern crate self as structural;

pub use structural_derive::Structural;

#[doc(hidden)]
pub use structural_derive::{
    old_fp_impl_,
    //new_fp_impl_,
    _field_path_aliases_impl,
    _FP_impl_,
    structural_alias_impl,
};


#[macro_use]
mod macros;

pub mod docs;
pub mod mut_ref;
pub mod field_traits;
pub mod structural_trait;
pub mod utils;

#[cfg(test)]
pub mod tests{
    mod multi_nested_fields;
    mod structural_derive;
    mod structural_alias;
    mod macro_tests;
}


pub mod type_level;

#[doc(hidden)]
pub mod chars;

pub use crate::{
    field_traits::{
        GetField,GetFieldMut,IntoField,IntoFieldMut,
        GetFieldExt,
        GetFieldType,GetFieldType2,GetFieldType3,GetFieldType4,
        RevGetFieldType,RevGetFieldType_,
    },
    structural_trait::{Structural,StructuralDyn},
};



/// Reexports from the `core_extensions` crate.
pub mod reexports{
    pub use core_extensions::{
        type_asserts::AssertEq,
        MarkerType,
        SelfOps,
        TIdentity,
        TypeIdentity,
    };
}

// pmr(proc macro reexports):
// Reexports for the proc macros in structural_derive.
#[doc(hidden)]
pub mod pmr{
    pub use crate::type_level::*;
    pub use crate::type_level::ident::TString;
    pub use crate::type_level::proc_macro_aliases::*;
    pub use crate::type_level::collection_traits::*;
    pub use crate::chars::*;
    pub use core_extensions::{MarkerType,TIdentity,TypeIdentity};
    pub use crate::std_::marker::PhantomData;
}


