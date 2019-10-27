/*!

This library provides abstractions over fields,
allowing for limited emulation of structural types.

# Features

These are the features this library provides:

- Derivation of per-field accessor traits (GetField/GetFieldMut/IntoField).

- Declaration of trait alises for the field accessor traits,with convenient syntax.

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

This demonstrates how you can define a trait alias for a single read-only field accessor.

For more details you can look at the docs for the 
[`structural_alias`](./macro.structural_alias.html) macro.

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



*/
#![no_std]

#[doc(hidden)]
pub extern crate core as std_;

extern crate self as structural;

pub use structural_derive::Structural;


#[macro_use]
mod macros;

pub mod docs;
pub mod mut_ref;
pub mod field_traits;
pub mod structural_trait;
pub mod type_level;
pub mod utils;

#[cfg(test)]
pub mod tests{
    mod structural_derive;
    mod structural_alias;
}



#[doc(hidden)]
pub use crate::type_level::ident as chars;

pub use crate::{
    field_traits::{
        GetField,GetFieldMut,IntoField,IntoFieldMut,
        GetFieldExt,
        GetFieldType,
    },
    structural_trait::Structural,
};



/// Reexports from the `core_extensions` crate.
pub mod reexports{
    pub use core_extensions::{MarkerType,SelfOps};
}

// Reexports for the proc macros in structural_derive.
#[doc(hidden)]
pub mod proc_macro_reexports{
    pub use crate::type_level::ident::*;
    pub use core_extensions::MarkerType;
}
