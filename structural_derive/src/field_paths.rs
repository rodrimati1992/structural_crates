use crate::{
    ident_or_index::IdentOrIndex,
    ignored_wrapper::Ignored,
    parse_utils::ParseBufferExt,
    tokenizers::{tstr_tokens, variant_field_tokens, variant_name_tokens},
};

// use as_derive_utils::spanned_err;

use core_extensions::SelfOps;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens};

use syn::{
    parse::{self, Parse, ParseStream},
    Ident, Token,
};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub(crate) struct FieldPaths {
    prefix: Option<NestedFieldPath>,
    paths: Vec<NestedFieldPath>,
    path_uniqueness: PathUniqueness,
}

impl FieldPaths {
    pub(crate) fn from_ident(soi: Ident) -> Self {
        soi.piped(NestedFieldPath::from_ident)
            .piped(Self::from_path)
    }

    pub(crate) fn from_path(path: NestedFieldPath) -> Self {
        Self {
            prefix: None,
            paths: vec![path],
            path_uniqueness: PathUniqueness::Unique,
        }
    }

    pub(crate) fn contains_aliased_paths(paths: &[NestedFieldPath]) -> bool {
        paths.iter().enumerate().any(|(i, path)| {
            paths[..i]
                .iter()
                .chain(&paths[i + 1..])
                .any(|p| path.is_prefix_of(p))
        })
    }

    pub(crate) fn from_iter<I>(mut paths: I) -> Self
    where
        I: ExactSizeIterator<Item = NestedFieldPath>,
    {
        match paths.len() {
            1 => paths.next().unwrap().piped(FieldPaths::from_path),
            _ => {
                let paths = paths.collect::<Vec<NestedFieldPath>>();

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
    pub(crate) fn type_tokens(&self) -> TokenStream2 {
        if self.is_set() {
            let path = self.paths.iter().map(|x| x.to_token_stream());
            let uniqueness = self.path_uniqueness;

            if let Some(prefix) = &self.prefix {
                let prefix_tokens = prefix.to_token_stream();
                quote!(
                    ::structural::NestedFieldPathSet<
                        #prefix_tokens,
                        (#(#path,)*),
                        #uniqueness
                    >
                )
            } else {
                quote!(
                    ::structural::FieldPathSet<(#(#path,)*),#uniqueness>
                )
            }
        } else {
            self.paths[0].to_token_stream()
        }
    }

    /// Gets a tokenizer that outputs a type-level NestedFieldPath(Set) value.
    pub(crate) fn inferred_expression_tokens(&self) -> TokenStream2 {
        if self.is_set() {
            if self.prefix.is_some() {
                quote!(unsafe { structural::NestedFieldPathSet::NEW.set_uniqueness() })
            } else {
                quote!(unsafe { structural::FieldPathSet::NEW.set_uniqueness() })
            }
        } else {
            quote!(structural::pmr::ConstDefault::DEFAULT)
        }
    }
}

impl Parse for FieldPaths {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        let mut prefix = None::<NestedFieldPath>;
        let mut paths = Vec::<NestedFieldPath>::new();
        while !input.is_empty() {
            let path = input.parse::<NestedFieldPath>()?;
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

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub(crate) struct NestedFieldPath {
    list: Vec<FieldPathComponent>,
    normalized: Vec<String>,
}

impl Parse for NestedFieldPath {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        let mut list = Vec::<FieldPathComponent>::new();

        let mut is_first = true;
        while !is_field_path_terminator(input) {
            let (fpc, second) = FieldPathComponent::parse(input, IsFirst::new(is_first))?;
            list.push(fpc);
            if let Some(second) = second {
                list.push(second);
            }
            is_first = false;
        }

        Ok(Self::from_components(list))
    }
}

impl NestedFieldPath {
    pub(crate) fn from_components(list: Vec<FieldPathComponent>) -> Self {
        let mut normalized = Vec::new();

        for component in &list {
            component.write_normalized(&mut normalized);
        }

        NestedFieldPath { list, normalized }
    }
    pub(crate) fn from_ident(ident: Ident) -> Self {
        Self::from_components(vec![FieldPathComponent::from_ident(ident)])
    }
    pub(crate) fn write_str(&self, buff: &mut String) {
        for fpc in &self.list {
            fpc.write_str(buff);
        }
    }

    pub(crate) fn is_prefix_of(&self, other: &Self) -> bool {
        let min_len = self.normalized.len().min(other.normalized.len());

        self.normalized
            .iter()
            .take(min_len)
            .eq(other.normalized.iter().take(min_len))
    }

    pub(crate) fn to_token_stream(&self) -> TokenStream2 {
        if self.list.len() == 1 {
            let path_component = self.list[0].single_tokenizer();
            path_component.into_token_stream()
        } else {
            let tuple = self.tuple_tokens();
            quote!( structural::NestedFieldPath<#tuple> )
        }
    }

    pub(crate) fn tuple_tokens(&self) -> TokenStream2 {
        let strings = self.list.iter().map(|x| x.single_tokenizer());
        quote!( (#(#strings,)*) )
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum FieldPathComponent {
    /// A field
    Ident(IdentOrIndex),
    VariantField {
        variant: IdentOrIndex,
        field: IdentOrIndex,
    },
    VariantName {
        variant: IdentOrIndex,
    },
}

impl FieldPathComponent {
    pub(crate) fn from_ident(ident: Ident) -> Self {
        let x = IdentOrIndex::Ident(ident);
        FieldPathComponent::Ident(x)
    }
    fn write_normalized(&self, normalized: &mut Vec<String>) {
        use self::FieldPathComponent as FPC;
        match self {
            FPC::Ident(ident) => {
                normalized.push(ident.to_string());
            }
            FPC::VariantField { variant, field } => {
                normalized.push(variant.to_string());
                normalized.push(field.to_string());
            }
            FPC::VariantName { variant } => {
                normalized.push(variant.to_string());
            }
        }
    }
    pub(crate) fn write_str(&self, buff: &mut String) {
        use self::FieldPathComponent as FPC;
        use std::fmt::Write;

        match self {
            FPC::Ident(ident) => {
                let _ = write!(buff, ".{}", ident.to_token_stream());
            }
            FPC::VariantField { variant, field } => {
                let _ = write!(
                    buff,
                    "::{}.{}",
                    variant.to_token_stream(),
                    field.to_token_stream()
                );
            }
            FPC::VariantName { variant } => {
                let _ = write!(buff, "::{}", variant.to_token_stream());
            }
        }
    }

    pub(crate) fn parse(
        input: ParseStream<'_>,
        is_first: IsFirst,
    ) -> parse::Result<(Self, Option<Self>)> {
        let fork = input.fork();

        let prefix_token = if input.peek_parse(Token!(::))?.is_some() {
            PrefixToken::Colon2
        } else if input.peek_parse(Token!(.))?.is_some() {
            PrefixToken::Dot
        } else if input.peek(Token!(?)) {
            PrefixToken::Question
        } else {
            PrefixToken::Nothing
        };

        if let PrefixToken::Question = prefix_token {
            let question = input.parse::<Token!(?)>()?;
            let span = Ignored::new(question.spans[0]);
            Ok((
                FieldPathComponent::VariantField {
                    variant: IdentOrIndex::Str {
                        str: "Some".to_string(),
                        span,
                    },
                    field: IdentOrIndex::Str {
                        str: "0".to_string(),
                        span,
                    },
                },
                None,
            ))
        } else if let PrefixToken::Colon2 = prefix_token {
            let (first, second) = parse_field(input)?;
            let variant = first;

            if let Some(field) = second {
                Ok((FieldPathComponent::VariantField { variant, field }, None))
            } else if input.peek_parse(Token!(.))?.is_some() {
                let (field, extra) = parse_field(input)?;
                Ok((
                    FieldPathComponent::VariantField { variant, field },
                    extra.map(FieldPathComponent::Ident),
                ))
            } else if is_field_path_terminator(input) {
                Ok((FieldPathComponent::VariantName { variant }, None))
            } else {
                Err(input.error("Expected either a `.field_name`,the end of the field path."))
            }
        } else {
            let (first, second) = parse_field(input)?;
            if let (PrefixToken::Nothing, IsFirst::No) = (prefix_token, is_first) {
                return Err(fork.error("expected a period"));
            }
            Ok((
                FieldPathComponent::Ident(first),
                second.map(FieldPathComponent::Ident),
            ))
        }
    }

    fn single_tokenizer(&self) -> TokenStream2 {
        use self::FieldPathComponent as FPC;

        match self {
            FPC::Ident(ident) => tstr_tokens(ident.to_string()),
            FPC::VariantField { variant, field } => {
                variant_field_tokens(variant.to_string(), field.to_string())
            }
            FPC::VariantName { variant } => variant_name_tokens(variant.to_string()),
        }
    }
}

fn is_field_path_terminator(input: ParseStream<'_>) -> bool {
    input.is_empty() || input.peek(Token!(,)) || input.peek(Token!(=>))
}

fn make_ident_or_index(digits: &str) -> syn::Result<Option<IdentOrIndex>> {
    if digits.is_empty() {
        return Ok(None);
    }
    syn::parse_str::<IdentOrIndex>(digits).map(Some)
}

/// For parsing path components.
///
/// This function returns a second `IdentOrIndex` if the first token is
/// a floating point number.
pub(crate) fn parse_field(
    input: ParseStream<'_>,
) -> parse::Result<(IdentOrIndex, Option<IdentOrIndex>)> {
    if input.peek(syn::LitFloat) {
        let f = input.parse::<syn::LitFloat>()?;
        let digits = f.base10_digits();
        let mut iter = digits.split('.');

        let first = make_ident_or_index(iter.next().unwrap())?
            .expect("float literals can't have a leading `.`");

        // Handling non-integer fields ie:`0."hello"` and `0.world` here,
        // so that I don't have to store whether a float was parsed.
        let second = match make_ident_or_index(iter.next().unwrap())? {
            Some(x) => Some(x),
            // Parsing the IdentOrIndex after the `###.`  flota literal,
            None => Some(IdentOrIndex::parse(input)?),
        };
        Ok((first, second))
    } else {
        Ok((IdentOrIndex::parse(input)?, None))
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum IsFirst {
    No,
    Yes,
}

impl IsFirst {
    pub(crate) fn new(v: bool) -> Self {
        if v {
            IsFirst::Yes
        } else {
            IsFirst::No
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

#[derive(Copy, Clone)]
enum PrefixToken {
    Colon2,
    Dot,
    Question,
    Nothing,
}
