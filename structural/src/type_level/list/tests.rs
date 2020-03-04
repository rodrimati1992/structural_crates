use crate::type_level::collection_traits::{AppendOut, FlattenOut, PushBackOut};

use core_extensions::type_asserts::AssertEq;

struct A;
struct B;
struct C;
struct D;
struct E;
struct F;
struct G;

#[test]
fn push() {
    let _: AssertEq<PushBackOut<TList![], A>, TList![A]>;
    let _: AssertEq<PushBackOut<TList![A], B>, TList![A, B]>;
    let _: AssertEq<PushBackOut<TList![A, B], C>, TList![A, B, C]>;
    let _: AssertEq<PushBackOut<TList![A, B, C], D>, TList![A, B, C, D]>;
    let _: AssertEq<PushBackOut<TList![A, B, C, D], E>, TList![A, B, C, D, E]>;
}

#[test]
fn append() {
    let _: AssertEq<AppendOut<TList![], TList![]>, TList![]>;
    let _: AssertEq<AppendOut<TList![], TList![E]>, TList![E]>;
    let _: AssertEq<AppendOut<TList![], TList![E, F]>, TList![E, F]>;
    let _: AssertEq<AppendOut<TList![], TList![E, F, G]>, TList![E, F, G]>;
    let _: AssertEq<AppendOut<TList![A], TList![]>, TList![A]>;
    let _: AssertEq<AppendOut<TList![A, B], TList![]>, TList![A, B]>;
    let _: AssertEq<AppendOut<TList![A, B, C], TList![]>, TList![A, B, C]>;
    let _: AssertEq<AppendOut<TList![A], TList![E]>, TList![A, E]>;
    let _: AssertEq<AppendOut<TList![A, B], TList![E, F]>, TList![A, B, E, F]>;
    let _: AssertEq<AppendOut<TList![A, B, C], TList![E, F, G]>, TList![A, B, C, E, F, G]>;
}

#[test]
fn flatten() {
    let _: AssertEq<FlattenOut<TList![]>, TList![]>;
    let _: AssertEq<FlattenOut<TList![TList![A]]>, TList![A]>;
    let _: AssertEq<FlattenOut<TList![TList![A], TList![]]>, TList![A]>;
    let _: AssertEq<FlattenOut<TList![TList![], TList![A]]>, TList![A]>;
    let _: AssertEq<FlattenOut<TList![TList![A], TList![B]]>, TList![A, B]>;
    let _: AssertEq<FlattenOut<TList![TList![A, B], TList![C]]>, TList![A, B, C]>;
    let _: AssertEq<FlattenOut<TList![TList![A, B], TList![C, D]]>, TList![A, B, C, D]>;
    let _: AssertEq<FlattenOut<TList![TList![A, B], TList![], TList![C, D]]>, TList![A, B, C, D]>;
    let _: AssertEq<
        FlattenOut<TList![TList![A, B], TList![C, D], TList![E, F]]>,
        TList![A, B, C, D, E, F],
    >;
}
