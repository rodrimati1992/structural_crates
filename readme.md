[![Build Status](https://travis-ci.org/rodrimati1992/structural_crates.svg?branch=master)](https://travis-ci.org/rodrimati1992/structural_crates) [![Join the chat at https://gitter.im/structural_crates/community](https://badges.gitter.im/structural_crates/community.svg)](https://gitter.im/structural_crates/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![](https://img.shields.io/crates/v/structural.svg)][crates-io]
[![](https://docs.rs/structural/badge.svg)][api-docs]

[crates-io]: https://crates.io/crates/structural
[api-docs]: https://docs.rs/structural


This library provides abstractions over fields,
allowing for limited emulation of structural types.

# Features

These are the features this library provides:

- Derivation of per-field accessor traits (GetField/GetFieldMut/IntoField/IntoFieldMut)
with the `Structural` derive macro.

- Declaration of trait aliases for the field accessor traits,
with the `structural_alias` macro.

- Construction of anonymous structs with the `make_struct` macro.

# Changelog

The changelog is in the "Changelog.md" file.

# Examples


### Structural Derive

This demonstrates how you can use any type with a superset of the
fields of another one in a function.

```rust
use structural::{GetFieldExt,Structural,ti};

#[derive(Structural)]
#[struc(public)]
struct Point3D<T>{
    x:T,
    y:T,
    z:T,
}


fn reads_point4<S>(point:&S)
where
    // The `Structural` derive macro generated the `Point3D_SI` trait,
    // aliasing the accessor traits for Point3D.
    // You can disable generation of the trait with the `#[struc(no_trait)]` attribute.
    S:Point3D_SI<u32>
{
    let (a,b,c)=point.fields(ti!(x,y,z));
    
    assert_eq!(a,&0);
    assert_eq!(b,&11);
    assert_eq!(c,&33);
}

//////////////////////////////////////////////////////////////////////////
////        In another crate

#[derive(Structural)]
#[struc(public)]
struct Point4D<T>{
    x:T,
    y:T,
    z:T,
    a:T,
}

#[derive(Structural)]
#[struc(public)]
struct Point5D<T>{
    x:T,
    y:T,
    z:T,
    a:T,
    b:T,
}


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

```

### Structural alias

This demonstrates how you can define a trait alias for a single read-only field accessor.

For more details you can look at the docs for the `structural_alias` macro.

```rust

use structural::{GetFieldExt,Structural,structural_alias,ti};

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
    println!("Hello, {}!",this.field_(ti!(name)).borrow() )
}

// most structural aliases are object safe
fn print_name_dyn<S>(this:&dyn Person<S>)
where
    S:Borrow<str>,
{
    println!("Hello, {}!",this.field_(ti!(name)).borrow() )
}


//////////////////////////////////////////////////////////////////////////
////          The stuff here could be defined in a separate crate

#[derive(Structural)]
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

For more details you can look at the docs for the `make_struct` macro.

```rust

use structural::{GetFieldExt,make_struct,structural_alias,ti};

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
    println!("Hello, {}!",this.field_(ti!(name)) );

    let list=vec!["what".into()];
    *this.field_mut(ti!(value))=list.clone();
    assert_eq!( this.field_(ti!(value)), &list );
    assert_eq!( this.into_field(ti!(value)), list );
}

// most structural aliases are object safe
fn print_name_dyn(mut this:Box<dyn Person<Vec<String>>>){
    println!("Hello, {}!",this.field_(ti!(name)) );

    let list=vec!["what".into()];
    *this.field_mut(ti!(value))=list.clone();
    assert_eq!( this.field_(ti!(value)), &list );
    assert_eq!( this.box_into_field(ti!(value)), list );
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

    print_name_dyn(Box::new(worker));
    print_name_dyn(Box::new(student));
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Cents(u64);



```




# no-std support

To use `structural` in no_std contexts disable the default-feature.

```toml
structural={version="<insert_version_number_here>",default_features=false}
```

This crate has few features that require the standard library (instead of core/alloc),
it is required by default so that users that are not aware of the core/alloc libraries don't have 
to pass a feature to enable std support.

# Cargo Features

These are the cargo features in structural:

- `std`: Enables std support,this is enabled by default.

- `alloc`:
    Enables alloc crate support,this is enabled by default.
    If this is enabled on a version prior to 1.36 it will enable `std` support.

- `1.36`:
    A feature is for enabling support of Rust versions from 1.36 onwards ,
    this is automatically enabled by `structural`'s build script.
    This feature is required because the `alloc` crate was stabilized for Rust 1.36,
    while this library supports Rust back to 1.34.

- `specialization`:
    Enables specialization inside structural,without enabling the nightly feature flag.
    This is for the case that specialization is stabilized after the last update to this library.

- `nightly_specialization`:
    Enables specialization inside structural,
    requires nightly because it enables the nightly feature.

- `better_ti`:
    This enables the `TI` macro to take in an identifier or a string literal.
    This requires proc macros in type position,
    which is as of 2019-11-02 stabilizes on Rust 1.40.

- `nightly_better_to`
    This enables the `TI` macro to take in an identifier or a string literal,
    as well as enable the nightly features required before it was 
    marked as stable for Rust 1.40 (as of 2019-11-02).



Specialization is used inside `structural` for performance reasons.
There are no benchmarks comparing when specialization is enabled and disabled yet.

# Minimum Rust version

This crate support Rust back to 1.34,
and will use a build script to automatically enable features from newer versions.

# Cargo Features

If it becomes possible to disable build scripts,
you can manually enable support for Rust past 1.34 features with the `rust_*_*` cargo features.

