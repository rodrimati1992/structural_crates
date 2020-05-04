/// Type-level string,used for identifiers in field paths.
///
/// This type is always zero sized.
///
/// This cannot be converted to a `&'static str` constant
/// (if you can figure out a cheap way to do that please create an issue/pull request).
///
/// # Semver concerns
/// 
/// The private `__TS` type appears as a type argument of `TStr`
/// in the output of macros from this crate,
/// the `__TS` type must not be used by name outside of the `structural` and `structural_derive`
/// crates.
///
/// Direct use of the `__TS` type will cause compilation errors
/// whenever any other crate uses the "use_const_str" cargo feature,
/// which changes `__TS` to use const generics to improve error messages.
///
/// Also,using the "use_const_str" feature to use the `__TS` type by name wouldn't 
/// protect from breakage,
/// since other crates can use the "disable_const_str" feature to disable 
/// const generics (this feature is useful to work around bugs in const generics).
///
/// # TStr type
/// 
/// You can get a TStr type (to use as a type argument) with the [`TS`](./macro.TS.html) macro,
/// which takes a string literal/ident/integer as input.
///
/// # TStr construction
///
/// `TStr<_>` can be constructed with:
///
/// - the [`ts`] macro,which takes a string literal/ident/integer as input.
///
/// - the [`fp`] macro,when a single string literal/ident/integer in passed,
/// prefer using `ts` if you want a `TStr` to always be constructed,
/// since [`fp`] can produce other types depending on the arguments.
///
/// - the [`NEW`] inherent associated constant.
///
/// - The `<TStr<_> as ConstDefault>::DEFAULT` associated constant.
///
/// Examples:
///
/// - `ts!(foo)`: TStr equivalent of "foo"
///
/// - `ts!("bar")`: TStr equivalent of "bar"
///
/// - `ts!(1)`: TStr equivalent of "1"
///
/// - `ts!(100)`: TStr equivalent of "100"
///
/// - `fp!(foo)`: TStr equivalent of "foo"
///
/// - `fp!("bar")`: TStr equivalent of "bar"
/// - `fp!("@me")`: TStr equivalent of "@me"
///
/// - `fp!(100)`: TStr equivalent of "100"
///
/// - `<TS!(0)>::NEW`: TStr equivalent of "0"
///
/// - `<TS!(0)>::DEFAULT`: TStr equivalent of "0"
/// (requires importing the `ConstDefault` trait)
///
/// - `<TS!("hello")>::NEW`: TStr equivalent of "hello"
///
/// - `<TS!(world)>::NEW`: TStr equivalent of "world"
///
/// - `<TS!(100)>::NEW`: TStr equivalent of "100"
///
///
///
///
/// 
/// # Example
/// 
/// For an example of constructing `TStr` using the [`ts`] macro,
/// and constructing other field paths with it,
/// you can look in the docs for the [`ts`] macro.
/// 
/// [`ts`]: ./macro.ts.html
/// [`fp`]: ./macro.fp.html
/// [`NEW`]: #associatedconstant.NEW
/// 
pub struct TStr<T>(pub(crate) PhantomData<T>);


/// This allows accessing the `F` field inside the `V` enum variant.
///
/// This is the type that `fp!(::Foo.bar)` constructs.
///
/// Both the `V` and `F` type parameters are [TStr](./struct.TStr.html).
///
/// # Construction
///
/// You can construct this using (not an exhaustive list):
///
/// - The [`fp`] macro, with `fp!(::Foo.bar)`
///
/// - The `VariantField{variant,field}` struct literal
///
/// - The [`new`] constructor.
///
/// - The [`NEW`] associated constant,if both `V` and `F` implement 
/// `core_extensions::ConstDefault`
/// (reexported in `structural::reexports::ConstDefault`).
///
/// # Example
///
/// ```rust
/// use structural::{StructuralExt, fp, ts};
/// use structural::for_examples::Variants;
/// use structural::path::VariantField;
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
/// [`fp`]: ./macro.fp.html
/// [`NEW`]: #associatedconstant.NEW
/// [`new`]: #method.new
///
#[derive(Copy, Clone)]
pub struct VariantField<V, F> {
    pub variant: V,
    pub field: F,
}

/// This allows accessing the `V` enum variant
/// (by constructing a [VariantProxy](./enums/struct.VariantProxy.html) representing that variant).
///
/// This is the type that `fp!(::Foo)` constructs.<br>
/// Note that `fp!(::Foo.bar)` constructs a [VariantField](./struct.VariantField.html) instead.
///
/// The `V` type parameters is a [TStr](./struct.TStr.html).
///
/// # Construction
///
/// You can construct this using (not an exhaustive list):
///
/// - [`fp`] macro,with `fp!(::Foo)`
///
/// - The `VariantName{name}` struct literal
///
/// - The [`new`] constructor.
///
/// - The [`NEW`] associated constant,if `V` implements 
/// `core_extensions::ConstDefault`
/// (reexported in `structural::reexports::ConstDefault`)
///
/// # Example
///
/// ```rust
/// use structural::{StructuralExt, fp, ts};
/// use structural::for_examples::Variants;
/// use structural::path::VariantName;
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
/// [`fp`]: ./macro.fp.html
/// [`NEW`]: #associatedconstant.NEW
/// [`new`]: #method.new
///
#[derive(Default, Copy, Clone)]
pub struct VariantName<V> {
    pub name: V,
}




/// A type-level representation of a chain of field accesses,like `.a.b.c.d`.
///
/// This is the type that `fp!(a.b)` and `fp!(::Foo.bar.baz)` construct.<br>
/// Note: `fp!(::Foo.bar)` constructs a [`VariantField`].
///
/// [`VariantField`]: ./struct.VariantField.html
///
/// # Construction
///
/// You can construct this using (not an exhaustive list):
///
/// - [`fp`] macro,when you access a nested field
///
/// - The `NestedFieldPath{list}` struct literal
///
/// - The [`one`] or [`many`] constructors.
///
/// - The [`NEW`] associated constant,if `T` implements 
/// `core_extensions::ConstDefault`
/// (reexported in `structural::reexports::ConstDefault`)
///
/// # Examples
/// 
/// You can look for examples of using this in the single-field 
/// [StructuralExt](./trait.StructuralExt.html) methods,
/// like [`field_`] and [`field_mut`].
///
/// [`field_`]: ./trait.StructuralExt.html#method.field_
/// [`field_mut`]: ./trait.StructuralExt.html#method.field_mut
/// [`fp`]: ./macro.fp.html
/// [`NEW`]: #associatedconstant.NEW
/// [`one`]: #method.one
/// [`many`]: #method.many
#[repr(transparent)]
#[derive(Default, Copy, Clone)]
pub struct NestedFieldPath<T> {
    pub list: T,
}


/// A list of field paths to access multiple fields,
/// whose uniqueness is determined by the `U` type parameter.
///
/// This is the type that `fp!(a, b.c, ::D.e, ::F)` constructs.
///
/// # Construction
///
/// You can construct this using (not an exhaustive list):
///
/// - [`fp`] macro,when you access multiple fields
/// (using `=>` constructs a [`NestedFieldPathSet`] instead).
///
/// - The [`one`], [`many`], or [`large`] constructors.
///
/// - The [`NEW`] associated constant,if `T` implements 
/// `core_extensions::ConstDefault`
/// (reexported in `structural::reexports::ConstDefault`)
///
/// # Uniqueness
///
/// If the `U` type parameter is a:
///
/// - [`UniquePaths`]: all the field paths are unique,
/// and this can be passed to `StructuralExt::fields_mut`.
///
/// - [`AliasedPaths`]: there might be repeated field paths,
/// and this cannot be passed to `StructuralExt::fields_mut`,
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
/// You can look for examples of using this in the multi-field 
/// [StructuralExt](./trait.StructuralExt.html)
/// methods, like [`fields`] and [`fields_mut`].
///
/// [`fields`]: ./trait.StructuralExt.html#method.fields
/// [`fields_mut`]: ./trait.StructuralExt.html#method.fields_mut
/// [`fp`]: ./macro.fp.html
/// [`NEW`]: #associatedconstant.NEW
/// [`one`]: #method.one
/// [`many`]: #method.many
/// [`large`]: #method.large
/// [`NestedFieldPathSet`]: ./struct.NestedFieldPathSet.html
/// [`UniquePaths`]: ./path/struct.UniquePaths.html
/// [`AliasedPaths`]: ./path/struct.AliasedPaths.html
///
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FieldPathSet<T, U> {
    // The ManuallyDrop allows every const fn to be defined as that.
    paths: ManuallyDrop<T>,
    uniqueness: PhantomData<U>,
}


/// Allows accessing multiple fields inside of some nested field.
///
/// This is most useful for accessing multiple fields inside of a (nested) enum.
///
/// This is the type that `fp!(a.b => b, c, d)` and `fp!(::Foo => bar, baz, qux)` construct.
///
/// # Uniqueness
///
/// If the `U` type parameter is a:
///
/// - [`UniquePaths`]: all the field paths are unique,
/// and this can be passed to `StructuralExt::fields_mut`.
///
/// - [`AliasedPaths`]: there might be repeated field paths,
/// and this cannot be passed to `StructuralExt::fields_mut`,
/// because it might borrow the same field mutably twice.
///
/// # Construction
///
/// NestedFieldPathSet can be constructed in these ways:
///
/// - Using the [`fp`] macro.<br>
/// Example:
/// `fp!(::Foo=>a,b)`,
/// this gets the `a`,and `b` fields from inside the `Foo` variant.<br>
/// Example:
/// `fp!(a.b=>uh,what)`,
/// this gets the `uh`,and `what` fields from inside the `a.b` field.<br>
///
/// - Constructing it from a [`NestedFieldPath`] and a [`FieldPathSet`].<br>
/// Example:
/// `NestedFieldPathSet::new( fp!(a.b.c), fp!(foo,bar,baz) )`,
/// this gets the `foo`,`bar`,and `baz` fields from inside the `a.b.c` field.<br>
/// Example:
/// `NestedFieldPathSet::new( fp!(::Foo), fp!(a,b) )`,
/// this gets the `a`,and `b` fields from inside the `Foo` variant.
///
/// - Using the [`NEW`] associated constant,
/// if `F` and `S` implements 
/// `core_extensions::ConstDefault`
/// (reexported in `structural::reexports::ConstDefault`)
/// Example: `<FP!(::Foo=>a,b,c)>::NEW`
///
/// # Drop Types
///
/// To make all the inherent methods in this type `const fn`
/// this type wraps the `NestedFieldPath<F>` inside a `ManuallyDrop`,
/// which means that `F` won't be dropped inside.
/// If that is a problem don't construct a NestedFieldPathSet with an `F`
/// that owns some resource.
///
/// # Examples
/// 
/// You can look for examples of using this in the multi-field 
/// [StructuralExt](./trait.StructuralExt.html) 
/// methods, like [`fields`] and [`fields_mut`] (look for the enum examples).
///
/// [`fields`]: ./trait.StructuralExt.html#method.fields
/// [`fields_mut`]: ./trait.StructuralExt.html#method.fields_mut
/// [`fp`]: ./macro.fp.html
/// [`NEW`]: #associatedconstant.NEW
/// [`NestedFieldPath`]: ./struct.NestedFieldPath.html
/// [`FieldPathSet`]: ./struct.FieldPathSet.html
/// [`UniquePaths`]: ./path/struct.UniquePaths.html
/// [`AliasedPaths`]: ./path/struct.AliasedPaths.html
/// 
#[derive(Debug, Clone, Copy)]
pub struct NestedFieldPathSet<F, S, U> {
    /// The path to a nested field.
    nested: ManuallyDrop<F>,
    /// The field path for fields accessed inside of the nested field.
    set: FieldPathSet<S, U>,
}

////////////////////////////////////////////////////////////////////////////////
