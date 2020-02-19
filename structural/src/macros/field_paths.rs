/// Constructs a FieldPath(Set) value,
/// which determines the fields accessed in [GetFieldExt] methods.
///
/// When passed a single argument,this instantiates a `FieldPath`,
/// which can be passed to the
/// `GetFieldExt::{field_,field_mut,into_field,box_into_field}` methods
/// to access a field.
///
/// When passed multiple arguments,this instantiates a `FieldPathSet`.
/// It can then be passed to the `GetFieldExt::{fields,cloned_fields}` methods.<br>
///
/// To be passed to `GetFieldExt::fields_mut`,
/// the arguments to`fp!()` must be unique paths,
/// since there is no *cheap* way to check for equality of type-level strings yet.
/// All paths are checked against every other path to ensure that
/// none of them is a prefix of any other.
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
/// ### Nested Multiple fields
///
/// You can access multiple fields inside of a nested field with the `=>` in
/// `fp!(foo.bar.baz => 0,1,2)`.
///
/// This is most useful when accessing multiple fields inside of an
/// optional field or inside an enum variant.
///
/// The `=>` operator was defined for ergonomics,
/// `this.fields(fp!(::Foo=>0,1,2))` is equivalent to
/// `this.field_(fp!(::Foo)).map(|v| v.fields(fp!(0,1,2)) )`.
///
///
/// # Example:Multiple fields
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
/// # Example:Nested Fields
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
/// # #[struc(no_trait)]
/// #[struc(public)]
/// struct Bar{
///     aaa:(u32,u32),
/// }
///
/// // `Foo_SI` was declared by the `Structural` derive on `Foo`
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
/// # Example:Multiple fields insde a nested field
///
/// ```rust
/// use structural::{GetFieldExt,Structural,fp};
///
/// // `EnumA_SI` was declared by the `Structural` derive on `EnumA`
/// fn with_foo(foo:&mut impl EnumA_SI){
///     assert_eq!( foo.fields(fp!(::Foo=>0,1)), Some((&5,&8)) );
///     assert_eq!( foo.fields_mut(fp!(::Foo=>0,1)), Some((&mut 5,&mut 8)) );
///
///     assert_eq!( foo.fields(fp!(::Bar=>x,y)), None );
///     assert_eq!( foo.fields_mut(fp!(::Bar=>x,y)), None );
/// }
///
/// // `EnumA_SI` was declared by the `Structural` derive on `EnumA`
/// fn with_bar(bar:&mut impl EnumA_SI){
///     assert_eq!( bar.fields(fp!(::Foo=>0,1)), None );
///     assert_eq!( bar.fields_mut(fp!(::Foo=>0,1)), None );
///
///     assert_eq!( bar.fields(fp!(::Bar=>x,y)), Some((&"wha",&false)) );
///     assert_eq!( bar.fields_mut(fp!(::Bar=>x,y)), Some((&mut "wha",&mut false)) );
/// }
///
/// with_foo(&mut EnumA::Foo(5,8));
/// with_foo(&mut EnumB::Foo(5,8,13));
///
/// with_bar(&mut EnumA::Bar{ x:"wha", y:false });
/// with_bar(&mut EnumB::Bar{ x:"wha", y:false, z:None });
///
/// #[derive(Structural)]
/// enum EnumA{
///     Foo(u32,u64),
///     Bar{
///         x:&'static str,
///         y:bool,
///     },
/// }
///
/// #[derive(Structural)]
/// # #[struc(no_trait)]
/// enum EnumB{
///     Foo(u32,u64,i32),
///     Bar{
///         x:&'static str,
///         y:bool,
///         z:Option<()>,
///     },
/// }
///
///
///
/// ```
///
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
        $crate::FieldPath::one($crate::_construct_tstr_from_ident!{$ident})
    );
    (0)=>{ $crate::field_path::aliases::index_0 };
    (1)=>{ $crate::field_path::aliases::index_1 };
    (2)=>{ $crate::field_path::aliases::index_2 };
    (3)=>{ $crate::field_path::aliases::index_3 };
    (4)=>{ $crate::field_path::aliases::index_4 };
    (5)=>{ $crate::field_path::aliases::index_5 };
    (6)=>{ $crate::field_path::aliases::index_6 };
    (7)=>{ $crate::field_path::aliases::index_7 };
    (8)=>{ $crate::field_path::aliases::index_8 };
    ($($everything:tt)*) => ({
        $crate::_delegate_fp_inner!( [normal] $($everything)* )
    })
}

#[macro_export]
#[doc(hidden)]
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
/// # Variants of the macro
///
/// ### Identifier
///
/// <span id="improved-macro"></span>
///
/// The variant of the macro takes in an identifier,or an integer.
///
/// Examples: `FP!(hello)`,`FP!(100)`
///
/// This variant of the macro requires one of these:
///
/// - Use Rust 1.40 or greater.
///
/// - Use the `nightly_better_macros` cargo feature.
///
/// - Use the `better_macros` cargo feature.
///
/// ### Space separated characters
///
/// This variant of the macro takes in space separated characters.
///
/// This exists to support Rust versions older than 1.40.
///
/// Example: `FP!(f o o)` `FP!(4 1 3)` `FP!(c o u n t d o w n)`
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
///     // Before 1.40, you can also use `field_path_aliases!{ name }` before this function,
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
            $crate::pmr::TStr_<$crate::p::TS<($($crate::TChar!($char),)*)>>,
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
    // field paths used to access multiple fields must be wrapped in parentheses.
    d=(a,b,c),
    // Accesses the variant,if the enum is currently that variant
    e=::Foo,
    // Accesses the a,b,and c fields inside of the Foo variant.
    f=(::Foo=>a,b,c),
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
            // field paths used to access multiple fields must be wrapped in parentheses.
            d=(a,b,c),
            // Accesses the variant,if the enum is currently that variant
            e=::Foo,
            // Accesses the a,b,and c fields inside of the Foo variant.
            f=(::Foo=>a,b,c),
        }
    }
}
```

# Example

```rust
use structural::{field_path_aliases,structural_alias,GetField,GetFieldExt,Structural};
use structural::enums::VariantProxy;

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

    boom=Boom,
    boom_variant=::Boom,
    boom_a=::Boom.a,
    boom_b=::Boom.b,
    boom_both=(::Boom=>a,b),
    boom_both_individually=(::Boom.a,::Boom.b),

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

structural_alias!{
    trait BoomVariant{
        Boom {
            a: &'static [u8],
            b: &'static [u16],
        }
    }
}

fn assert_variant<T>(this:&T)
where
    // Field paths cannot be used to bound enum fields,
    // for that you need to use type-level-strings.
    // The `tstr_aliases` macro
    // (which is the equivalent of this macro for type-level-strings)
    // has an example for how to manually write bounds for enum fields.
    T: BoomVariant,
{
    let _:&VariantProxy<T,boom>=this.field_(boom_variant).unwrap();

    // Accessing individual enum fields
    assert_eq!( this.field_(boom_a), Some(&&b"hello"[..]) );
    assert_eq!( this.field_(boom_b), Some(&&[0,1,2,3][..]) );

    // Accessing multiple enum fields.
    assert_eq!(
        this.fields(boom_both),
        Some(( &&b"hello"[..], &&[0,1,2,3][..] ))
    );

    // Accessing multiple enum fields,individually.
    // Note how you have to match on multiple options,
    // even though all of them are Some or None at the same time,
    // this is why `fp!(::Foo=>a,b,c)` was created.
    assert_eq!(
        this.fields(boom_both_individually),
        ( Some(&&b"hello"[..]), Some(&&[0,1,2,3][..]) )
    );

}

fn main(){
    assert_fields(&(2,3,5));
    assert_fields(&(2,3,5,8));
    assert_fields(&(2,3,5,8,13));
    assert_fields(&(2,3,5,8,13,21));

    assert_variant(&Variants::Boom {
        a: b"hello",
        b: &[0,1,2,3],
    })
}

#[derive(Structural, Copy, Clone, Debug, PartialEq)]
# #[struc(no_trait)]
pub enum Variants {
    Boom {
        a: &'static [u8],
        b: &'static [u16],
    },
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
