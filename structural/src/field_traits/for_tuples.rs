/*!
Contains structural aliases for tuples,
with shared,mutable,and by value access to every field of the tuple.

`Tuple*` traits can be used with any tuple at least as large as the size indicated
by the trait.
You can use `Tuple3` with any tuple type starting with `(A,B,C`.
Eg:`(A,B,C)`,`(A,B,C,D)`,`(A,B,C,D,E)`,etcetera.


# Example

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

*/

pub use super::tuple_impls::{
    Tuple1,
    Tuple2,
    Tuple3,
    Tuple4,
    Tuple5,
    Tuple6,
    Tuple7,
    Tuple8,
    Tuple9,
    Tuple10,
    Tuple11,
    Tuple12,
};