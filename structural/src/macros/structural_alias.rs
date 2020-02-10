/**

The `structural_alias` macro defines a trait alias for multiple field accessors.

# Syntax

```
# use structural::structural_alias;
# pub trait SuperTrait{}

structural_alias!{
    #[struc( /* attributes detailed in the attributes section */ )]
    pub trait Foo<'a,T:Copy>:SuperTrait
    where
        T:SuperTrait
    {
             a:u32,
        ref  b:T,
        mut  c:i64,
        move d:String,
        mut move e:String,
        # /*
        i:impl Bar,
        # */

        /////////////////////
        // Variants
        A{
            foo:u32,
            ref bar:u64,
        },

        ref B(
            String,
            mut Vec<()> // The last `,` is optional
        ),

        mut C(
            u64,
            mut move u64,
        ),

        move D{
            bar:&'static str
        },

        mut move E{
            baz:&'static str,
            ref bam:&'static [u8],
        },

        F,

        /////////////////////
        // Extension method
        fn hello(){}

        /////////////////////
        // Extension constant
        const FOO:usize=0;
    }
}

# fn main(){}
```

Outside of the `{...}` the trait syntax is the same as the
regular one,with the same meaning.

Inside the `{...}` is a list of fields,variants,methods,and constants,
in any order.
The fields and variants get turned into supertraits of `Foo`.
The methods and constants turn into defaulted methods and defaulted constants in `Foo`.

### Fields

- `     a:u32`:
    Corresponds to the `IntoFieldMut<FP!(a),Ty=u32>` trait,
    allowing shared,mutable,and by value access to the field.

- `ref  b:T`:
    Corresponds to the `GetFieldImpl<FP!(b),Ty=T>` shared reference
    field accessor trait.

- `mut  c:i64`:
    Corresponds to the `GetFieldMutImpl<FP!(c),Ty=i64>` mutable reference
    field accessor trait (which`itself implies `GetFieldImpl`).

- `move d:String`:
    Corresponds to the `IntoFieldImpl<FP!(d),Ty=String>` by value
    field accessor trait (which itself implies `GetFieldImpl`).

- `mut move e:String`:
    Corresponds to the `IntoFieldMut<FP!(e),Ty=String>` trait,
    allowing shared,mutable,and by value access to the field.

- `i:impl Bar`:
    Corresponds to the `IntoFieldMut<FP!(i),Ty:Bar>` trait,
    allowing shared,mutable,and by value access to
    a field that implements the Bar trait.<br>
    This requires the `nightly_impl_fields` or `impl_fields` cargo feature.

### Variants

Variants follow the same pattern as fields regarding access
(`<none>`/`ref`/`mut`/`move`/`mut move`).
```ignore
mut Foo{
    // This is equivalent to `mut foo:String`,`mut` is inherited from the variant.
    // Corresponds to `GetVariantFieldMut<TStr!(Foo),TStr!(foo),Ty= String>`
    foo:String,
    // Corresponds to `GetVariantField<TStr!(Foo),TStr!(bar),Ty= u32>`
    ref bar:u32,
    // Corresponds to `IntoVariantFieldMut<TStr!(Foo),TStr!(baz),Ty= Vec<u32>>`
    mut move baz:Vec<u32>,
},

```

### Functions and constants

You can define defaulted functions and constants.


# Attributes

These are attributes for each individual trait declared using this macro.

### `#[struc(debug_print)]`

Causes a compiletime error,printing the generated code for a trait.

### `#[struc(exhaustive_enum)]`

Makes the structural alias require an enum that has the same variants,
with at least the required fields.

Without this attribute,an enum can also have a superset of the required variants.

### `#[struc(and_exhaustive_enum)]`

Generates an additional trait alias,with this trait as a supertarit,
also requiring an exhaustive enum as described in the docs
for [`#[struc(exhaustive_enum)]`](#strucexhaustive_enum).

Variants of the attribute:

- `#[struc(and_exhaustive_enum)]`:
    The generated trait is named `<the_name_of_this_trait>_Exhaustive`

- `#[struc(and_exhaustive_enum(suffix="ident"))]`:
    The generated trait is named `<the_name_of_this_trait><suffix_parameter>`

- `#[struc(and_exhaustive_enum(name="ident"))]`
    Uses the name parameter as the name of the generated trait

Without any attributes,an enum can have a superset of the required variants.

# Supertraits

### Structural aliases as supertraits

Structural aliases are regular traits,
so you can use them as supertraits in your own traits.

The defaulted methods in `MyTrait` could move to `Fields` if MyTrait had a blanket impl,
or because the defaulted methods are not supposed to be overriden.

```
use structural::{GetFieldExt,structural_alias,fp};

structural_alias!{
    trait Fields{
        ref foo:usize,
        ref bar:String,
    }
}

trait MyTrait:Fields{
    fn multiply_foo(&self,n:usize)->usize{
        n * self.field_(fp!(foo))
    }
    fn print_bar(&self){
        println!("{}", self.field_(fp!(bar)) );
    }
}


# fn main(){}

```

### Same field names

Structural aliases can have other structural aliases as supertraits,
even ones with the same fields/variants.

An exception is when the structural alias is for exhaustive enums,
using the `#[struc(*exaustive_enum)]` attributes,
where subtraits cannot add variants,but can add fields.

In this example:

```rust
use structural::structural_alias;

structural_alias!{
    trait Point<T>{
        move x:T,
        move y:T,
    }

    trait Rectangle<T>:Point<T>{
        ref x:T,
        ref y:T,
        ref w:T,
        ref h:T,
    }
}

# fn main(){}
```
It is legal to repeat the `x` and `y` fields in subtraits,
and those fields get the most permissive access specified,
which here is shared and by value access to both `x` and `y`.


<br>

It is not legal is to redeclare the field with an incompatible type:

```compile_fail
use structural::structural_alias;

structural_alias!{
    trait Point<T>{
        x:T,
        y:T,
    }

    trait Rectangle<T>:Point<T>{
        x:usize,
        y:T,
        w:T,
        h:T,
    }
}

# fn main(){}
```


# impl Trait fields

This requires the `nightly_impl_fields` cargo feature
(or `impl_fields` if associated type bounds stabilized after the latest release).

You can declare a field with `impl Bar` as its type to declare that the field
implements Bar,without specifying a particular type.

Using `impl Trait` fields makes a `Foo` structural alias unusable as a `dyn Foo`.

### Example

This demonstrates using impl trait fields.

*/
#[cfg_attr(not(feature = "nightly_impl_fields"), doc = "```ignore")]
#[cfg_attr(feature = "nightly_impl_fields", doc = "```rust")]
/**
// Remove this if associated type bounds (eg: `T: Iterator<Item: Debug>`)
// work without it.
#![feature(associated_type_bounds)]

use structural::{structural_alias,fp,make_struct,GetFieldExt};

structural_alias!{
    trait Foo{
        foo:impl Bar,
    }

    trait Bar{
        dimension:impl Dim<u32>,
    }

    trait Dim<T>{
        width:T,
        height:T,
    }
}

fn with_foo(this:&impl Foo){
    let dim=this.field_(fp!(foo.dimension));
    assert_eq!( dim.field_(fp!(width)), &200 );
    assert_eq!( dim.field_(fp!(height)), &201 );
}


fn main(){
    with_foo(&make_struct!{
        foo:make_struct!{
            dimension:make_struct!{
                width:200,
                height:201,
            }
        }
    });
}


```

# Examples

### Defining a Point trait alias

```rust
use structural::{structural_alias,fp,GetFieldExt,Structural};

use core::{
    cmp::PartialEq,
    fmt::{Debug,Display},
};


structural_alias!{
    trait Point<T>{
        // Using `ref` because we just want to read the fields
        ref x:T,
        ref y:T,
    }
}

fn print_point<T,U>(value:&T)
where
    T:Point<U>,
    U:Debug+Display+PartialEq,
{
    // This gets references to the `x` and `y` fields.
    let (x,y)=value.fields(fp!(x,y));
    assert_ne!(x,y);
    println!("x={} y={}",x,y);
}

fn main(){

    print_point(&Point3D{ x:100, y:200, z:6000 });

    print_point(&Rectangle{ x:100, y:200, w:300, h:400 });

    print_point(&Entity{ x:100.0, y:200.0, id:PersonId(0xDEAD) });

}



#[derive(Structural)]
struct Point3D<T>{
    pub x:T,
    pub y:T,
    pub z:T,
}

#[derive(Structural)]
struct Rectangle<T>{
    pub x:T,
    pub y:T,
    pub w:T,
    pub h:T,
}

#[derive(Structural)]
struct Entity{
    pub id:PersonId,
    pub x:f32,
    pub y:f32,
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct PersonId(u64);


```

### Defining a trait aliases with all accessibilities

```
use structural::{
    structural_alias,
    fp,
    GetFieldExt,
};

structural_alias!{
    trait Person{
        // shared,mutable,and by value access to the field)
        id:PersonId,

        // shared access (a & reference to the field)
        ref name:String,

        // mutable access (a &mut reference to the field),as well as shared access.
        mut friends:Vec<PersonId>,

        // by value access to the field (as well as shared)
        move candy:Candy,

        // shared,mutable,and by value access to the field)
        mut move snack:Snack,
    }
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Seconds(u64);

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct PersonId(u64);

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Candy;

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Snack;

# fn main(){}

```

### Exhaustive enums

This also demonstrates how you can define a trait alias for exhaustive enums,
along with extension methods.

```rust

use std::path::PathBuf;
# /*
use dependency::YesWith;
# */

#[derive(Structural)]
#[struc(no_trait)]
enum WasFileFound{
    Yes(PathBuf),
    No,
}

fn find_file(name:&str)->WasFileFound{
    WasFileFound::Yes( PathBuf::from(name) )
}

fn main(){
    let filename="hello";

    // The `into_yes` method comes from the BooleanEnum trait
    if let Some(file)= find_file(filename).into_yes() {
        println!("Found '{}' at this path: {} ",filename,file.display());
    }
}


///////////////////////////////////////////////////////////////////////////////
////                    In a dependency
///////////////////////////////////////////////////////////////////////////////

use structural::{
    fp,structural_alias,switch,
    field_traits::{GetVariantFieldType,IntoVariantField},
    GetFieldExt,
    Structural,
};

structural_alias!{
    #[struc(exhaustive_enum)]
    pub trait BooleanEnum{
        Yes{},
        No{},

        fn is_yes(&self)->bool{
            switch!{ref self;
                Yes=>true,
                No=>false
            }
        }

        fn is_no(&self)->bool{
            switch!{ref self;
                No=>true,
                Yes=>false
            }
        }
    }

    pub trait YesWith<T>: BooleanEnum {
        Yes(T),

        fn into_yes(self)->Option<T>
        where
            Self: Sized
        {
            self.into_field(fp!(::Yes.0))
        }
    }
}
```

*/
#[macro_export]
macro_rules! structural_alias{
    ( $($everything:tt)* )=>{
        $crate::structural_alias_impl!{ $($everything)* }
    }
}
