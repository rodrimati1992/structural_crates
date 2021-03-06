/// Constructs a field path value,
/// which determines the field(s) accessed in [StructuralExt](./trait.StructuralExt.html) methods.
///
/// ### Type
///
/// The type produced by `fp` can be one of:
///
/// - [A path component](#path-components):<br>
/// When it's the only thing passed to the macro.
/// This allows accessing a non-nested field.<br>
/// Eg: `fp!(a)`, `fp!(::Foo.bar)`, `fp!(::Foo)`
///
/// - [NestedFieldPath](./struct.NestedFieldPath.html), [example](#examplenested-fields): <br>
/// When multiple [path components](#path-components) are passed to the macro.
/// This allows accessing a nested field.<br>
/// Eg: `fp!(a.b)`, `fp!(::Foo.bar.baz)`, `fp!(a.b?.c)`, `fp!(::Foo.bar?.baz)`
///
/// - [FieldPathSet](./struct.FieldPathSet.html), [example](#examplemultiple-fields): <br>
/// When a comma separated list of paths are passed to the macro.
/// This allows accessing multiple fields.<br>
/// Eg: `fp!(a, b.c.d, c::Some.0.bar)`, `fp!(::Foo.bar, baz, ::Boo)`
///
/// - [NestedFieldPathSet](./struct.NestedFieldPathSet.html),
/// [example](#examplemultiple-fields-insde-a-nested-field):<br>
/// When a `=>` is passed to the macro.
/// This allows accessing multiple fields from within a nested field.<br>
/// Eg: `fp!(a => b, c)`, `fp!(::Foo => bar, baz, bam)`
///
/// If you want type aliases and constants for a particular field path,
/// you can use the [field_path_aliases](./macro.field_path_aliases.html) macro.
///
/// You can use [the FP macro](./macro.FP.html) to get the type of any field path.
///
/// ### Identifier
///
/// The macro takes in identifiers,integers,or strings literals
/// for the names of variants and fields.
///
/// String literals are used as a workaround for non-ascii identifiers not being
/// supported in Rust.
/// If the contents of the string literal is a valid identifier,
/// then you can also write it as one,
/// eg:`fp!("Foo")` is equivalent to `fp!(Foo)`.
///
/// ### Path Components
///
/// These are the basic building blocks for field paths:
///
/// - `foo`: A [TStr](./struct.TStr.html)
/// with the name of a field,which accesses the `foo` field.<br>
/// A `.` prefixing the field name is required after other path components.<br>
/// Examples: `fp!(foo)`, `fp!(0)`
///
/// - `::Foo.bar`: A [VariantField](./struct.VariantField.html),
/// which accesses the `bar` field in the `Foo` variant.<br>
/// The `::` prefix is required to distinguish between `::Foo`
/// and field access to a `Foo` field.<br>
/// Examples: `fp!(::Foo.bar)`, `fp!(::Boom.0)`
///
/// - `::Foo`: A [VariantName](./struct.VariantName.html),
/// which wraps the type in a `VariantProxy<Self,TS!(Foo)>`.
/// If this is directly followed by a field access,
/// it'll be a [VariantField](./struct.VariantField.html) instead.<br>
/// The `::` prefix is required to distinguish between `::Foo`
/// and field access to a `Foo` field.<br>
/// Examples: `fp!(::Foo)`, `fp!(::Boom)`
///
/// - `?`: Syntactic sugar for `::Some.0`,used to access the value inside an Option.
/// Examples: `fp!(foo?.bar)`, `fp!(::Quax.foo?.0)`
///
/// These can be passed to the
/// `StructuralExt::{field_,field_mut,into_field,box_into_field}` methods
/// to access a single non-nested field.
///
/// More Examples:
///
/// - `fp!(hello)`: accesses the `hello` field.
///
/// - `fp!(100)`: accesses the `100` field.
///
/// - `fp!(::"@hello")`,accesses the `@hello` variant.
///
/// - `fp!(::1337."wh.at")`,accesses the `wh.at` field in the `1337` variant.
/// (the `.` in `"wh.at"` is part of the field name)
///
/// - `fp!("hello")` (equivalent to `fp!(hello)`)
///
/// ### Nested fields
///
/// You can construct field paths with a sequence of [path components](#path-components)
/// to access nested fields,eg:`fp!(a.b.c)`.
///
/// Doing `this.field_(fp!(0.1.2))` is equivalent to `&((this.0).1).2`
/// (except that it can also be done in a generic context).
///
/// This can be passed to the
/// `StructuralExt::{field_,field_mut,into_field,box_into_field}` methods
/// to access a nested field.
///
/// ### Multiple fields
///
/// You can access multiple fields simultaneously with `fp!(0,1,2)`
/// where doing `this.fields_mut(fp!(a,b,c))`
/// is equivalent to `(&mut this.a,&mut this.b,&mut this.c)`
///
/// This can be passed to the `StructuralExt::*fields*` methods.<br>
/// [`StructuralExt::fields_mut`] requires the field paths to be for disjoint fields.
///
/// ### Nested Multiple fields
///
/// You can access multiple fields inside of a nested field with the `=>` in
/// `fp!(foo.bar.baz => 0,1,2)`.
///
/// This is most useful when accessing multiple fields inside of an enum.
///
/// The `=>` operator was defined for ergonomics,
/// `this.fields(fp!(::Foo=>0,1,2))` is equivalent to
/// `this.field_(fp!(::Foo)).map(|v| v.fields(fp!(0,1,2)) )`.
///
/// This can be passed to the `StructuralExt::*fields*` methods.<br>
/// [`StructuralExt::fields_mut`] requires the field paths to be for disjoint fields.
///
/// [`StructuralExt::fields_mut`]: ./trait.StructuralExt.html#method.fields_mut
///
/// # Aliasing
///
/// For the purpose of detecting aliasing field paths,
/// `fp!(::foo)` and `fp!(foo)` are considered to be the same path,
/// which means that you can't pass `fp!(::foo, foo)` to [`StructuralExt::fields_mut`].
///
/// # Example:Multiple fields
///
/// ```
/// use structural::{StructuralExt,fp,structural_alias};
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
/// use structural::{StructuralExt,Structural,fp,make_struct};
///
/// #[derive(Structural)]
/// #[struc(public)]
/// struct Foo{
///     bar:Bar,
///     baz:u32,
///     ooo:Option<(u32,u32)>,
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
///     assert_eq!( foo.field_mut(fp!( bar.aaa )), &mut (300,301) );
///
///     assert_eq!( foo.field_mut(fp!( ooo )), &mut Some((66,99)) );
///
///     // You can use the `?` operator inside of `fp` to access fields from inside an Option.
///     //
///     // `?` is syntactic sugar for `::Some.0`,so if you defined your own enum with
///     // a `Some(T)` variant,you could also use the operator with that enum.
///     assert_eq!( foo.field_mut(fp!( ooo? )), Some(&mut (66,99)) );
///     assert_eq!( foo.field_mut(fp!( ooo?.0 )), Some(&mut 66) );
///     assert_eq!( foo.field_mut(fp!( ooo?.1 )), Some(&mut 99) );
/// }
///
/// fn main(){
///     let bar=Bar{aaa: (300,301) };
///
///     with_foo(&mut Foo{
///         bar:bar.clone(),
///         baz:44,
///         ooo:Some((66,99)),
///     });
///
///     with_foo(&mut make_struct!{
///         bar:bar.clone(),
///         baz:44,
///         ooo:Some((66,99)),
///     });
///
/// }
/// ```
///
/// # Example:Multiple fields insde a nested field
///
/// ```rust
/// use structural::{StructuralExt,Structural,fp};
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
    ($ident:ident) => (
        <$crate::_TStr_from_ident!{$ident}>::NEW
    );
    (0)=>{ $crate::path::aliases::index_0 };
    (1)=>{ $crate::path::aliases::index_1 };
    (2)=>{ $crate::path::aliases::index_2 };
    (3)=>{ $crate::path::aliases::index_3 };
    (4)=>{ $crate::path::aliases::index_4 };
    (5)=>{ $crate::path::aliases::index_5 };
    (6)=>{ $crate::path::aliases::index_6 };
    (7)=>{ $crate::path::aliases::index_7 };
    (8)=>{ $crate::path::aliases::index_8 };
    ($lit:literal) => (
        <$crate::_FP_literal_!($lit)>::NEW
    );
    // This branch of the macro is defined like this to not leak implementation details,
    // users should not be able to see the macro definition.
    ($($everything:tt)*) => ({
        type Path=$crate::FP!($($everything)*);

        const VALUE:Path=unsafe{ <Path>::NEW_ALIASED.set_uniqueness() };

        VALUE
    });
}

/// Constructs a field path type for use as a generic parameter.
///
/// # Input
///
/// <span id="improved-macro"></span>
///
/// This takes the same input as [the fp macro](./macro.fp.html),
/// getting the type of that field path.
///
/// # Struct Example
///
/// ```rust
///
/// use structural::{GetField, StructuralExt, FP, fp, make_struct};
///
/// greet_entity(&make_struct!{ name: "Bob" }, &(99,999,999));
///
/// type Path_0 = FP!(0);
///
/// // Equivalent to `type Path_name = FP!(name);`,
/// // the `FP` and `fp` macros use string literals to emulate non-ascii identifiers.
/// type Path_name = FP!("name");
///
/// fn greet_entity<S, This, Tup>(entity:&This, tuple:&Tup)
/// where
///     This: GetField<FP!(name), Ty = S>,
///     Tup : GetField<Path_0, Ty = u64>,
///     S: AsRef<str>,
/// {
///     assert_eq!( entity.field_(fp!(name)).as_ref(), "Bob" );
///     assert_eq!( entity.field_(Path_name::NEW).as_ref(), "Bob" );
///
///     assert_eq!( tuple.field_(fp!(0)), &99 );
///     assert_eq!( tuple.field_(Path_0::NEW), &99 );
/// }
///
/// ```
///
/// # Enum Example
///
/// ```rust
///
/// use structural::{GetVariantField, Structural, StructuralExt, FP, TS};
///
/// assert_eq!( get_number(&Enum::Foo(10)), Some(10) );
/// assert_eq!( get_number(&Enum::Bar{value: 20}), Some(20) );
///
/// assert_eq!( get_number(&Other::Foo(30, "foo")), Some(30) );
/// assert_eq!( get_number(&Other::Bar{value: 40, uh: None}), Some(40) );
/// assert_eq!( get_number(&Other::Baz), None );
///
///
/// type Path_Foo_0 = FP!(::Foo.0);
///
/// // Equivalent to `type Path_name = FP!(::Bar.value);`,
/// // the `FP` and `fp` macros use string literals to emulate non-ascii identifiers.
/// type Path_Bar_value = FP!(::Bar."value");
///
/// fn get_number<This>(this: &This)->Option<u32>
/// where
///     // The `*VariantField*` traits require that you pass a `TStr`,
///     // using the `TS` macro or a type alias.
///     This: GetVariantField<TS!(Foo), TS!(0), Ty = u32> +
///           GetVariantField<TS!(Bar), TS!(value), Ty = u32>,
/// {
///     this.field_(Path_Foo_0::NEW)
///         .or(this.field_(Path_Bar_value::NEW))
///         .cloned()
/// }
///
/// #[derive(Structural)]
/// enum Enum{
///     Foo(u32),
///     Bar{value: u32},
/// }
///
/// #[derive(Structural)]
/// enum Other{
///     Foo(u32, &'static str),
///     Bar{value: u32, uh: Option<u32>},
///     Baz,
/// }
///
/// ```
///
///
///
#[macro_export]
macro_rules! FP {
    ($ident:ident) => {
        $crate::_TStr_from_ident!($ident)
    };
    (0)=>{ $crate::path::string_aliases::str_0 };
    (1)=>{ $crate::path::string_aliases::str_1 };
    (2)=>{ $crate::path::string_aliases::str_2 };
    (3)=>{ $crate::path::string_aliases::str_3 };
    (4)=>{ $crate::path::string_aliases::str_4 };
    (5)=>{ $crate::path::string_aliases::str_5 };
    (6)=>{ $crate::path::string_aliases::str_6 };
    (7)=>{ $crate::path::string_aliases::str_7 };
    (8)=>{ $crate::path::string_aliases::str_8 };
    (9)=>{ $crate::path::string_aliases::str_9 };
    (_)=>{ $crate::path::string_aliases::str_underscore };
    ($lit:literal)=>{
        $crate::_FP_literal_!($lit)
    };
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
    c=(0,1,2),
    d=(::Foo=>bar,baz),
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
            d=(a,b,c,"#D"),
            e=::Foo,
            f=(::Foo=>a,b,c),
        }
    }
}
```

# Example

```rust
use structural::{ GetField, StructuralExt, IntoVariantFieldMut, Structural, field_path_aliases };
use structural::enums::VariantProxy;

field_path_aliases!{
    // Equivalent to hello = hello
    hello,
    // Equivalent to world = world
    world,

    // `?` allows you to access the value inside an `Option`.
    //
    // It is syntactic sugar for `::Some.0`,
    // making it usable with any other type implementing
    // the `GetVariantField<TS!(Some),TS!(0))` trait and subtraits.
    zero_a=0?.0,
    zero_b=0?.1,
    zero=0,
    one=1,
    two=2,

    // Used to access the `0`,`1`,and `2` fields
    // with the fields or fields_mut method.
    FirstThree=(0,1,2),

    h=(a,b,c),

    j=(p), // The identifier can also be parenthesised

    boom=Boom,
    path_a=a,
    path_b=b,

    // Accesses the `Boom` variant,if the enum is currently that variant
    boom_variant=::Boom,

    boom_a=::Boom.a,
    boom_b=::Boom.b,

    // Accesses the a,and b fields inside of the `Boom` variant,
    // roughly returning a `Option<(_,_)>`.
    boom_both=(::Boom=>a,b),

    // Accesses the a,and b fields inside of the `Boom` variant separately,
    // roughly returning a `(Option<_>,Option<_>)`.
    boom_both_individually=(::Boom.a,::Boom.b),
}


fn main(){
    assert_fields(&(Some((8,13)),3,5));
    assert_fields(&(Some((8,13)),3,5,8));
    assert_fields(&(Some((8,13)),3,5,8,13));
    assert_fields(&(Some((8,13)),3,5,8,13,21));

    assert_variant(&Variants::Boom {
        a: b"hello",
        b: &[0,1,2,3],
    })
}

fn assert_fields<T>(this:&T)
where
    T:GetField<zero,Ty=Option<(u16,u16)>>+
        GetField<one,Ty=i32>+
        GetField<two,Ty=i32>
{
    assert_eq!( this.field_(zero), &Some((8,13)) );
    assert_eq!( this.field_(zero_a), Some(&8) );
    assert_eq!( this.field_(zero_b), Some(&13) );
    assert_eq!( this.field_(one), &3 );
    assert_eq!( this.field_(two), &5 );
    assert_eq!( this.fields(FirstThree), (&Some((8,13)),&3,&5) );
}

fn assert_variant<T>(this:&T)
where
    T: IntoVariantFieldMut<boom,path_a,Ty= &'static [u8]> +
        IntoVariantFieldMut<boom,path_b,Ty= &'static [u16]>,
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
use structural::{field_path_aliases,make_struct,structural_alias,StructuralExt};

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

# Example

This example demonstrates syntax not used in other examples

```rust
use structural::field_path_aliases;

field_path_aliases!{
    // This is a `b_str` alias with the same value as the `b` alias
    b_str="b",

    // strings allow for arbitrary identifiers.
    at_me="@me",

    // Accesses the a,b,and c fields inside of the `ñ` variant.
    g=(::"ñ"=>a,b,c),
}
# fn main(){}
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
        /// Type aliases and constants for NestedFieldPath and FieldPathSet
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
