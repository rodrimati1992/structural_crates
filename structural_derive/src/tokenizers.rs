use proc_macro2::{
    TokenStream as TokenStream2,
    Span,
};

use quote::{quote,ToTokens};

use syn::Ident;


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



/// Represents a crate-visible module with a bunch of type aliases for TStrings.
pub(crate) struct NamedModuleAndTokens{
    pub(crate) names_module:Ident,
    pub(crate) alias_names:Vec<Ident>,
    pub(crate) mod_tokens:TokenStream2,
}

impl NamedModuleAndTokens{
    pub fn new<'a,I,S>(thing_ident:&'a syn::Ident,iter:I)->Self
    where
        I:IntoIterator<Item=S>+Clone,
        S:std::fmt::Display+'a,
    {

        let names_module=Ident::new(
            &format!("{}_names_module",thing_ident),
            Span::call_site(),
        );

        let alias_names=iter.clone().into_iter()
            .map(|ident| Ident::new(&format!("STR_{}",ident),Span::call_site()) )
            .collect::<Vec<Ident>>();

        let aliases_names=alias_names.iter()
            .zip( iter.clone().into_iter() )
            .map(|(alias_name,field_name)|{
                let field_name=tstring_tokenizer(field_name.to_string());
                quote!(pub type #alias_name=#field_name;)
            });

        let mod_tokens=quote!(
            pub(crate) mod #names_module{
                use structural::proc_macro_reexports::*;

                #(#aliases_names)*
            }
        );
        
        Self{
            names_module,
            alias_names,
            mod_tokens,
        }
    }
}


impl ToTokens for NamedModuleAndTokens{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.mod_tokens.to_tokens(tokens);
    }
}






