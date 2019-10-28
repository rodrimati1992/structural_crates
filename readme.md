[![Build Status](https://travis-ci.org/rodrimati1992/structural_crates.svg?branch=master)](https://travis-ci.org/rodrimati1992/structural_crates) [![Join the chat at https://gitter.im/structural_crates/community](https://badges.gitter.im/structural_crates/community.svg)](https://gitter.im/structural_crates/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![](https://img.shields.io/crates/v/structural.svg)][crates-io]
[![](https://docs.rs/structural/badge.svg)][api-docs]

[crates-io]: https://crates.io/crates/structural
[api-docs]: https://docs.rs/structural


This library provides abstractions over fields,
allowing for limited emulation of structural types.

# Features

These are the features this library provides:

- Derivation of per-field accessor traits (GetField/GetFieldMut/IntoField).

- Declaration of trait alises for the field accessor traits,with convenient syntax.

# Changelog

The changelog is in the "Changelog.md" file.

# Examples


### Structural Derive

This demonstrates how you can use any type with a superset of the
fields of another one in a function.

```rust
use structural::{GetFieldExt,Structural,tstr};

#[derive(Structural)]
#[struc(public)]
struct Point4<T>(T,T,T,T);


fn reads_point4<S>(point:&S)
where
    // The `Structural` derive macro generated the `Point4_SI` trait,
    // aliasing the accessor traits for Point4.
    S:Point4_SI<u32>
{
    let (a,b,c,d)=point.fields(tstr!("0","1","2","3"));
    
    assert_eq!(a,&0);
    assert_eq!(b,&11);
    assert_eq!(c,&33);
    assert_eq!(d,&66);
}

reads_point4(&Point4(0,11,33,66));
reads_point4(&(0,11,33,66));
reads_point4(&(0,11,33,66,0xDEAD));
reads_point4(&(0,11,33,66,0xDEAD,0xBEEF));

```

### Structural alias

This demonstrates how you can define a trait alias for a single read-only field.

For more details you can look at the docs for the `structural_alias` macro.

```rust

use structural::{GetFieldExt,Structural,structural_alias,tstr};

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
    println!("Hello, {}!",this.field_(tstr!("name")).borrow() )
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
    print_name(&Worker{
        name:"John Doe".into(),
        salary:Cents(1_000_000_000_000_000),
    });
    
    print_name(&Student{
        name:"Jake English".into(),
        birth_year:1995,
    });
}

```

# no-std support

This library is #[no_std] by default and requires enabling the `std` feature for 
implementations of std traits and for std types.

# Minimum Rust version

This crate support Rust back to 1.34,
and will use a build script to automatically enable features from newer versions.

# Cargo Features

If it becomes possible to disable build scripts,
you can manually enable support for Rust past 1.34 features with the `rust_*_*` cargo features.

