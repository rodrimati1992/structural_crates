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
use structural::{GetFieldExt,Structural,ti};

#[derive(Structural)]
#[struc(public)]
struct Point4<T>(T,T,T,T);


fn reads_point4<S>(point:&S)
where
    // The `Structural` derive macro generated the `Point4_SI` trait,
    // aliasing the accessor traits for Point4.
    S:Point4_SI<u32>
{
    let (a,b,c,d)=point.fields(ti!(0,1,2,3));
    
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
# no-std support

To use `structural` in no_std contexts disable the default-feature.

This crate has few features that require the standard library (instead of core/alloc),
it is required by default so that users that are not aware of the core/alloc libraries don't have 
to pass a feature to enable std support.

# Features

These are the cargo features in structural:

- `std`: Enables std support,this is enabled by default.

- `alloc`:
    Enables alloc crate support,this is enabled by default.
    If this is enabled on a version prior to 1.36 it will enable `std` support.

- `1.36`:
    A feature is for enabling support of Rust versions from 1.36 onwards ,
    this is automatically enabled by `structural`'s build script.

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

nightly_type_pmacros


Specialization is used inside structural for performance reasons.
There are no benchmarks comparing when specialization is enabled and disabled yet.

# Minimum Rust version

This crate support Rust back to 1.34,
and will use a build script to automatically enable features from newer versions.

# Cargo Features

If it becomes possible to disable build scripts,
you can manually enable support for Rust past 1.34 features with the `rust_*_*` cargo features.

