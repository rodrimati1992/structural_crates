use crate::{
    arenas::Arenas,
    datastructure::StructOrEnum,
    parse_utils::ParseBufferExt,
    structural_alias_impl_mod::{TinyStructuralField, TypeParamBounds},
};

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens, TokenStreamExt};

use syn::parse::{discouraged::Speculative, Parse, ParseStream};

pub(crate) fn impl_(data: ImplStructHack) -> Result<TokenStream2, syn::Error> {
    Ok(data.tokens)
}

pub(crate) struct ImplStructHack {
    pub(crate) tokens: TokenStream2,
}

pub(crate) struct ImplStruct<'a> {
    bounds: TypeParamBounds,
    fields: Vec<TinyStructuralField<'a>>,
}

impl<'a> Parse for ImplStructHack {
    fn parse(input: ParseStream<'_>) -> Result<Self, syn::Error> {
        let arenas = Arenas::default();

        let impl_struct = ImplStruct::parse(&arenas, input)?;
        Ok(ImplStructHack {
            tokens: impl_struct.into_token_stream(),
        })
    }
}

impl<'a> ImplStruct<'a> {
    fn parse(arenas: &'a Arenas, input: ParseStream<'_>) -> Result<Self, syn::Error> {
        let forked = input.fork();
        let mut bounds = TypeParamBounds::parse_separated_nonempty_with(&forked, Parse::parse).ok();
        let _ = forked.peek_parse(syn::Token!(+));
        let sep = forked.parse::<syn::Token!(;)>();
        if sep.is_ok() {
            input.advance_to(&forked);
        } else {
            bounds = None;
        }

        let mut fields = Vec::new();

        while !input.is_empty() {
            let access = input.parse()?;
            fields.push(TinyStructuralField::parse(access, &arenas, input)?);
            input.peek_parse(syn::Token![,])?;
        }

        let bounds = bounds.unwrap_or_default();
        Ok(ImplStruct { bounds, fields })
    }
}

impl<'a> ToTokens for ImplStruct<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let bounds = self.bounds.iter();
        let fields = self.fields.iter().map(|f| f.tokens(StructOrEnum::Struct));
        tokens.append_all(quote!(
            impl #( #bounds+ )* #( #fields+ )*
        ));
    }
}
