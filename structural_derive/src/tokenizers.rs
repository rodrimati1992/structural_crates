use proc_macro2::{
    TokenStream as TokenStream2,
    Span,
};

use quote::{quote,ToTokens};

/// Tokenizes a `TString<>` in which each character is written as just its identifier,
/// requires that this be used in a module that imports everything from 
/// `structural::proc_macro_reexports`.
pub(crate) fn tstring_tokenizer<S>(string:S)->impl ToTokens
where
    S:AsRef<str>
{
    use std::fmt::Write;
    let mut buffer=String::new();
    let bytes=string.as_ref().bytes()
        .map(move|b|{
            buffer.clear();
            let c=b as char;
            let _=if (c.is_alphanumeric() || c=='_' )&& b < 128 {
                write!(buffer,"_{}",c)
            }else{
                write!(buffer,"B{}",b)
            };
            syn::Ident::new(&buffer, Span::call_site())
        });
    quote!( TString<( #(#bytes,)* )> )
}


/// Generates a crate-local module with a bunch of type aliases for TStrings.
pub(crate) fn names_modules<'a,I,A,S>(mod_name:&'a syn::Ident,iter:I)->TokenStream2
where
    I:IntoIterator<Item=(&'a A,S)>,
    A:ToTokens+'a,
    S:AsRef<str>,
{
    let iter=iter.into_iter()
        .map(|(alias_name,field_name)|{
            let field_name=tstring_tokenizer(field_name);
            quote!(pub type #alias_name=#field_name;)
        });

    quote!(
        pub(crate) mod #mod_name{
            use structural::proc_macro_reexports::*;

            #(#iter)*
        }
    )
}