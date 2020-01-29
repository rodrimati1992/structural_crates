/*!

`structural` supports enums,allowing them to be used in both static and dynamic dispatch.

# Things to keep in mind

When declaring a newtype variant(a single fieldtuple variant), 
you often should use the `#[struc(newtype(bounds="Newtype_VSI<@variant>"))]`  attribute.
Without the attribute the generated trait will require the exact wrapped type.
With the attribute,any type that satisfy the bounds can be used.
<br>
ie:`Bar::Foo((0,1))` would be  compatible with `Baz::Foo([0,1])` if the 
`#[struc(replace_bounds="ArrayVariant2<@variant,u64>")` attribute was used on the 
`Foo` variant in both types.
<br>
The `*_VSI` trait is generated for structs that derive `Structural` and 
don't have a `#[struc(no_trait)]` attribute.
This trait allows the struct to be used as an enum variant,
taking the name of the variant as a additional type parameter.<br>
Example: `Foo_VSI<'a,T,TStr!(Bar)>` is the trait for `Foo` being used as the bound 
for a `Bar` variant.



# Generated code

The Structural derive macro generates these items for enums:

- Option-returning impls for newtype variants
(variants which have the `#[struc(newtype)]` attribute).
Accessing the single field of the variant by passing `fp!(VariantName)` to 
the GetFieldExt methods.
This is not included as a bound in the `*_SI` and `*_ESI` traits generated for the enum.

- Option-returning variant accessor impls for the every variant
(accessed with `fp!(::VariantName)`) which return the VariantProxy type.

- Option-returning accessor impls for variant fields 
(accessed with `fp!(::VariantName.field)`).

- IsVariant impls for every variant,
to query whether the enum is a particular variant with `.ìs_variant(fp!(Foo))`.

- A `<DerivingType>_SI` trait,aliasing the traits implemented by the enum,
this allows the variant name and count of types bounded by it to be 
a superset of `<DerivingType>`.
If you match on a type bounded by this trait inside the `switch` macro,
you'll be required to have a default branch (eg:`_=>{}`).

- A `<DerivingType>_ESI` trait,aliasing the traits implemented by the enum,
also requiring that the variant name and count match exactly with `<DerivingType>`.
This is useful for doing exhaustive matching inside the `switch` macro.

<br>
`VariantProxy<Enum,FP!(NameOfVariant)>`
has accessors impls for all the fields of the `NameOfVariant` variant of the enum that it wraps
(accessed with `fp!(field_name)`),
those accessors are only optional if the field is marked as optional.

# Examples

### Exhaustiveness

This example demonstrates the `switch` macro,
and the difference between the `*_SI`(nonexhaustive enum) and `*_ESI`(exhaustive enum) traits.

```rust

use structural::{
    field_traits::TupleVariant2,
    GetFieldExt,Structural,
    fp,switch,
};


fn main(){
    assert_eq!( sum_fields(&Foo::Bar), Some(0));
    assert_eq!( sum_fields(&Foo::Baz{ a:77, b:23 }), Some(100));
    assert_eq!( sum_fields(&Foo::Bam((24,64))), Some(88));

    // Fields that the Foo_SI trait doesn't require are ignored by the function.
    // `Foo_SI` requires the fields declared in the `Foo` enum.
    assert_eq!( sum_fields(&Boom::Bar{ignored:0xDEAD}), Some(0));
    assert_eq!( sum_fields(&Boom::Baz(Baz{ a:77, b:23 })), Some(100));
    assert_eq!( sum_fields(&Boom::Bam(24,64)), Some(88));
    // sum_fields can't handle the `Pow` variant.
    assert_eq!( sum_fields(&Boom::Pow(66)), None);


    // This function requires the enum to implement the `Foo_ESI` trait,
    // which is `Foo_SI` with the additional requirement that the 
    // amount and name of variants is the same as `Foo`'s.
    assert_eq!( sum_fields_exhaustive_variants(&Foo::Bar), 0);
    assert_eq!( sum_fields_exhaustive_variants(&Foo::Baz{ a:77, b:23 }), 100);
    assert_eq!( sum_fields_exhaustive_variants(&Foo::Bam((24,64))), 88);

    assert_eq!( sum_fields_exhaustive_variants(&Foom::Bar), 0);
    assert_eq!( sum_fields_exhaustive_variants(&Foom::Baz{ a:77, b:23, c:1000 }), 100);
    assert_eq!( sum_fields_exhaustive_variants(&Foom::Bam((24,64,300))), 88);

    // `Boom` can't be used with the `sum_fields_exhaustive_variants` function,
    // because it has more variants than the `Foo_ESI` trait allows.
    // assert_eq!( sum_fields_exhaustive_variants(&Boom::Pow(66)), 0);
}

fn sum_fields(this: &dyn Foo_SI)->Option<u64> {
    Some(switch!{ref this;
        Bar=>0,
        Baz{a,b}=>*a as u64 + *b as u64,
        Bam(&t0,&t1)=>t0 + t1,
        // The default branch is required because `Foo_SI` allows the enum to 
        // have more variants than `Bar`,`Baz`,and `Bam`.
        _=>return None
    })
}


fn sum_fields_exhaustive_variants(this: &impl Foo_ESI)->u64 {
    // `ref` before the name of every single variant
    // is equivalent to `ref` before the matched expression.
    switch!{this;
        Bar=>0,
        ref Baz{&a,&b}=>a as u64 + b as u64,
        ref Bam(t0,t1)=>*t0 + *t1,
        // No need for a default branch,since `Foo_ESI` requires the variants 
        // to be `Bar`,`Baz`,`Bam`,and no more
    }
}


#[derive(Structural)]
enum Foo{
    Bar,
    Baz{
        a:u32,
        b:u32,
    },
    // This attribute allows tuple variants with at least `0:u64` and `1:u64` fields
    // to be used with the generated `Foo_SI` structural alias.
    // ie:`Bam(u64,u64)`,`Bam([u64;8])`,`Bam((u64,u64,String,Vec<u64>))`
    #[struc(newtype(bound="TupleVariant2<@variant,u64,u64>"))]
    Bam((u64,u64))
}

#[derive(Structural)]
#[struc(no_trait)] // The traits for this aren't used,so no point in generating them.
enum Foom{
    Bar,
    Baz{
        a:u32,
        b:u32,
        c:u64
    },
    #[struc(newtype)]
    Bam((u64,u64,u64))
}

#[derive(Structural)]
#[struc(no_trait)] // The traits for this aren't used,so no point in generating them.
enum Boom{
    Bar{
        ignored:u64,
    },
    #[struc(newtype)]
    Baz(Baz),
    Bam(u64,u64),
    Pow(u64),
}

#[derive(Structural)]
struct Baz{
    pub a:u32,
    pub b:u32,
}
```




*/
