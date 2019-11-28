[![Build Status](https://travis-ci.org/rodrimati1992/structural_crates.svg?branch=master)](https://travis-ci.org/rodrimati1992/structural_crates) [![Join the chat at https://gitter.im/structural_crates/community](https://badges.gitter.im/structural_crates/community.svg)](https://gitter.im/structural_crates/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![](https://img.shields.io/crates/v/structural.svg)][crates-io]
[![](https://docs.rs/structural/badge.svg)][api-docs]

[crates-io]: https://crates.io/crates/structural
[api-docs]: https://docs.rs/structural


This library provides field accessor traits,and emulation of structural types.

# Features

These are the features this library provides:

- [Derivation of the 3 accessor traits for every public field](https://docs.rs/structural/0.2/structural/docs/structural_macro/index.html)
(GetField/GetFieldMut/IntoField).

- [Declaration of trait aliases for accessor trait bounds,using field-in-trait syntax,with the `structural_alias` macro.
](https://docs.rs/structural/0.2/structural/macro.structural_alias.html).

- [Construction of anonymous structs with `make_struct`](https://docs.rs/structural/0.2/structural/macro.make_struct.html)

# Changelog

The changelog is in the "Changelog.md" file.

# Examples



### Structural Derive

This demonstrates how you can use any type with a superset of the
fields of another one in a function.

For details on the [Structural derive macro look here
](https://docs.rs/structural/0.2/structural/docs/structural_macro/index.html).

```rust
use structural::{GetFieldExt,Structural,fp};


fn reads_point4<S>(point:&S)
where
    // The `Structural` derive macro generated the `Point3D_SI` trait for `Point3D`,
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
[`structural_alias`
](https://docs.rs/structural/0.2/structural/docs/structural_macro/index.html) macro.

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
[`make_struct`
](https://docs.rs/structural/0.2/structural/macro.make_struct.html) macro.

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

# Future plans

### 0.2

This will improve the ergonomics of accessing nested fields,
using `value.field_(fp!(a.b.c))` to access `&value.a.b.c`,
where the equivalent in `0.1` is `value.field_(ti!(a)).field_(ti!(b)).field_(ti!(c))`.


# no-std support

To use `structural` in no_std contexts disable the default-feature.

```toml
structural={version="<insert_version_number_here>",default_features=false}
```

This crate has few items that require the standard library (instead of core/alloc),
it is required by default so that users that are not aware of the core/alloc libraries don't have 
to pass a feature to enable std support.

# Cargo Features

These are the cargo features in structural:

- `std`: Enables std support,this is enabled by default.

- `alloc`:
    Enables alloc crate support,this is enabled by default.
    If this is enabled on a version prior to 1.36 it will enable `std` support.

- `rust_1_36`:
    For enabling support of Rust versions from 1.36 onwards ,
    this is automatically enabled by `structural`'s build script.
    This feature is required because the `alloc` crate was stabilized for Rust 1.36,
    while this library supports Rust back to 1.34.

- `rust_1_40`:
    For enabling support of Rust versions from 1.40 onwards ,
    this is automatically enabled by `structural`'s build script.
    This automatically enables the "better_macros" feature.

- `specialization`:
    Enables specialization inside structural,without enabling the nightly feature flag.
    This is for the case that specialization is stabilized after the last update to this library.

- `nightly_specialization`:
    Enables specialization inside structural,
    requires nightly because it enables the nightly feature.

- `better_macros`:
    This enables the `FP` macro to take in the same syntax as the `fp` macro.
    This requires proc macros in type position,which stabilizes in Rust 1.40.

- `nightly_better_macros`
    Equivalent to the "better_macros" feature,
    as well as enable the nightly features required before it was 
    stabilized in Rust 1.40.

- `impl_fields`:
    This allows using `field_name:impl Foo` fields in the `structural_alias` macro,
    which as of 2019-11-23 requires the `associated_type_bounds` Rust nightly feature.<br>
    If this doesn't work,try using the "nightly_impl_fields" feature in Rust nightly instead.

- `nightly_impl_fields`
    Equivalent to the `impl_fields` feature,
    as well as enabling the `associated_type_bounds` nightly features required
    for using the feature as of 2019-11-23.



Specialization is used inside `structural` for performance reasons.
There are no benchmarks comparing when specialization is enabled and disabled yet.

If it becomes possible to disable build scripts,
you can manually enable support for Rust past 1.34 features with the `rust_*_*` cargo features.


# Minimum Rust version

This crate support Rust back to 1.34,
and uses a build script to automatically enable features from newer versions.
