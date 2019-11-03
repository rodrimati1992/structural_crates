use crate::{
    parse_utils::ParsePunctuated,
    tokenizers::{tident_tokenizer,FullPathForChars},
    str_or_ident::{IdentOrIndex,StrOrIdent},
    tstring_set::TStringSet,
};

use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote,ToTokens};

use syn::{
    parse::{self,Parse,ParseStream},
    Ident,LitStr,
};


#[allow(non_snake_case)]
pub(crate) fn TI_impl(parsed: CapTIInput) -> Result<TokenStream2,syn::Error> {
    let string=parsed.value();

    tident_tokenizer(&string,FullPathForChars::Yes)
        .into_token_stream()
        .piped(Ok)
}

/// The parsed input for the implementation of the TI macro.
pub(crate) enum CapTIInput{
    Str(LitStr),
    Idents(Vec<IdentOrIndex>),
}


impl CapTIInput{
    fn value(&self)->String{
        match self {
            CapTIInput::Str(s)=>{
                s.value()
            }
            CapTIInput::Idents(list)if list.len()==1 =>{
                list[0].to_string()
            }
            CapTIInput::Idents(list)=>{
                let mut buffer=String::new();
                for x in list{
                    use std::fmt::Write;
                    let _=write!(buffer,"{}",x);
                }
                buffer
            }
        }
    }
}


impl Parse for CapTIInput{
    fn parse(input: ParseStream) -> parse::Result<Self>{
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            input.parse().map(CapTIInput::Str)
        } else {
            let mut xs=Vec::<IdentOrIndex>::new();
            while !input.is_empty() {
                xs.push(input.parse()?);
            }
            Ok(CapTIInput::Idents(xs))
        }
    }
}



///////////////////////////////////////////////////////////////////////////////

pub(crate) fn ti_impl(
    strings_: ParsePunctuated<StrOrIdent,syn::Token!(,)>,
) -> Result<TokenStream2,syn::Error> {
    let set=TStringSet::from_iter(strings_.list.into_iter())?;
    let const_name=Ident::new("VALUE",proc_macro2::Span::call_site());
    let constant=set.constant_named(&const_name,FullPathForChars::No);

    Ok(quote!(
        use structural::pmr::*;
        #constant
    ))
}

