[![Build Status](https://travis-ci.org/rodrimati1992/structural_crates.svg?branch=master)](https://travis-ci.org/rodrimati1992/structural_crates) [![Join the chat at https://gitter.im/structural_crates/community](https://badges.gitter.im/structural_crates/community.svg)](https://gitter.im/structural_crates/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![](https://img.shields.io/crates/v/structural.svg)][crates-io]
[![](https://docs.rs/structural/badge.svg)][api-docs]

[crates-io]: https://crates.io/crates/structural
[api-docs]: https://docs.rs/structural


This library provides field accessor traits,and emulation of structural types.

# Features

These are the features this library provides:

- [Derivation of the 3 accessor traits for every public field](https://docs.rs/structural/0.2/structural/docs/structural_macro/index.html)
(GetField/GetFieldMut/IntoField).

- [Declaration of trait aliases for accessor trait bounds,using field-in-trait syntax,with the `structural_alias` macro.
](https://docs.rs/structural/0.2/structural/macro.structural_alias.html).

- [Construction of anonymous structs with `make_struct`](https://docs.rs/structural/0.2/structural/macro.make_struct.html)

# Changelog

The changelog is in the "Changelog.md" file.

# Examples

TODO: link to future docs.rs urls

# Future plans

None right now.

# no-std support

To use `structural` in no_std contexts disable the default-feature.

```toml
structural={version="<insert_version_number_here>",default_features=false}
```

This crate has few items that require the standard library (instead of core/alloc),
it is required by default so that users that are not aware of the core/alloc libraries don't have 
to pass a feature to enable std support.

# Cargo Features

These are the cargo features in structural:

- `std`: Enables std support,this is enabled by default.

- `alloc`:
    Enables alloc crate support,this is enabled by default.

- `specialization`:
    Enables specialization inside structural,without enabling the nightly feature flag.
    This is for the case that specialization is stabilized after the last update to this library.

- `nightly_specialization`:
    Enables specialization inside structural,
    requires nightly because it enables the nightly feature.

- `impl_fields`:
    This allows using `field_name:impl Foo` fields in the `structural_alias` macro,
    which as of 2019-11-23 requires the `associated_type_bounds` Rust nightly feature.<br>
    If this doesn't work,try using the "nightly_impl_fields" feature in Rust nightly instead.

- `nightly_impl_fields`
    Equivalent to the `impl_fields` feature,
    as well as enabling the `associated_type_bounds` nightly features required
    for using the feature as of 2019-11-23.

- `use_const_str`:
    Changes the internal implementation of `TStr` (the type level string type)
    to use a `&'static str` const parameter instead of types.<br>
    Use this if const generics (eg:`struct S<const S: &'static str>;`) are usable on stable.

- `nightly_use_const_str`:
    Equivalent to the `impl_fields` feature,
    which also enables the nightly Rust features required for const generics as of 2020-03-01.



Specialization is used inside `structural` for performance reasons.
There are no benchmarks comparing when specialization is enabled and disabled yet.

# Minimum Rust version

This crate support Rust back to 1.40,
and uses a build script to automatically enable features from newer versions.
