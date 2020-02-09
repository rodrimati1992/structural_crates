use crate::{
    ident_or_index::IdentOrIndex,
    parse_utils::ParseBufferExt,
    tokenizers::{tident_tokens, variant_field_tokens, variant_name_tokens, FullPathForChars},
};

use as_derive_utils::spanned_err;

use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens, TokenStreamExt};

use syn::{
    parse::{self, Parse, ParseStream},
    Ident, Token,
};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub(crate) struct FieldPaths {
    prefix: Option<FieldPath>,
    paths: Vec<FieldPath>,
    path_uniqueness: PathUniqueness,
}

impl FieldPaths {
    pub(crate) fn from_ident(soi: Ident) -> Self {
        soi.piped(FieldPath::from_ident).piped(Self::from_path)
    }

    pub(crate) fn from_path(path: FieldPath) -> Self {
        Self {
            prefix: None,
            paths: vec![path],
            path_uniqueness: PathUniqueness::Unique,
        }
    }

    pub(crate) fn contains_aliased_paths(paths: &[FieldPath]) -> bool {
        paths
            .iter()
            .enumerate()
            .any(|(i, path)| paths[..i].iter().any(|p| p.is_prefix_of(path)))
    }

    pub(crate) fn from_iter<I>(mut paths: I) -> Self
    where
        I: ExactSizeIterator<Item = FieldPath>,
    {
        match paths.len() {
            1 => paths.next().unwrap().piped(FieldPaths::from_path),
            _ => {
                let paths = paths.collect::<Vec<FieldPath>>();

                let path_uniqueness = if Self::contains_aliased_paths(&paths) {
                    PathUniqueness::Aliased
                } else {
                    PathUniqueness::Unique
                };

                Self {
                    prefix: None,
                    paths,
                    path_uniqueness,
                }
            }
        }
    }

    pub(crate) fn is_set(&self) -> bool {
        self.prefix.is_some() || self.paths.len() != 1
    }

    /// Outputs the inside of `fp!`/`FP!` invocation that constructed this FieldPaths.
    pub(crate) fn write_fp_inside(&self, buff: &mut String) {
        #[cfg(feature = "test_asserts")]
        let start = buff.len();

        if let Some(prefix) = &self.prefix {
            prefix.write_str(buff);
            buff.push_str("=>");
        }
        for (i, path) in self.paths.iter().enumerate() {
            path.write_str(buff);
            if i + 1 != self.paths.len() {
                buff.push_str(", ")
            }
        }

        #[cfg(feature = "test_asserts")]
        {
            match syn::parse_str::<Self>(&buff[start..]) {
                Ok(x) => assert_eq!(*self, x),
                Err(e) => panic!("Could not parse `{}` as {:#?}", e, self),
            }
        }
    }

    /// Gets a the type-level identifier.
    pub(crate) fn type_tokens(&self, char_path: FullPathForChars) -> TokenStream2 {
        if self.is_set() {
            let path = self.paths.iter().map(|x| x.to_token_stream(char_path));
            let uniqueness = self.path_uniqueness;

            if let Some(prefix) = &self.prefix {
                let prefix = prefix.tuple_tokens(char_path);
                quote!(
                    ::structural::pmr::NestedFieldSet<
                        #prefix,
                        (#(#path,)*),
                        #uniqueness
                    >
                )
            } else {
                quote!(
                    ::structural::pmr::FieldPathSet<(#(#path,)*),#uniqueness>
                )
            }
        } else {
            self.paths[0].to_token_stream(char_path)
        }
    }

    /// Gets a const item with the type-level identifier.
    pub(crate) fn constant_named(
        &self,
        name: &syn::Ident,
        char_path: FullPathForChars,
    ) -> TokenStream2 {
        let type_ = self.type_tokens(char_path);
        let mut ret = quote!(pub const #name:#type_=);
        ret.append_all(match (&self.prefix, self.is_set()) {
            (None, false) => quote!(structural::pmr::MarkerType::MTVAL),
            (None, true) => quote!(unsafe { structural::pmr::FieldPathSet::new_unchecked() }),
            (Some(_), _) => quote!(unsafe { structural::pmr::NestedFieldSet::new_unchecked() }),
        });
        <Token!(;)>::default().to_tokens(&mut ret);
        ret
    }

    /// Gets a const item with the type-level identifier.
    pub(crate) fn constant_from_single(
        const_name: &syn::Ident,
        value: &IdentOrIndex,
        char_path: FullPathForChars,
    ) -> TokenStream2 {
        let type_ = tident_tokens(value.to_string(), char_path);
        quote!(
            pub const #const_name: ::structural::pmr::FieldPath1<#type_>=
                structural::pmr::MarkerType::MTVAL;
        )
    }

    /// Gets a tokenizer that outputs a type-level FieldPath(Set) value.
    pub(crate) fn inferred_expression_tokens(&self) -> TokenStream2 {
        if self.is_set() {
            if self.prefix.is_some() {
                quote!(unsafe { structural::pmr::NestedFieldSet::new_unchecked() })
            } else {
                quote!(unsafe { structural::pmr::FieldPathSet::new_unchecked() })
            }
        } else {
            quote!(structural::pmr::MarkerType::MTVAL)
        }
    }
}

impl Parse for FieldPaths {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let forked = input.fork();
        // If this is space separated characters(which start with two idents)
        // then this only parses a sequence of IdentOrIndex.
        if let (Ok { .. }, Ok { .. }) = (
            forked.parse::<IdentOrIndex>(),
            forked.parse::<IdentOrIndex>(),
        ) {
            let mut chars = Vec::<IdentOrIndex>::new();
            while !input.is_empty() {
                chars.push(input.parse::<IdentOrIndex>()?);
            }
            FieldPath::from_chars(chars)
                .piped(FieldPaths::from_path)
                .piped(Ok)
        } else {
            let mut prefix = None::<FieldPath>;
            let mut paths = Vec::<FieldPath>::new();
            while !input.is_empty() {
                let path = input.parse::<FieldPath>()?;
                if input.peek(Token!(=>)) {
                    if prefix.is_some() {
                        return Err(input.error("Cannot use `=>` multiple times."));
                    } else if !paths.is_empty() {
                        return Err(input.error("Cannot use `=>` after multiple field accesses."));
                    }
                    input.parse::<Token!(=>)>()?;
                    prefix = Some(path);
                } else if input.peek(Token!(,)) {
                    paths.push(path);
                    input.parse::<Token!(,)>()?;
                } else if input.is_empty() {
                    paths.push(path);
                } else {
                    return Err(input.error("Expected a `=>`,a `,`, or the end of the input"));
                }
            }

            let mut this = FieldPaths::from_iter(paths.into_iter());
            this.prefix = prefix;
            Ok(this)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub(crate) struct FieldPath {
    list: Vec<FieldPathComponent>,
}

impl Parse for FieldPath {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut list = Vec::<FieldPathComponent>::new();
        let mut next_is_period = false;
        while !is_field_path_terminator(input) {
            let is_period = std::mem::replace(&mut next_is_period, false)
                || input.peek_parse(Token!(.))?.is_some();

            if input.peek(syn::LitFloat) {
                let f = input.parse::<syn::LitFloat>()?;
                let digits = f.base10_digits();
                let make_int = |digits: &str| -> syn::Result<FieldPathComponent> {
                    syn::Index {
                        index: digits.parse().map_err(|e| spanned_err!(f, "{}", e))?,
                        span: f.span(),
                    }
                    .piped(FieldPathComponent::from_index)
                    .piped(Ok)
                };

                let mut iter = digits.split('.');
                if digits.starts_with('.') {
                    let _ = iter.next();
                    list.push(make_int(iter.next().unwrap().trim_start_matches('.'))?);
                } else if digits.ends_with('.') {
                    list.push(make_int(iter.next().unwrap())?);
                    next_is_period = true;
                } else {
                    list.push(make_int(iter.next().unwrap())?);
                    list.push(make_int(iter.next().unwrap())?);
                }
            } else {
                let fpc = FieldPathComponent::parse(input, IsPeriod::new(is_period))?;
                list.push(fpc);
            }
        }

        Ok(FieldPath { list })
    }
}

impl FieldPath {
    pub(crate) fn from_ident(ident: Ident) -> Self {
        Self {
            list: vec![FieldPathComponent::from_ident(ident)],
        }
    }
    pub(crate) fn from_chars(chars: Vec<IdentOrIndex>) -> Self {
        Self {
            list: vec![FieldPathComponent::Chars(chars)],
        }
    }

    pub(crate) fn write_str(&self, buff: &mut String) {
        for fpc in &self.list {
            fpc.write_str(buff);
        }
    }

    pub(crate) fn is_prefix_of(&self, other: &Self) -> bool {
        let len = self.list.len();

        len <= other.list.len() && Iterator::eq(self.list.iter(), &other.list[..len])
    }

    pub(crate) fn to_token_stream(&self, char_path: FullPathForChars) -> TokenStream2 {
        let tuple = self.tuple_tokens(char_path);
        quote!( structural::pmr::FieldPath<#tuple> )
    }

    pub(crate) fn tuple_tokens(&self, char_path: FullPathForChars) -> TokenStream2 {
        let strings = self.list.iter().map(|x| x.single_tokenizer(char_path));
        quote!( (#(#strings,)*) )
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum FieldPathComponent {
    /// A field
    Chars(Vec<IdentOrIndex>),
    /// A field
    Ident(IdentOrIndex),
    VariantField {
        variant: Ident,
        field: IdentOrIndex,
    },
    VariantName {
        variant: Ident,
    },
}

impl FieldPathComponent {
    pub(crate) fn from_ident(ident: Ident) -> Self {
        let x = IdentOrIndex::Ident(ident);
        FieldPathComponent::Ident(x)
    }
    pub(crate) fn from_index(index: syn::Index) -> Self {
        let x = IdentOrIndex::Index(index);
        FieldPathComponent::Ident(x)
    }

    pub(crate) fn write_str(&self, buff: &mut String) {
        use self::FieldPathComponent as FPC;
        use std::fmt::Write;

        match self {
            FPC::Chars(list) => {
                for c in list {
                    let _ = write!(buff, "{} ", c);
                }
            }
            FPC::Ident(ident) => {
                let _ = write!(buff, ".{}", ident);
            }
            FPC::VariantField { variant, field } => {
                let _ = write!(buff, "::{}.{}", variant, field);
            }
            FPC::VariantName { variant } => {
                let _ = write!(buff, "::{}", variant);
            }
        }
    }

    pub(crate) fn parse(input: ParseStream, is_period: IsPeriod) -> parse::Result<Self> {
        if is_period == IsPeriod::No && input.peek_parse(Token!(::))?.is_some() {
            let variant = input.parse::<Ident>()?;

            if input.peek_parse(Token!(.))?.is_some() {
                let field = input.parse::<IdentOrIndex>()?;
                Ok(FieldPathComponent::VariantField { variant, field })
            } else if is_field_path_terminator(input) {
                Ok(FieldPathComponent::VariantName { variant })
            } else {
                Err(input.error("Expected either a `.field_name`,the end of the input,or a `,`."))
            }
        } else {
            let _ = input.peek_parse(Token!(.))?;
            input.parse::<IdentOrIndex>().map(FieldPathComponent::Ident)
        }
    }

    fn single_tokenizer(&self, char_path: FullPathForChars) -> TokenStream2 {
        use self::FieldPathComponent as FPC;

        match self {
            FPC::Chars(chars) => {
                let mut buffer = String::with_capacity(chars.len());
                for char_ in chars {
                    use std::fmt::Write;
                    let _ = write!(buffer, "{}", char_);
                }
                tident_tokens(buffer, char_path)
            }
            FPC::Ident(ident) => tident_tokens(ident.to_string(), char_path),
            FPC::VariantField { variant, field } => {
                variant_field_tokens(variant.to_string(), field.to_string(), char_path)
            }
            FPC::VariantName { variant } => variant_name_tokens(variant.to_string(), char_path),
        }
    }
}

fn is_field_path_terminator(input: ParseStream) -> bool {
    input.is_empty() || input.peek(Token!(,)) || input.peek(Token!(=>))
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum IsPeriod {
    No,
    Yes,
}

impl IsPeriod {
    pub(crate) fn new(v: bool) -> Self {
        if v {
            IsPeriod::Yes
        } else {
            IsPeriod::No
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum PathUniqueness {
    Unique,
    Aliased,
}

impl ToTokens for PathUniqueness {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        match *self {
            PathUniqueness::Unique => quote!(structural::pmr::UniquePaths),
            PathUniqueness::Aliased => quote!(structural::pmr::AliasedPaths),
        }
        .to_tokens(ts);
    }
}

//////////////////////////////////////////////////////////////////////////////
