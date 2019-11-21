/*!

This library provides abstractions over fields,emulating structural types.

# Features

These are the features this library provides:

- [Derivation of per-field accessor traits](./docs/structural_macro/index.html)
(GetField/GetFieldMut/IntoField).

- [Declaration of trait aliases for the field accessor traits
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
    let (a,b,c)=point.fields(fp!(x,y,z));
    
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

This demonstrates how you can define a trait alias for a single read-only field accessor.

For more details you can look at the docs for the 
[`structural_alias`](./macro.structural_alias.html) macro.

```rust

use structural::{GetFieldExt,Structural,structural_alias,fp};

use std::borrow::Borrow;

structural_alias!{
    trait Person<S>{
        name:S,
    }
}

fn print_name<T,S>(this:&T)
where
    T:Person<S>,
    S:Borrow<str>,
{
    println!("Hello, {}!",this.field_(fp!(name)).borrow() )
}

// most structural aliases are object safe
fn print_name_dyn<  S>(this:&dyn Person<S>)
where
    S:Borrow<str>,
{
    println!("Hello, {}!",this.field_(fp!(name)).borrow() )
}


//////////////////////////////////////////////////////////////////////////
////          The stuff here could be defined in a separate crate

#[derive(Structural)]
// Using the `#[struc(public)]` attribute tells the derive macro to 
// generate the accessor trait impls for non-`pub` fields.
#[struc(public)]
struct Worker{
    name:String,
    salary:Cents,
}

#[derive(Structural)]
#[struc(public)]
struct Student{
    name:String,
    birth_year:u32,
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Cents(u64);

fn main(){
    let worker=Worker{
        name:"John Doe".into(),
        salary:Cents(1_000_000_000_000_000),
    };

    let student=Student{
        name:"Jake English".into(),
        birth_year:1995,
    };

    print_name(&worker);
    print_name(&student);

    print_name_dyn(&worker);
    print_name_dyn(&student);
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
        name:String,
        // We have shared,mutable,and by value access to the field.
        mut move value:T,
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

*/
#![cfg_attr(feature="alloc",doc=r###"
// most structural aliases are object safe
fn print_name_dyn(mut this:Box<dyn Person<Vec<String>>>){
    println!("Hello, {}!",this.field_(fp!(name)) );

    let list=vec!["what".into()];
    *this.field_mut(fp!(value))=list.clone();
    assert_eq!( this.field_(fp!(value)), &list );
    assert_eq!( this.box_into_field(fp!(value)), list );
}

"###)]
/*!
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
*/
#![cfg_attr(feature="alloc",doc=r###"
    print_name_dyn(Box::new(worker));
    print_name_dyn(Box::new(student));
"###)]
/*!

}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Cents(u64);

```



*/
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
    _FP_impl_,
    structural_alias_impl,
    declare_name_aliases,
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
pub mod chars;

pub use crate::{
    field_traits::{
        GetField,GetFieldMut,IntoField,IntoFieldMut,
        GetFieldExt,
        GetFieldType,
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


