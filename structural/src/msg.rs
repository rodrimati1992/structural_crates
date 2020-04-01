//! These types are hacks to print error messages, when only type errors are available.
//!
//! These types are private to `structural`, and may change at any time
//! (including patch releases).
#![allow(non_camel_case_types)]

use std_::marker::PhantomData;

pub struct switch_that_matches_on_all_variants<Count>(PhantomData<Count>);

pub struct switch_that_does_not_match_on_all_variants;

pub struct switch_with_a_default_branch<T>(PhantomData<T>);

pub struct switch_without_a_default_branch<T>(PhantomData<T>);
