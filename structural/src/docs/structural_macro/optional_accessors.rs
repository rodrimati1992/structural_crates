/*!

Optional accessors are field accessors that return `Option<_>`s in GetFieldExt methods.

You can use the same `GetFieldExt` methods for optional accessors that
you can use for non-optional ones,
because structural uses some type-level machinery to figure out whether it returns an
`&Option<Foo>` or an `Option<&Foo>` for an accessor impl.

Every enum field has accessors impls with `FP!(::Foo.bar)` type parameters,
all of which are optional accessors.

For structs,calling `GetFieldExt::field_` for `Option<T>` fields goes from returning a
`&Option<T>` to `Option<&T>`.

For enums,calling `GetFieldExt::field_` for `Option<T>` fields goes from returning a
`Option<&Option<T>>` to `Option<&T>`.

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
}

let mut this=Foo{
    a:0,
    b:Some(99),
    c:Some(33),
    d:None,
    e:None,
};

// This works like always
assert_eq!( this.field_(fp!(a)), &0 );

// This evaluates to an `&Option<_>` instead of `Option<&_>`
// because of the `#[struc(not_optional)]` attribute.
assert_eq!( this.field_(fp!(b)), &Some(99) );

// This evaluates to an `Option<&_>` because of the `implicit_optionality`.
assert_eq!( this.field_(fp!(c)), Some(&33) );

// This evaluates to an `Option<&_>` because of the `implicit_optionality`.
assert_eq!( this.field_(fp!(d)), None );

// This evaluates to an `&Option<u32>` because the field was not written as `Option<_>`.
// If the field had a `#[struc(optional)]` attribute,this would evaluate to an `Option<&u32>`.
assert_eq!( this.field_(fp!(e)), &None );
this.e=Some(13);
assert_eq!( this.field_(fp!(e)), &Some(13) );


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

assert_eq!( this.field_(fp!(::Bar.a)), Some(&0) );
assert_eq!( this.field_(fp!(::Bar.b)), Some(&Some(99)) );
assert_eq!( this.field_(fp!(::Bar.c)), Some(&33) );
assert_eq!( this.field_(fp!(::Bar.d)), None );

```


*/
