/// Constructs an anonymous struct,
/// which implements all the accessor traits for its field.
///
/// # Syntax
///
/// Here is the entire syntax for this macro:
///
/// ```
/// # use structural::make_struct;
///
/// # let foo=();
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
///     foo, // This initializes a `foo` field with the `foo` variable.
/// }
/// # ;
/// ```
///
/// # Example
///
/// ```rust
/// use structural::{StructuralExt,make_struct,structural_alias,fp};
///
/// use std::fmt::Debug;
///
/// structural_alias!{
///     pub trait Runner{
///         name:String,
///         stamina:u32,
///     }
/// }
///
/// // Everything below could be on a separate crate (ignoring imports)
///
/// # fn main(){
///
/// fn get_runner(this:&(impl Runner+Debug) ){
///     assert_eq!( this.field_(fp!(name)).as_str(), "hello","{:?}",this);
///     assert_eq!( this.field_(fp!(stamina)), &100,"{:?}",this);
/// }
///
/// get_runner(&make_struct!{
///     #![derive(Debug)]
///     name:"hello".into(),
///     stamina:100,
/// });
///
///
/// fn ret_get_runner(name:String)->impl Runner+Clone{
///     make_struct!{
///         #![derive(Clone)]
///         name,
///         stamina:4_000_000_000,
///     }
/// }
///
/// {
///     let runner=ret_get_runner("hello".into());
///     # let _=runner.field_(fp!(name));
///     # let _=runner.field_(fp!(stamina));
///     let (name,stamina)=runner.fields(fp!( name, stamina ));
///     assert_eq!( name, "hello" );
///     assert_eq!( *stamina, 4_000_000_000 );
/// }
///
#[cfg_attr(
    feature = "alloc",
    doc = r###"
fn get_dyn_runner()->Box<dyn Runner>{
    Box::new(make_struct!{
       name:"hello".into(),
       stamina:4_000_000_000,
    })
}

{
    let runner=get_dyn_runner();
    assert_eq!( runner.field_(fp!(name)).as_str(), "hello" );
    assert_eq!( runner.field_(fp!(stamina)), &4_000_000_000 );
}

"###
)]
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
            $field_name:ident $( : $field_value:expr )?
        ),*
        $(,)?
    ) => ({
        $( $( let $field_name=$field_value; )? )*

        {
            #[allow(non_camel_case_types)]
            #[allow(unused_imports)]
            mod _anonyous_struct_{
                #[allow(unused_imports)]
                use super::*;

                $crate::tstr_aliases!{
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

                enum __Indices{
                    $($field_name,)*
                }

                $crate::_private_impl_getters_for_derive_struct!{
                    impl[$($field_name,)*] __Anonymous_Struct<$($field_name,)*>
                    where[]
                    {
                        DropFields{ drop_fields=just_fields, }

                        $((
                            IntoFieldMut<
                                $field_name : $field_name,
                                __Indices::$field_name as u8,
                                _names_module_::$field_name,
                                stringify!($field_name),
                            >
                        ))*
                    }
                }
            }
            _anonyous_struct_::__Anonymous_Struct{
                $($field_name,)*
            }
        }
    });
}
