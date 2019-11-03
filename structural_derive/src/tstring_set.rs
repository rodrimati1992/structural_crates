use crate::{
    parse_utils::ParsePunctuated,
    tokenizers::{tident_tokenizer,FullPathForChars},
    str_or_ident::StrOrIdent,
};

use as_derive_utils::ToTokenFnMut;

use core_extensions::SelfOps;

use quote::{quote,ToTokens};

use syn::parse::{self,Parse,ParseStream};

///////////////////////////////////////////////////////////////////////////////

pub(crate)enum TStringSet{
    Single(String),
    Multiple(MultipleIdents),
}

impl TStringSet{
    pub(crate) fn from_single(soi:StrOrIdent)->Self{
        TStringSet::Single(soi.value())
    }

    pub(crate) fn from_iter<I>(mut strings:I)->Result<Self,syn::Error>
    where
        I:ExactSizeIterator<Item=StrOrIdent>
    {
        if strings.len()==1 {
            strings.next().unwrap()
                .piped(TStringSet::from_single)
        }else{
            let mut prev_strings=Vec::<String>::new();
            let mut list=Vec::new();
            for string_lit in strings {
                let string=string_lit.value();
                if prev_strings.contains(&string) {
                    return Err(syn::Error::new(
                        string_lit.span(),
                        "Field names cannot be used more than once"
                    ));
                }else{
                    prev_strings.push(string.clone());
                    list.push(string);
                }
            }
            MultipleIdents{list}
                .piped(TStringSet::Multiple)
        }.piped(Ok)
    }

    /// Gets a tokenizer that outputs the type-level identifier.
    pub(crate) fn type_tokenizer(&self,char_path:FullPathForChars)->impl ToTokens+'_{
        ToTokenFnMut::new(move|ts|{
            match self {
                TStringSet::Single(s)=>{
                    tident_tokenizer(s,char_path).to_tokens(ts)
                }
                TStringSet::Multiple(many)=>{
                    let tstring=many.list().iter().map(|x| tident_tokenizer(x,char_path) );

                    quote!(
                        ::structural::type_level::TStringSet<(#(#tstring),*)>
                    ).to_tokens(ts);
                }
            }
        })
    }

    /// Gets a tokenizer that outputs a const item with the type-level identifier.
    pub(crate) fn constant_named<'a>(
        &'a self,
        name:&'a syn::Ident,
        char_path:FullPathForChars,
    )->impl ToTokens+'a{
        let type_=self.type_tokenizer(char_path);
        ToTokenFnMut::new(move|ts|{
            match self {
                TStringSet::Single{..}=>quote!(
                    pub const #name:#type_=structural::pmr::MarkerType::MTVAL; 
                ),
                TStringSet::Multiple{..}=>quote!(
                    pub const #name:#type_=unsafe{ structural::pmr::TStringSet::new() }; 
                ),
            }.to_tokens(ts)
        })
    }
}


impl Parse for TStringSet{
    fn parse(input: ParseStream) -> parse::Result<Self>{
        input.parse::<ParsePunctuated<StrOrIdent,syn::Token!(,)>>()?
            .list
            .into_iter()
            .piped(TStringSet::from_iter)
    }
}


///////////////////////////////////////////////////////////////////////////////

pub(crate)struct MultipleIdents{
    list:Vec<String>,
}

impl MultipleIdents{
    pub(crate)fn list(&self)->&[String]{
        &self.list
    }
    #[allow(dead_code)]
    pub(crate)fn into_list(self)->Vec<String>{
        self.list
    }
}

