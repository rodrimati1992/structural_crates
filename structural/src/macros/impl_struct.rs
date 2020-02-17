/**
For declaring an anonymous structural type,this expands to an `impl Trait`.

**This is only usable from Rust 1.40 onwards**

To construct an anonymous struct you can to use the
[`make_struct` macro](./macro.make_struct.html).

To declare a trait aliasing multiple accessor traits,you can use the
[`structural_alias` macro](./macro.structural_alias.html).

# Non-accessor bounds

You can add non-accessor bounds by listing them before fields,
then separating them from the fields with `;`.

Example: `impl_struct!{ Clone + Debug; foo:u32, bar:u64 }`

# Access modifiers

Fields can optionally be prefixed with `ref`,`mut`,`move`,or `mut move`
to specify which accessor traits are required for the field.

If none is specified,then the `IntoFieldMut` trait will be required for that field,
with shared,mutable,and by value access to the field.

# Example

This demonstrates `impl_struct` with non-accessor bounds.

*/
#[cfg_attr(feature = "better_macros", doc = " ```rust")]
#[cfg_attr(not(feature = "better_macros"), doc = " ```ignore")]
/**

use structural::{fp,impl_struct,make_struct,GetFieldExt};

fn into_hi_ho_fields(
    this: impl_struct!{ Clone; hi:u32, ho:String }
)-> (u32,String) {
    (
        this.clone().into_field(fp!(hi)),
        this.into_field(fp!(ho)),
    )
}

assert_eq!(
    into_hi_ho_fields(make_struct!{
        #![derive(Clone)]

        hi: 99,
        ho: "what".into(),
    }),
    (99,"what".to_string())
);



```

# Example

This demonstrates the field access modifiers.
In this case the fields are read only.

*/
#[cfg_attr(feature = "better_macros", doc = " ```rust")]
#[cfg_attr(not(feature = "better_macros"), doc = " ```ignore")]
/**

use structural::{fp,impl_struct,make_struct,GetFieldExt};

fn into_colors(
    mut this: u32,
)-> impl_struct!{ ref red:u8, ref green:u8, ref blue:u8, ref alpha:u8 } {
    make_struct!(
        red  :this as u8,
        green:(this>>8) as u8,
        blue :(this>>16) as u8,
        alpha:(this>>24) as u8,
    )
}

let colors = into_colors(0x40_30_20_10);
assert_eq!( colors.field_(fp!(red)), &0x10 );
assert_eq!( colors.field_(fp!(green)), &0x20 );
assert_eq!( colors.field_(fp!(blue)), &0x30 );
assert_eq!( colors.field_(fp!(alpha)), &0x40 );

```
*/
#[macro_export]
macro_rules! impl_struct {
    ( $($macro_params:tt)* ) => (
        $crate::_impl_struct_impl!($($macro_params)*)
    )
}
