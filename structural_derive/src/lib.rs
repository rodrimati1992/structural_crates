/*!
An implementation detail of abi_stable.
*/

#![recursion_limit="192"]
// #![deny(unused_variables)]
// #![deny(unused_imports)]
// #![deny(unused_parens)]
// #![deny(unused_assignments)]
// #![deny(unused_mut)]
#![deny(unreachable_patterns)]
#![deny(unused_doc_comments)]
#![deny(unconditional_recursion)]

extern crate proc_macro;


mod parse_utils;


use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;


/**
The implementation of the tstr macro when const parameters aren't supported.
*/
#[proc_macro]
pub fn tstr_impl(input: TokenStream1) -> TokenStream1 {
    use crate::parse_utils::ParsePunctuated;

    parse_or_compile_err(input,|str_lit: ParsePunctuated<syn::LitStr,syn::Token!(,)>|{
        use std::fmt::Write;
        let strings=str_lit.list.into_iter()
            .map(|string|{
                let mut buffer=String::new();
                let string=string.value();
                let bytes=string.bytes()
                    .map(move|b|{
                        buffer.clear();
                        let _=write!(buffer,"B{}",b);
                        syn::Ident::new(&buffer, proc_macro2::Span::call_site())
                    });
                quote!( TString<( #(#bytes,)* )> )
            });

        Ok(quote!(
            use structural::proc_macro_reexports::*;

            pub const VALUE:(#(#strings),*)=MarkerType::MTVAL;
        ))
    }).into()
}





////////////////////////////////////////////////////////////////////////////////


fn parse_or_compile_err<P,F>(input:TokenStream1,f:F)->TokenStream2
where 
    P:syn::parse::Parse,
    F:FnOnce(P)->Result<TokenStream2,syn::Error>
{
    syn::parse::<P>(input)
        .and_then(f)
        .unwrap_or_else(|e| e.to_compile_error() )
}
