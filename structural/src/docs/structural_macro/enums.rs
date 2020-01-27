/*!

`structural` supports enums,allowing them to be used in both static and dynamic dispatch.

# Things to keep in mind

The `*_SI` and `structural_aliases` generated traits are by default non-exhaustive,
and so require the default case for the `switch!` macro to evaluate to non-`()`.
<br>
The `*_ESI` trait (generated for enums by the Structural derive macro) and 
`structural_aliases` generated traits with `VariantCount` as a supertrait are exhaustive enums,
which means that they don't require a default case for the `switch!` macro to 
evaluate to non-`()`.
If there is no default case switch will require exhaustive enums to 
be exhaustively matched,otherwise causing a compile-time error.



When declaring a newtype variant(a single fieldtuple variant), 
you often should use the `#[struc(newtype(bounds="Newtype_VSI<@variant>"))]`  attribute.
Without the attribute the generated trait will require the exact wrapped type.
With the attribute,any type that satisfy the bounds can be used.
<br>
ie:`Bar::Foo((0,1))` would be  compatible with `Baz::Foo([0,1])` if the 
`#[struc(replace_bounds="ArrayVariant2<@variant,u64>")` attribute was used on the 
`Foo` variant in both types.

# Impls

The Structural derive macro generates these items for enums:

- Option-returning variant accessor impls for newtype variants
(variants which have the `#[struc(newtype)]` attribute),
(accessed with `fp!(VariantName)`)for accessing the single field of the variant,
this is not included as a bound in the `*_SI` and `*_ESI` traits generated for the enum.

- Option-returning variant accessor impls for the every variant
(accessed with `fp!(::VariantName)`) which return the VariantProxy type.

- Option-returning accessor impls for Individual fields 
(accessed with `fp!(::VariantName.field)`).

- IsVariant impls for every variant,
to query whether the enum is a particular variant with `.ìs_variant(fp!(Foo))`.

- A `<DerivingType>_SI` trait,aliasing the traits implemented by the enum.
<br>
If `#[struc(newtype(bounds="Foo"))]`  is used on a variant,
then the bound for the accessor of the newtype variant 
(accessed with `fp!(VariantName)`)
is replaced with the bounds passed to the attribute.

`VariantProxy<Enum,FP!(NameOfVariant)>`
has accessors impls for all the fields of the variant that it wraps
(accessed with `fp!(field_name)`),
those accessors are only optional if the field is marked as optional.

# Example 

```rust

use structural::{
    field_traits::TupleVariant2,
    GetFieldExt,
    Structural,
    switch,
};

fn sum_fields(this: &dyn Foo_SI)->Option<u64> {
    Some(switch!{ this;
        Bar=>0,
        Baz=>{
            let (a,b)=this.fields(fp!(a,b));
            (*a as u64) + (*b as u64)
        }
        Bam=>{
            let tup=this.fields(fp!(0,1));
            tup.0 + tup.1
        }
        _=>return None
    })
}

fn sum_fields_exact(this: &dyn Foo_ESI)->Option<u64> {
    Some(switch!{ this;
        Bar=>0,
        Baz=>{
            let (a,b)=this.fields(fp!(a,b));
            (*a as u64) + (*b as u64)
        }
        Bam=>{
            let tup=this.fields(fp!(0,1));
            tup.0 + tup.1
        }
    })
}


#[derive(Structural)]
enum Foo{
    Bar,
    Baz{
        a:u32,
        b:u32,
    },
    // This attribute allows tuple variants with at least `0` and `1` fields
    // to be used with the generated `Foo_SI` structural alias.
    // ie:`Bam(u64,u64)`,`Bam([u64;8])`,`Bam((u64,u64,String,Vec<u64>))`
    #[struc(newtype("TupleVariant2<@variant,u64,u64>"))]
    Bam((u64,u64))
}

assert_eq!( sum_fields(&Foo::Bar), Some(0));
assert_eq!( sum_fields(&Foo::Baz{ a:77, b:23 }), Some(100));
assert_eq!( sum_fields(&Foo::Bam((24,64))), Some(88));

#[derive(Structural)]
enum Boom{
    Bar{
        ignored:u64,
    },
    #[struc(newtype(bounds="Baz_VSI<@variant>"))]
    Baz(Baz),
    Bam(u64,u64),
    Pow(u64),
}

#[derive(Structural)]
struct Baz{
    pub a:u32,
    pub b:u32,
}

// Fields that the Foo_SI trait doesn't require are ignored by the function.
assert_eq!( sum_fields(&Boom::Bar{ignored:0xDEAD}), Some(0));
assert_eq!( sum_fields(&Boom::Baz(Baz{ a:77, b:23 })), Some(100));
assert_eq!( sum_fields(&Boom::Bam(24,64)), Some(88));
// sum_fields can't handle the `Pow` variant.
assert_eq!( sum_fields(&Boom::Pow(66)), None);

```


*/