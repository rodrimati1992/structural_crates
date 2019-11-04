This is the changelog,summarising changes in each version(some minor changes may be ommited).

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
