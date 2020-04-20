// I am using `.clone()` to indicate that I want an independent copy,not a move.
#![allow(clippy::clone_on_copy)]

// Why is this linted against in `#[test]` functions?
#![allow(clippy::redundant_clone)]

// Long tests are all composed of independent statements.
#![allow(clippy::cognitive_complexity)]

// Every instance of this warning in tests is wrong
#![allow(clippy::let_unit_value)]

mod delegation;
mod enum_derive;
mod field_paths;
mod impl_struct;
mod into_fields;
mod make_struct;
mod multi_fields;
mod multi_nested_fields;
mod optional_fields;
mod rev_field_traits;
mod structural_alias;
mod structural_derive;
mod switch;
