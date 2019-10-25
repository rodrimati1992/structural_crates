/*!

This library provides abstractions over fields,
allowing for limited emulation of structural types.

# Features

These are the features this library provides:

- Derivation of per-field accessor traits (GetField/GetFieldMut/IntoField).

- Declaration of trait alises for the field accessor traits,with convenient syntax.

# Changelog

The changelog is in the "Changelog.md" file.

# Example

This example demonstrates how you can use any type with the
same fields as another one in a function.

```rust
use structural::{GetFieldExt,Structural,tstr};

#[derive(Structural)]
#[struc(public)]
struct Point4<T>(T,T,T,T);


fn reads_point4<S,T>(point:&S)
where
    // Point4_SI aliases the accessor traits for Point4,
    // this allows passing in tuples larger than 4 elements
    S:Point4_SI<T>
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


# Minimum Rust version

This crate support Rust back to 1.34,
and will use a build script to automatically enable features from newer versions.

# Cargo Features

If it becomes possible to disable build scripts,
you can manually enable support for Rust past 1.34 features with the `rust_*_*` cargo features.

