This is the changelog,summarising changes in each version(some minor changes may be ommited).

### 0.1.0

Defined per-field accessor traits (GetField/GetFieldMut/intoField) and 
implemented them for standard library types.

Defined extension trait GetFieldExt that uses the accessor traits,
this is the intended way to call field accessor methods.

Defined Structural derive macro,to implement the accessor traits for the fields of structs.

Defined the tstr macro to instntiate a field name to use as a parameter to GetFieldExt methods.

Defined the TStr macro to pass the field name as a generic parameter.
