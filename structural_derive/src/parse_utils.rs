use syn::{
    parse,
    punctuated::Punctuated,
};



pub struct ParsePunctuated<T,P>{
    pub list:Punctuated<T,P>,
}

impl<T,P> parse::Parse for ParsePunctuated<T,P>
where
    T:parse::Parse,
    P:parse::Parse,
{
    fn parse(input: parse::ParseStream) -> parse::Result<Self>{
        Ok(Self{
            list: Punctuated::parse_terminated(input)?
        })
    }
}


////////////////////////////////////////////////////////////////////////////////

