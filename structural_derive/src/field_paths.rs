use crate::{
    ident_or_index::IdentOrIndex,
    parse_utils::ParseBufferExt,
    tokenizers::{tident_tokens, FullPathForChars},
};

use core_extensions::{matches, SelfOps, ValSliceExt};

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens};

use syn::{
    parse::{self, Parse, ParseStream},
    Ident, Token,
};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub(crate) struct FieldPaths {
    paths: Vec<FieldPath>,
    path_uniqueness: PathUniqueness,
}

impl FieldPaths {
    pub(crate) fn from_ident(soi: Ident) -> Self {
        soi.piped(FieldPath::from_ident).piped(Self::from_path)
    }

    pub(crate) fn from_path(path: FieldPath) -> Self {
        Self {
            paths: vec![path],
            path_uniqueness: PathUniqueness::Unique,
        }
    }

    pub(crate) fn contains_aliased_paths(paths: &[FieldPath]) -> bool {
        paths.iter().any(FieldPath::contains_interpolated)
            || paths
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
                    paths,
                    path_uniqueness,
                }
            }
        }
    }

    pub(crate) fn is_set(&self) -> bool {
        self.paths.len() != 1
    }

    /// Outputs the inside of `fp!`/`FP!` invocation that constructed this FieldPaths.
    pub(crate) fn write_fp_inside(&self, buff: &mut String) {
        #[cfg(feature = "test_asserts")]
        let start = buff.len();

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

    /// Gets a tokenizer that outputs the type-level identifier.
    pub(crate) fn type_tokens(&self, char_path: FullPathForChars) -> TokenStream2 {
        if self.is_set() {
            let path = self.paths.iter().map(|x| x.to_token_stream(char_path));
            let uniqueness = self.path_uniqueness;

            quote!(
                ::structural::pmr::FieldPathSet<(#(#path,)*),#uniqueness>
            )
        } else {
            self.paths[0].to_token_stream(char_path)
        }
    }

    /// Gets a tokenizer that outputs a const item with the type-level identifier.
    pub(crate) fn constant_named(
        &self,
        name: &syn::Ident,
        char_path: FullPathForChars,
    ) -> TokenStream2 {
        let type_ = self.type_tokens(char_path);
        if self.is_set() {
            quote!(
                pub const #name:#type_=unsafe{
                    structural::pmr::FieldPathSet::new_unchecked()
                };
            )
        } else {
            quote!(
                pub const #name:#type_=structural::pmr::MarkerType::MTVAL;
            )
        }
    }

    /// Gets a tokenizer that outputs a type-level FieldPath(Set) value.
    pub(crate) fn inferred_expression_tokens(&self) -> TokenStream2 {
        if self.is_set() {
            quote!(unsafe { structural::pmr::FieldPathSet::new_unchecked() })
        } else {
            quote!(structural::pmr::MarkerType::MTVAL)
        }
    }
}

impl Parse for FieldPaths {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let forked = input.fork();
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
            input
                .parse_terminated::<_, Token!(,)>(FieldPath::parse)
                .map(|x| FieldPaths::from_iter(x.into_iter()))
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub(crate) struct FieldPath {
    list: Vec<FieldPathComponent>,
    contains_splice: bool,
}

impl Parse for FieldPath {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut list = Vec::<FieldPathComponent>::new();
        let mut contains_splice = false;
        let mut is_first = IsFirst::Yes;
        let mut next_is_period = false;
        while !input.peek(Token![,]) && !input.is_empty() {
            let is_period = std::mem::replace(&mut next_is_period, false)
                || input.peek_parse(Token!(.)).is_some();

            if input.peek(syn::LitFloat) {
                let f = input.parse::<syn::LitFloat>()?;
                let digits = f.base10_digits();
                let make_int = |digits: &str| {
                    syn::LitInt::new(digits, f.span()).piped(FieldPathComponent::from_index)
                };

                let mut iter = digits.split('.');
                if digits.starts_with('.') {
                    let _ = iter.next();
                    list.push(make_int(iter.next().unwrap().trim_start_matches('.')));
                } else if digits.ends_with('.') {
                    list.push(make_int(iter.next().unwrap()));
                    next_is_period = true;
                } else {
                    list.push(make_int(iter.next().unwrap()));
                    list.push(make_int(iter.next().unwrap()));
                }
            } else {
                let fpc = FieldPathComponent::parse(input, is_first, is_period)?;
                contains_splice = contains_splice || matches!(FieldPathComponent::Splice{..}=fpc);
                list.push(fpc);
            }
            is_first = IsFirst::No;
        }

        Ok(FieldPath {
            list,
            contains_splice,
        })
    }
}

impl FieldPath {
    pub(crate) fn from_ident(ident: Ident) -> Self {
        Self {
            list: vec![FieldPathComponent::from_ident(ident)],
            contains_splice: false,
        }
    }
    pub(crate) fn from_chars(chars: Vec<IdentOrIndex>) -> Self {
        Self {
            list: vec![FieldPathComponent::Chars(chars)],
            contains_splice: false,
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

    pub(crate) fn contains_interpolated(&self) -> bool {
        self.contains_splice || self.list.iter().any(FieldPathComponent::is_interpolated)
    }

    pub(crate) fn to_token_stream(&self, char_path: FullPathForChars) -> TokenStream2 {
        if self.contains_splice {
            let flattened_lists = self
                .list
                .split_while(FieldPathComponent::is_splice)
                .map(|ks| {
                    let s = ks.slice;
                    if ks.key {
                        let tys = s.iter().filter_map(|x| x.as_splice());
                        quote!( #( structural::pmr::ToTList<#tys>, )* )
                    } else {
                        let tys = s.iter().map(|x| x.single_tokenizer(char_path));
                        quote!(structural::TList![ #( #tys, )* ],)
                    }
                });
            quote!(
                structural::pmr::FlattenedFieldPath<(#(#flattened_lists)*)>
            )
        } else {
            let strings = self.list.iter().map(|x| x.single_tokenizer(char_path));
            quote!( structural::pmr::FieldPath<(#(#strings,)*)> )
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum FieldPathComponent {
    Chars(Vec<IdentOrIndex>),
    Ident(IdentOrIndex),
    /// This is for using a `TString<_>` type in that position,
    /// as well as `FieldPath<(TString<_>,)>`,
    ///
    /// Examples(assuming that Foo is a `TString<_>`):
    /// With `type Foo=FP!(aaa);` and `type Bar=TP!(bbb);`
    ///
    /// - `FP!( [Foo] )` is equivalent to `FP!(aaa)`.
    ///
    /// - `FP!( [Foo][Bar] )` is equivalent to `FP!(aaa.bbb)`.
    ///
    /// - `FP!( [Foo].bar )` is equivalent to `FP!(aaa.bar)`.
    ///
    /// - `FP!( [Foo].bar.baz )` is equivalent to `FP!(aaa.bar.baz)`.
    ///
    /// - `FP!( foo[Bar].baz )` is equivalent to `FP!(foo.bbb.baz)`.
    ///
    #[allow(dead_code)]
    Insert(syn::Type),
    /// This is for splicing a `FieldPath<_>` type into that position.
    /// Examples:
    /// With `type Foo=TP!(a.b.c);` and `type Bar=TP!(d.e.f);`
    /// - `FP!( (Foo) )` is equivalent to just `Foo`.
    /// - `FP!( (Foo).(Bar) )` is equivalent to `TP!(a.b.c.d.e.f)`.
    /// - `FP!( (Foo).bar )` is equivalent to `TP!(a.b.c.bar)`.
    /// - `FP!( (Foo).bar.baz )` is equivalent to `TP!(a.b.c.bar.baz)`.
    /// - `FP!( foo.(Bar).baz )` is equivalent to `TP!(foo.d.e.f.baz)`.
    #[allow(dead_code)]
    Splice(syn::Type),
}

impl FieldPathComponent {
    pub(crate) fn from_ident(ident: Ident) -> Self {
        let x = IdentOrIndex::Ident(ident);
        FieldPathComponent::Ident(x)
    }
    pub(crate) fn from_index(index: syn::LitInt) -> Self {
        let x = IdentOrIndex::Index(index);
        FieldPathComponent::Ident(x)
    }

    #[allow(dead_code)]
    fn parse_path_or_empty(input: ParseStream) -> parse::Result<syn::Type> {
        if input.is_empty() {
            quote!(structural::pmr::FieldPath<()>)
                .piped(syn::Type::Verbatim)
                .piped(Ok)
        } else {
            input.parse::<syn::Type>()
        }
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
            FPC::Insert(ty) => {
                let _ = write!(buff, "[{}]", ty.to_token_stream());
            }
            FPC::Splice(ty) => {
                let _ = write!(buff, ".({})", ty.to_token_stream());
            }
        }
    }

    pub(crate) fn parse(
        input: ParseStream,
        _is_first: IsFirst,
        _is_period: bool,
    ) -> parse::Result<Self> {
        // if input.peek(syn::token::Bracket) {
        //     let content;
        //     let _=syn::bracketed!(content in input);
        //     content.parse::<syn::Type>()
        //         .map(FieldPathComponent::Insert)
        // }else{
        //     if is_first==IsFirst::No && !is_period {
        //         return Err(input.error("expected a '.'"));
        //     }
        //     if input.peek(syn::token::Paren) {
        //         let content;
        //         let _=syn::parenthesized!(content in input);
        //         content.piped_ref(Self::parse_path_or_empty)
        //             .map(FieldPathComponent::Splice)
        //     }else{
        //         input.parse::<IdentOrIndex>()
        //             .map(FieldPathComponent::Ident)
        //     }
        // }
        input.parse::<IdentOrIndex>().map(FieldPathComponent::Ident)
    }

    pub(crate) fn is_splice(&self) -> bool {
        core_extensions::matches!(FieldPathComponent::Splice{..}=self)
    }
    pub(crate) fn as_splice(&self) -> Option<&syn::Type> {
        match self {
            FieldPathComponent::Splice(ty) => Some(ty),
            _ => None,
        }
    }

    pub(crate) fn is_interpolated(&self) -> bool {
        use self::FieldPathComponent as FPC;

        // Using a match block to ensure that
        // adding variants requires explicit handling in this functino.
        match self {
            FPC::Chars { .. } | FPC::Ident { .. } => false,
            FPC::Insert { .. } | FPC::Splice { .. } => true,
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
            FPC::Insert(ty) => quote!( structural::pmr::ToTString<#ty> ),
            FPC::Splice { .. } => panic!("FieldPathComponent::Splice can't be tokenized"),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum IsFirst {
    Yes,
    No,
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
