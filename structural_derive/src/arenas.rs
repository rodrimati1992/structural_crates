use std::fmt;

use typed_arena::Arena;

macro_rules! declare_arenas {
    (
        $( $field_name:ident : $arena_type:ty , )*
    ) => {
        pub(crate) struct Arenas {
            $( $field_name : Arena<$arena_type>, )*
        }

        impl Arenas{
            pub(crate) fn alloc<T>(&self, value:T)-> &T
            where
                Self:AllocMethods<T>
            {
                self.alloc_ref(value)
            }
        }

        impl Default for Arenas{
            fn default()->Self{
                Arenas{
                    $( $field_name:Arena::new(), )*
                }
            }
        }

        impl fmt::Debug for Arenas{
            fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
                fmt::Debug::fmt("Arenas{..}",f)
            }
        }

        pub(crate) trait AllocMethods<T>{
            fn alloc_ref(&self, value: T) -> &T;
        }


        $(
            impl AllocMethods<$arena_type> for Arenas{
                fn alloc_ref(&self, value: $arena_type) -> &$arena_type {
                    self.$field_name.alloc(value)
                }
            }

        )*

    }
}

declare_arenas! {
    ident: syn::Ident,
    types: syn::Type,
    lit_int: syn::LitInt,
    type_param_bounds: crate::structural_alias_impl_mod::TypeParamBounds,
}
