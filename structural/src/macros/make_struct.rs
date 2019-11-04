/// Constructs an anonymous struct,
/// which implements the `IntoFieldMut` accessor trait for all its field.
///
/// # Syntax
///
/// Here is the entire syntax for this macro:
///
/// ```
/// # use structural::make_struct;
///
/// # let _=
/// make_struct!{
///     // This is an inner attribute,which is applied to the struct declaration.
///     // In this case it's deriving `Debug` for the struct.
///     #![derive(Debug)]
///     
///     // This is an attribute on the first field.
///     #[doc="you must put attributes for the first field after the inner attributes"]
///     size_cm:0,
///     place:"Anctartica",
/// }
/// # ;
/// ```
///
/// # Example
///
/// ```rust
/// use structural::{GetFieldExt,make_struct,structural_alias,ti};
///
/// use std::fmt::Debug;
///
/// structural_alias!{
///     pub trait Runner{
///         mut move name:String,
///         mut move stamina:u32,
///     }
/// }
///
/// // Everything bellow could be on a separate crate (ignoring imports)
///
/// # fn main(){
///
/// fn get_runner(this:&(impl Runner+Debug) ){
///     assert_eq!( this.field_(ti!(name)).as_str(), "hello","{:?}",this);
///     assert_eq!( this.field_(ti!(stamina)), &100,"{:?}",this);
/// }
///
/// get_runner(&make_struct!{
///     #![derive(Debug)]
///     name:"hello".into(),
///     stamina:100,
/// });
///
/// // Unfortunately,due to a rustc bug as of 2019-11-02,
/// // you can't call the accessor methods provided by GetFieldExt
/// // on the return value of this function.
/// // Issue for the rustc bug: https://github.com/rust-lang/rust/issues/66057
/// // fn get_runner()->impl Runner+Copy+Clone{
/// //     make_struct!{
/// //         #![derive(Copy,Clone)]
/// //         name:"hello".into(),
/// //         stamina:4_000_000_000,
/// //     }
/// // }
///
#[cfg_attr(feature="alloc",doc=r###"
fn get_dyn_runner()->Box<dyn Runner>{
    Box::new(make_struct!{
       #![derive(Copy,Clone)]
       name:"hello".into(),
       stamina:4_000_000_000,
    })
}

{
    let runner=get_dyn_runner();
    assert_eq!( runner.field_(ti!(name)).as_str(), "hello" );
    assert_eq!( runner.field_(ti!(stamina)), &4_000_000_000 );
}

"###)]
///
/// # }
///
/// ```
#[macro_export]
macro_rules! make_struct {
    (
        $( #![$inner_attrs:meta] )*
        $(
            $( #[$field_attrs:meta] )*
            $field_name:ident : $field_value:expr,
        )*
    ) => ({
        #[allow(non_camel_case_types)]
        mod _anonyous_struct_{
            #[allow(unused_imports)]
            use super::*;

            $crate::declare_names_module!{
                pub mod _names_module_{
                    $( $field_name, )*
                }
            }

            $( #[$inner_attrs] )*
            pub struct __Anonymous_Struct<$($field_name),*>{
                $(
                    $( #[$field_attrs] )*
                    pub $field_name:$field_name,
                )*
            }
            
            $crate::impl_getters_for_derive!{
                impl[$($field_name,)*] __Anonymous_Struct<$($field_name,)*> 
                where[]
                {
                    $((
                        IntoFieldMut<
                            $field_name : $field_name,
                            _names_module_::$field_name,
                            stringify!($field_name),
                        >
                    ))*
                }
            }
        }
        use _anonyous_struct_::__Anonymous_Struct;
        


        __Anonymous_Struct{
            $($field_name:$field_value,)*
        }
    })
}
