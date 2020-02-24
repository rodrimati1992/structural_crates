/// Type-level string,used for identifiers in field paths.
///
///
/// This cannot be converted to a `&'static str` constant
/// (if you can figure out a cheap way to do that please create an issue/pull request).
///
/// # Semver concerns
/// 
/// `TStr` is parameterized by a private type.
///
/// Manually expanding the macros used to get a `TStr` type is not supported,
/// and is allowed to break whenever a crate uses the "use_const_str" cargo feature,
/// which changes the private type to use const generics to improve error messages.
///
/// # TStr construction
///
/// `TStr<_>` can be constructed with:
///
/// - the `ts` macro,which takes a string literal/ident/integer as an input.
///
/// - the `fp` macro,when a single string literal/ident/integer in passed,
/// prefer using `ts` if you want a `TStr` to always be constructed,
/// since `fp` can produce other types depending on the arguments.
///
/// - the `NEW` inherent associated constant,
///
/// - The `<TStr<_> as MarkerType>::MTVAL` associated constant.
///
/// Examples of constructing a `TStr<_>`:
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
/// - `fp!(foo)` (in every Rust version)
///
/// - `fp!("bar")` (in every Rust version)
///
/// - `fp!(100)` (in every Rust version)
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
/// 
/// # Example
/// 
/// For an example of constructing `TStr` using the [ts] macro,
/// and constructing other field paths with it,
/// you can look in the docs for the [ts] macro.
/// 
pub struct TStr<T>(pub(crate) PhantomData<T>);


/// This allows accessing the `F` field inside the `V` enum variant.
///
/// This is the type that `fp!(::Foo.bar)` constructs.
///
/// Both the `V` and `F` type parameters are [TStr]s,
///
/// # Construction
///
/// You can construct this using (not an exhaustive list):
///
/// - [fp] macro,with `fp!(::Foo.bar)`
///
/// - The `VariantField{variant,field}` struct literal
///
/// - The `new` constructor.
///
/// - The `NEW` associated constant,if both `V` and `F` implement [core_extensions::MarkerType].
///
/// # Example
///
/// ```rust
/// use structural::{GetFieldExt, fp, ts};
/// use structural::enums::example_enums::Variants;
/// use structural::field_path::VariantField;
///
/// let mut foo=Variants::Foo(3,5);
///
/// assert_eq!( foo.field_(fp!(::Foo.0)), Some(&3) );
/// assert_eq!( foo.field_(fp!(::Foo.1)), Some(&5) );
/// assert_eq!( foo.field_(fp!(::Boom.a)), None );
/// assert_eq!( foo.field_(fp!(::Boom.b)), None );
/// 
/// assert_eq!( foo.field_(VariantField::new(ts!(Foo), ts!(0))), Some(&3) );
/// assert_eq!( foo.field_(VariantField::new(ts!(Foo), ts!(1))), Some(&5) );
/// assert_eq!( foo.field_(VariantField::new(ts!(Boom), ts!(a))), None );
/// assert_eq!( foo.field_(VariantField::new(ts!(Boom), ts!(b))), None );
///
///
/// assert_eq!( foo.field_mut(fp!(::Foo.0)), Some(&mut 3) );
/// assert_eq!( foo.field_mut(fp!(::Foo.1)), Some(&mut 5) );
/// assert_eq!( foo.field_mut(fp!(::Boom.a)), None );
/// assert_eq!( foo.field_mut(fp!(::Boom.b)), None );
/// 
/// assert_eq!( foo.field_mut(VariantField::new(ts!(Foo), ts!(0))), Some(&mut 3) );
/// assert_eq!( foo.field_mut(VariantField::new(ts!(Foo), ts!(1))), Some(&mut 5) );
/// assert_eq!( foo.field_mut(VariantField::new(ts!(Boom), ts!(a))), None );
/// assert_eq!( foo.field_mut(VariantField::new(ts!(Boom), ts!(b))), None );
///
///
/// ```
///
#[derive(Copy, Clone)]
pub struct VariantField<V, F> {
    pub variant: V,
    pub field: F,
}

/// This allows accessing the `V` enum variant
/// (by constructing a [VariantProxy](enums::VariantProxy) representing that variant).
///
/// This is the type that `fp!(::Foo)` constructs.<br>
/// Note that `fp!(::Foo.bar)` constructs a [VariantField] instead.
///
/// The `V` type parameters is a [TStr].
///
/// # Construction
///
/// You can construct this using (not an exhaustive list):
///
/// - [fp] macro,with `fp!(::Foo)`
///
/// - The `VariantName{name}` struct literal
///
/// - The `new` constructor.
///
/// - The `NEW` associated constant,if `V` implements [core_extensions::MarkerType].
///
/// # Example
///
/// ```rust
/// use structural::{GetFieldExt, fp, ts};
/// use structural::enums::example_enums::Variants;
/// use structural::field_path::VariantName;
///
/// let mut foo=Variants::Foo(3,5);
///
/// {
///     let proxy= foo.field_(fp!(::Foo)).unwrap();
///     assert_eq!( proxy.field_(fp!(0)), &3 );
///     assert_eq!( proxy.field_(fp!(1)), &5 );
/// }
/// assert_eq!( foo.field_(fp!(::Boom)), None );
/// 
/// {
///     let proxy= foo.field_(VariantName::new(ts!(Foo))).unwrap();
///     assert_eq!( proxy.field_(fp!(0)), &3 );
///     assert_eq!( proxy.field_(fp!(1)), &5 );
/// }
/// assert_eq!( foo.field_(VariantName::new(ts!(Boom))), None );
/// 
/// 
/// {
///     let proxy= foo.field_mut(fp!(::Foo)).unwrap();
///     assert_eq!( proxy.field_mut(fp!(0)), &mut 3 );
///     assert_eq!( proxy.field_mut(fp!(1)), &mut 5 );
/// }
/// assert_eq!( foo.field_mut(fp!(::Boom)), None );
/// 
/// {
///     let proxy= foo.field_mut(VariantName::new(ts!(Foo))).unwrap();
///     assert_eq!( proxy.field_mut(fp!(0)), &mut 3 );
///     assert_eq!( proxy.field_mut(fp!(1)), &mut 5 );
/// }
/// assert_eq!( foo.field_mut(VariantName::new(ts!(Boom))), None );
/// 
/// 
///
/// ```
///
#[derive(Default, Copy, Clone)]
pub struct VariantName<V> {
    pub name: V,
}




/// A type-level representation of a chain of field accesses,like `.a.b.c.d`.
///
/// # Construction
///
/// You can construct this using (not an exhaustive list):
///
/// - [fp] macro,when you access a nested field
///
/// - The `FieldPath{list}` struct literal
///
/// - The `one` or `many` constructors.
///
/// - The `NEW` associated constant,if `T` implements [core_extensions::MarkerType].
///
/// # Examples
/// 
/// You can look for examples of using this in the single-field [GetFieldExt] methods.
#[repr(transparent)]
#[derive(Default, Copy, Clone)]
pub struct FieldPath<T> {
    pub list: T,
}


/// A list of field paths to access multiple fields,whose uniqueness is determined by `U`.
///
/// # Construction
///
/// You can construct this using (not an exhaustive list):
///
/// - [fp] macro,when you access multiple field (without using `=>`).
///
/// - The `one` or `many` constructors.
///
/// - The `NEW` associated constant,if `T` implements [core_extensions::MarkerType].
///
/// # Uniqueness
///
/// If `U` is a `UniquePaths` then all the field paths are unique,
/// and this can be passed to `GetFieldExt::fields_mut`.
///
/// If `U` is a `AliasedPaths` then there might be repeated field paths,
/// and this cannot be passed to `GetFieldExt::fields_mut`,
/// because it might borrow the same field mutably twice.
///
/// # Drop Types
///
/// To make all the inherent methods in this type `const fn`
/// this type wraps the `T` inside a `ManuallyDrop`,
/// which means that `T` won't be dropped inside.
/// If that is a problem don't construct a FieldPathSet with a `T` that owns some resource.
///
/// # Examples
/// 
/// You can look for examples of using this in the multi-field [GetFieldExt] 
/// methods.
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
/// # Uniqueness
///
/// If `U` is a `UniquePaths` then all the field paths are unique,
/// and this can be passed to `GetFieldExt::fields_mut`.
///
/// If `U` is a `AliasedPaths` then there might be repeated field paths,
/// and this cannot be passed to `GetFieldExt::fields_mut`,
/// because it might borrow the same field mutably twice.
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
/// - Constructing it from a FieldPath and a FieldPathSet.<br>
/// Example:
/// `NestedFieldPathSet::new( fp!(a.b.c), fp!(foo,bar,baz) )`,
/// this gets the `foo`,`bar`,and `baz` fields from inside the `a.b.c` field.<br>
/// Example:
/// `NestedFieldPathSet::new( fp!(::Foo), fp!(a,b) )`,
/// this gets the `a`,and `b` fields from inside the `Foo` variant.
///
/// - Using the `NEW` associated constant,
/// if `F` and `S` implements [core_extensions::MarkerType].
/// Example: `<FP!(::Foo=>a,b,c)>::NEW`
///
/// # Drop Types
///
/// To make all the inherent methods in this type `const fn`
/// this type wraps the `FieldPath<F>` inside a `ManuallyDrop`,
/// which means that `F` won't be dropped inside.
/// If that is a problem don't construct a NestedFieldPathSet with an `F`
/// that owns some resource.
///
/// # Examples
/// 
/// You can look for examples of using this in the multi-field [GetFieldExt] 
/// methods (look for the enum examples).
/// 
#[derive(Debug, Clone, Copy)]
pub struct NestedFieldPathSet<F, S, U> {
    /// The path to a nested field.
    nested: ManuallyDrop<F>,
    /// The field path for fields accessed inside of the nested field.
    set: FieldPathSet<S, U>,
}
