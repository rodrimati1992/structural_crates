use proc_macro2::{
    TokenStream as TokenStream2,
    Span,
};

use quote::{quote,ToTokens};

use syn::Ident;


/// Whether to use the full path to an item when refering to it.
#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub(crate) enum FullPathForChars{
    Yes,
    No,
    StructPmr,
}


////////////////////////////////////////////////////////////////////////////////////////////


pub(crate) fn struct_pmr_prefix()->TokenStream2{
    quote!( __struct_pmr:: )
}

#[allow(dead_code)]
pub(crate) fn struct_pmr()->TokenStream2{
    quote!( __struct_pmr )
}



/// Tokenizes a `TString<>` in which each character is written as a type.
pub(crate) fn tident_tokens<S>(string:S,char_verbosity:FullPathForChars)->TokenStream2
where
    S:AsRef<str>
{
    let path_prefix=match char_verbosity {
        FullPathForChars::Yes=>quote!(::structural::chars::),
        FullPathForChars::No=>quote!(),
        FullPathForChars::StructPmr=>struct_pmr_prefix(),
    };

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
    quote!( ::structural::type_level::TString<( #( #path_prefix #bytes,)* )> )
}


////////////////////////////////////////////////////////////////////////////////////////////


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
                let field_name=tident_tokens(field_name.to_string(),FullPathForChars::No);
                quote!(pub type #alias_name=structural::pmr::FieldPath<(#field_name,)>;)
            });

        let mod_tokens=quote!(
            pub(crate) mod #names_module{
                use structural::pmr::*;

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






