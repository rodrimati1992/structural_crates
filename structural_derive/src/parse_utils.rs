use syn::{
    parse::{self, Parse, ParseBuffer, ParseStream, Peek},
    punctuated::Punctuated,
};

pub struct ParsePunctuated<T, P> {
    pub list: Punctuated<T, P>,
}

impl<T, P> parse::Parse for ParsePunctuated<T, P>
where
    T: parse::Parse,
    P: parse::Parse,
{
    fn parse(input: ParseStream) -> parse::Result<Self> {
        Ok(Self {
            list: Punctuated::parse_terminated(input)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct ParseVec<T> {
    pub list: Vec<T>,
}

impl<T> parse::Parse for ParseVec<T>
where
    T: parse::Parse,
{
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut list = Vec::new();
        while !input.is_empty() {
            list.push(input.parse::<T>()?);
        }
        Ok(Self { list })
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) trait ParseBufferExt {
    fn peek_parse<F, X, P>(&self, f: F) -> Result<Option<P>, syn::Error>
    where
        F: FnOnce(X) -> P + Peek,
        P: Parse;
}

impl ParseBufferExt for ParseBuffer<'_> {
    fn peek_parse<F, X, P>(&self, f: F) -> Result<Option<P>, syn::Error>
    where
        F: FnOnce(X) -> P + Peek,
        P: Parse,
    {
        if self.peek(f) {
            self.parse::<P>().map(Some)
        } else {
            Ok(None)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
