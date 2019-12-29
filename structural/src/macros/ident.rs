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
// /// ### Splicing
// ///
// /// You can use a `FieldPath` type (not a value)
// /// inside the `fp` macro with the `( FooType )` syntax.
// ///
// /// This will splice the `FieldPath` into the position it was used in.
// ///
// /// An example:
// /// ```
// /// use structural::{fp,FP,field_path_aliases};
// /// use structural::reexports::AssertEq;
// ///
// /// field_path_aliases!{
// ///     wooo,
// ///     chain=b.c.d,
// ///     get_x=pos.x,
// /// }
// ///
// /// # fn main(){
// ///
// /// AssertEq::new( fp!( a.(wooo).e ) , fp!(a.wooo.e) );
// ///
// /// AssertEq::new( fp!( a.(get_x).e ), fp!(a.pos.x.e) );
// ///
// /// # }
// ///
// /// ```
// ///
// /// ### Inserting
// ///
// /// You can use a `TString` type or a single-ident `FieldPath` type
// /// inside the `fp` macro with the `[ FooType ]` syntax.
// ///
// /// This inserts the value of the `TString`or of the single identifier `FieldPath`
// /// into that position.
// ///
// /// An example:
// /// ```
// /// use structural::{fp,FP,field_path_aliases};
// /// use structural::reexports::AssertEq;
// ///
// /// field_path_aliases!{
// ///     foo,
// ///     bar=what,
// ///     baz=the,
// /// }
// ///
// /// // This can also be `type RectangleStr=FP!(rectangle);` from Rust 1.40 onwards
// /// type RectangleStr=FP!(r e c t a n g l e);
// ///
// ///
// /// # fn main(){
// /// let _:foo;
// /// let _:bar;
// /// let _:baz;
// ///
// /// AssertEq::new( fp!( a[foo].e ), fp!(a.foo.e) );
// /// AssertEq::new( fp!( a[bar].e ), fp!(a.what.e) );
// /// AssertEq::new( fp!( a[baz].e ), fp!(a.the.e) );
// /// AssertEq::new( fp!( a[RectangleStr].e ), fp!(a.rectangle.e) );
// ///
// /// # }
// ///
// /// ```
// ///
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
    ($($everything:tt)*) => ({
        #[allow(unused_imports)]
        use structural::pmr as __struct_pmr;

        struct __Dummy;

        impl __Dummy{
            $crate::old_fp_impl_!{$($everything)*}
        }
        __Dummy::VALUE
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
            $crate::pmr::TString<($($crate::TChar!($char),)*)>,
        )>
    )
}

#[macro_export]
#[doc(hidden)]
#[cfg(feature = "better_macros")]
macro_rules! _delegate_FP {
    ($($everything:tt)*) => (
        $crate::_FP_impl_!($($everything)*)
    )
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

Declares a module with aliases for field paths,used to access fields.

Every one of these aliases are types and constants of the same name.

# Example

```rust
use structural::{field_path_aliases_module,GetField,GetFieldExt};

field_path_aliases_module!{
    pub mod names{
        // Equivalent to _a=_a
        _a,
        // Equivalent to _b=_b
        _b,
        // Equivalent to _0=_0
        _0,
        // Equivalent to c=c
        c,
        zero=0,
        one=1,
        two=2,
        e=10,
        g=abcd,

        // Used to access the `0`,`1`,and `2` fields
        // with the fields or fields_mut method.
        FirstThree=(0,1,2),
        h=(a,b,c),
        i=(0,3,5),

        j=(p), // The identifier can also be parenthesised

    }
}


fn assert_fields<T>(this:&T)
where
    T:GetField<names::zero,Ty=i32>+
        GetField<names::one,Ty=i32>+
        GetField<names::two,Ty=i32>
{
    assert_eq!( this.field_(names::zero), &2 );
    assert_eq!( this.field_(names::one), &3 );
    assert_eq!( this.field_(names::two), &5 );
    assert_eq!( this.fields(names::FirstThree), (&2,&3,&5) );
}

fn main(){
    assert_fields(&(2,3,5));
    assert_fields(&(2,3,5,8));
    assert_fields(&(2,3,5,8,13));
    assert_fields(&(2,3,5,8,13,21));
}


```

# Example

This demonstrates defining and using aliases for nested fields.

```rust
use structural::{field_path_aliases_module,make_struct,structural_alias,GetFieldExt};

field_path_aliases_module!{
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
macro_rules! field_path_aliases_module {
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
        /// to the inplementation of the `field_path_aliases_module` macro
        /// instead of where this module is declared.
        $vis mod $mod_name{
            use super::*;
            $crate::_field_path_aliases_impl!{
                $($mod_contents)*
            }
        }
    );
}

/**

Declares aliases for field paths,used to access fields.

Every one of these aliases are types and constants of the same name.

As of Rust 1.39,this macro cannot be invoked within a function,
you *can* use
[`field_path_aliases_module`](./macro.field_path_aliases_module.html)
within functions though.

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

This demonstrates defining and using aliases for nested fields.

```rust
use structural::{field_path_aliases,structural_alias,GetFieldExt};


field_path_aliases!{
    nested_a=0.0, // This is for accessing the `.0.0` nested field.
    nested_b=0.1, // This is for accessing the `.0.1` nested field.
    nested_c=foo.bar, // This is for accessing the `.foo.bar` nested field.
    nested_d=foo.bar.baz, // This is for accessing the `.foo.bar.baz` nested field.
}

structural_alias!{
    trait Tuple2<T>{
        ref 0:(T,T),
        ref 1:(T,T),
    }
}

fn assert_nested<T>(this:&T)
where
    T:Tuple2<u32>
{
    assert_eq!( this.field_(nested_a), &100 );
    assert_eq!( this.field_(nested_b), &101 );
}

fn main(){
    assert_nested(&(
        (100,101),
        (200,201),
    ));
}

```


*/
#[macro_export]
macro_rules! field_path_aliases {
    (
        $($mod_contents:tt)*
    ) => (
        $crate::_field_path_aliases_impl!{
            $($mod_contents)*
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! tstring_aliases {
    (
        $($macro_params:tt)*
    ) => (
        $crate::_tstring_aliases_impl!{
            $($macro_params)*
        }
    )
}

#[macro_export]
macro_rules! tstring_aliases_module {
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
        $vis mod $mod_name{
            $crate::_tstring_aliases_impl!{
                $($mod_contents)*
            }
        }
    )
}
