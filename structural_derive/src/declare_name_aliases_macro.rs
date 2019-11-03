use crate::{
    str_or_ident::StrOrIdent,
    tokenizers::FullPathForChars,
    tstring_set::TStringSet,
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
        .map(|x| x.value.type_tokenizer(char_verbosity) );
    let constants=parsed.aliases.iter()
        .map(|x| x.value.constant_named(&x.name,char_verbosity) );

    Ok(quote!(
        #(
            pub type #aliases_names_a=#field_name;
            #constants
        )*
    ))
}




pub(crate) struct NameAliases{
    aliases:Punctuated<NameAlias,Token![,]>,
}

pub(crate) struct NameAlias{
    name:Ident,
    value:TStringSet,
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
                content.parse::<TStringSet>()?
            }else {
                let soi=input.parse::<StrOrIdent>()?;
                TStringSet::from_single(soi)
            }
        }else{
            TStringSet::Single(name.to_string())
        };
        Ok(Self{name,value})
    }
}