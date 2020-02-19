use syn::{
    parse::{self, Parse, ParseBuffer, ParseStream, Peek},
    punctuated::Punctuated,
    GenericArgument, PathArguments, Type, TypePath,
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

pub(crate) fn extract_option_parameter(ty: &syn::Type) -> Option<&Type> {
    let path = match ty {
        Type::Path(TypePath {
            qself: _,
            path: syn::Path { segments, .. },
        }) if !segments.is_empty() => segments,
        _ => return None,
    };

    let p0 = &path[0];

    let arguments = if p0.ident == "Option" {
        &p0.arguments
    } else if (p0.ident == "core" || p0.ident == "std")
        && path.len() == 3
        && path[1].ident == "option"
        && path[2].ident == "Option"
    {
        &path[2].arguments
    } else {
        return None;
    };

    let arguments = match arguments {
        PathArguments::AngleBracketed(arguments) => arguments,
        _ => return None,
    };

    match arguments.args.iter().next() {
        Some(GenericArgument::Type(ty)) => Some(ty),
        _ => None,
    }
}
