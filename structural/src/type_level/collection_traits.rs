/*!
Traits for type-level collections.
*/

mod push_back;
mod tuple_impls;

///////////////////////////////////////////////////////////

/// A trait which allows getting a heterogeneous collection type (like tuples),
/// in which `Type` was added after the last element.
pub trait PushBack<Type> {
    type Output;
}

/// This allows getting a heterogeneous collection type (like tuples),
/// in which `Type` was added after the last element.
pub type PushBackOut<This, Type> = <This as PushBack<Type>>::Output;

///////////////////////////////////////////////////////////

/// Gets the `TList` equivalent of `Self`.
#[doc(hidden)]
pub trait ToTList {
    type Output;
}

/// Gets the `TList` type equivalent of `This`.
#[doc(hidden)]
pub type ToTListOut<This> = <This as ToTList>::Output;

///////////////////////////////////////////////////////////

/// Gets the `TStr` equivalent of `Self`.
#[doc(hidden)]
pub trait ToTString {
    type Output;
}

/// Gets the `TStr` equivalent of `This`.
#[doc(hidden)]
pub type ToTStringOut<This> = <This as ToTString>::Output;

///////////////////////////////////////////////////////////

/// Gets a tuple type equivalent of `Self`.
pub trait ToTuple {
    type Output;
}

/// Gets a tuple type equivalent of `This`.
pub type ToTupleOut<This> = <This as ToTuple>::Output;

///////////////////////////////////////////////////////////

/// Gets this collection type with `Other` appended at the end.
pub trait Append<Other> {
    type Output;
}

/// Gets the `This` collection type with `Other` appended at the end.
pub type AppendOut<This, Other> = <This as Append<Other>>::Output;

///////////////////////////////////////////////////////////

/// FlattenOuts a collection of collection.
pub trait Flatten {
    type Output;
}

/// FlattenOuts a collection of collection.
pub type FlattenOut<This> = <This as Flatten>::Output;
