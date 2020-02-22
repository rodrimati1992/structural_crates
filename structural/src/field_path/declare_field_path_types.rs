/// Type-level string,used for identifiers in field paths.
///
/// This cannot be converted to a `&'static str` constant
/// (if you can figure out a cheap way to do that please create an issue/pull request).
///
/// # TStr construction
///
/// `TStr_<_>` can be constructed with:
///
/// - the `ts` macro,which takes a string literal/ident/integer as an input.
///
/// - the `NEW` inherent associated constant,
///
/// - The `<TStr_<_> as MarkerType>::MTVAL` associated constant.
///
/// Examples of constructing a `TStr_<_>`:
///
/// - `ts!(foo)` (in every Rust version)
///
/// - `ts!(f o o)` (in every Rust version)
///
/// - `ts!("bar")` (in every Rust version)
///
/// - `ts!(1)` (in every Rust version)
///
/// - `ts!(100)` (in every Rust version)
///
/// - `ts!(1 0 0)` (in every Rust version)
///
/// - `<TS!("hello")>::NEW` (from Rust 1.40 onwards)
///
/// - `<TS!(world)>::NEW` (from Rust 1.40 onwards)
///
/// - `<TS!(100)>::NEW` (from Rust 1.40 onwards)
///
/// - `<TS!(w o r l d)>::NEW` (in every Rust version)
///
/// - `<TS!(0)>::NEW` (in every Rust version)
///
/// - `<TS!(1 0 0)>::NEW` (in every Rust version)
///
/// - `<TS!(0)>::MTVAL`(requires importing the `MarkerType` trait)
pub struct TStr_<T>(pub(crate) PhantomData<T>);

/// A pair of identifiers for the `F` field inside the `V` variant.
///
/// This is the type parameter of the `FieldPath<_>` in `fp!(::Foo.bar)`.
///
/// Both the V and F type parameters are [::structural::field_path::TStr_]s,
#[derive(Copy, Clone)]
pub struct VariantField<V, F> {
    pub variant: V,
    pub field: F,
}

/// The identifier for the `V` variant.
///
/// This is the type parameter of the `FieldPath<_>` in `fp!(::Foo)`.
/// Note that `fp!(::Foo.bar)` constructs a `FieldPath<(VariantField<_,_>,)>` instead.
///
/// The V type parameters is a [::structural::field_path::TStr_]s.
#[derive(Default, Copy, Clone)]
pub struct VariantName<V> {
    pub name: V,
}




/// A type-level representation of a chain of field accesses,like `.a.b.c.d`.
///
#[repr(transparent)]
#[derive(Default, Copy, Clone)]
pub struct FieldPath<T> {
    pub list: T,
}


/// A list of `FieldPath`s whose uniqueness is determined by `U`.
///
/// If `U` is a `UniquePaths` then all the `FieldPath`s are unique,
/// and this can be passed to `GetFieldExt::fields_mut`,
/// since you can't have aliasing mutable references to the same field.
///
/// If `U` is a `AliasedPaths` then there might be repeated `FieldPath`s,
/// and this cannot be passed to `GetFieldExt::fields_mut`,
/// because it might borrow the same field mutably twice.
///
/// # Drop Types
///
/// To make all the inherent methods in this type `const fn`
/// this type wraps the `T` inside a `ManuallyDrop`,
/// which means that `T` won't be dropped inside.
/// If that is a problem don't construct a FieldPathSet with a `T` that owns some resource.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FieldPathSet<T, U> {
    // The ManuallyDrop allows every const fn to be defined as that.
    paths: ManuallyDrop<T>,
    uniqueness: PhantomData<U>,
}


/// Allows accessing multiple fields inside of some nested field.
///
/// This is useful for accessing multiple fields inside of an optional one,
/// including accessing the fields in an enum variant.
///
/// # Construction
///
/// NestedFieldPathSet can be constructed in these ways:
///
/// - Using `fp!`.<br>
/// Example:
/// `fp!(::Foo=>a,b)`,
/// this gets the `a`,and `b` fields from inside the `Foo` variant.<br>
/// Example:
/// `fp!(a.b=>uh,what)`,
/// this gets the `uh`,and `what` fields from inside the `a.b` field.<br>
///
/// - Constructing it from a FieldPath and a FieldPathSet using the struct literal.<br>
/// Example:
/// `NestedFieldPathSet::new( fp!(a.b.c), fp!(foo,bar,baz) )`,
/// this gets the `foo`,`bar`,and `baz` fields from inside the `a.b.c` field.<br>
/// Example:
/// `NestedFieldPathSet::new( fp!(::Foo), fp!(a,b) )`,
/// this gets the `a`,and `b` fields from inside the `Foo` variant.
///
/// # Drop Types
///
/// To make all the inherent methods in this type `const fn`
/// this type wraps the `FieldPath<F>` inside a `ManuallyDrop`,
/// which means that `F` won't be dropped inside.
/// If that is a problem don't construct a NestedFieldPathSet with an `F`
/// that owns some resource.
#[derive(Debug, Clone, Copy)]
pub struct NestedFieldPathSet<F, S, U> {
    /// The path to a nested field.
    nested: ManuallyDrop<FieldPath<F>>,
    /// The field path for fields accessed inside of the nested field.
    set: FieldPathSet<S, U>,
}
