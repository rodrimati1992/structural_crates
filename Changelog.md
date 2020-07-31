This is the changelog,summarising changes in each version(some minor changes may be ommited).

# 0.4.1

- Added opt-in FromStructural/TryFromStructural derivation,
with the `#[struc(from_structural)]` helper attribute of the `Structural` derive.

Dependencies:

- Bumped the `as_derive_utils` dependency to `0.8.3`,
which works around a false positive lifetime error in some nightly and beta rustc versions.

# 0.4.0

- Changed `Into*Field traits` to support moving out fields.

- Defined `StructuralExt::into_fields` and `StrucWrapper::vals` method,
to convert a type into multiple fields by value.

- Defined `FromStructural`,`TryFromStructural`,`Intotructural`,and `TryIntotructural` traits
to convert between structural types.

- Defined `z_impl_from_structural`,and `z_impl_try_from_structural_for_enum` macros to 
implement the structural conversion traits more conveniently.

- Implemented `*FromStructural` traits for std types that implement accessor traits.

- Defined the `StructuralExt::into_struc`,`StructuralExt::try_into_struc`,
`StrucWrapper::into_struc`,and `StrucWrapper::try_into_struc` methods,
which delegate to the structural conversion traits.

- Defined the `FieldCloner` wrapper type,
to allow types with real-only access to fields to provide by-value access to those fields,
by cloning them.

- Removed the accessor methods that take a `Box<Self>` parameter.

- Made `IntoField` unsafe to implement,to reqqire that the `move_out_field_` method doesn't
move out the same field as other impls for the same type
(the same requirement for `IntoVariantField`).

- Added `ownership` submodule in `structural::field`,
with traits and types for moving out fields and dropping the type afterwards,
the most important being:

    - `DropFields`: Methods for dropping the type before and after some fields are moved out.

    - `MovedOutFields`: 64 bit drop flags,storing which fields have been moved out.
    
    - `PrePostDropFields`: For use by implementation macros,
    to define logic that runs after some fields were moved out,
    before and after the remaining fields are dropped.

- Added `DropFields` as a supertrait of `Into*Field`,
to drop the type after some fields were been moved out.

- Added `structural::structural_aliases` module with structural aliases for std types,
moving the array and tuple structural aliases into it.

- Moved `ArrayPath` and `IsPathForArray` to the `structural::path::array_paths` module.

- Added a check that there aren't more than 64 struct/variant fields accessible by value,
since that's the amount of fields that `MovedOutFields` can track.

- Added the `ShallowFieldPath` marker trait for non-nested field paths (and path sets).

- Added the `RevMoveOutFieldImpl` trait to move out a shallow field,
`RevIntoMultiField*` to convert a type into multiple fields by value,
and `RevMoveOutMultiField*` to move out multiple shallow fields.

- Moved out the `Err` associated type out of `RevGetFieldImpl` into the new `RevFieldErr` trait,
made that a supertrait of all single field `Rev*` traits.

- Removed the lifetime parameter of `RevIntoField*`,and made `RevFieldErr` its only supertrait.

- Defined the `RevIntoFieldRef` and `OptRevIntoFieldRef` trait aliases.

- Added these attributes for the `Structural` derive:

    - `#[struc(pre_post_drop_fields)]`:
    When the type is converted into its fields by value,
    this adds calls to the `PrePostDropFields` methods before and after the remaining fields
    are dropped in `DropFields`.

    - `#[struc(pre_move="foo")]`:
    Right before type is converted into its fields by value,
    this adds a call to the `foo` function,acting as the destructor for the type.

- In updating VariantProxy to implement `DropFields`,
it aborts the process if the enum changes the active variant in its `DropFields::pre_move` impl.

- Added support for accessing from 9 up to 64 fields,
returning a tuple of (8 element) tuples when 9 or more fields are accessed.

- Defined these items to access over 8 fields:

    - `DerefNested` trait: to dereference tuples of `*mut` into tuples of `&mut`.

    - `LargePathSet` struct: to have separate implementations for 9 up to 64 fields,
    implemented for `FieldPathSet<LargePathSet<_>,_>`.

    - `field_pat` macro: to destructure tuples of accessed fields of any length,
    without writing nested tuples over 8 fields.
    
    - `path_tuple` macro: usable when constructing a `FieldPathSet<LargePathSet<_>,_>`s manually.

    - `FieldPathSet::large` function: to construct a `FieldPathSet<LargePathSet<_>,_>`,


- Fixed `fp!(foo=>bar)` reverse accessor impls.

- Defined the `RunOnDrop` helper type,to run code on Drop.

- Added support in `switch` macro for by-value destructuring of fields.

- Removed the macros to implement the `box_into_*field` methods.

- Removed the `IntoField` impls for:
    
    - `RangeInclusive`: since those couldn't be partially moved.
    
    - `ManuallyDrop`: since nested field access drops fields.

Dependencies:

- Bumped `core_extensions` dependency to `0.1.16`.

- Bumped the `rustc_version` dependency to `0.2.3`

- Bumped the `as_derive_utils` dependency to `0.8.2`

# 0.3.1

Added generic variant and field names in `structural_alias` macro,
allowing the name of a variant or field name to be determined by a type parameter or 
type alias.

# 0.3.0

Added support for enums,
defining the `GetVariantField` `GetVariantFieldMut` `IntoVariantField` traist for enums,
the `IntoVariantFieldMut` trait alias, and `GetVariantFieldType` type alias.

Moved the `GetField::Ty` to the newly created `FieldType` trait,
supertrait of both `GetField` and `GetVariantField`.

Changed output of single "path component" (non nested field/variant/variant field access)
`fp` or `FP` macro invocation from `FieldPath<(S,)>` to `S`.

Renamed `GetFieldExt` trait to `StructuralExt`.

Renamed `FieldPath` to `NestedFieldPath`,since they are always for nested fields.

Added the `StrucWrapper` wrapper type,
which defines inherent method equivalents of all `StructuralExt` methods,
with shorter method names.

Moved `type_level::field_path` to the root module and renamed it to `path`,
moved the `TStr`,`NestedFieldPath`,`FieldPathSet`
to the root module for error messages to be shorter.

Moved field accessor traits to the `structural::field` module.

Declared these structs in the root module:
- `VariantName` (what `FP!(::Variant)` desugars to),for accessing an enum variant.
- `VariantField` (what `FP!(::Variant.field)` desugars to),for accessing a variant field.
- `NestedFieldPathSet` (what `FP!(foo=>bar,baz)` desugars to),
for accessing multiple fields from inside a nested field/variant.

Made string literals/integers/identifiers usable as varianble and field names
in the field path macros,
using the string literals to implement *arbitrary identifiers*.

Added `?` syntactic sugar for field path macros,to get the value of an option field.
`?` is sugar because it's transformed into `::Some.0` as soon as it's parsed.

Fixed a few parsing errors in field path macros due to `0.1` being tokenized
as a float literal
(tricky in combination with the rest of the syntax that those macros support)

Changed representation of `NestedFieldPath`,`FieldPathSet` to store their tuple type parameter 
as a field (the same for `VariantName`,`VariantField`,`NestedFieldPathSet`),
constructible from their fields by value
(some with a constructor,some with the struct literal).<br>
Also changed how `FieldPathSet`/`NestedFieldPathSet` with a `UniquePaths` are 
constructed to using `Type::NEW.upgrade_unchecked()`.

Replaced all `core_extensions::MarkerType` impls with `core_extensions::ConstDefault`
to allow field paths to be constructed from non-zero-sized types.

Defining these trait to the generated code for enums:

- `*_SI`: Which aliases accessor trait impls of the enum.

- `*_ESI`: similar to the `*_SI` with the additional requirement that the 
variant count and names must match exactly.

- `*_VC`(opt-in): The amount of variants in the enum as a decimal `TStr`.

Added `*_VSI` trait to the generated code for structs,
to use it as the bound for an enum variant with compatible fields.

Changed the `field_path_aliases` macro to also accept a module as an argument,
removing `field_path_aliases_module`.

Exposing the `TStr` type-level string type publically.
Using `TS` to get the type(in a type context),and `ts` to construct it.

Defined the `tstr_aliases` macro,similar to `field_path_aliases` but for 
`TStr`(the type-level string type).

Defined the `VariantProxy` type to access variant fields directly.

Defined the `EnumExt` trait to construct `VariantProxy` fallibly.

Defined the `IsVariant` trait to query whether the current variant is a particular one,
adding `StructuralExt::is_variant` that delegates to this for convenience.

Defined the `VariantCount` trait to get the amount of variants as a decimal `TStr`,
eg: `TS!(9)`,`TS!(17)`,etc.

Defined the `switch` macro,to match on structural enums,either exhaustive or nonexhaustive.

Updated `structural_alias` macro to also support enums,and defaulted constants/functions.

Added these attributes to structural aliases(usable on each trait):

- `#[struc(debug_print)]`:
Panics at compile-time,printing the what structural_alias outputs for the trait.

- `#[struc(exhaustive_enum)]`: marks the structural alias as being for an exhaustive enum.

- `#[struc(and_exhaustive_enum)]`:
Creates a subtrait of this structural alias for an exhaustive versioned of the aliased enum.


Added `impl_struct` macro to get an `impl Trait` for a structural struct type,
with permutations of the `GetField` `GetFieldMut` `IntoField` `IntoFieldMut` traits.

Defined example types used in documentation examples in `for_examples`.

Removed the type metadata in `Structural` trait,
and removed `StructuralDyn` because it interacted badly with enums.

Added generated documentation for public items,and Structural trait impls.

Added these attributes to the `Structural` derive:

- `#[struc(bound="T:Trait")]`: to add bounds to the generated impls

- `#[struc(replace_bounds="Trait<@variant>")]` attribute for enum variants,
to replace the bound for the fields of the enum variant with the ones passed to the attribute.
`@variant` arguments are replaced with `TS!( Foo )`,
where Foo is the name of the variant that the attribute was used on.

- `#[struc(newtype)]` attribute for newtype enum variants,
delegating field accessors to the wrapped type.
This attribute has an optional argument to do what `#[struc(replace_bounds)]` does.

- `#[struc(variant_count_alias)]` attribute for enums,
that outputs `*_VC` (a type alias with the amount of variants in the enum as a decimal `TStr` )
along with the rest of the rest of the generated code.

- `#[struc(no_docs)]`: disables the docs in generated code.

- `#[non_exhaustive]`: 
This built-in `#[non_exhaustive]` attribute marks an enum as having non-exhaustive variants,
changing the generated code to not rely on the amount of variants of the enum,
and disallowing exhaustive matching with the `switch` macro.


Changed these attributes:

- `#[struc(delegate_to)` now accepts optional arguments for 
additional bounds on the accessor trait impls.

- `#[struc(rename="foo")`  is now also usable on variants.


Renamed the delegation macro to `unsafe_delegate_structural_with`,updated it to handle enums,
added the `specialization_params` parameter to specify how specialization of 
the raw pointer accessor method works.

Added impls of accessor traits for:

- arrays up to 32 elements.

- Option:with regular Some and None variants

- Result:with regular Ok and Err variants (eg:using `fp!(::Ok.0)` to access `T`)

Added structural aliases for arrays and tuples (up to the implemented sizes).

Improved the docs for virtually everything,
including by having examples for enums in everything that's for both enums and structs.

Replaced `IsFieldPath` and `IsFieldPathSet` with `IsSingleFieldPath` `IsMultiFieldPath`,
implementing them for all the types that implement the corresponding `Rev*` traits,
making their supertraits just `Sized`.

Changed `StructuralExt` to accept any argument implementing the appropriate `Rev*` trait,
where before it would only `*Path*` types defined in the `structural` crate.

Added back the what used to be the `GetMultiField` traits as the 
`RevGetMultiFieldImpl` and `RevGetMultiFieldMutImpl` traits.<br>
Defined `RevGetMultiField` and `RevGetMultiField`extension traits that
transform the `Result<_,_>` returned by `Rev*MultiField*Impl` traits
into `T` or `Option<T>` depending on the error type.

Removed MutRef entirely,replacing all its uses with raw pointers.

Removed the `RevIntoFieldImpl::rev_box_into_field` method

Renamed `RevGetField`/`RevGetFieldMut`/`RevIntoField` to
`RevGetFieldImpl`/`RevGetFieldMutImpl`/`RevIntoFieldImpl`,
and changed them to return a single field,<br>
Made `RevGetFieldImpl` a supertrait of both `RevGetFieldMutImpl` and `RevIntoFieldImpl`,
with RevGetFieldMut as an unsafe trait.<br>
Those traits also have trait aliases,
named [Opt]RevGetField/[Opt]RevGetFieldMut/[Opt]RevIntoField/[Opt]RevIntoFieldMut.

Defined `use_const_str` and `nightly_use_const_str` features to use const generics as an
implementation detail of `TStr`.
Also defined the `disable_const_str` feature to disable const generics if there's 
a bug in the compiler,and other libraries also use them.

Removed the `FieldPath1` type alias.

Removed `From` impls for field paths.

Removed the `utils::coerce_slice` function.

Added `NormalizeFields` trait,
for returning both optional and non-optional fields from `StructuralExt`,

Added `IsFieldErr` marker trait for the valid error types (`FailedAccess` and `InfallibleAccess`),
and the `CombinedErrs` trait to get the optionality of a nested field.

Defined the `z_raw_borrow_enum_field` macro,for raw borrowing of enums.

Defined the `z_unsafe_impl_get_vfield_raw_mut_fn` macro,
to implement function-pointer-getter methods.

Defined the `IntoAliasing` trait and `IntoAliasingOut` type alias,
to get a field path set type which can only be used to get shared access to multiple fields.


Dependencies:

- Bumped the minimum supported Rust version to 1.40,
so that proc macros in type position could be used in examples.

- Added `generational_arena` depedency to `structural_derive` crate.

- Bumped `core_extensions` dependency to `0.1.15` for `structural` crate.

# 0.2.2

Added `GetFieldExt::cloned_fields` method.

Fixed support of alloc crate.

# 0.2.0

Replaced field identifiers with field paths,
which can represent both nested fields `.a.b.c`(with the FieldPath type),
as well as accessing multiple fields `.a, .b, .foo.c` (with the FieldPathSet type).

Replaced `ti` and `TI` macros with `fp` and `FP` macros.

Simplified the definition of GetFieldMut,removing the `as_mutref` method.
Also replaced `get_field_mutref` with `get_field_raw_mut`,
which takes in an erased raw pointer,and returns a raw pointer to a field.

Removed `GetMultiField*` traits,replacing them with `RevField*` traits,
used for both nested field access (when `FieldPath` implements it),
as well as multiple field access (when `FieldPathSet` implements it).

Renamed "better_ti" and "nightly_better_ti" cargo features to
"better_macros" and "nightly_better_macros"

Fixed `make_struct` to allow nested invocations of the macro,
like `make_struct!{ foo:make_struct!{ bar:() } }`.

Added field initialization shorthand syntax to `make_struct`,
eg:`let foo=0; make_struct!{ foo }`.

Changed default field access to mutable,and by value.
(before it was defaulted to shared access).

Changed `structural_alias` to allow declaring multiple traits.

Renamed implementation macros to include a `z_` prefix,
including these:
- `z_delegate_structural_with`
- `z_impl_box_into_field_method`
- `z_unsafe_impl_get_field_raw_mut_method`

Added impls of accessor traits for `&` and `&mut`,
so that the GetFieldExt methods can be called on trait object references 
without dereferencing them.

Removed the `Structural::Fields` associated type,since it was serving no purpose.

Added a "rust_1_40" features,which enables the "better macros" feature,
and is automatically enabled by the build script.

Added `impl Trait` fields in `structural_alias` macro,
with "impl_fields" and "nightly_impl_fields" cargo features to enable them.

Added `#[struc(impl=">trait_bounds>")]` helper attribute to Structural derive macro,
which affects the generated `<deriving_type>_SI` trait,
this is equivalent to using an `impl Trait` field in the `structural_alias` macro
(including requiring the "impl_fields" to enable support for the attribute).

Added a `#[struc(delegate_to)]` helper attribute in `Structural`,
for delegating the implementation of the Structural and accessor trait impls to a field.

Added `GetFieldType2`,`GetFieldType3`,`GetFieldType4` type aliases to access
nested fields.

Added `RevGetFieldType_` trait,and `RevGetFieldType` type alias,
to query the type of a nested field.

Rewrote many examples that operate on concrete types to operate on a generic type,
inside a function.

Changes to `z_delegate_structural_with`:
    
    - Requires a trailing comma in `impl[T,]`

    - Renamed `field_ty` argument to `delegating_to_type`

    - Added `field_name_param=( field_name : FieldPath );` argument.

    - Added `as_delegating_raw{ ..... }` argument,
      to get a raw pointer to the delegated to variable.


Implements accessor traits for arrays up to 32 elements.

Added structural aliases for arrays up to 32 elements,and tuples up to 12 elements.

Added type-level integers and std::cmp::{Ordering,Ord} equivalents,
for use by array impls of accessor traits.

Hid {TString,TList,TNil,chars},turning them into an implementation detail of `structural`.
For `TString` this is so that it can be replaced with 
`pub struct TString<const STR:&'static str>;`.

For advanced users:
Added traits and types for manipulating field paths on the type-level,
inside of `structural::type_level::collection_traits`.


# 0.1.0

Declared per-field accessor traits (GetField/GetFieldMut/intoField/IntoFieldMut) and 
implemented them for standard library types.

Declared extension trait `GetFieldExt` that uses the accessor traits,
this is the intended way to call field accessor methods.

Declared `Structural` derive macro,
to implement the accessor traits for the fields of structs.

Declared `Structural` and `StructuralDyn` traits to describe the fields that a type has.

Declared the `ti` macro to 
instantiate field name(s) to use as a parameter to `GetFieldExt` methods.

Declared the `TI` macro to pass a field name as a generic parameter.

Declared the `make_struct` macro to construct an anonymous struct
which implements the `IntoFieldMut` trait for all its fields.

Declared the `declare_names_module` for declaring a module with aliases for field names.

Declared `delegate_structural_with` macro to delegate the implementation 
of the field accessor traits.

Declared some helper macros for manually implementing the `Structural` and accessor traits.
