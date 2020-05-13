#![cfg_attr(feature = "nightly_impl_fields", feature(associated_type_bounds))]
#![cfg_attr(feature = "nightly_specialization", feature(specialization))]
#![allow(non_camel_case_types)]
// The associated constants from this crate use trait bounds,
// so they can't be translated to `const fn`.
// Also,the constants don't use cell types,they're just generic.
#![allow(clippy::declare_interior_mutable_const)]
// This triggers for types that represent values, like `NestedFieldPath<(TS!(0), TS!(1))>`,
// so it's mostly noise in this crate.
#![allow(clippy::type_complexity)]
// This lint is silly
#![allow(clippy::blacklisted_name)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::shadow_unrelated)]
#![deny(clippy::wildcard_imports)]
// I am using `.clone()` to indicate that I want an independent copy,not a move.
#![allow(clippy::clone_on_copy)]
// Why is this linted against in `#[test]` functions?
#![allow(clippy::redundant_clone)]
// Long tests are all composed of independent statements.
#![allow(clippy::cognitive_complexity)]
// Every instance of this warning in tests is wrong
#![allow(clippy::let_unit_value)]
#![deny(rust_2018_idioms)]

mod submodules {
    mod accessing_fields;
    mod accessing_many_fields;
    mod delegation;
    mod enum_derive;
    mod field_cloner;
    mod from_structural;
    mod impl_struct;
    mod into_fields;
    mod make_struct;
    mod multi_fields;
    mod multi_nested_fields;
    mod optional_fields;
    mod std_structural_aliases;
    mod structural_alias;
    mod structural_derive;
    mod switch;
}
