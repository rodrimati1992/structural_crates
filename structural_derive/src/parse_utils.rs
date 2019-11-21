use syn::{
    parse::{self,Parse,ParseBuffer,ParseStream,Peek},
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


pub(crate) trait ParseBufferExt{
    fn peek_parse<F,X,P>(&self,f:F)->Option<Result<P,syn::Error>>
    where
        F:FnOnce(X)->P + Peek,
        P:Parse;
}

impl ParseBufferExt for ParseBuffer<'_>{
    fn peek_parse<F,X,P>(&self,f:F)->Option<Result<P,syn::Error>>
    where
        F:FnOnce(X)->P + Peek,
        P:Parse,
    {
        if self.peek(f) {
            Some(self.parse::<P>())
        }else{
            None
        }
    }
}