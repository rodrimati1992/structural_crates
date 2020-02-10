/*!

Optional accessors are field accessors that return `Option<_>`s in GetFieldExt methods.
For example: `GetFieldExt::field_` returns an `Option<&_>`,
and `GetFieldExt::field_mut` returns an `Option<&mut _>`.

You can use the same `GetFieldExt` methods for optional accessors that
you can use for non-optional ones,
because structural uses some type-level machinery to figure out whether it returns an
`&Option<Foo>` or an `Option<&Foo>` for an accessor impl.

Every enum field has accessors impls callable with `fp!(::Foo.bar)`,
all of which are optional accessors.

# Return types of GetFieldExt methods

For structs,making a field optional causes calling `GetFieldExt::field_` for
 `Option<T>` fields to go from returning an `&Option<T>` to returning an `Option<&T>`.

For enums,making a field optional causes calling `GetFieldExt::field_` for
`Option<T>` fields to go from returning an `Option<&Option<T>>` to returning an `Option<&T>`.

The same goes for every other method in `GetFieldExt`.


# Attributes

Here are links to the attributes related to optional accessors in
the `Structural` derive macro.

Container attributes:

- [`#[struc(implicit_optionality)]`](../index.html#strucimplicit_optionality)

Field Attributes

- [`#[struc(not_optional)]`](../index.html#strucnot_optional)

- [`#[struc(optional)]`](../index.html#strucoptional)

# Struct Example

This example shows how the `#[struc(implicit_optionality)]` attribute works.

```rust
use structural::{GetFieldExt,Structural,fp};

type OptionU32=Option<u32>;

#[derive(Structural)]
#[struc(public,implicit_optionality)]
struct Foo{
    a:u32,
    #[struc(not_optional)]
    b:Option<u32>,
    c:Option<u32>,
    d:Option<u32>,
    e:OptionU32,
    #[struc(not_optional)]
    f:Option<(u32,u32,u32)>,
    #[struc(optional)]
    g:Option<(u32,u32,u32)>,
}

let mut this=Foo{
    a:0,
    b:Some(99),
    c:Some(33),
    d:None,
    e:None,
    f:Some((3,5,8)),
    g:Some((13,21,34)),
};

// This works like always
assert_eq!( this.field_(fp!(a)), &0 );

// A `#[struc(not_optional)]` field inside a `#[struc(implicit_optionality)]` type.
assert_eq!( this.field_(fp!(b)), &Some(99) );

// An optional field inside a `#[struc(implicit_optionality)]` type.
assert_eq!( this.field_(fp!(c)), Some(&33) );

// An optional field inside a `#[struc(implicit_optionality)]` type.
assert_eq!( this.field_(fp!(d)), None );

// An non-optional field inside a `#[struc(implicit_optionality)]` type,
// this is because the macro only recognizes `Òption<_>` as an `Option`.
assert_eq!( this.field_(fp!(e)), &None );
this.e=Some(13);
assert_eq!( this.field_(fp!(e)), &Some(13) );

// Here we access multiple fields inside the `f` `#[struc(not_optional)]` Option field.
assert_eq!( this.fields(fp!(f::Some=>0,1,2)), Some((&3,&5,&8)) );

// Here we access multiple fields inside the `g` optional field.
assert_eq!( this.fields(fp!(g=>0,1,2)), Some((&13,&21,&34)) );


```

# Enum Example

This example shows the difference between optional and non-optional fields in enums,
using explicit optionality.

```rust

use structural::{GetFieldExt,Structural,fp};

#[derive(Structural)]
enum Foo{
    Bar{
        a:u32,
        b:Option<u32>,
        #[struc(optional)]
        c:Option<u32>,
        #[struc(optional)]
        d:Option<u32>,
    }
}

let this=Foo::Bar{
    a:0,
    b:Some(99),
    c:Some(33),
    d:None,
};

//A regular field
assert_eq!( this.field_(fp!(::Bar.a)), Some(&0) );

// A regular `Option<u32>` field,this is not an optional field
// because the `#[struc(optional)]` attribute wasn't used.
assert_eq!( this.field_(fp!(::Bar.b)), Some(&Some(99)) );

// An optional `u32` field,because of the `#[struc(optional)]` attribute
assert_eq!( this.field_(fp!(::Bar.c)), Some(&33) );

// An optional `u32` field,because of the `#[struc(optional)]` attribute
assert_eq!( this.field_(fp!(::Bar.d)), None );

assert_eq!(
    // `=>` allows accessing multiple fields inside an optional field/variant,
    // without having to unwrap them individually
    // (half of this enum's fields are optional,so it makes less of a difference).
    this.fields(fp!(::Bar=>a,b,c,d)),
    Some((
        &0,        //A regular field
        &Some(99), //A regular `Option<u32>` field
        Some(&33), //An optional `u32` field
        None,      //An optional `u32` field
    )),
);

```


*/
