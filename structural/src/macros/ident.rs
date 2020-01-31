/// Constructs a FieldPath(Set) value,
/// which determines the fields accessed in [GetFieldExt] methods.
///
/// When passed a single argument,this instantiates a `FieldPath`,
/// which can be passed to the
/// `GetFieldExt::{field_,field_mut,into_field,box_into_field}` methods
/// to access a field.
///
/// When passed multiple arguments,this instantiates a `FieldPathSet`.
/// It can then be passed to the `GetFieldExt::fields` method.<br>
/// To be passed to `GetFieldExt::fields_mut`,
/// `FieldPathSet` must be constructed with syntactically unique paths,
/// since there is no cheap way to check for equality of type-level strings yet.
///
/// ### Nested fields
///
/// You can construct field paths to access nested fields with `fp!(a.b.c)`,
/// where doing `this.field_(fp!(0.1.2))` is equivalent to `&((this.0).1).2`.
///
/// ### Multiple fields
///
/// You can access multiple fields simultaneously with `fp!(0,1,2)`
/// where doing `this.fields_mut(fp!(a,b,c))`
/// is equivalent to `(&mut this.a,&mut this.b,&mut this.c)`
///
///
///
///
/// # Example
///
/// ```
/// use structural::{GetFieldExt,fp,structural_alias};
///
/// structural_alias!{
///     trait Tuple3<A,B,C>{
///         0:A,
///         1:B,
///         2:C,
///     }
/// }
///
/// fn with_tuple3<'a>(tup:impl Tuple3<&'a str,&'a str,&'a str>){
///     assert_eq!( tup.field_(fp!(0)), &"I" );
///     assert_eq!( tup.field_(fp!(1)), &"you" );
///     assert_eq!( tup.field_(fp!(2)), &"they" );
///    
///     assert_eq!( tup.fields(fp!(0,1)), (&"I",&"you") );
///    
///     assert_eq!( tup.fields(fp!(0,1,2)), (&"I",&"you",&"they") );
/// }
///
/// fn main(){
///     with_tuple3(("I","you","they"));
///     with_tuple3(("I","you","they","this is not used"));
///     with_tuple3(("I","you","they","_","this isn't used either"));
/// }
/// ```
///
/// # Example
///
/// An example which accesses nested fields.
///
/// ```
/// use structural::{GetFieldExt,Structural,fp,make_struct};
///
/// #[derive(Structural)]
/// #[struc(public)]
/// struct Foo{
///     bar:Bar,
///     baz:u32,
///     ooo:(u32,u32),
/// }
///
/// #[derive(Debug,Clone,PartialEq,Structural)]
/// #[struc(public)]
/// struct Bar{
///     aaa:(u32,u32),
/// }
///
///
/// fn with_foo(foo:&mut dyn Foo_SI){
///     let expected_bar=Bar{aaa: (300,301) };
///
///     assert_eq!( foo.field_(fp!(bar)), &expected_bar );
///
///     assert_eq!( foo.field_(fp!(bar.aaa)), &(300,301) );
///
///     assert_eq!( foo.field_(fp!(bar.aaa.0)), &300 );
///
///     assert_eq!( foo.field_(fp!(bar.aaa.1)), &301 );
///
///     assert_eq!(
///         foo.fields_mut(fp!( bar.aaa, ooo.0, ooo.1 )),
///         ( &mut (300,301), &mut 66, &mut 99 )
///     );
/// }
///
/// fn main(){
///     let bar=Bar{aaa: (300,301) };
///
///     with_foo(&mut Foo{
///         bar:bar.clone(),
///         baz:44,
///         ooo:(66,99),
///     });
///
///     with_foo(&mut make_struct!{
///         bar:bar.clone(),
///         baz:44,
///         ooo:(66,99),
///     });
///
/// }
/// ```
///
#[macro_export]
macro_rules! fp {
    ( $($strings:tt)* ) => {{
        $crate::_delegate_fp!{$($strings)*}
    }};
}

#[macro_export]
#[doc(hidden)]
//#[cfg(not(feature="better_macros"))]
macro_rules! _delegate_fp {
    ($ident:ident) => (
        $crate::_delegate_fp_inner!( [ident] $ident )
    );
    (0)=>{ $crate::type_level::field_path::aliases::index_0 };
    (1)=>{ $crate::type_level::field_path::aliases::index_1 };
    (2)=>{ $crate::type_level::field_path::aliases::index_2 };
    (3)=>{ $crate::type_level::field_path::aliases::index_3 };
    (4)=>{ $crate::type_level::field_path::aliases::index_4 };
    (5)=>{ $crate::type_level::field_path::aliases::index_5 };
    (6)=>{ $crate::type_level::field_path::aliases::index_6 };
    (7)=>{ $crate::type_level::field_path::aliases::index_7 };
    (8)=>{ $crate::type_level::field_path::aliases::index_8 };
    ($($everything:tt)*) => ({
        $crate::_delegate_fp_inner!( [normal] $($everything)* )
    })
}

#[macro_export]
#[doc(hidden)]
//#[cfg(not(feature="better_macros"))]
macro_rules! _delegate_fp_inner {
    ($($everything:tt)*) => ({
        mod dummy{
            #[allow(unused_imports)]
            use structural::pmr as __struct_pmr;
            $crate::low_fp_impl_!{$($everything)*}
        }

        dummy::VALUE
    })
}

// #[macro_export]
// #[doc(hidden)]
// #[cfg(feature="better_macros")]
// macro_rules! _delegate_fp {
//     ($($everything:tt)*) => (
//         let $crate::new_fp_impl_!($($everything)*)
//     )
// }

/// Constructs a FieldPath(Set) for use as a generic parameter.
///
/// # Improved macro
///
/// To get an improved version of this macro which can use the same syntax
/// as the `fp` macro,you can do any of:
///
/// - Use Rust 1.40 or greater
///
/// - Use the `nightly_better_macros` cargo feature.
///
/// - Use the `better_macros` cargo feature.
///
///
/// # Examples
///
/// This demonstrates how one can bound types by the accessor traits in a where clause.
///
/// ```rust
/// use structural::{GetField,GetFieldExt,fp,FP};
///
/// fn greet_entity<This,S>(entity:&This)
/// where
///     // From 1.40 onwards you can also write `FP!(name)`.
///     //
///     // Before 1.40, you can use `field_path_aliases!{ name }` before this function,
///     // then write this as `This:GetField<name,Ty=S>`
///     This:GetField<FP!(n a m e),Ty=S>,
///     S:AsRef<str>,
/// {
///     println!("Hello, {}!",entity.field_(fp!(name)).as_ref() );
/// }
///
/// ```
///
/// # Example
///
/// This demonstrates [the improved version of this macro](#improved-macro).
///
#[cfg_attr(feature = "better_macros", doc = " ```rust")]
#[cfg_attr(not(feature = "better_macros"), doc = " ```ignore")]
/// use structural::{GetField,GetFieldExt,fp,FP};
///
/// fn greet_entity<This,S>(entity:&This)
/// where
///     This:GetField<FP!(name),Ty=S>,
///     S:AsRef<str>,
/// {
///     println!("Hello, {}!",entity.field_(fp!(name)).as_ref() );
/// }
///
/// type NumericIdent=FP!(0);
/// type StringyIdent=FP!(huh);
///
/// ```
///
#[macro_export]
macro_rules! FP {
    ($($char:tt)*) => {
        $crate::_delegate_FP!($($char)*)
    };
}

#[macro_export]
#[doc(hidden)]
#[cfg(not(feature = "better_macros"))]
macro_rules! _delegate_FP {
    ($($char:tt)*) => (
        $crate::pmr::FieldPath<(
            $crate::pmr::TStr_<($($crate::TChar!($char),)*)>,
        )>
    )
}

#[macro_export]
#[doc(hidden)]
#[cfg(feature = "better_macros")]
macro_rules! _delegate_FP {
    ($($everything:tt)*) => (
        $crate::_FP_impl_!($($everything)*)
    );
}

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

////////////////////////////////////////////////////////////////////////////////

/**

Declares aliases for field paths,used to access fields.

Every one of these aliases are types and constants of the same name.

# Variants

### Inline

Where the aliases are declared at the scope that the macro is invoked.

This variant cannot be invoked within functions.

Small example:
```rust
use structural::field_path_aliases;

field_path_aliases!{
    a,
    b=b,
    c=d.e,
}
# fn main(){}
```
### Module

Where the aliases are declared inside a nested module.

This variant can be invoked within functions.

Small example:
```rust
use structural::field_path_aliases;

fn hello(){
    field_path_aliases!{
        mod hello{
            a,
            b=b,
            c=d.e,
        }
    }
}
```

# Example

```rust
use structural::{field_path_aliases,GetField,GetFieldExt};

field_path_aliases!{
    // Equivalent to hello=hello
    hello,

    // Equivalent to world=world
    world,

    zero=0,
    one=1,
    two=2,

    // Used to access the `0`,`1`,and `2` fields
    // with the fields or fields_mut method.
    FirstThree=(0,1,2),

    h=(a,b,c),

    j=(p), // The identifier can also be parenthesised

}


fn assert_fields<T>(this:&T)
where
    T:GetField<zero,Ty=i32>+
        GetField<one,Ty=i32>+
        GetField<two,Ty=i32>
{
    assert_eq!( this.field_(zero), &2 );
    assert_eq!( this.field_(one), &3 );
    assert_eq!( this.field_(two), &5 );
    assert_eq!( this.fields(FirstThree), (&2,&3,&5) );
}

fn main(){
    assert_fields(&(2,3,5));
    assert_fields(&(2,3,5,8));
    assert_fields(&(2,3,5,8,13));
    assert_fields(&(2,3,5,8,13,21));
}
```

# Example

This demonstrates defining aliases inside a module.

```rust
use structural::{field_path_aliases,make_struct,structural_alias,GetFieldExt};

field_path_aliases!{
    pub mod names{
        a=0.0, // This is for accessing the `.0.0` nested field.
        b=0.1, // This is for accessing the `.0.1` nested field.
        c=foo.boo, // This is for accessing the `.foo.bar` nested field.
        d=foo.bar.baz, // This is for accessing the `.foo.bar.baz` nested field.
    }
}

structural_alias!{
    trait Foo<T>{
        foo:T,
    }

    trait Bar<T>{
        boo:u32,
        bar:T,
    }

    trait Baz<T>{
        baz:T,
    }
}

fn assert_nested<A,B,C>(this:&A)
where
    A:Foo<B>,
    B:Bar<C>,
    C:Baz<u32>,
{
    assert_eq!( this.field_(names::c), &100 );
    assert_eq!( this.field_(names::d), &101 );
}

fn main(){
    assert_nested(&make_struct!{
        foo:make_struct!{
            boo:100,
            bar:make_struct!{
                baz:101,
            }
        }
    });
}

```


*/
#[macro_export]
macro_rules! field_path_aliases {
    (
        $(#[$attr:meta])*
        $vis:vis mod $mod_name:ident{
            $($mod_contents:tt)*
        }
    ) => (
        #[allow(non_camel_case_types)]
        #[allow(non_upper_case_globals)]
        #[allow(unused_imports)]
        $(#[$attr])*
        /// Type aliases and constants for FieldPath and FieldPathSet
        /// (from the structural crate).
        ///
        /// The source code for this module can only be accessed from
        /// the type aliases and constants.<br>
        /// As of writing this documentation,`cargo doc` links
        /// to the inplementation of the `field_path_aliases` macro
        /// instead of where this module is declared.
        $vis mod $mod_name{
            use super::*;
            $crate::_field_path_aliases_impl!{
                $($mod_contents)*
            }
        }
    );
    (
        $($mod_contents:tt)*
    ) => (
        $crate::_field_path_aliases_impl!{
            $($mod_contents)*
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

/**
Declares type aliases for `TStr_<_>`(type-level string).

`TStr_<_>` itself is hidden from the docs because this library reserves
the right to change its generic parameter from a tuple of type-level characters,
to a `&'static str` const parameter (or `&'static [char]`).

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

### String literal

From Rust 1.40 onwards you can call this macro with a string literal,

Small Example:
*/
#[cfg_attr(not(feature = "better_macros"), doc = "```ignore")]
#[cfg_attr(feature = "better_macros", doc = "```rust")]
/**
use structural::TStr;

type Foo=TStr!("foo");

```

### Space separated characters

You can call this macro with space separated characters.

This variant of the macro exists to support Rust versions before 1.40.

You can also use [`tstr_aliases`](./macro.tstr_aliases.html) macro
if you prefer it to typing space separated characters.

Small Example:

```rust
use structural::TStr;

type Foo=TStr!(f o o);
```

# Example

This example demonstrates the space separated variant of the macro,
and uses the TStr macro to access a field,
instead of the [`FP`](./macro.FP.html) or [`fp`](./macro.fp.html) macros.

```rust
use structural::{GetField,GetFieldExt,Structural,TStr};
use structural::type_level::FieldPath1;

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
        $crate::TStr!(@chars $($char)*)
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
}

#[macro_export]
#[doc(hidden)]
#[cfg(feature = "better_macros")]
macro_rules! _delegate_TStr {
    ($string:literal) => {
        $crate::_TStr_impl_!($string)
    };
}
