[![Build Status](https://travis-ci.org/rodrimati1992/structural_crates.svg?branch=master)](https://travis-ci.org/rodrimati1992/structural_crates) [![Join the chat at https://gitter.im/structural_crates/community](https://badges.gitter.im/structural_crates/community.svg)](https://gitter.im/structural_crates/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![](https://img.shields.io/crates/v/structural.svg)][crates-io]
[![](https://docs.rs/structural/badge.svg)][api-docs]

[crates-io]: https://crates.io/crates/structural
[api-docs]: https://docs.rs/structural


This library provides field accessor traits,and emulation of structural types.

# Features

These are some of the features this library provides:

- [`Structural`] derive macro to implement accessor traits for every public field:
[`GetField`]/[`GetFieldMut`]/[`IntoField`] for structs,
and [`GetVariantField`]/[`GetVariantFieldMut`]/[`IntoVariantField`] for enums.

- The [`StructuralExt`] extension trait,which defines the main methods to access fields,
so long as the type implements the accessor traits for those fields.

- The [`StrucWrapper`] wrapper type,defined as an alternative to [`StructuralExt`].

- The [`structural_alias`] macro, to declare trait aliases for accessor traits,
using field-in-trait syntax.

- The [`impl_struct`] macro to declare structural parameter/return types,
as well as [`make_struct`] to construct anonymous structs

- The [`FromStructural`] and [`TryFromStructural`] conversion traits,
similar (but not identical) to the standard library `From` and `TryFrom` traits
for structural types.

# Examples

For **examples** you can look at
[the examples section of the documentation for the root module of the structural crate
](https://docs.rs/structural/0.4/structural/index.html#root-mod-examples)

# Clarifications

The way that this library emulates structural types is by using traits as bounds
or trait objects.

All the `structural` traits are dyn-compatible(also known as object-safe),
and no change will be made to make them not dyn-compatible.

By default all structural types are open,
structs and enums can have more variants and or fields than are required.<br>
The only exception to this is exhaustive enums,
in which the variant count and names must match exactly,
this is useful for exhaustive matching of variants (in the [`switch`] macro).

Every trait with the `_SI`/`_ESI`/`_VSI` suffixes in the examples are traits
generated by the `Structural` derive macro.
These traits alias the accessor traits implemented by the type they're named after.

### Required macros

The only macros that are required to use this crate are the ones for [`TStr`],
every other macro expands to code that can be written manually
(except for the [`__TS`] type, that is an implementation detail that only macros from
this crate should use by name).

# Changelog

The changelog is in the "Changelog.md" file.

# Future plans

Making the [`FromStructural`] and [`TryFromStructural`] traits derivable.

# no-std support

To use `structural` in no_std contexts disable the default-feature.

```toml
structural={version="0.4",default_features=false}
```

This crate has few items that require the `std` crate (instead of core).

The "std" and "alloc" features are enabled by default so that users that are not
aware of the `core`/`alloc` crates don't have to pass a feature to enable `std` support.

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
    This allows using `field_name: impl Foo` fields in the [`structural_alias`] macro,
    which as of 2020-03-21 requires the `associated_type_bounds` Rust nightly feature.<br>
    If this doesn't work,try using the "nightly_impl_fields" feature in Rust nightly instead.

- `nightly_impl_fields`
    Equivalent to the `impl_fields` feature,
    as well as enabling the `associated_type_bounds` nightly features required
    for using the feature as of 2020-03-21.

- `use_const_str`:
    Changes the internal implementation of `TStr` (the type level string type)
    to use a `&'static str` const parameter instead of types.<br>
    Use this if const generics (eg:`struct Foo<const S: &'static str>;`) are usable on stable.

- `nightly_use_const_str`:
    Equivalent to the `use_const_str` feature,
    which also enables the nightly Rust features required for const generics as of 2020-03-21.

- `disable_const_str`:
    Disables const generics,
    useful if other crates enabling const generics causes internal errors in this Rust version.

Specialization is used inside `structural` for performance reasons.
There are no benchmarks comparing when specialization is enabled and disabled yet.

# Minimum Rust version

This crate support Rust back to 1.40,
and uses a build script to automatically enable features from newer versions.

It requires Rust 1.40 to use proc macros in type position,eg: [`TS`] and [`FP`].

# License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Structural by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. 




[`Structural`]:
https://docs.rs/structural/0.4/structural/docs/structural_macro/index.html
[`GetField`]:
https://docs.rs/structural/0.4/structural/field/trait.GetField.html
[`GetFieldMut`]:
https://docs.rs/structural/0.4/structural/field/trait.GetFieldMut.html
[`IntoField`]:
https://docs.rs/structural/0.4/structural/field/trait.IntoField.html
[`GetVariantField`]:
https://docs.rs/structural/0.4/structural/field/trait.GetVariantField.html
[`GetVariantFieldMut`]:
https://docs.rs/structural/0.4/structural/field/trait.GetVariantFieldMut.html
[`IntoVariantField`]:
https://docs.rs/structural/0.4/structural/field/trait.IntoVariantField.html

[`StrucWrapper`]: 
https://docs.rs/structural/0.4/structural/struct.StrucWrapper.html

[`StructuralExt`]:
https://docs.rs/structural/0.4/structural/trait.StructuralExt.html
[`impl_struct`]:
https://docs.rs/structural/0.4/structural/macro.impl_struct.html
[`make_struct`]:
https://docs.rs/structural/0.4/structural/macro.make_struct.html
[`structural_alias`]:
https://docs.rs/structural/0.4/structural/macro.structural_alias.html
[`switch`]:
https://docs.rs/structural/0.4/structural/macro.switch.html
[`TStr`]:
https://docs.rs/structural/0.4/structural/struct.TStr.html
[`TS`]:
https://docs.rs/structural/0.4/structural/macro.TS.html
[`FP`]:
https://docs.rs/structural/0.4/structural/macro.FP.html

[`__TS`]: ./struct.TStr.html#semver-concerns

[`FromStructural`]:
https://docs.rs/structural/0.4/structural/convert/trait.FromStructural.html

[`TryFromStructural`]:
https://docs.rs/structural/0.4/structural/convert/trait.TryFromStructural.html