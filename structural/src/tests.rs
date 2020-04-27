// I am using `.clone()` to indicate that I want an independent copy,not a move.
#![allow(clippy::clone_on_copy)]

// Why is this linted against in `#[test]` functions?
#![allow(clippy::redundant_clone)]

// Long tests are all composed of independent statements.
#![allow(clippy::cognitive_complexity)]

// Every instance of this warning in tests is wrong
#![allow(clippy::let_unit_value)]

mod field_paths;
