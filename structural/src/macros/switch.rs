/**
Provides basic pattern matching for structural enums.

The basicness of `switch` resides in that it can only branch based on the variant of
the structural enum.
Once a branch is taken(based solely on what the current variant is),
all the listed fields of the variant are destructured into the pattern for the field
(ie:in ` Foo{x:(a,b)}=>{} `,x is destructured into `a` and `b`).

`switch` is intentionally limited,
it does not support pattern matching nested structural types,
or branching on the value of fields.

# Exhaustiveness

Switch can handle both exhaustive and nonexhaustive enums.

When matching exhaustive enums,eiher all variants must be matched,
or the last branch must be the default branch (`_=>....`).

When matching nonexhaustive enums,
the last branch must be the default branch (`_=>....`).

The `Structural` derive macro by default generates
the `*_SI` nonexhaustive enum trait alias,
and the `*_ESI` exhaustive enum trait alias.

The `switch` macro considers enums it knows implement `VariantCount`
(a supertrait of `*_ESI` traits) to be exhaustive,
otherwise they're considered non-exhaustive.

# Syntax Examples

These examples demonstrates all the syntax of the `switch` macro.

[For a detailed description of the syntax look here](#syntax)

This demonstrates variant access modes:
```
use structural::{GetFieldExt,Structural,fp,switch};

#[derive(Debug,Copy,Clone,Structural)]
enum Enum{
    Foo{a:u32},
    Bar{b:u64},
    Baz{c:&'static str},
    Bam{
      #[struc(optional)]
      d:Option<usize>
    },
}

let mut this=Enum::Foo{a:100};

assert_eq!( takes_exhaustive(Enum::Foo{a:100}), 0 );
assert_eq!( takes_exhaustive(Enum::Bar{b:100}), 1 );
assert_eq!( takes_exhaustive(Enum::Baz{c:"hello"}), 2 );
assert_eq!( takes_exhaustive(Enum::Bam{d:Some(400)}), 3 );

// This function takes any type bounded by `Enum_ESI`,
// the exhaustive version of the trait generated for `Enum` by the `Structural` macro.
fn takes_exhaustive<T>(mut this: T)->u32
where
    T: Enum_ESI,
{
    // The matched expression is not moved if
    // it's a single identifier,and no branch takes it by value.
    //
    // To match the enum by reference only, you can use `switch{ref this; .... }`,
    // and not override it in switch branches.
    // Overriding looks like `ref mut Foo{x}=>{}`,and not overriding looks like `Foo{x}=>{}`
    //
    // To match the enum by mutable reference only, you can use `switch{ref mut this; .... }`,
    // and not override it in switch branches.
    switch!{ this;
        // `ref` here is the access mode,
        // and it means that fields are accessed as `&`references.
        ref Foo{a}=>{
            assert_eq!(a,&100);
            0
        }

        // `ref mut` heremeans that fields are accessed as mutable`references.
        ref mut Bar{b}=>{
            assert_eq!(b,&mut 100);
            1
        }

        // `move` here means that the enum is taken by value,
        // wrapped in a `VariantProxy<T,TS!(Baz)>`,
        move Baz=>{
            assert_eq!(this.into_field(fp!(c)) ,"hello");
            2
        }

        // The enum is taken by value into the branch(wrapped in a `VariantProxy<T,TS!(Bam)>`),
        // because the default access mode in this switch is `move`,
        // since none was specified in the switch header.
        Bam=>{
            assert_eq!(this.into_field(fp!(d)) ,Some(400));
            3
        }
        // No `_=>{4}` branch is necessary,because this is an exhaustive enum.
        //
        // This would be necessary if the `Enum_SI` trait was used instead of `Enum_ESI`,
        // with the advantage that enums with more variants than `Enum` would be supported..
    }
}

```

This demonstates that you can use access modes in the switch header,
and how you can copy Copy fields in with different access modes:
```rust
use structural::{GetFieldExt,Structural,fp,switch};

# #[derive(Debug,Copy,Clone,Structural)]
# #[struc(no_trait)]
# enum Enum{
#     Foo{a:u32},
#     Bar{b:u64},
#     Baz{c:&'static str},
#     Bam{
#       #[struc(optional)]
#       d:Option<usize>
#     },
# }

// The same enum as the first example
let mut this=Enum::Foo{a:100};

// `ref` here sets default access for all variants,
// which is overridable per-variant,destructuring fields into references by default.
switch!{ ref this ;
    Foo=>{
        // The `this` here is a `&VariantProxy<Enum,TS!(Foo)>`,
        // which allows accessing fields in variants non-optionally.
        assert_eq!( this.field_(fp!(a)), &100 )
    }
    // You can copy `Copy` fields in `ref` branches with `&field_name`
    // The `ref`ness of this branch is inherited from
    // the `ref` at the top of the macro invocation.
    Bar{ &b }=>assert_eq!( b, 200 ),

    // You can copy `Copy` fields in `ref mut` branches with `&mut field_name`
    ref mut Baz{ &mut c }=>assert_eq!( c, "hello" ),

    // The `ref` here is redundant,since it's inherited from the switch header
    ref Bam{d}=>assert_eq!( d, Some(&400) )// The `,` is optional after the last switch arm.
}
```

This demonstrates `if`,`if let`,and `_=>` branches,
as well as using the variant proxy after the matches fields(in the `Bam` branch).

```rust
use structural::{GetFieldExt,Structural,fp,switch};

# #[derive(Debug,Copy,Clone,Structural)]
# #[struc(no_trait)]
# enum Enum{
#     Foo{a:u32},
#     Bar{b:u64},
#     Baz{c:&'static str},
#     Bam{
#       #[struc(optional)]
#       d:Option<usize>
#     },
# }

// The same enum as the first example
let this=Enum::Baz{ c:"55" };

let text="99";

// `other = <expression>` is used here to be able to use
// the `VariantProxy<Enum,TS!(VariantName)>` inside the switch branch,
// to access any field of the matched variant(especially those that weren't destructured).
//
// If it was just the expression,then the `VariantProxy` would be inaccessible.
let number = switch!{ other = this;
    // `if`s can only be used as guards on the `_` pattern,
    // never as a guard when matching on a variant
    if 2+2!=4 => unreachable!("2+2 is 4, silly!"),

    Baz => other
        .into_field(fp!(c))
        .parse::<u32>()
        .unwrap(),  // The `,` is required here

    // `if can only be used as guards on the `_` pattern,
    // never as a guard when matching on a variant
    if let Ok(x@99)=text.parse::<u32>() => {
        println!("{:?} parses to {}u32",text,x);

        // The type of `other` is `Enum` in non-matching branches.
        println!("{:?}", other );

        x
    }
    ref mut Bam{d} =>{
        assert_eq!( d, Some(&mut 9999) );

        // The `other` here is a `&mut VariantProxy<Enum,TS!(Bam)>`,
        // you can access all the fields using it,
        // but only after the last use of destructured fields.
        assert_eq!( other.field_mut(fp!(d)), Some(&mut 9999) );

        100
    }, // The `,` is optional after `{...}`
    _=>101  // The `,` is optional after the last switch arm.
};

assert_eq!(number,55);

```

# Example

This gets the human-readable name of the direction the enum represents.

The enum can have any data so long as it has exactly the
same amount and name of variants as `Direction4`.

```
use structural::{Structural,switch};

fn main(){
    assert_eq!( Direction4::Left.direction4_to_str(), "left" );
    assert_eq!( Direction4::Right.direction4_to_str(), "right" );

    assert_eq!( GenericDirection4::Up("hi").direction4_to_str(), "up" );
    assert_eq!( GenericDirection4::Down(vec![0,1,2]).direction4_to_str(), "down" );
}


// This is an extension trait for all enums with variants of the same name as `Direction4`
trait Direction4Ext: Direction4_ESI{
    fn direction4_to_str(&self)->&'static str {
        // `switch!{ref self; .... }` isn't necesasry here,since no field is accessed.
        switch!{ self;
            Left=>"left",
            Right=>"right",
            Down=>"down",
            Up=>"up",
        }
    }
}

impl<This> Direction4Ext for This
where
    This:?Sized+Direction4_ESI
{}


#[derive(Structural)]
enum Direction4{
    Left,
    Right,
    Down,
    Up,
}

#[derive(Structural)]
enum GenericDirection4<T>{
    Left(T),
    Right(T),
    Down(T),
    Up(T),
}

```

# More Examples

For more examples you can look at the ones
[in the docs for enums](./docs/enums/index.html)


# Syntax

This uses a macro_rules-like syntax to describe the input to this macro.
If you want to see a example that uses all the syntax,go to the
[Syntax Example](#syntax-example) section.

```text
switch!{
    $switch_header:switch_header


    $(
        $switch_branch:switch_branch
    )*

    // The default branch:this is required when matching over nonexhaustive enums
    $(_=> $branch_expr:branch_expr )?
}
```

<br/>

The `switch_header` allows passing the matched expression,
declaring the default access mode for all variants,
and the name of the proxy for accessing fields of a variant.

Syntax(in the order in which the syntax is matched):

- ` $($default_access:access_mode)? $proxy:identifier ;`

- ` $($default_access:access_mode)? $proxy:identifier = $matched_value:expression` ;

- ` $($default_access:access_mode)? $matched_value:expression` ;



$default_access determins the default access mode for branches that don't specify it.
When it's not specified in the switch header,it is `move`.

If no `$matched_value` is passed,and `$proxy` is specified,
the identifier of the matched enum will be reused for `$proxy`.

If `$matched_value` *is* passed,and `$proxy` is not specified,
it'll be stored into an anonymous proxy variable,inaccessible inside the switch branches.

If `$matched_value` *is* passed,and `$proxy` is specified,
it is stored in the `$proxy` variable by value before any pattern matching happens.

In switch arms that match on the variant of the enum,
`$proxy` is a `VariantProxy<TypeOfMatchedEnum, TS!(NameOfVariant)>`,
which allows directly accessing variant fields.

In switch arms that don't match on the variant of the enum,
`$proxy` is the type of the matched enum.

<br>

`access_mode` is the way that variant fields are accessed.

It can be any of:

- `ref` variant fields will be accessed by reference.

- `ref mut` variant fields will be accessed by mutable reference.

- `move` this is not usable yet for fields,
it's currently only allowed in branches that don't list fields for the variant
(eg: `move Bar=>{}`).
You can manually convert the variant into a single field by doing
`name_of_proxy.into_field(fp!(field_name))` inside the branch.

<br>

A `switch_branch` is any of:

- `$($access:access_mode)? $variant:ident $fields:fields => $branch_expr:branch_expr`:
Checks whether the enum is the `$variant` variant,and if it is that variant,
destructures the fields,and runs the `$branch_expr` code
with the enum variant bound (a `VariantProxy<_,TS!($variant)>`) to the `$proxy` variable
(if it was declared).

- `$(_)? if $condition:expression => $branch_expr:branch_expr`:
A regular if expression,where `$branch_expr` is run if `$condition` evaluates to `true`.

- `$(_)? if $pattern:pattern = $value:expression => $branch_expr:branch_expr`:
A regular if let expression,
where `$branch_expr` is run if `$value` matches the `$pattern` pattern,
with access to the variables declared inside the pattern.



<br>

`fields` can be any of:

- `{ $( $field:named_field_destructure ),* }`:
    A braced variant,with named fields,
    in which the fields can be bound by their names,
    or into an optional pattern.<br>
    Example: `Foo{x}=>{}`,the `x` field is bound to the x variable.<br>
    Example: `Foo{x:(y,z)}=>{}`,the `x` field is destructured into the y and z variables.<br>

- `( $( $pattern:pattern ),* )`:
    A tuple variant,in which fields don't have a name,
    and can be bound into a pattern.
    Example: `Foo(x)=>{}`,the 0th field is bound to the x variable.<br>
    Example: `Foo((y,z))=>{}`,the 0th field is destructured into the y and z variables.<br>

- ` `:
    A unit variant,used for querying the variant of the enum.
    The fields of the variants (if any) can access through the `$proxy`
    (if it was declared in the switch header).


`named_field_destructure` can be any of:

- `$field_name:identifier`:
    Accesses the field as:
        - A refernce if the variant is accessed by `ref`.
        - A mutable reference if the variant is accessed by `ref mut`.<br>
    Example: `ref Foo{x}=>{}`,the `x` field is bound as the `x` reference.<br>
    Example: `ref mut Foo{x}=>{}`,the `x` field is bound as the `x` mutable reference.<br>

- `& $field_name:identifier`: Copies the field from a variant accessed by `ref`.<br>
    Example: `ref Foo{&x}=>{}`,the `x` Copy field is copied into the `x` variable.<br>

- `&mut $field_name:identifier`:
Copies the field from a variant accessed by `ref mut`.<br>

- `$field_name:identifier : $pattern:pattern`:
    Destructures the field into an irrefutable pattern,<br>
    Example: `ref Foo{x: &x } copies the x field into an x variable.`<br>
    Example: `ref Foo{x: (a,b) }`
    destructures the x field into a pair of references,`a` and `b`<br>
    Example: `ref mut Foo{x: &mut x } copies the x field into an x variable.`<br>
    Example: `ref mut Foo{x: (a,b) }`
    destructures the x field into a pair of mutable references,`a` and `b`<br>

<br>
<br>

A `branch_expr` can be either:

- `$match_expr:expression ,` (the comma is necessary before any other branch):
A single expression.

- `{ $($anything:token)* } $(,)?`:
Any tokens wrapped inside braces,with an optional comma trailing comma.




*/
#[macro_export]
macro_rules! switch{
    ( ref mut $($rem:tt)* )=>{
        $crate::switch_inn!{@top [refmut] $($rem)* }
    };
    ( &mut $($rem:tt)* )=>{
        $crate::switch_inn!{@top [refmut] $($rem)* }
    };
    ( ref $($rem:tt)* )=>{
        $crate::switch_inn!{@top [ref] $($rem)* }
    };
    ( & $($rem:tt)* )=>{
        $crate::switch_inn!{@top [ref] $($rem)* }
    };
    (move $($rem:tt)* )=>{
        $crate::switch_inn!{@top [move] $($rem)* }
    };
    (mut $($rem:tt)* )=>{
        compile_error!("Expected `ref`/`ref mut`/`move`,found `mut`")
    };
    ( $($rem:tt)+ )=>{
        $crate::switch_inn!{@top [move] $($rem)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! switch_inn{
    (@top
        [$def_access:ident]
        $matched:ident;
        $($match_arms:tt)+
    )=>{
        $crate::switch_inn!{
            @top_ident
            [ $def_access ]
            $matched,$matched;
            $($match_arms)*
        }
    };
    // Have to match on the same ident twice because the `self` passed to the macro
    // is not the same as `self` passed from within the macro.
    (@top_ident
        [$def_access:ident]
        $matched:ident,self;
        $($match_arms:tt)+
    )=>{
        $crate::switch_inn!{
            @branch
            [ $def_access (unassigned _struc_proxy_ $matched) ]
            [ vari() code() default() ]
            $($match_arms)*
        }
    };
    (@top_ident
        [$def_access:ident]
        $matched:ident,$matched2:ident;
        $($match_arms:tt)+
    )=>{
        $crate::switch_inn!{
            @branch
            [ $def_access (unassigned $matched $matched) ]
            [ vari() code() default() ]
            $($match_arms)*
        }
    };
    (@top
        [$def_access:ident]
        $proxy_:ident = $matched:expr ;
        $($match_arms:tt)+
    )=>{{
        #[allow(unused_mut)]
        let mut $proxy_=$matched;
        $crate::switch_inn!{
            @branch
            [ $def_access (assigned $proxy_ $proxy_) ]
            [ vari() code() default() ]
            $($match_arms)*
        }
    }};
    (@top
        [$def_access:ident]
        $matched:expr;
        $($match_arms:tt)+
    )=>{
        $crate::switch_inn!{
            @top
            [$def_access]
            _struc_proxy_=$matched;
            $($match_arms)+
        }
    };
    (@top
        $($proxy_:ident)? $(= $matched:expr)? ;
    )=>{
        compile_error!("expected a non-empty switch")
    };
    (@finish
        [ $def_access:ident ($ass:ident $proxy_:ident $self_:tt) ]
        [
            vari $variants:tt
            code $code:tt
            default()
        ]
    )=>{
        $crate::switch_inn!{
            @finish
            [$def_access ($ass $proxy_ $self_)]
            [
                vari $variants
                code $code
                default({unsafe{
                    #![allow(unused_unsafe)]
                    use structural::pmr::{ GetElseValue, GetVariantCountHack };
                    let (mut _expected,found,ret)=
                        structural::pmr::as_phantomdata(&($self_))
                        .structural_get_variant_count(<_switch_fp_::VariantCount>::NEW)
                        .get_else_values();
                    _expected=found;
                    ret
                }})
            ]
        }
    };
    (@finish
        $top:tt
        [
            vari( $($variant:tt)* )
            code(
                (
                    $($first_branch_code:tt)*
                )
                $((
                    $($code:tt)*
                ))*
            )
            default( $($default:tt)+ )
        ]
    )=>{{
        #[allow(non_upper_case_globals)]
        pub mod _switch_fp_{
            $crate::_switch_tstring_aliases!{
                $($variant)*
            }
        }

        $($first_branch_code)*
        $(
            else $($code)*
        )*
        else $($default)*
    }};
    (@branch $top:tt $vars:tt )=>{
        $crate::switch_inn!{@finish $top $vars }
    };
    (@branch $top:tt [ vari $variants:tt code( $($code:tt)* ) default $default_b:tt ]
        $(_)? if $cond:expr => $($rem:tt)*
    )=>{
        $crate::switch_inn!{
            @skip_expr
            $top
            [
                vari $variants
                code(
                    $($code)*
                    (if $cond {
                        $crate::switch_inn!(@get_expr $($rem)*)
                    })
                )
                default $default_b
            ]
            $($rem)*
        }
    };

    (@branch $top:tt [ vari $variants:tt code( $($code:tt)* ) default $default_b:tt ]
        $(_)? if let $pat:pat = $expr:expr => $($rem:tt)*
    )=>{
        $crate::switch_inn!{
            @skip_expr
            $top
            [
                vari $variants
                code(
                    $($code)*
                    (if let $pat = $expr {
                        $crate::switch_inn!(@get_expr $($rem)*)
                    })
                )
                default $default_b
            ]
            $($rem)*
        }
    };

    (@branch $top:tt [ vari $variants:tt code $code:tt default () ]
        _ => $($rem:tt)*
    )=>{
        $crate::switch_inn!{
            @assert_last_branch

            $top
            [
                vari $variants
                code $code
                default ({ $crate::switch_inn!(@get_expr $($rem)*) })
            ]
            $($rem)*
        }
    };

    (@branch $top:tt $vars:tt ref mut $($rem:tt)* )=>{
        $crate::switch_inn!(@branch_1 $top $vars [refmut] $($rem)* )
    };
    (@branch $top:tt $vars:tt ref $($rem:tt)* )=>{
        $crate::switch_inn!(@branch_1 $top $vars [ref] $($rem)* )
    };
    (@branch $top:tt $vars:tt move $($rem:tt)* )=>{
        $crate::switch_inn!(@branch_1 $top $vars [move] $($rem)* )
    };
    (@branch $top:tt $vars:tt mut $($rem:tt)* )=>{
        compile_error!("Expected `ref` or `ref mut`,found `mut`")
    };
    (@branch
        [ $def_access:ident $proxy_:tt ]
        $vars:tt
        $variant:ident
        $($rem:tt)+
    )=>{
        $crate::switch_inn!(@branch_1
            [$def_access $proxy_]
            $vars
            [$def_access]
            $variant $($rem)+
        )
    };
    (@branch_1
        $top:tt $vars:tt [$access:ident]
        $variant:ident => $($rem:tt)*
    )=>{
        $crate::switch_inn!(@branch_2
            $top $vars [$access $variant ()]
            $($rem)*
        )
    };
    (@branch_1
        $top:tt $vars:tt [$access:ident]
        $variant:ident ($($fields:tt)*) => $($rem:tt)*
    )=>{
        $crate::switch_inn!(@branch_2
            $top $vars [$access $variant ($($fields)*)]
            $($rem)*
        )
    };
    (@branch_1
        $top:tt $vars:tt [$access:ident]
        $variant:ident{$($fields:tt)*} => $($rem:tt)*
    )=>{
        $crate::switch_inn!(@branch_2
            $top $vars [$access $variant {$($fields)*}]
            $($rem)*
        )
    };
    (@branch_2
        [ $def_access:ident ($ass:ident $proxy_:ident $self_:tt) ]
        [
            vari( $($prev_variants:tt)* )
            code( $($code:tt)* )
            default $default_b:tt
        ]
        [ $access:ident $variant:ident $fields:tt ]
        $($rem:tt)*
    )=>{
        $crate::switch_inn!{
            @skip_expr
            [$def_access ($ass $proxy_ $self_)]
            [
                vari($($prev_variants)* $variant $fields )
                code(
                    $($code)*
                    (if {
                        use $crate::pmr::_Structural_BorrowSelf;
                        $crate::pmr::IsVariant::is_variant_(
                            $self_._structural_borrow_self(),
                            _switch_fp_::v::$variant::NEW
                        )
                    } {
                        $crate::switch_inn!{@make_proxy $access $variant ($ass $proxy_ $self_) }

                        $crate::switch_inn!{
                            @access_f
                            [$access $variant ($ass $proxy_ $self_)]
                            $fields
                        }

                        $crate::switch_inn!(@get_expr $($rem)* )
                    })
                )
                default $default_b
            ]
            $($rem)*
        }
    };
    (@branch $top:tt $vars:tt $($anything:tt)* )=>{
        compile_error!(concat!(
            "switch branch has invalid syntax:\n\t`",
            stringify!($($anything)*),
            "`"
        ))
    };
    (@make_proxy $access:ident $variant:ident (unassigned $proxy_:ident $self_:tt) )=>{
        $crate::switch_inn!(
            @make_proxy_1 $access $variant #[allow(unused_variables)] ($proxy_ $self_)
        )
    };
    (@make_proxy $access:ident $variant:ident (assigned $proxy_:ident $self_:tt) )=>{
        $crate::switch_inn!(
            @make_proxy_1 $access $variant ($proxy_ $self_)
        )
    };
    (@make_proxy_1 ref $variant:ident $(#$attr:tt)? ($proxy_:ident $self_:tt) )=>{
        $(#$attr)?
        let $proxy_=unsafe{
            use $crate::pmr::_Structural_BorrowSelf;
            $crate::pmr::VariantProxy::from_ref(
                $self_._structural_borrow_self(),
                _switch_fp_::v::$variant::NEW,
            )
        };
    };
    (@make_proxy_1 refmut $variant:ident $(#$attr:tt)? ($proxy_:ident $self_:tt) )=>{
        $(#$attr)?
        let $proxy_=unsafe{
            use $crate::pmr::_Structural_BorrowSelf;
            $crate::pmr::VariantProxy::from_mut(
                $self_._structural_borrow_self_mut(),
                _switch_fp_::v::$variant::NEW,
            )
        };
    };
    (@make_proxy_1 move $variant:ident $(#$attr:tt)? ($proxy_:ident $self_:tt) )=>{
        $(#$attr)?
        #[allow(unused_mut)]
        let mut $proxy_=unsafe{
            $crate::pmr::VariantProxy::new(
                $self_,
                _switch_fp_::v::$variant::NEW,
            )
        };
    };
    (@access_f $vars:tt () )=>{};
    (@access_f $vars:tt {} )=>{};
    (@access_f [ move $variant:ident $proxy_:tt ] $fields:tt )=>{
        compile_error!{concat!(
            "
Cannot move fields from enum variants in switch branches yet.
You can destructure a variant into references or mutable references by prefixing the variant name (or the matched expression) with `ref` or `ref mut` respectively,
ie: `ref Foo{x,&y}` // Gets a reference to x, copies the y field.
ie: `switch!{ ref this; Foo{x,&y}=>(x,y) }` // equivalent to the other example.
")}
    };
    (@access_f $vars:tt ($($patterns:tt)*) )=>{
        $crate::switch_inn!{@access_f_1 $vars [] tuple ($($patterns)*) }
    };
    (@access_f $vars:tt {$($patterns:tt)*} )=>{
        $crate::switch_inn!{@access_f_1 $vars [] brace ($($patterns)*) }
    };
    (@access_f_1
        $vars:tt
        $prev_fields:tt
        brace
        ( $field_name:tt :  $($rem:tt)* )
    )=>{
        $crate::switch_inn!{@access_f_patt
            $vars
            $prev_fields
            brace
            []
            ($($rem)*)
        }
    };
    (@access_f_1
        $vars:tt
        $prev_fields:tt
        $vkind:ident
        ( $($anything:tt)+ )
    )=>{
        $crate::switch_inn!{@access_f_patt
            $vars
            $prev_fields
            $vkind
            []
            ( $($anything)* )
        }
    };
    (@access_f_1
        [ $access:ident $variant:ident self ]
        [ $($variable_pats:tt)* ]
        $vkind:ident
        ()
    )=>{
        let ($($variable_pats)*)={
            use $crate::GetFieldExt;
            $crate::switch_inn!{@call_field_method $access $variant _struc_proxy_ }
        };
    };
    (@access_f_1
        [ $access:ident $variant:ident $proxy_:tt ]
        [ $($variable_pats:tt)* ]
        $vkind:ident
        ()
    )=>{
        let ($($variable_pats)*)={
            use $crate::GetFieldExt;
            $crate::switch_inn!{@call_field_method $access $variant $proxy_ }
        };
    };
    (@access_f_patt
        $vars:tt
        $prev_fields:tt
        $vkind:ident
        []
        ( _ $(, $($rem:tt)* )? )
    )=>{
        $crate::switch_inn!{
            @access_f_1
            $vars
            $prev_fields
            $vkind
            ($($($rem)*)?)
        }
    };
    (@access_f_patt
        $vars:tt
        [ $($prev_fields:tt)* ]
        $vkind:ident
        [$($patt_tokens:tt)*]
        ( $t0:tt  $(, $($rem:tt)* )? )
    )=>{
        $crate::switch_inn!{@access_f_1
            $vars
            [ $($prev_fields)* $($patt_tokens)* $t0, ]
            $vkind
            ($($($rem)*)?)
        }
    };
    (@access_f_patt
        $vars:tt
        [ $($prev_fields:tt)* ]
        $vkind:ident
        [$($patt_tokens:tt)*]
        ( $t0:tt $t1:tt  $(, $($rem:tt)* )? )
    )=>{
        $crate::switch_inn!{@access_f_1
            $vars
            [ $($prev_fields)* $($patt_tokens)* $t0 $t1, ]
            $vkind
            ($($($rem)*)?)
        }
    };
    (@access_f_patt
        $vars:tt
        [ $($prev_fields:tt)* ]
        $vkind:ident
        [$($patt_tokens:tt)*]
        ( $t0:tt $t1:tt $t2:tt  $(, $($rem:tt)* )? )
    )=>{
        $crate::switch_inn!{@access_f_1
            $vars
            [ $($prev_fields)* $($patt_tokens)* $t0 $t1 $t2, ]
            $vkind
            ($($($rem)*)?)
        }
    };
    (@access_f_patt
        $vars:tt
        $prev_fields:tt
        $vkind:ident
        [$($patt_tokens:tt)*]
        ( $t0:tt $t1:tt $t2:tt $($rem:tt)* )
    )=>{
        $crate::switch_inn!{@access_f_patt
            $vars
            $prev_fields
            $vkind
            [$($patt_tokens)* $t0 $t1 $t2]
            ($($rem)*)
        }
    };
    (@access_f_patt
        $vars:tt
        [ $($prev_fields:tt)* ]
        $vkind:ident
        ( $variable_name:ident  $(, $($rem:tt)* )? )
    )=>{
        $crate::switch_inn!{@access_f_1
            $vars
            [ $($prev_fields)* ($variable_name), ]
            $vkind
            ($($($rem)*)?)
        }
    };
    (@access_f_patt
        $vars:tt
        [ $($prev_fields:tt)* ]
        $vkind:ident
        ( $variable:tt  $(, $($rem:tt)* )? )
    )=>{
        $crate::switch_inn!{@access_f_1
            $vars
            [ $($prev_fields)* ($variable), ]
            $vkind
            ($($($rem)*)?)
        }
    };
    ////////////////////////////////////////////////
    (@call_field_method ref $variant:ident ($ass:ident $proxy_:ident $self_:tt) )=>{
        $proxy_.fields(_switch_fp_::f::$variant)
    };
    (@call_field_method refmut $variant:ident ($ass:ident $proxy_:ident $self_:tt) )=>{
        $proxy_.fields_mut(_switch_fp_::f::$variant)
    };
    ////////////////////////////////////////////////
    (@skip_expr $top:tt $vars:tt $expr:expr $(,$($rem:tt)*)? )=>{
        $crate::switch_inn!{ @branch $top $vars $($($rem)*)? }
    };
    (@skip_expr $top:tt $vars:tt $expr:block $($rem:tt)*)=>{
        $crate::switch_inn!{ @branch $top $vars $($rem)* }
    };
    ////////////////////////////////////////////////
    (@get_expr $expr:block $($rem:tt)*)=>{
        $expr
    };
    (@get_expr $expr:expr $(, $($rem:tt)*)? )=>{
        $expr
    };
    ////////////////////////////////////////////////
    (@assert_last_branch $top:tt $vars:tt $expr:expr $(,)?)=>{
        $crate::switch_inn!{ @branch $top $vars }
    };
    (@assert_last_branch $top:tt $vars:tt $block:block)=>{
        $crate::switch_inn!{ @branch $top $vars }
    };
    (@assert_last_branch $top:tt $vars:tt $expr:expr , $($rem:tt)+)=>{
        $crate::switch_inn!{@last_branch_err $($rem)+}
    };
    (@assert_last_branch $top:tt $vars:tt $block:block $($rem:tt)+)=>{
        $crate::switch_inn!{@last_branch_err $($rem)+}
    };
    ////////////////////////////////////////////////
    (@last_branch_err $($rem:tt)+)=>{
        compile_error!{concat!(
            "Expected no switch arms after the default one, found: ",
            stringify!($($rem)+)
        )}
    };
}
