/*!

This library provides abstractions over fields,
allowing for limited emulation of structural types.

# Features

These are the features this library provides:

- Derivation of per-field accessor traits (GetField/GetFieldMut/IntoField).

- Declaration of trait alises for the field accessor traits,with convenient syntax.

# Example

This example demonstrates how you can use any type sharing the
same fields as another one in a function.

```rust
use structural::{GetFieldExt,Structural,tstr};

#[derive(Structural)]
#[struc(public)]
struct Point4<T>(T,T,T,T);


fn reads_point4<S>(point:&S)
where
    // Point4_SI aliases the accessor traits for Point4,
    // this allows passing in tuples larger than 4 elements
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

*/

extern crate self as structural;

pub use structural_derive::Structural;


#[macro_use]
mod macros;

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
    field_traits::{GetField,GetFieldMut,IntoField,GetFieldExt,GetFieldType},
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
