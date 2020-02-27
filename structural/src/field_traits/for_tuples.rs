/*!
Contains structural aliases for tuples,
with shared,mutable,and by value access to every field of the tuple.

`Tuple*` traits can be used with any tuple at least as large as the size indicated
by the trait.
You can use `Tuple3` with any tuple type starting with `(A,B,C`.
Eg:`(A,B,C)`,`(A,B,C,D)`,`(A,B,C,D,E)`,etcetera.


# `Tuple*` Example

Demonstrates that you can use the `Tuple*` trait with structs.

Note that the `Tuple*Variant` traits require the fields to have mutable and
by-value non-optional accessor traits,
satisfying the ([IntoFieldMut trait](crate::field_traits::IntoFieldMut))

```
use structural::field_traits::for_tuples::Tuple4;
use structural::{GetFieldExt,Structural, fp};

fn sum_tuple_4(tuple: impl Tuple4<u8, u16, u32, u64>) -> u64 {
    let (a, b, c, d) = tuple.fields(fp!(0, 1, 2, 3));
    *a as u64 + *b as u64 + *c as u64 + *d
}

assert_eq!(sum_tuple_4((3, 5, 8, 13)), 29);

assert_eq!(sum_tuple_4((1, 2, 4, 8, "what?")), 15);

assert_eq!(sum_tuple_4((1, 3, 9, 27, "Noooooo", "Impossible!")), 40);

assert_eq!(sum_tuple_4(MyTuple4(1, 1, 1, 1)), 4);

assert_eq!(sum_tuple_4(MyTuple5(2, 2, 2, 2, "foo".into())), 8);


#[derive(Structural)]
struct MyTuple4(pub u8,pub u16,pub u32,pub u64);

#[derive(Structural)]
struct MyTuple5(pub u8,pub u16,pub u32,pub u64, String);

```

# `Tuple*Variant` Example

Demonstrates that you can use the `Tuple*Variant` trait with enums.

Note that the `Tuple*Variant` traits require the fields to have mutable and
by-value non-optional accessor traits,
satisfying the ([IntoVariantFieldMut trait](crate::field_traits::IntoVariantFieldMut))

```
use structural::field_traits::Tuple2Variant;
use structural::{GetFieldExt,Structural,TS,fp};

use std::cmp::{Ordering,PartialEq};
use std::fmt::Debug;

fn first_2<This,T>(mut foo: This, mut not_foo: This)
where
    // `TS!(F o o)` can also be written as `TS!(Foo)` from Rust 1.40 onwards
    This: Tuple2Variant<&'static str, T, TS!(F o o)> + Copy,
    T: Debug + From<u8> + PartialEq,
{
    {
        assert_eq!( foo.fields(fp!(::Foo=>0,1)), Some(( &"heh", &T::from(88) )) );

        assert_eq!(
            foo.fields_mut(fp!(::Foo=>0,1)),
            Some(( &mut "heh", &mut T::from(88) )),
        );

        assert_eq!( foo.into_field(fp!(::Foo.0)), Some("heh") );
        assert_eq!( foo.into_field(fp!(::Foo.1)), Some(T::from(88)) );

        assert_eq!( foo.is_variant(fp!(Foo)), true );
    }
    {
        assert_eq!( not_foo.fields(fp!(::Foo=>0,1)), None );

        assert_eq!( not_foo.fields_mut(fp!(::Foo=>0,1)), None );

        assert_eq!( not_foo.into_field(fp!(::Foo.0)), None );
        assert_eq!( not_foo.into_field(fp!(::Foo.1)), None );

        assert_eq!( not_foo.is_variant(fp!(Foo)), false );
    }
}

first_2(
    Enum::Foo("heh", 88_u8, Ordering::Less),
    Enum::Bar,
);
first_2(
    OtherEnum::Foo("heh", 88_u64, false),
    OtherEnum::Bar,
);

#[derive(Structural,Copy,Clone)]
# #[struc(no_trait)]
enum Enum<T>{
    Foo(&'static str,T,Ordering),
    Bar,
}

#[derive(Structural,Copy,Clone)]
# #[struc(no_trait)]
enum OtherEnum<T>{
    Foo(&'static str,T,bool),
    Bar,
}

```


*/

pub use super::tuple_impls::*;
