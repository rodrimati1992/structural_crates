This is the changelog,summarising changes in each version(some minor changes may be ommited).

### 0.2.0

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

    - Added `field_name_param=( field_name : FieldName );` argument.

    - Added `as_delegating_raw{ ..... }` argument,
      to get a raw pointer to the delegated to variable.


Implements accessor traits for arrays up to 32 elements.

Added type-level integers and std::cmp::{Ordering,Ord} equivalents,
for use by array impls of accessor traits.

Hid {TString,TList,TNil,chars},turning them into an implementation detail of `structural`.
For `TString` this is so that it can be replaced with 
`pub struct TString<const STR:&'static str>;`.

For advanced users:
Added traits and types for manipulating field paths on the type-level,
inside of `structural::type_level::collection_traits`.


### 0.1.0

Declared per-field accessor traits (GetField/GetFieldMut/intoField/IntoFieldMut) and 
implemented them for standard library types.

Declared extension trait GetFieldExt that uses the accessor traits,
this is the intended way to call field accessor methods.

Declared `Structural` derive macro,
to implement the accessor traits for the fields of structs.

Declared `Structural` and `StructuralDyn` traits to describe the fields that a type has.

Declared the `ti` macro to 
instantiate field name(s) to use as a parameter to GetFieldExt methods.

Declared the `TI` macro to pass a field name as a generic parameter.

Declared the `make_struct` macro to construct an anonymous struct
which implements the `IntoFieldMut` trait for all its fields.

Declared the `declare_names_module` for declaring a module with aliases for field names.

Declared `delegate_structural_with` macro to delegate the implementation 
of the field accessor traits.

Declared some helper macros for manually implementing the `Structural` and accessor traits.
