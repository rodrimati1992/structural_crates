/*!
Contains traits for accessing multiple fields at once.
*/

use super::*;

/// This trait allows a TStringSet to borrow the fields it names.
pub trait GetMultiField<'a,This:?Sized>{
    type MultiTy:'a;

    fn multi_get_field_(this:&'a This)->Self::MultiTy;
}

/// This trait allows a TStringSet to borrow the fields it names mutably.
pub trait GetMultiFieldMut<'a,This:?Sized>:Sized{
    type MultiTy:'a;

    fn multi_get_field_mut_(this:&'a mut This,_:TStringSet<Self>)->Self::MultiTy;
}


macro_rules! impl_get_multi_field {
    ( $($fname:ident)* ) => (
        impl<'a,This:?Sized,$($fname,)*> GetMultiField<'a,This> for ($($fname,)*)
        where
            $(
                This:GetField<$fname>,
                GetFieldType<This,$fname>:'a,
            )*
        {
            type MultiTy=(
                $(
                    &'a GetFieldType<This,$fname>,
                )*
            );

            fn multi_get_field_(this:&'a This)->Self::MultiTy{
                (
                    $(
                        GetField::<$fname>::get_field_(this),
                    )*
                )
            }
        }        

        impl<'a,This:?Sized,$($fname,)*> GetMultiFieldMut<'a,This> for ($($fname,)*)
        where
            $(
                This:GetFieldMut<$fname>,
                GetFieldType<This,$fname>:'a,
            )*
        {
            type MultiTy=(
                $(
                    &'a mut GetFieldType<This,$fname>,
                )*
            );

            default_if!{
                #[allow(non_snake_case)]
                cfg(feature="specialization")

                fn multi_get_field_mut_(this:&'a mut This,_:TStringSet<Self>)->Self::MultiTy{
                    $(
                        let $fname:GetFieldMutRefFn<$fname,GetFieldType<This,$fname>>=
                            this.get_field_mutref_func();
                    )*
                    let this=GetFieldMut::<F0>::as_mutref(this);
                    // unsafe:
                    // This is passing the pointer obtained from the `as_mutref` function,
                    // which ought be the same for all impls of GetFieldMut on the same type.
                    unsafe{
                        (
                            $(
                                ($fname.func)(this.clone(),$fname),
                            )*
                        )
                    }
                }
            }

        }


        #[cfg(feature="specialization")]
        impl<'a,This,$($fname,)*> GetMultiFieldMut<'a,This> for ($($fname,)*)
        where
            $(
                This:GetFieldMut<$fname>,
                GetFieldType<This,$fname>:'a,
            )*
        {
            #[allow(non_snake_case)]
            fn multi_get_field_mut_(this:&'a mut This,_:TStringSet<Self>)->Self::MultiTy{
                $(
                    let $fname:GetFieldMutRefFn<$fname,GetFieldType<This,$fname>>=
                        this.get_field_mutref_func();
                )*
                let this=GetFieldMut::<F0>::as_mutref(this);
                unsafe{
                    (
                        $(
                            <This as GetFieldMut<$fname>>::get_field_mutref(this.clone(),$fname),
                        )*
                    )
                }
            }
        }

    )
}


impl_get_multi_field!{F0}
impl_get_multi_field!{F0 F1}
impl_get_multi_field!{F0 F1 F2}
impl_get_multi_field!{F0 F1 F2 F3}
impl_get_multi_field!{F0 F1 F2 F3 F4}
impl_get_multi_field!{F0 F1 F2 F3 F4 F5}
impl_get_multi_field!{F0 F1 F2 F3 F4 F5 F6}
impl_get_multi_field!{F0 F1 F2 F3 F4 F5 F6 F7}

