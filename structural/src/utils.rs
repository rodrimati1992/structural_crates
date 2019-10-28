/*!
Some helper functions.
*/

/// Used to coerce `&[T;N]` to `&[T]`.
pub const fn coerce_slice<'a,T>(slic:&'a [T])->&'a [T]{
    slic
}