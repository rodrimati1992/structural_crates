use crate::{
    tokenizers::FullPathForChars,
    field_paths::{FieldPaths,FieldPath},
};

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use quote::quote;

use syn::{
    parse::{self,Parse,ParseStream},
    punctuated::Punctuated,
    Ident,Token,
};



pub(crate) fn impl_(parsed: NameAliases) -> Result<TokenStream2,syn::Error> {

    // This uses the full path to each character to allow aliases
    // with the same name as the characters themselves.
    let char_verbosity=FullPathForChars::Yes;

    let aliases_names_a=parsed.aliases.iter().map(|x|&x.name);
    let field_name=parsed.aliases.iter()
        .map(|x| x.value.type_tokens(char_verbosity) );
    let constants=parsed.aliases.iter()
        .map(|x| x.value.constant_named(&x.name,char_verbosity) );

    Ok(quote!(
        #(
            #[allow(non_camel_case_types)]
            pub type #aliases_names_a=#field_name;
            #[allow(non_upper_case_globals)]
            #constants
        )*
    ))
}



#[derive(Debug)]
pub(crate) struct NameAliases{
    aliases:Punctuated<NameAlias,Token![,]>,
}

#[derive(Debug)]
pub(crate) struct NameAlias{
    name:Ident,
    value:FieldPaths,
}


impl Parse for NameAliases{
    fn parse(input: ParseStream) -> parse::Result<Self>{
        Ok(NameAliases{
            aliases:input.parse_terminated(Parse::parse)?,
        })
    }
}


impl Parse for NameAlias{
    fn parse(input: ParseStream) -> parse::Result<Self>{
        let name=input.parse::<Ident>()?;
        let value=if input.peek(Token!(=)) {
            input.parse::<Token![=]>()?;
            let content;
            if input.peek(syn::token::Paren) {
                let _=syn::parenthesized!(content in input);
                content.parse::<FieldPaths>()?
            }else {
                input.parse::<FieldPath>()?
                    .piped(FieldPaths::from_path)
            }
        }else{
            FieldPaths::from_ident(name.clone())
        };
        Ok(Self{name,value})
    }
}