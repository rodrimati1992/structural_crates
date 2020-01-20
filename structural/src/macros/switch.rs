/**
Provides basic pattern matching for structural enums.

structural enums are nonexhaustive.
`switch` will only evaluate to a non-`()` type
if you handle the case where the enum is none of the known variants
with `_ => some_expression`.

# Syntax

This uses a macro_rules-like syntax to describe the input to this macro.
If you want to see a example that uses all the syntax,go to the
[Syntax Example](#TODO) section.

```text
switch!{
    $proxy:identifier $( = $matched_value:expression )? ;

    $(
        $switch_branch:switch_branch
    )*
}
```

If no `$matched_value` is passed,the `$proxy` variable will be taken from the scope.

If `$matched_value` *is* passed,it is stored in the `$proxy` variable.

In switch arms that match on the variant of the enum,
`$proxy` is a `VariantProxy<TypeOfMatchedValue, fp!(NameOfVariant)>`.

In switch arms that don't match on the variant of the enum,
`$proxy` is the type of the matched value.

<br>

A `switch_branch` is any of:

- ` $variant:ident => $branch_expr:branch_expr`:
Checks whether the enum is the `$variant` variant,and if it is that variant,
runs the `$branch_expr` code,
with the enum variant bound (a `VariantProxy<_,fp!($variant)>`) to the `$proxy` variable.

- `if $condition:expression => $branch_expr:branch_expr`:
Like a regular if expression,where `$branch_expr` is run if `$condition` evaluates to `true`.

- `if $pattern:pattern = $value:expression => $branch_expr:branch_expr`:
Like a regular if let expression,
where `$branch_expr` is run if `$value` matches the `$pattern` pattern,
with access to the variables declared inside the pattern.

- `_=> $branch_expr:branch_expr`

The default case,required for the `switch!{}` macro to evaluate to a non-`()` value.
This must be the last switch arm.

<br>

A `branch_expr` can be either:

- `$match_expr:expression ,` (the comma is mandatory): A single expression

- `{ $($anything:token)* } $(,)?`:
Any tokens wrapped inside braces,with an optional comma after the block.



# Syntax Example

This example demonstrates all the syntax of the `switch` macro.

```
# use structural::{GetFieldExt,Structural,fp,switch};
#
# #[derive(Debug,Structural)]
# enum Enum{
#     Foo{a:u32},
#     Bar{b:u64},
#     Baz{c:&'static str},
#     Bam{
#       #[struc(optional)]
#       d:Option<usize>
#     },
# }

let this=Enum::Foo{a:100};

switch!{ this ;
    Foo=>{
        assert_eq!( this.field_(fp!(a)), &100 )
    }
    Bar=>assert_eq!( this.field_(fp!(b)), &200 ),
    _=>() // The `,` is optional after the last switch arm.
}

let text="99";
let number = switch!{ other = Enum::Baz{ c:"55" } ;
    if 2+2!=4 => unreachable!("2+2 is 4, silly!"),
    Baz => other
        .into_field(fp!(c))
        .parse::<u32>()
        .unwrap(),  // The `,` is required here
    if let Ok(x@99)=text.parse::<u32>() => {
        println!("{:?} parses to {}u32",text,x);

        // The type of `other` is `Enum` in non-matching branches.
        println!("{:?}", other );

        x
    }
    Bam =>{
        assert_eq!( other.field_mut(fp!(d)), Some(&mut 9999) );
        100
    }, // The `,` is optional after `{...}`
    _=>101  // The `,` is optional after the last switch arm.
};

assert_eq!(number,55);

```

# Example

This gets the human-readable name of the direction the enum represents.

```
use structural::{Structural,switch};

fn main(){
    assert_eq!( Direction4::Left.direction4_to_str(), "left" );
    assert_eq!( Direction4::Right.direction4_to_str(), "right" );

    assert_eq!( Direction6::Up.direction4_to_str(), "up" );
    assert_eq!( Direction6::Down.direction4_to_str(), "down" );
    assert_eq!( Direction6::Forward.direction4_to_str(), "unknown direction" );
    assert_eq!( Direction6::Backward.direction4_to_str(), "unknown direction" );
}


// This is an extension trait for all enums with at least the variants of `Direction4`
trait Direction4Ext: Direction4_SI{
    fn direction4_to_str(&self)->&'static str {
        switch!{ this= self;
            Left=>"left",
            Right=>"right",
            Down=>"down",
            Up=>"up",
            _=>"unknown direction",
        }
    }
}

impl<This> Direction4Ext for This
where
    This:?Sized+Direction4_SI
{}


#[derive(Structural)]
enum Direction4{
    Left,
    Right,
    Down,
    Up,
}

#[derive(Structural)]
enum Direction6{
    Left,
    Right,
    Down,
    Up,
    Forward,
    Backward,
}

```

# More Examples

For more examples you can look at the ones [in the docs for enums](TODO)

*/
#[macro_export]
macro_rules! switch{
    (
        $proxy_:ident $( = $matched:expr )?;
        // No switch arms
    )=>{{
        $(let $proxy_=$matched;)?
        $crate::switch!{
            @branch
            [ $proxy_]
            [ vari() code((if false {})) default() ]
        }
    }};

    (
        $proxy_:ident $( = $matched:expr )?;
        $( $match_block:tt )+
    )=>{{
        $(let $proxy_=$matched;)?
        $crate::switch!{
            @branch
            [ $proxy_]
            [ vari() code() default() ]
            $( $match_block )*
        }
    }};
    ( $expr:expr; $($rem:tt)* )=>{
        compile_error!(concat!(
            "Expected `identifer` or `identifier = expression`,instead found: ",
            stringify!($expr)
        ))
    };

    (@branch
        [ $proxy_:ident ]
        [
            vari $variants:tt
            code $code:tt
            default()
        ]
    )=>{
        $crate::switch!{
            @branch
            [$proxy_]
            [
                vari $variants
                code $code
                default({unsafe{
                    #![allow(unused_unsafe)]
                    use structural::pmr::{ GetElseValue, GetVariantCountHack };
                    let (mut expected,found,ret)=
                        structural::pmr::as_phantomdata(&$proxy_)
                        .structural_get_variant_count(<_switch_fp_::__TString_Aliases_Count>::NEW)
                        .get_else_values();
                    expected=found;
                    ret
                }})
            ]
        }
    };
    (@branch
        $top:tt
        [
            vari( $($variant:ident)* )
            code(
                (
                    $($first_branch_code:tt)*
                )
                $((
                    $($code:tt)*
                ))*
            )
            default( $($default:tt)* )
        ]
    )=>{
        mod _switch_fp_{
            $crate::tstr_aliases!{
                pub mod _switch_tstr_{
                    @count
                    $($variant,)*
                }
            }

            pub use _switch_tstr_::__TString_Aliases_Count;

            $(
                pub type $variant= structural::pmr::FieldPath1<_switch_tstr_::$variant>;
                pub const $variant: $variant=<$variant>::NEW;
            )*
        }

        $($first_branch_code)*
        $(
            else $($code)*
        )*
        else $($default)*
    };

    (@branch $top:tt [ vari $variants:tt code( $($code:tt)* ) default $default_b:tt ]
        $(_)? if $cond:expr => $($rem:tt)*
    )=>{
        $crate::switch!{
            @skip_expr
            $top
            [
                vari $variants
                code(
                    $($code)*
                    (if $cond {
                        $crate::switch!(@get_expr $($rem)*)
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
        $crate::switch!{
            @skip_expr
            $top
            [
                vari $variants
                code(
                    $($code)*
                    (if let $pat = $expr {
                        $crate::switch!(@get_expr $($rem)*)
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
        $crate::switch!{
            @assert_last_branch

            $top
            [
                vari $variants
                code $code
                default ({ $crate::switch!(@get_expr $($rem)*) })
            ]
            $($rem)*
        }
    };

    (@branch
        [ $proxy_:ident ]
        [
            vari( $($prev_variants:ident)* )
            code( $($code:tt)* )
            default $default_b:tt
        ]
        $variant:ident=> $($rem:tt)*
    )=>{
        $crate::switch!{
            @skip_expr
            [$proxy_]
            [
                vari($($prev_variants)* $variant)
                code(
                    $($code)*
                    (if $crate::pmr::IsVariant::is_variant_(&$proxy_,_switch_fp_::$variant) {
                        let mut $proxy_=unsafe{
                            $crate::pmr::VariantProxy::<_,_switch_fp_::$variant>::new(
                                $proxy_
                            )
                        };

                        $crate::switch!(@get_expr $($rem)* )
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

    (@skip_expr $top:tt $vars:tt $expr:expr, $($rem:tt)*)=>{
        $crate::switch!{ @branch $top $vars $($rem)* }
    };
    (@skip_expr $top:tt $vars:tt $expr:block $($rem:tt)*)=>{
        $crate::switch!{ @branch $top $vars $($rem)* }
    };
    (@get_expr $expr:block $($rem:tt)*)=>{
        $expr
    };
    (@get_expr $expr:expr , $($rem:tt)*)=>{
        $expr
    };
    (@get_expr $expr:expr)=>{
        $expr
    };
    (@assert_last_branch $top:tt $vars:tt $expr:expr $(,)?)=>{
        $crate::switch!{ @branch $top $vars }
    };
    (@assert_last_branch $top:tt $vars:tt $block:block)=>{
        $crate::switch!{ @branch $top $vars }
    };
    (@assert_last_branch $top:tt $vars:tt $expr:expr , $($rem:tt)+)=>{
        $crate::switch!{@last_branch_err $($rem)+}
    };
    (@assert_last_branch $top:tt $vars:tt $block:block $($rem:tt)+)=>{
        $crate::switch!{@last_branch_err $($rem)+}
    };
    (@last_branch_err $($rem:tt)+)=>{
        compile_error!{concat!(
            "Expected no switch arms after the default one, found: ",
            stringify!($($rem)+)
        )}
    };
}
