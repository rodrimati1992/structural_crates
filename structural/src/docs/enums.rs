/*!

`structural` supports enums.
Structural enum traits can be both statically and dynamically dispatched.

Every instance of `<DerivingType>` in the documentation is the name of the enum.
If have a `Kind` enum,`<DerivingType>_Foo` means `Kind_Foo`.

# Generated code

<span id="default_behavior"></span>

The Structural derive macro generates these items+impls for enums:

- Implementation of the `Structural` trait for the deriving type,
with documentation describing all the accessor trait impls for the type.

- Impls of the [`GetVariantField`]/[`GetVariantFieldMut`]/[`IntoVariantField`]
variant field accessor traits (accessed with `fp!(::VariantName.field)`).

- [`IsVariant`] impls for every variant,
to query whether the enum is a particular variant with `.ìs_variant(fp!(Foo))`.

- [`VariantCount`] impl for the enum,with the amount of variants in it.<br>
This isn't generated if the `#[non_exhaustive]` attribute was used on the enum.<br>

- Enums with the `#[struc(variant_count_alias)]` attribute
have the `<DerivingType>_VC` type alias,
a [`TStr`] with the amount of variants in the enum,
which can be used in [`VariantCount`]`<Count= _ >` bounds.

- A `<DerivingType>_SI` trait,aliasing the traits implemented by the enum,
this allows using other enums that have a similar structure
(they can have more variants or more fields in the variants).
If you match on a type bounded by this trait inside the `switch` macro,
you'll be required to have a default branch (eg:`_=>{}`).<br>
This isn't generated if the `#[struc(no_trait)]` attribute was used on the enum.

- A `<DerivingType>_ESI` trait,aliasing the traits implemented by the enum,
also requiring that the variant name and count match exactly with `<DerivingType>`.
This is useful for doing exhaustive matching inside the `switch` macro.<br>
This isn't generated if either the `#[struc(no_trait)]` or `#[non_exhaustive]`
attributes were used on the enum.<br>

# Things to keep in mind


### Newtype Variants

You can move variant fields to a to struct,
then use the `#[struc(newtype(...))]` attribute on the variant to delegate the
variant fields to the struct.

For example,
you can transition from a `Bar{x:u32,y:u64}` variant to a `Bar` struct,
like it's done in this example:
```rust
use structural::{StructuralExt,Structural,fp};

#[derive(Structural)]
#[struc(public)]
struct Bar{
    x:u32,
    y:u64,
}

#[derive(Structural)]
enum Foo{
    // The `Bar_VSI` trait was generated for teh `Bar` struct by the `Structural` derive.
    #[struc(newtype(bounds="Bar_VSI<@variant>"))]
    Bar(Bar)
}

let mut foo=Foo::Bar(Bar{x:10,y:64});

assert_eq!( foo.field_(fp!(::Bar.x)), Some(&10) );
assert_eq!( foo.field_(fp!(::Bar.y)), Some(&64) );
assert_eq!( foo.fields_mut(fp!(::Bar=>x,y)), Some((&mut 10, &mut 64)) );

```

[Docs for the `#[struc(newtype)]` attribute](../structural_macro/index.html#strucnewtype)

The `*_VSI` trait is generated for structs that derive `Structural` and
don't have a `#[struc(no_trait)]` attribute,
it is for enum variants with the same structure as the struct.<br>
Example:
`Foo_VSI<'a,T,TS!(Bar)>` is the trait for a `Bar` variant with the same structure as `Foo`

### Bounds for Variants

Regarding what bounds are generated for the variant in the
`<DerivingType>_SI` and `<DerivingType>_ESI` traits:

- A regular variant will alias the `*VariantField*` trait bounds for each variant field.

- `#[struc(newtype)]` variants only get the [`IsVariant`] bound(like every variant).

- `#[struc(newtype(bounds="Foo_VSI<'a,T,@variant>"))]` variants
will get `Foo_VSI<'a,T,TS!(NameOfTheVariant)>` as the bound for the variant.<br>

Every variant also gets a [`IsVariant`] bound.

[`IsVariant`]: ../../enums/trait.IsVariant.html
[`VariantCount`]: ../../enums/trait.VariantCount.html
[`TStr`]: ../../struct.TStr.html
[`GetVariantField`]: ../../field/trait.GetVariantField.html
[`GetVariantFieldMut`]: ../../field/trait.GetVariantFieldMut.html
[`IntoVariantField`]: ../../field/trait.IntoVariantField.html

# Examples

### Accessing Fields

This example shows many of the ways that fields can be accessed.

```
use structural::{
    enums::VariantProxy,
    TS,StructuralExt,Structural,
    fp,switch,
};

use std::fmt::Debug;

fn main(){
    with_enum( FooEnum::Foo(3,false) );

    with_enum( BarEnum::Foo(3,false,5) );
}

fn with_enum<This>(mut foo:This)
where
    This: FooEnum_SI<bool> + Clone + Debug,
{

    assert_eq!( foo.field_(fp!(::Foo.0)), Some(&3) );
    assert_eq!( foo.field_(fp!(::Foo.1)), Some(&false) );

    assert_eq!( foo.field_mut(fp!(::Foo.0)), Some(&mut 3) );
    assert_eq!( foo.field_mut(fp!(::Foo.1)), Some(&mut false) );

    assert_eq!( foo.clone().into_field(fp!(::Foo.0)), Some(3) );
    assert_eq!( foo.clone().into_field(fp!(::Foo.1)), Some(false) );

    assert_eq!( foo.fields(fp!(::Foo.0, ::Foo.1)), (Some(&3),Some(&false)) );
    assert_eq!( foo.fields(fp!(::Foo=>0,1)), Some((&3,&false)) );

    assert_eq!( foo.fields_mut(fp!(::Foo.0, ::Foo.1)), (Some(&mut 3),Some(&mut false)) );
    assert_eq!( foo.fields_mut(fp!(::Foo=>0,1)), Some((&mut 3,&mut false)) );

    //////////////////////////////////////////////
    ////    Demonstrating variant proxies

    let _: &VariantProxy<This,TS!(Foo)>= foo.field_(fp!(::Foo)).unwrap();
    let _: &mut VariantProxy<This,TS!(Foo)>= foo.field_mut(fp!(::Foo)).unwrap();
    {
        let mut proxy: VariantProxy<This,TS!(Foo)>=
            foo.clone().into_field(fp!(::Foo)).unwrap();

        assert_eq!( proxy.field_(fp!(0)), &3 );
        assert_eq!( proxy.field_mut(fp!(0)), &mut 3 );
        assert_eq!( proxy.clone().into_field(fp!(0)), 3 );

        assert_eq!( proxy.field_(fp!(1)), &false );
        assert_eq!( proxy.field_mut(fp!(1)), &mut false );
        assert_eq!( proxy.clone().into_field(fp!(1)), false );

        assert_eq!( proxy.fields(fp!(0, 1)), (&3,&false) );
        assert_eq!( proxy.fields_mut(fp!(0, 1)), (&mut 3,&mut false) );

        assert_eq!( proxy.fields(fp!(=>0,1)), (&3,&false) );
        assert_eq!( proxy.fields_mut(fp!(=>0,1)), (&mut 3,&mut false) );
    }

    //////////////////////////////////////////////
    ////    Demonstrating the `switch` macro

    switch!{foo;
        ref Foo(f0,f1)=>{
            assert_eq!( f0, &3 );
            assert_eq!( f1, &false );

            // `foo` is a `&VariantProxy<_,_>` inside here
            let _: &VariantProxy<This,TS!(Foo)>= foo;

            assert_eq!( foo.fields(fp!(0,1)), (&3,&false) );
        }
        _=>{}
    }
    switch!{foo;
        ref mut Foo(f0,f1)=>{
            assert_eq!( f0, &mut 3 );
            assert_eq!( f1, &mut false );

            // `foo` is a `&mut VariantProxy<_,_>` inside here
            let _: &mut VariantProxy<This,TS!(Foo)>= foo;

            assert_eq!( foo.fields_mut(fp!(0,1)), (&mut 3,&mut false) );
        }
        _=>{}
    }
    switch!{variant = foo.clone();
        // Can't destructure an enum into multiple fields by value yet.
        Foo=>{
            let _: VariantProxy<This,TS!(Foo)>= variant;

            assert_eq!( variant.clone().into_field(fp!(0)), 3 );
            assert_eq!( variant.clone().into_field(fp!(1)), false );
        }
        _=>{}
    }
}

#[derive(Structural,Debug,Clone)]
enum FooEnum<T>{
    Foo(u32,T),
    Bar,
}

#[derive(Structural,Debug,Clone)]
# #[struc(no_trait)]
enum BarEnum<T>{
    Foo(u32,T,u64),
    Bar
}


```

### Exhaustiveness

This example demonstrates the `switch` macro,
and the difference between the `*_SI`(nonexhaustive enum) and `*_ESI`(exhaustive enum) traits.

```rust

use structural::{
    field::Tuple2Variant,
    StructuralExt,Structural,
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
        // This dereferences t0 and t1 in the pattern.
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
    #[struc(newtype(bounds="Tuple2Variant<u64,u64,@variant>"))]
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

# VSI Example

This example demonstrates the ways that the `*_VSI` traits
(which are generated for structs by the Structural derive macro) can be used.

```rust
use structural::{Structural,TS,switch,tstr_aliases};
use structural::enums::VariantCount;

fn main(){
    with_enum_si(EnumTupleVari::U32(3,&5));
    with_enum_si(EnumTupleVari::U64(8,&13));

    with_enum_si(EnumNoTrait::U32(Wrapper(3,&5)));
    with_enum_si(EnumNoTrait::U64(Wrapper(8,&13)));

    with_enum_si(Enum::U32(Wrapper(3,&5)));
    with_enum_si(Enum::U64(Wrapper(8,&13)));
}

// `Enum_ESI` was generated for `Enum` by the `Structural` derive macro,
// aliasing the accessor impls of `Enum`.
//
// `Enum_ESI` has supertraits that are a superset of what `with_wrapper_vsi` requires,
// so `this` can be passed without issues.
fn with_enum_si<'a>(this:impl Enum_ESI<'a>){
    with_wrapper_vsi(this)
}

// `Wrapper_VSI` was generated for `Wrapper` by the `Structural` derive macro,
// it's for enum variants with the same structure as `Wrapper`.
//
// The `Wrapper_VSI<'a,u32,TS!(U32)>` bound:
// is for a `U32` variant that's structurally equivalent to `Wrapper<'a,u32>`.
//
// The `Wrapper_VSI<'a,u64,TS!(U64)>` bound:
// is for a `u64` variant that's structurally equivalent to `Wrapper<'a,u64>`.
//
// `VariantCount<Count=TS!(2)>`
// makes this require an enum with only 2 variants
// without it the enum would be nonexhaustive,
// and the switch would require a `_=>` branch.
fn with_wrapper_vsi<'a>(
    this: impl
        Wrapper_VSI<'a,u32,TS!(U32)> +
        Wrapper_VSI<'a,u64,TS!(U64)> +
        VariantCount<Count=TS!(2)>
){
    switch!{ref this;
        U32(field0,field1)=>{
            assert_eq!(*field0,3);
            assert_eq!(**field1,5);
        }
        U64(field0,field1)=>{
            assert_eq!(*field0,8);
            assert_eq!(**field1,13);
        }
    }
}

#[derive(Structural)]
// `#[struc(no_trait)]` disables the generation of the `*_SI` and `*_ESI` traits
#[struc(no_trait)]
enum EnumTupleVari<'a>{
    U32(u32,&'a u32),
    U64(u64,&'a u64),
}


#[derive(Structural)]
#[struc(no_trait)]
enum EnumNoTrait<'a>{
    // `#[struc(newtype)]` allows accessing `Wrapper`'s fields as though they
    // were declared in the variant itself.
    // It's best to only use `#[struc(newtype)]` without any arguments
    // if the `#[struc(no_trait)]` attribute was used on the enum
    // (`#[struc(no_trait)]` disables the generation of the `*_SI` and `*_ESI` traits ),
    //
    // Not passing the `bounds` argument to this attribute causes the
    // `*_SI` trait to treat the variant as having no fields.
    // With the `bounds` argument,the bounds for the fields of
    // the variant are replaced with the bounds that were passed.
    #[struc(newtype)]
    U32(Wrapper<'a,u32>),
    #[struc(newtype)]
    U64(Wrapper<'a,u64>),
}

#[derive(Structural)]
enum Enum<'a>{
    // `#[struc(newtype)]` allows accessing `Wrapper`'s fields as though they
    // were declared in the variant itself.
    // The `bound="..."` part replaces the bounds for the variant in the
    // generated `Enum_SI` and `Enum_ESI` traits with `Wrapper_VSI<'a,u32,TS!(U32)>`
    // `Wrapper_VSI` was generated for `Wrapper` by the `Structural` derive macro,
    // it's for enum variants with the same structure as `Wrapper`.
    #[struc(newtype(bounds="Wrapper_VSI<'a,u32,@variant>"))]
    U32(Wrapper<'a,u32>),
    // The `bound="..."` here replaces the bounds for the variant in the
    // generated traits with `Wrapper_VSI<'a,u64,TS!(U64)>`
    #[struc(newtype(bounds="Wrapper_VSI<'a,u64,@variant>"))]
    U64(Wrapper<'a,u64>),
}


#[derive(Structural)]
#[struc(public)]
// This bound is unfortunately required
#[struc(bound="T:'a")]
struct Wrapper<'a,T>(T,&'a T);


```

*/
