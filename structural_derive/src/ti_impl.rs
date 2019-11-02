use crate::{
    parse_utils::ParsePunctuated,
    tokenizers::{tident_tokenizer,FullPathForChars},
    str_or_ident::{IdentOrIndex,StrOrIdent},
};

use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote,ToTokens};

use syn::{
    parse::{self,Parse,ParseStream},
    LitStr,
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
    let strings=strings_.list;
    if strings.len()==1 {
        let string=strings[0].value();
        let tstring=tident_tokenizer(string,FullPathForChars::No);

        quote!(
            use structural::proc_macro_reexports::*;

            pub const VALUE:#tstring=MarkerType::MTVAL;
        )
    }else{
        let mut prev_strings=Vec::<String>::new();
        let mut tstring=Vec::new();
        for string_lit in strings {
            let string=string_lit.value();
            if prev_strings.contains(&string) {
                return Err(syn::Error::new(
                    string_lit.span(),
                    "Field names cannot be used more than once"
                ));
            }else{
                prev_strings.push(string.clone());
                tstring.push(tident_tokenizer(string,FullPathForChars::No));
            }
        }
        quote!(
            use structural::proc_macro_reexports::*;

            pub const VALUE:TStringSet<(#(#tstring),*)>=unsafe{
                TStringSet::new()
            };
        )
    }.piped(Ok)
}

