/*!

The Structural derive macro implements the Structural trait,
as well as accessor traits(GetFieldImpl/GetFieldMutImpl/IntoFieldImpl) for fields.

# Default Behavior

By default,this derive generates:

- Implementation of the structural trait for the deriving type.

- Implementations of the accessor traits (GetFieldImpl/GetFieldMutImpl/IntoFieldImpl) for pub fields.

- A trait named `<deriving_type>_SI`,aliasing the accessor traits for the type,
with a blanket implementation for all types with the same fields.

All of these can be overriden.

# Container Attributes

### `#[struc(debug_print)]`

Prints the output of the derive macro by panicking.

### `#[struc(no_trait)]`

Disables the generation of the `<deriving_type>_SI` trait.

[Here is an example using this attribute](#disabling-the-trait-alias)

### `#[struc(implicit_optionality)]`

Changes accessors of `Option<_>` fields to be optional.

For structs,calling `GetFieldExt::field_` for `Option<T>` fields goes from returning a
`&Option<T>` to `Option<&T>`.

For enums,calling `GetFieldExt::field_` for `Option<T>` fields goes from returning a
`Option<&Option<T>>` to `Option<&T>`.

Only fields written like this are treated as optional:

- `Option<_>`

- `std::option::Option<_>`

- `core::option::Option<_>`

(where `_` stands for any type)

If the field is written any differently,then it will not be treated as an optional field,
and you will be required to use the `#[struc(optional)]` attribute.

# Variant Attributes

### `#[struc(replace_bounds="bounds")]

Replaces (in the generated trait) the bounds for this particular variant with
the ones in the attribute.

This is most useful for newtype variants (single field tuple variants),
where you want the generated trait to require the fields of the wrapped type.

All `@variant` in the bounds will be replaced with the name of the variant,

Example:`#[struc(replace_bounds = "Foo_VSI<@variant>")]`

Example:`#[struc(replace_bounds = "Bar_VSI<T,U,@variant>")]`

Example:`#[struc(replace_bounds = "Baz_VSI<'a,u8,@variant>")]`

# Field Attributes

### `#[struc(rename="<new_name>")]`

Changes the name for the field in the accessor trait impls.

### `#[struc(impl="<trait bounds>")]`

This requires the `nightly_impl_fields` cargo feature
(or `impl_fields` if associated type bounds stabilized after the latest release).

Changes the `<deriving_type>_SI` trait (which aliases the accessor traits for this type)
not to refer to the type of this field,
instead it will be required to implement the bounds passed to this attribute.

Note that these bounds are only added to the `<deriving_type>_SI` trait.

[Here is the example for this attribute.](#impl-trait-fields)

### `#[struc(delegate_to)]`

Delegates the implementation of the Structural and accessor traits to this field.

You can only delegate the implementation and Structural and accessor traits
to a single field.

Using this attribute will disable the generation of the `<deriving_type>_SI` trait.

Optional arguments for `delegate_to`:

- `bound="T:bound"`: Adds the constraint to all the trait impls.
- `mut_bound="T:bound"`: Adds the constraint to the `GetFieldImpl` impl.
- `into_bound="T:bound"`: Adds the constraint to the `IntoFieldImpl` impl.

### `#[struc(optional)]`

Forces a field to have an optional accessor.

As opposed to `#[struc(implicit_optionality)]`,
this also allows type aliases of Option to be used,

[Here are more details on how optional fields work](./optional_accessors/index.html)

Example:
```rust
use structural::{GetFieldExt,Structural,fp};

type OptionU64=Option<u64>;

#[derive(Structural)]
struct WithOptional{
    #[struc(optional)]
    pub foo:OptionU64,

    pub not_optional:OptionU64,
}

let mut this=WithOptional{
    foo: Some(99),
    not_optional: Some(13),
};
assert_eq!( this.field_(fp!(foo)), Some(&99) );
assert_eq!( this.field_(fp!(not_optional)), &Some(13) );

this.foo=None;
this.not_optional=None;
assert_eq!( this.field_(fp!(foo)), None );
assert_eq!( this.field_(fp!(not_optional)), &None );

```


### `#[struc(not_optional)]`

Forces a field to not have an optional accessor.

This allows using `Option<_>` fields with `#[struc(implicit_optionality)]`,
without making their accessor optional.

[Here are more details on how optional fields work](./optional_accessors/index.html)

Example:
```rust
use structural::{GetFieldExt,Structural,fp};

type OptionU64=Option<u64>;

#[derive(Structural)]
#[struc(implicit_optionality)]
struct WithOptional{
    pub foo:Option<u64>,

    #[struc(not_optional)]
    pub not_optional:Option<u64>,
}

let mut this=WithOptional{
    foo: Some(99),
    not_optional: Some(13),
};
assert_eq!( this.field_(fp!(foo)), Some(&99) );
assert_eq!( this.field_(fp!(not_optional)), &Some(13) );

this.foo=None;
this.not_optional=None;
assert_eq!( this.field_(fp!(foo)), None );
assert_eq!( this.field_(fp!(not_optional)), &None );

```


# Container/Field Attributes

Unless stated otherwise,
when these attributes are put on the container it will have the same effect as
being put on the field,and are overriden by attributes directly on the field.

### `#[struc(public)]`

Marks the fields as public,generating the accessor traits for the field.

### `#[struc(not_public)]`

Marks the fields as private,not generating the accessor traits for the field.

### `#[struc(access="")]`

Changes the implemented accessor traits for the field(s).

`#[struc(access="ref")]`:
Generates impls of the `GetFieldImpl` trait for the field(s).

`#[struc(access="mut")]`:
Generates impls of the `GetFieldImpl`+`GetFieldMutImpl` traits for the field(s).

`#[struc(access="move")]`:
Generates impls of the `GetFieldImpl`+`IntoFieldImpl` traits for the field(s).

`#[struc(access="mut move")]`:
Generates impls of the `GetFieldImpl`+`GetFieldMutImpl`+`IntoFieldImpl` traits for the field(s).

When this attribute is used on a non-pub field,
it'll mark the field as public for the purpose of generating accessor trait impls.

# Examples

### Basic example

```rust
use structural::{Structural,GetFieldExt,structural_alias,fp};


fn reads_pair<O>(pair:&O)
where
    // This uses the trait generated by `#[derive(Structural)]`,
    // aliasing the accessor traits implemented for `Hello`,
    // allowing any type with (at least) those fields to be passed here.
    O:Hello_SI
{
    let (a,b)=pair.fields(fp!( a, b ));
    assert_eq!(a,&11);
    assert_eq!(b,&33);
}


#[derive(Debug,Structural,PartialEq,Eq)]
#[struc(public)]
struct Hello{
    a:u32,
    b:u32
}

#[derive(Structural)]
#[struc(access="mut move")]
#[struc(public)]
struct World{
    run:String,
    a:u32,
    b:u32,
}

fn main(){
    reads_pair(&Hello{ a:11, b:33 });

    reads_pair(&World{ run:"nope".into(), a:11, b:33 });
}


```

### Mutating fields

```rust
use structural::{Structural,GetFieldExt,structural_alias,fp};


structural_alias!{
    trait Tuple2<T>{
        0:T,
        1:T,
    }
}


fn mutates_pair<O>(pair:&mut O)
where
    O:Tuple2<u32>
{
    let a=pair.field_mut(fp!(0));
    assert_eq!(a,&mut 14);
    *a*=2;

    let b=pair.field_mut(fp!(1));
    assert_eq!(b,&mut 16);
    *b*=2;
}


#[derive(Debug,Structural,PartialEq,Eq)]
struct Point(
    #[struc(public)]
    u32,

    #[struc(public)]
    u32,

    #[struc(not_public)]
    pub u32,
);

fn main(){
    let mut point=Point(14,16,11);
    let mut tuple=(14,16);

    mutates_pair(&mut point);
    mutates_pair(&mut tuple);

    assert_eq!(point,Point(28,32,11));
    assert_eq!(tuple,(28,32));
}

```

### Disabling the trait alias

This example demonstrates how one disables the generation of the
`<deriving_type>_SI` trait to declare it manually.

```rust
use structural::{Structural,IntoFieldMut,GetFieldExt,FP};

#[derive(Debug,Structural,PartialEq,Eq)]
#[struc(no_trait)]
#[struc(access="mut move")]
struct Hello{
    pub hello:u32,
    pub world:String,
}


pub trait Hello_SI:
    // From `1.40` onwards you can write those bounds like this:
    // ```
    // IntoFieldMut<FP!(hello), Ty=u32>+
    // IntoFieldMut<FP!(world), Ty=String>
    // ```
    // Alternatively,you could use the `field_path_aliases` macro,
    // and use those aliases here instead of using `FP!`.
    IntoFieldMut<FP!(h e l l o), Ty=u32>+
    IntoFieldMut<FP!(w o r l d), Ty=String>
{}

impl<T> Hello_SI for T
where
    T:?Sized+
        IntoFieldMut<FP!(h e l l o), Ty=u32>+
        IntoFieldMut<FP!(w o r l d), Ty=String>
{}

```

### Impl trait fields

This is an example of using the `#[struc(impl="<trait_bounds>")]` attribute

This requires the `nightly_impl_fields` cargo feature
(or `impl_fields` if associated type bounds stabilized after the latest release).

*/
#![cfg_attr(not(feature="nightly_impl_fields"),doc="```ignore")]
#![cfg_attr(feature="nightly_impl_fields",doc="```rust")]
/*!

// Remove this if associated type bounds (eg: `T: Iterator<Item: Debug>`)
// work without it.
#![feature(associated_type_bounds)]

use std::borrow::Borrow;

use structural::{Structural,fp,make_struct,GetFieldExt};


#[derive(Structural)]
#[struc(public)]
struct Person{
    #[struc(impl="Borrow<str>")]
    name:String,

    #[struc(impl="Copy+Into<u64>")]
    height_nm:u64
}


fn takes_person(this:&impl Person_SI){
    let (name,height)=this.fields(fp!(name,height_nm));
    assert_eq!( name.borrow(), "bob" );

    assert_eq!( (*height).into(), 1_500_000_000 );
}


// Notice how `name` is a `&'static str`,and `height_nm` is a `u32`?
//
// This is possible because the concrete types of the fields weren't used in
// the `Person_SI` trait.
takes_person(&make_struct!{
    name:"bob",
    height_nm: 1_500_000_000_u32,
});

takes_person(&Person{
    name:"bob".to_string(),
    height_nm: 1_500_000_000_u64,
});

```

### Delegation

This is an example of using the `#[struc(delegate_to)]` attribute.

```
use structural::{fp,make_struct,GetFieldExt,Structural};


#[derive(Structural,Clone)]
struct Foo<T>{
    #[struc(delegate_to)]
    value:T
}

# // ensuring that Foo_SI wasn't generated
# trait Foo_SI{}

#[derive(Structural,Clone)]
#[struc(public,access="ref")]
struct AnimalCounts{
    cows:u32,
    chickens:u32,
    pigs:u32,
}


fn total_count(animals:&dyn AnimalCounts_SI)->u64{
    *animals.field_(fp!(cows)) as u64+
    *animals.field_(fp!(chickens)) as u64+
    *animals.field_(fp!(pigs)) as u64
}

{
    let count=total_count(&Foo{
        value:make_struct!{
            cows:100,
            chickens:200,
            pigs:300,
        }
    });

    assert_eq!( count, 600 );
}

{
    let count=total_count(&Foo{
        value:AnimalCounts{
            cows:0,
            chickens:500,
            pigs:0,
        }
    });

    assert_eq!( count, 500 );
}

{
    let count=total_count(&AnimalCounts{
        cows:0,
        chickens:500,
        pigs:1_000_000_000,
    });

    assert_eq!( count, 1_000_000_500 );
}





```

### Delegation,with bounds

This is an example of using the `#[struc(delegate_to())]` attribute with
extra bounds in the accessor trait impls.

```
use structural::{fp,make_struct,GetFieldExt,Structural};

use std::{
    fmt::Debug,
    ops::Add,
};


#[derive(Structural,Debug,Copy,Clone,PartialEq)]
struct Foo<T>{
    #[struc(delegate_to(
        bound="T:PartialEq",
        mut_bound="T:Copy",
        into_bound="T:Debug",
    ))]
    value:T
}

#[derive(Structural,Debug,Copy,Clone,PartialEq)]
#[struc(public)]
struct AnimalCounts<T>{
    cows:T,
    chickens:T,
    pigs:T,
}

fn total_count<T>(animals:&dyn AnimalCounts_SI<T>)->T
where
    T: Clone+Add<Output=T>,
{
    let (a,b,c)=animals.cloned_fields(fp!( cows, chickens, pigs ));
    a + b + c
}

{
    let count=total_count(&Foo{
        value:AnimalCounts{
            cows:100,
            chickens:200,
            pigs:300,
        }
    });

    assert_eq!( count, 600 );
}

// This doesn't compile because
// AddableString doesn't satisfy the Copy bound added by `mut_bound="T:Copy"`
/*
{
    let count=total_count(&Foo{
        value: AnimalCounts::<AddableString> {
            cows: "foo".into(),
            chickens: "bar".into(),
            pigs: "baz".into(),
        }
    });

    assert_eq!( count.0, "foobarbaz" );
}
*/


#[derive(Debug,Clone,PartialEq)]
struct AddableString(String);

impl<'s> From<&'s str> for AddableString{
    fn from(s:&'s str)-> AddableString {
        AddableString( s.to_string() )
    }
}

impl Add for AddableString{
    type Output=Self;

    fn add(self,other:Self)->Self{
        AddableString( self.0 + other.0.as_str() )
    }
}

```


*/

pub mod optional_accessors;
