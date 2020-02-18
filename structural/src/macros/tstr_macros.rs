/**
Declares type aliases for `TStr_<_>`(type-level string).

`TStr_<_>` itself is hidden from the docs because this library reserves
the right to change its generic parameter from a tuple of type-level characters,
to a `&'static str` const parameter (or `&'static [char]`).

For more information on `TStr_` you can look at the docs for
[::structural::field_path::IsTStr].

# Variants

### Inline

Where the aliases are declared at the scope that the macro is invoked.

This variant cannot be invoked within functions.

Small example:
```rust
use structural::tstr_aliases;

tstr_aliases!{
    a, // Declares a type alias `a` with the "a" TStr_.
    b="b", // Declares a type alias `b` with the "b" TStr_.
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

You can use `tstr_aliases` or `TStr` to manually declare
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
        /// Type aliases for `TStr_` (type-level string)
        /// (from the structural crate).
        ///
        /// `TStr_` values can be constructed with the NEW associated constant.
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

For getting the type of a `TStr_<_>`(type-level string).

`TStr_<_>` itself is hidden from the docs because this library reserves
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
use structural::TStr;

type Foo=TStr!("foo");

type Bar=TStr!(foo); // Equivalent to `TStr!("foo")`

type Baz=TStr!(100); // Equivalent to `TStr!("100")`

```

### Space separated characters

You can call this macro with space separated characters.

This variant of the macro exists to support Rust versions before 1.40.

You can also use [`tstr_aliases`](./macro.tstr_aliases.html) macro
if you prefer string literals or identifiers to space separated characters,
and you are using Rust version older than 1.40.

Small Example:

```rust
use structural::TStr;

type Foo=TStr!(f o o);
```

# Example

**(Only works from Rust 1.40 onwards)**

This example demonstrates how `TStr` can be used to manually bound a
type parameter with the `*VariantField*` traits,to access a variant field.

*/
#[cfg_attr(not(feature = "better_macros"), doc = "```ignore")]
#[cfg_attr(feature = "better_macros", doc = "```rust")]
/**
use structural::{GetFieldExt,FP,Structural,TStr};
use structural::field_traits::{GetFieldType,GetVariantFieldType,IntoVariantFieldMut};
use structural::field_path::VariantFieldPath;

// `GetFieldType<This,FP!(::Ok.0)>` can also be written as
// `GetVariantFieldType<This,TStr!(Ok),TStr!(0)>`.
//
// `GetVariantFieldType` is useful in generic contexts where
// the name of the variant is taken  separately from the name of the field.
fn into_ok<This>(this: This)->Option<GetFieldType<This,FP!(::Ok.0)>>
where
    This: IntoVariantFieldMut<TStr!(Ok),TStr!(0)>
{
    // Equivalent to: `this.into_field(fp!(::Ok.0))`
    this.into_field(VariantFieldPath::<TStr!("Ok"),TStr!("0")>::NEW)
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
use structural::{GetField,GetFieldExt,Structural,TStr};
use structural::field_path::FieldPath1;

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

fn get_charge( this:&dyn GetField<FieldPath1<TStr!(c h a r g e)>, Ty=Charge> )-> Charge {
    this.field_(FieldPath1::<TStr!(c h a r g e)>::NEW).clone()
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
macro_rules! TStr {
    (0)=>{ $crate::TStr!(@chars 0) };
    (1)=>{ $crate::TStr!(@chars 1) };
    (2)=>{ $crate::TStr!(@chars 2) };
    (3)=>{ $crate::TStr!(@chars 3) };
    (4)=>{ $crate::TStr!(@chars 4) };
    (5)=>{ $crate::TStr!(@chars 5) };
    (6)=>{ $crate::TStr!(@chars 6) };
    (7)=>{ $crate::TStr!(@chars 7) };
    (8)=>{ $crate::TStr!(@chars 8) };
    (9)=>{ $crate::TStr!(@chars 9) };
    (_)=>{ $crate::TStr!(@chars _) };
    ( $string:literal )=>{
        $crate::_delegate_TStr!($string)
    };
    (@chars $($char:tt)*)=>{
        $crate::pmr::TStr_<($($crate::TChar!($char),)*)>
    };
    ($($char:tt)*) => {
        $crate::_delegate_TStr!($($char)*)
    };
}

#[macro_export]
#[doc(hidden)]
#[cfg(not(feature = "better_macros"))]
macro_rules! _delegate_TStr {
    ($string:literal) => {
        compile_error!(
            "\
`TStr!(\"foo\")` requires either Rust 1.40 or the \"better_macros\" cargo feature.

You can always use the `tstr_aliases` macro to declare aliases for type level strings.
        "
        )
    };
    ($($char:tt)*) => {
        $crate::TStr!(@chars $($char)*)
    };
}

#[macro_export]
#[doc(hidden)]
#[cfg(feature = "better_macros")]
macro_rules! _delegate_TStr {
    ($string:literal) => {
        $crate::_TStr_impl_!($string)
    };
    ($ident:tt) => {
        $crate::_TStr_impl_!($ident)
    };
    ($char0:tt $($char:tt)+) => {
        $crate::TStr!(@chars $char0 $($char)*)
    };
}

/**
Constructs a `TStr_` value,a type-level string used for identifiers in field paths.

For more information on `TStr_` you can look at the docs for
[::structural::field_path::IsTStr].

# Example

Here are examples of constructing field paths using this macro,
they are paired up with the `fp` macro for comparison.

```
use structural::{GetFieldExt, Structural, tstr, fp, field_path_aliases};
use structural::enums::VariantProxy;
use structural::field_path::{
    VariantField, VariantName, FieldPath, FieldPathSet, NestedFieldPathSet,
};

let tuple=( 3, 5, (8,80,800), (13,21,(34,55)), Some(('a','b','c')) );

////////////////////////////////////////////////////////////////////
////               Constructing `FieldPath`

let path_0=tstr!(0).to_path();
assert_eq!( tuple.field_(path_0), &3 );
assert_eq!( tuple.field_(fp!(0)), &3 );

let path_1=tstr!(1).to_path();
assert_eq!( tuple.field_(path_1), &5 );
assert_eq!( tuple.field_(fp!(1)), &5 );

let path_2_0=FieldPath::many((tstr!(2), tstr!(0)));
assert_eq!( tuple.field_(path_2_0), &8 );
assert_eq!( tuple.field_(fp!(2.0)), &8 );

let path_2_1=FieldPath::many((tstr!(2), tstr!(1)));
assert_eq!( tuple.field_(path_2_1), &80 );
assert_eq!( tuple.field_(fp!(2.1)), &80 );

let path_2_2=FieldPath::many((tstr!(2), tstr!(2)));
assert_eq!( tuple.field_(path_2_2), &800 );
assert_eq!( tuple.field_(fp!(2.2)), &800 );

let path_3_2_0=FieldPath::many((tstr!(3), tstr!(2), tstr!(0)));
assert_eq!( tuple.field_(path_3_2_0), &34 );
assert_eq!( tuple.field_(fp!(3.2.0)), &34 );

let path_3_2_1=FieldPath::many((tstr!(3), tstr!(2), tstr!(1)));
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
    left.field_(VariantName::new(tstr!(Left)).to_path()).unwrap();
let _:&VariantProxy<Binary, paths::Left>=
    left.field_(fp!(::Left)).unwrap();

assert_eq!( left.field_(VariantName::new(tstr!(Right)).to_path()), None);
assert_eq!( left.field_(fp!(::Right)), None);


let _:&VariantProxy<Binary, paths::Right>=
    right.field_(VariantName::new(tstr!(Right)).to_path()).unwrap();
let _:&VariantProxy<Binary, paths::Right>=
    right.field_(fp!(::Right)).unwrap();

assert_eq!( right.field_(VariantName::new(tstr!(Left)).to_path()), None);
assert_eq!( right.field_(fp!(::Left)), None);


////////////////////////////////////////////////////////////////////
////            Constructing VariantField

assert_eq!( left.field_(VariantField::new(tstr!(Left),tstr!(0)).to_path()), Some(&3) );
assert_eq!( left.field_(fp!(::Left.0)), Some(&3) );
assert_eq!( left.field_(VariantField::new(tstr!(Right),tstr!(c)).to_path()), None );
assert_eq!( left.field_(fp!(::Right.c)), None );

assert_eq!( right.field_(VariantField::new(tstr!(Right),tstr!(c)).to_path()), Some(&'a') );
assert_eq!( right.field_(fp!(::Right.c)), Some(&'a') );
assert_eq!( right.field_(VariantField::new(tstr!(Left),tstr!(0)).to_path()), None );
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
    VariantName::new(tstr!(Left)).to_path(),
    FieldPathSet::many((tstr!(0).to_path(), tstr!(1).to_path())),
);
let nested_b=NestedFieldPathSet::new(
    VariantName::new(tstr!(Right)).to_path(),
    FieldPathSet::many(( tstr!(c).to_path(), tstr!(is_true).to_path() )),
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
macro_rules! tstr {
    (0)=>{ <$crate::TStr!(@chars 0)>::NEW };
    (1)=>{ <$crate::TStr!(@chars 1)>::NEW };
    (2)=>{ <$crate::TStr!(@chars 2)>::NEW };
    (3)=>{ <$crate::TStr!(@chars 3)>::NEW };
    (4)=>{ <$crate::TStr!(@chars 4)>::NEW };
    (5)=>{ <$crate::TStr!(@chars 5)>::NEW };
    (6)=>{ <$crate::TStr!(@chars 6)>::NEW };
    (7)=>{ <$crate::TStr!(@chars 7)>::NEW };
    (8)=>{ <$crate::TStr!(@chars 8)>::NEW };
    (9)=>{ <$crate::TStr!(@chars 9)>::NEW };
    (_)=>{ <$crate::TStr!(@chars _)>::NEW };
    ( $string:literal )=>{
        $crate::_delegate_tstr!($string)
    };
    ($($char:tt)*) => {
        $crate::_delegate_tstr!($($char)*)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _delegate_tstr {
    (@emit $param:tt )=>{{
        mod dummy{
            #[allow(unused_imports)]
            use structural::pmr as __struct_pmr;
            $crate::_tstr_impl_!{$param}
        }
        dummy::VALUE
    }};
    ($string:literal) => {
        $crate::_delegate_tstr!(@emit $string )
    };
    ($ident:tt) => {
        $crate::_delegate_tstr!(@emit $ident )
    };
    ($char0:tt $($char:tt)+) => {
        <$crate::TStr!(@chars $char0 $($char)*)>::NEW
    };
}

///////////////////////////////////////////////////////////////////////////////

/*

Code to generate the non-default branches

fn main() {
    for b in 0..=255u8 {
        let c=b as char;
        if c.is_alphanumeric() && b<128 || c=='_' {
            println!("({})=>( $crate::chars::_{} );",c,c)
        }
    }
}

*/

#[doc(hidden)]
#[macro_export]
macro_rules! TChar {
    (0) => {
        $crate::chars::_0
    };
    (1) => {
        $crate::chars::_1
    };
    (2) => {
        $crate::chars::_2
    };
    (3) => {
        $crate::chars::_3
    };
    (4) => {
        $crate::chars::_4
    };
    (5) => {
        $crate::chars::_5
    };
    (6) => {
        $crate::chars::_6
    };
    (7) => {
        $crate::chars::_7
    };
    (8) => {
        $crate::chars::_8
    };
    (9) => {
        $crate::chars::_9
    };
    (A) => {
        $crate::chars::_A
    };
    (B) => {
        $crate::chars::_B
    };
    (C) => {
        $crate::chars::_C
    };
    (D) => {
        $crate::chars::_D
    };
    (E) => {
        $crate::chars::_E
    };
    (F) => {
        $crate::chars::_F
    };
    (G) => {
        $crate::chars::_G
    };
    (H) => {
        $crate::chars::_H
    };
    (I) => {
        $crate::chars::_I
    };
    (J) => {
        $crate::chars::_J
    };
    (K) => {
        $crate::chars::_K
    };
    (L) => {
        $crate::chars::_L
    };
    (M) => {
        $crate::chars::_M
    };
    (N) => {
        $crate::chars::_N
    };
    (O) => {
        $crate::chars::_O
    };
    (P) => {
        $crate::chars::_P
    };
    (Q) => {
        $crate::chars::_Q
    };
    (R) => {
        $crate::chars::_R
    };
    (S) => {
        $crate::chars::_S
    };
    (T) => {
        $crate::chars::_T
    };
    (U) => {
        $crate::chars::_U
    };
    (V) => {
        $crate::chars::_V
    };
    (W) => {
        $crate::chars::_W
    };
    (X) => {
        $crate::chars::_X
    };
    (Y) => {
        $crate::chars::_Y
    };
    (Z) => {
        $crate::chars::_Z
    };
    (_) => {
        $crate::chars::__
    };
    (a) => {
        $crate::chars::_a
    };
    (b) => {
        $crate::chars::_b
    };
    (c) => {
        $crate::chars::_c
    };
    (d) => {
        $crate::chars::_d
    };
    (e) => {
        $crate::chars::_e
    };
    (f) => {
        $crate::chars::_f
    };
    (g) => {
        $crate::chars::_g
    };
    (h) => {
        $crate::chars::_h
    };
    (i) => {
        $crate::chars::_i
    };
    (j) => {
        $crate::chars::_j
    };
    (k) => {
        $crate::chars::_k
    };
    (l) => {
        $crate::chars::_l
    };
    (m) => {
        $crate::chars::_m
    };
    (n) => {
        $crate::chars::_n
    };
    (o) => {
        $crate::chars::_o
    };
    (p) => {
        $crate::chars::_p
    };
    (q) => {
        $crate::chars::_q
    };
    (r) => {
        $crate::chars::_r
    };
    (s) => {
        $crate::chars::_s
    };
    (t) => {
        $crate::chars::_t
    };
    (u) => {
        $crate::chars::_u
    };
    (v) => {
        $crate::chars::_v
    };
    (w) => {
        $crate::chars::_w
    };
    (x) => {
        $crate::chars::_x
    };
    (y) => {
        $crate::chars::_y
    };
    (z) => {
        $crate::chars::_z
    };
    ($byte:ident) => {
        $crate::chars::$byte
    };
}