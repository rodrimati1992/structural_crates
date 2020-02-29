/**
Declares type aliases for [`TStr<_>`(type-level string)](crate::TStr).

# Variants

### Inline

Where the aliases are declared at the scope that the macro is invoked.

This variant cannot be invoked within functions.

Small example:
```rust
use structural::tstr_aliases;

tstr_aliases!{
    a, // Declares a type alias `a` with the "a" TStr.
    b="b", // Declares a type alias `b` with the "b" TStr.
}
# fn main(){}
```
### Module

Where the aliases are declared inside a nested module.

This variant can be invoked within functions.

Small example:
```rust
use structural::tstr_aliases;

fn hello(){
    tstr_aliases!{
        mod hello{
            a,
            b="b",
        }
    }
}
```

# Example

Writing a function that takes a `::Foo.bar` field.

You can use `tstr_aliases` or `TS` to manually declare
variant field accessor trait bounds.

```
use structural::{
    field_traits::variant_field::GetVariantField,
    GetFieldExt,Structural,
    tstr_aliases,fp,
};

tstr_aliases!{
    mod strs{
        Foo,
        bar,
    }
}

fn takes_enum( enum_:&dyn GetVariantField< strs::Foo, strs::bar, Ty= u32 > )-> Option<u32> {
    enum_.field_(fp!(::Foo.bar)).cloned()
}

#[derive(Structural)]
enum Baz{
    Foo{ bar:u32 },
    Bar,
}

fn main(){

    assert_eq!( takes_enum(&Baz::Foo{bar:0}), Some(0) );
    assert_eq!( takes_enum(&Baz::Foo{bar:5}), Some(5) );
    assert_eq!( takes_enum(&Baz::Bar), None );

}

```


*/
#[macro_export]
macro_rules! tstr_aliases {
    (
        $(#[$attr:meta])*
        $vis:vis mod $mod_name:ident{
            $($mod_contents:tt)*
        }
    ) => (
        /// Type aliases for [`TStr`](::structural::TStr)
        /// (from the structural crate).
        ///
        /// `TStr` values can be constructed with the NEW associated constant.
        ///
        /// The source code for this module can only be accessed from
        /// the type aliases.<br>
        /// As of writing this documentation,`cargo doc` links
        /// to the inplementation of the `field_path_aliases` macro
        /// instead of where this module is declared.
        #[allow(non_camel_case_types)]
        #[allow(non_upper_case_globals)]
        #[allow(unused_imports)]
        $(#[$attr])*
        $vis mod $mod_name{
            $crate::_tstring_aliases_impl!{
                $($mod_contents)*
            }
        }
    );
    (
        $($macro_params:tt)*
    ) => (
        $crate::_tstring_aliases_impl!{
            $($macro_params)*
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

/**

For getting the type of a `TStr<_>`(type-level string).

`TStr<_>` itself is hidden from the docs because this library reserves
the right to change its generic parameter from a tuple of type-level characters,
to a `&'static str` const parameter (or `&'static [char]`).

You can also use [`tstr_aliases`](./macro.tstr_aliases.html)
to declare one or more aliases for type-level strings.

# Variants

### String literal,or identifier

From Rust 1.40 onwards you can call this macro with a string literal,
or a single identifier(which will be stringified),

Small Example:
*/
#[cfg_attr(not(feature = "better_macros"), doc = "```ignore")]
#[cfg_attr(feature = "better_macros", doc = "```rust")]
/**
use structural::TS;

type Foo=TS!("foo");

type Bar=TS!(foo); // Equivalent to `TS!("foo")`

type Baz=TS!(100); // Equivalent to `TS!("100")`

```

### Space separated characters

You can call this macro with space separated characters.

This variant of the macro exists to support Rust versions before 1.40.

You can also use [`tstr_aliases`](./macro.tstr_aliases.html) macro
if you prefer string literals or identifiers to space separated characters,
and you are using Rust version older than 1.40.

Small Example:

```rust
use structural::TS;

type Foo=TS!(f o o);
```

# Example

**(Only works from Rust 1.40 onwards)**

This example demonstrates how `TStr` can be used to manually bound a
type parameter with the `*VariantField*` traits,to access a variant field.

*/
#[cfg_attr(not(feature = "better_macros"), doc = "```ignore")]
#[cfg_attr(feature = "better_macros", doc = "```rust")]
/**
use structural::{GetFieldExt,FP,Structural,TS};
use structural::field_traits::{GetFieldType,GetVariantFieldType,IntoVariantFieldMut};
use structural::field_path::VariantFieldPath;

// `GetFieldType<This,FP!(::Ok.0)>` can also be written as
// `GetVariantFieldType<This,TS!(Ok),TS!(0)>`.
//
// `GetVariantFieldType` is useful in generic contexts where
// the name of the variant is taken  separately from the name of the field.
fn into_ok<This>(this: This)->Option<GetFieldType<This,FP!(::Ok.0)>>
where
    This: IntoVariantFieldMut<TS!(Ok),TS!(0)>
{
    // Equivalent to: `this.into_field(fp!(::Ok.0))`
    this.into_field(VariantFieldPath::<TS!("Ok"),TS!("0")>::NEW)
}

#[derive(Structural)]
# #[struc(no_trait)]
enum ResultLike<T,E>{
    Ok(T),
    Err(E),
}

assert_eq!( into_ok(ResultLike::<_,()>::Ok(99)), Some(99));
assert_eq!( into_ok(ResultLike::<(),_>::Err(99)), None);

assert_eq!( into_ok(Result::<_,()>::Ok(99)), Some(99));
assert_eq!( into_ok(Result::<(),_>::Err(99)), None);


```


# Example

This example demonstrates the space separated variant of the macro,
and uses the TStr macro to access a field,
instead of the [`FP`](./macro.FP.html) or [`fp`](./macro.fp.html) macros.

```rust
use structural::{GetField,GetFieldExt,Structural,FP,TS};

fn main(){
    let phone=CellPhone{
        memory: Bytes{ bytes:64_000_000_000 },
        charge: Charge{ percent:50 },
    };
    assert_eq!( get_charge(&phone).percent, 50 );

    let battery=Battery{
        charge: Charge{ percent:70 },
    };
    assert_eq!( get_charge(&battery).percent, 70 );
}

// An `FP!(identifier)` is the same type as `TS!(identifier)`,
// but because it's more flexible from Rust 1.40 onwards it's used for field paths by default.
// Eg:You can write `GetField<FP!(::Foo.bar)>` with `FP` but not with `TS` from 1.40 onwards.
//
// `TS` always produces the `TStr` type,
// while FP produces different types depending on the input.
fn get_charge( this:&dyn GetField<FP!(c h a r g e), Ty=Charge> )-> Charge {
    this.field_(<TS!(c h a r g e)>::NEW).clone()
}

#[derive(Structural)]
struct CellPhone{
    pub memory: Bytes,
    pub charge: Charge,
}

#[derive(Structural)]
struct Battery{
    pub charge: Charge,
}

#[derive(Debug,Copy,Clone)]
struct Bytes{
    bytes: u64,
}

#[derive(Debug,Copy,Clone)]
struct Charge{
    percent: u8,
}


```



*/
#[macro_export]
macro_rules! TS {
    (0)=>{ $crate::field_path::string_aliases::str_0 };
    (1)=>{ $crate::field_path::string_aliases::str_1 };
    (2)=>{ $crate::field_path::string_aliases::str_2 };
    (3)=>{ $crate::field_path::string_aliases::str_3 };
    (4)=>{ $crate::field_path::string_aliases::str_4 };
    (5)=>{ $crate::field_path::string_aliases::str_5 };
    (6)=>{ $crate::field_path::string_aliases::str_6 };
    (7)=>{ $crate::field_path::string_aliases::str_7 };
    (8)=>{ $crate::field_path::string_aliases::str_8 };
    (9)=>{ $crate::field_path::string_aliases::str_9 };
    (_)=>{ $crate::field_path::string_aliases::str_underscore };
    ( $string:literal )=>{
        $crate::_TStr_from_literal!($string)
    };
    (@chars $($char:tt)*)=>{
        $crate::_TStr_from_chars!( $($char)* )
    };
    ($ident:ident) => {
        $crate::_TStr_from_ident!($ident)
    };
    ($($char:tt)*) => {
        $crate::_TStr_from_chars!( $($char)* )
    };
}

//////////

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "use_const_str")]
macro_rules! _TStr_from_chars {
    ($($char:tt)*)=>{
        $crate::_TStr_from_concatenated_chars!( $($char)* )
    }
}
#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "use_const_str"))]
macro_rules! _TStr_from_chars {
    ($($char:tt)*)=>{
        $crate::TStr<$crate::p::TS<($($crate::TChar!($char),)*)>>
    }
}

//////////

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "use_const_str")]
macro_rules! _TStr_from_literal {
    ( $string:literal )=>{
        $crate::TStr<$crate::p::TS<{
            $crate::const_generic_utils::StrFromLiteral::new($string,stringify!($string))
                .str_from_lit()
        }>>
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(not(feature = "use_const_str"), feature = "better_macros"))]
macro_rules! _TStr_from_literal {
    ( $string:literal ) => {
        $crate::_TStr_impl_!($string)
    };
}

#[doc(hidden)]
#[cfg(all(not(feature = "use_const_str"), not(feature = "better_macros")))]
#[macro_export]
macro_rules! _TStr_from_literal {
    ( $string:literal ) => {
        $crate::_TStr_error! {$string}
    };
}

//////////

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "use_const_str")]
macro_rules! _TStr_from_ident {
    ( $string:ident )=>{
        $crate::TStr<$crate::p::TS<{ stringify!($string) }>>
    }
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(not(feature = "use_const_str"), feature = "better_macros"))]
macro_rules! _TStr_from_ident {
    ( $string:ident ) => {
        $crate::_TStr_impl_!($string)
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(not(feature = "use_const_str"), not(feature = "better_macros")))]
macro_rules! _TStr_from_ident {
    ( $string:ident ) => {
        $crate::_TStr_error! {$string}
    };
}

//////////

#[macro_export]
#[doc(hidden)]
macro_rules! _TStr_error{
    ($($args:tt)*)=>{
        compile_error!(concat!(
            "`TS!(",
            $(stringify!($args),)*
            ")` requires either Rust 1.40 or the \"better_macros\" cargo feature.\n\
            \n\
            You can always use the `tstr_aliases` macro to declare aliases for type level strings.\n\
            "
        ))
    }
}

/**
Constructs a
[`TStr`](::structural::field_path::TStr)
value,a type-level string used for identifiers in field paths..

# Example

Here are examples of constructing field paths using this macro,
they are paired up with the `fp` macro for comparison.

```
use structural::{GetFieldExt, Structural, ts, fp, field_path_aliases};
use structural::enums::VariantProxy;
use structural::field_path::{
    VariantField, VariantName, FieldPath, FieldPathSet, NestedFieldPathSet,
};

let tuple=( 3, 5, (8,80,800), (13,21,(34,55)), Some(('a','b','c')) );

////////////////////////////////////////////////////////////////////
////               Constructing `FieldPath`

let path_0=ts!(0);
assert_eq!( tuple.field_(path_0), &3 );
assert_eq!( tuple.field_(fp!(0)), &3 );

let path_1=ts!(1);
assert_eq!( tuple.field_(path_1), &5 );
assert_eq!( tuple.field_(fp!(1)), &5 );

let path_2_0=FieldPath::many((ts!(2), ts!(0)));
assert_eq!( tuple.field_(path_2_0), &8 );
assert_eq!( tuple.field_(fp!(2.0)), &8 );

let path_2_1=FieldPath::many((ts!(2), ts!(1)));
assert_eq!( tuple.field_(path_2_1), &80 );
assert_eq!( tuple.field_(fp!(2.1)), &80 );

let path_2_2=FieldPath::many((ts!(2), ts!(2)));
assert_eq!( tuple.field_(path_2_2), &800 );
assert_eq!( tuple.field_(fp!(2.2)), &800 );

let path_3_2_0=FieldPath::many((ts!(3), ts!(2), ts!(0)));
assert_eq!( tuple.field_(path_3_2_0), &34 );
assert_eq!( tuple.field_(fp!(3.2.0)), &34 );

let path_3_2_1=FieldPath::many((ts!(3), ts!(2), ts!(1)));
assert_eq!( tuple.field_(path_3_2_1), &55 );
assert_eq!( tuple.field_(fp!(3.2.1)), &55 );

////////////////////////////////////////////////////////////////////
////            Constructing VariantName

#[derive(Debug,Structural,PartialEq)]
# #[struc(no_trait)]
enum Binary{
    Left(u32,u32),
    Right{
        c: char,
        is_true: bool,
    },
}

let left=Binary::Left(3,5);
let right=Binary::Right{c: 'a', is_true: false};

field_path_aliases!{
    mod paths{Left,Right}
}

let _:&VariantProxy<Binary, paths::Left>=
    left.field_(VariantName::new(ts!(Left))).unwrap();
let _:&VariantProxy<Binary, paths::Left>=
    left.field_(fp!(::Left)).unwrap();

assert_eq!( left.field_(VariantName::new(ts!(Right))), None);
assert_eq!( left.field_(fp!(::Right)), None);


let _:&VariantProxy<Binary, paths::Right>=
    right.field_(VariantName::new(ts!(Right))).unwrap();
let _:&VariantProxy<Binary, paths::Right>=
    right.field_(fp!(::Right)).unwrap();

assert_eq!( right.field_(VariantName::new(ts!(Left))), None);
assert_eq!( right.field_(fp!(::Left)), None);


////////////////////////////////////////////////////////////////////
////            Constructing VariantField

assert_eq!( left.field_(VariantField::new(ts!(Left),ts!(0))), Some(&3) );
assert_eq!( left.field_(fp!(::Left.0)), Some(&3) );
assert_eq!( left.field_(VariantField::new(ts!(Right),ts!(c))), None );
assert_eq!( left.field_(fp!(::Right.c)), None );

assert_eq!( right.field_(VariantField::new(ts!(Right),ts!(c))), Some(&'a') );
assert_eq!( right.field_(fp!(::Right.c)), Some(&'a') );
assert_eq!( right.field_(VariantField::new(ts!(Left),ts!(0))), None );
assert_eq!( right.field_(fp!(::Left.0)), None );


////////////////////////////////////////////////////////////////////
////               Constructing `FieldPathSet`
////
//// Note that you can't safely construct a FieldPathSet to
//// access multiple fields mutably (which might access overlapping fields),
//// it requires calling the unsafe `upgrade_unchecked` method after
//// constructing the FieldPathSet.

// These don't have an equivalent syntax in the `fp` macro.
assert_eq!( tuple.fields(FieldPathSet::one(path_0)), (&3,) );
assert_eq!( tuple.fields(FieldPathSet::one(path_1)), (&5,) );
assert_eq!( tuple.fields(FieldPathSet::one(path_2_0)), (&8,) );
assert_eq!( tuple.fields(FieldPathSet::one(path_2_1)), (&80,) );
assert_eq!( tuple.fields(FieldPathSet::one(path_2_2)), (&800,) );

assert_eq!( tuple.fields(FieldPathSet::many((path_0, path_1))), (&3,&5) );
assert_eq!( tuple.fields(fp!(0, 1)), (&3,&5) );

assert_eq!( tuple.fields(FieldPathSet::many((path_1, path_2_0))), (&5,&8) );
assert_eq!( tuple.fields(fp!(1, 2.0)), (&5,&8) );

assert_eq!(
    tuple.fields(FieldPathSet::many((path_2_0, path_2_1, path_2_2))),
    (&8, &80, &800),
);
assert_eq!( tuple.fields(fp!(2.0, 2.1, 2.2)), (&8, &80, &800));


////////////////////////////////////////////////////////////////////
////               Constructing `NestedFieldPathSet`
////
//// Note that you can't safely construct a NestedFieldPathSet to
//// access multiple fields mutably(which might access overlapping fields),
//// it requires calling the unsafe `upgrade_unchecked` method after
//// constructing the `NestedFieldPathSet`.

let left=Binary::Left(3,5);
let right=Binary::Right{c: 'a', is_true: false};

let nested_a=NestedFieldPathSet::new(
    VariantName::new(ts!(Left)),
    FieldPathSet::many(( ts!(0), ts!(1) )),
);
let nested_b=NestedFieldPathSet::new(
    VariantName::new(ts!(Right)),
    FieldPathSet::many(( ts!(c), ts!(is_true) )),
);

assert_eq!( left.cloned_fields(nested_a), Some((3,5)) );
assert_eq!( left.cloned_fields(fp!(::Left=>0,1)), Some((3,5)) );

assert_eq!( left.cloned_fields(nested_b), None );
assert_eq!( left.cloned_fields(fp!(::Right=>c,is_true)), None );


assert_eq!( right.cloned_fields(nested_a), None );
assert_eq!( right.cloned_fields(fp!(::Left=>0,1)), None );

assert_eq!( right.cloned_fields(nested_b), Some(('a',false)) );
assert_eq!( right.cloned_fields(fp!(::Right=>c,is_true)), Some(('a',false)) );






```


*/
#[macro_export]
macro_rules! ts {
    (0)=>{ $crate::field_path::string_aliases::str_0 };
    (1)=>{ $crate::field_path::string_aliases::str_1 };
    (2)=>{ $crate::field_path::string_aliases::str_2 };
    (3)=>{ $crate::field_path::string_aliases::str_3 };
    (4)=>{ $crate::field_path::string_aliases::str_4 };
    (5)=>{ $crate::field_path::string_aliases::str_5 };
    (6)=>{ $crate::field_path::string_aliases::str_6 };
    (7)=>{ $crate::field_path::string_aliases::str_7 };
    (8)=>{ $crate::field_path::string_aliases::str_8 };
    (9)=>{ $crate::field_path::string_aliases::str_9 };
    (_)=>{ $crate::field_path::string_aliases::str_underscore };
    ( $string:literal )=>{
        $crate::_construct_tstr_from_token!($string)
    };
    ($ident:tt) => {
        $crate::_construct_tstr_from_token!($ident)
    };
    ($($char:tt)+) => {
        <$crate::_TStr_from_chars!($($char)*)>::NEW
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(any(feature = "use_const_str", feature = "better_macros"))]
macro_rules! _construct_tstr_from_token {
    ($token:tt) => {
        <$crate::TS!($token)>::NEW
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(any(feature = "use_const_str", feature = "better_macros")))]
macro_rules! _construct_tstr_from_token {
    ($token:tt) => {
        $crate::_construct_tstr_in_module!($token)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _construct_tstr_in_module {
    ( $param:tt ) => {{
        mod dummy {
            #[allow(unused_imports)]
            use structural::pmr as __struct_pmr;
            $crate::_tstr_impl_! {$param}
        }
        dummy::VALUE
    }};
}

///////////////////////////////////////////////////////////////////////////////

/*

Code to generate the non-default branches

fn main() {
    for b in 0..=255u8 {
        let c=b as char;
        if c.is_alphanumeric() && b<128 || c=='_' {
            println!("({})=>( $crate::p::_{} );",c,c)
        }
    }
}

*/

#[doc(hidden)]
#[macro_export]
macro_rules! TChar {
    (0) => {
        $crate::p::_0
    };
    (1) => {
        $crate::p::_1
    };
    (2) => {
        $crate::p::_2
    };
    (3) => {
        $crate::p::_3
    };
    (4) => {
        $crate::p::_4
    };
    (5) => {
        $crate::p::_5
    };
    (6) => {
        $crate::p::_6
    };
    (7) => {
        $crate::p::_7
    };
    (8) => {
        $crate::p::_8
    };
    (9) => {
        $crate::p::_9
    };
    (A) => {
        $crate::p::_A
    };
    (B) => {
        $crate::p::_B
    };
    (C) => {
        $crate::p::_C
    };
    (D) => {
        $crate::p::_D
    };
    (E) => {
        $crate::p::_E
    };
    (F) => {
        $crate::p::_F
    };
    (G) => {
        $crate::p::_G
    };
    (H) => {
        $crate::p::_H
    };
    (I) => {
        $crate::p::_I
    };
    (J) => {
        $crate::p::_J
    };
    (K) => {
        $crate::p::_K
    };
    (L) => {
        $crate::p::_L
    };
    (M) => {
        $crate::p::_M
    };
    (N) => {
        $crate::p::_N
    };
    (O) => {
        $crate::p::_O
    };
    (P) => {
        $crate::p::p
    };
    (Q) => {
        $crate::p::_Q
    };
    (R) => {
        $crate::p::_R
    };
    (S) => {
        $crate::p::_S
    };
    (T) => {
        $crate::p::_T
    };
    (U) => {
        $crate::p::_U
    };
    (V) => {
        $crate::p::_V
    };
    (W) => {
        $crate::p::_W
    };
    (X) => {
        $crate::p::_X
    };
    (Y) => {
        $crate::p::_Y
    };
    (Z) => {
        $crate::p::_Z
    };
    (_) => {
        $crate::p::__
    };
    (-) => {
        $crate::p::B45
    };
    (a) => {
        $crate::p::_a
    };
    (b) => {
        $crate::p::_b
    };
    (c) => {
        $crate::p::_c
    };
    (d) => {
        $crate::p::_d
    };
    (e) => {
        $crate::p::_e
    };
    (f) => {
        $crate::p::_f
    };
    (g) => {
        $crate::p::_g
    };
    (h) => {
        $crate::p::_h
    };
    (i) => {
        $crate::p::_i
    };
    (j) => {
        $crate::p::_j
    };
    (k) => {
        $crate::p::_k
    };
    (l) => {
        $crate::p::_l
    };
    (m) => {
        $crate::p::_m
    };
    (n) => {
        $crate::p::_n
    };
    (o) => {
        $crate::p::_o
    };
    (p) => {
        $crate::p::p
    };
    (q) => {
        $crate::p::_q
    };
    (r) => {
        $crate::p::_r
    };
    (s) => {
        $crate::p::_s
    };
    (t) => {
        $crate::p::_t
    };
    (u) => {
        $crate::p::_u
    };
    (v) => {
        $crate::p::_v
    };
    (w) => {
        $crate::p::_w
    };
    (x) => {
        $crate::p::_x
    };
    (y) => {
        $crate::p::_y
    };
    (z) => {
        $crate::p::_z
    };
}
