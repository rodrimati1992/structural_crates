use crate::{
    ident_or_index::IdentOrIndex,
    parse_utils::ParseBufferExt,
    tokenizers::{tident_tokens, FullPathForChars},
};

use as_derive_utils::{datastructure::StructKind, return_syn_err};

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree as TokenTree2};

use quote::{quote, quote_spanned};

use syn::{
    parse::{self, Parse, ParseStream},
    spanned::Spanned,
    Ident, Token,
};

pub(crate) fn impl_(parsed: SwitchStrAliases) -> Result<TokenStream2, syn::Error> {
    let variant_fields = parsed.variants.iter().map(|vari| {
        if vari.fields.is_empty() {
            return TokenStream2::new();
        }

        let span = vari.name.span();
        let vari_name = &vari.name;
        let field_name = vari
            .fields
            .iter()
            .map(|fname| tident_tokens(fname, FullPathForChars::Yes));

        quote_spanned! {span=>
            pub type #vari_name=__struct_pmr::FieldPathSet<
                (
                    #( #field_name, )*
                ),
                __struct_pmr::UniquePaths
            >;
            pub const #vari_name:#vari_name=unsafe{
                __struct_pmr::FieldPathSet::NEW.upgrade_unchecked()
            };
        }
    });

    let variant_names = parsed.variants.iter().map(|vari| {
        let span = vari.name.span();
        let alias_name = &vari.name;
        let variant_name = tident_tokens(alias_name.to_string(), FullPathForChars::Yes);

        quote_spanned! {span=>
            pub type #alias_name=#variant_name;
        }
    });

    let variant_count_str = tident_tokens(parsed.variants.len().to_string(), FullPathForChars::Yes);

    Ok(quote! {
        pub type VariantCount=#variant_count_str;

        pub mod f{
            use structural::pmr as __struct_pmr;
            #(#variant_fields)*
        }

        pub mod v{
            use structural::pmr as __struct_pmr;
            #(#variant_names)*
        }
    })
}

pub(crate) struct SwitchStrAliases {
    variants: Vec<SwitchVariant>,
}

struct SwitchVariant {
    name: Ident,
    fields: Vec<String>,
}

impl Parse for SwitchStrAliases {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut variants = Vec::new();
        while !input.is_empty() {
            variants.push(SwitchVariant::parse(input)?);
        }
        Ok(Self { variants })
    }
}

impl Parse for SwitchVariant {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let name = input.parse::<Ident>()?;

        let content;
        let vkind = if input.peek(syn::token::Brace) {
            let _ = syn::braced!(content in input);
            StructKind::Braced
        } else {
            let _ = syn::parenthesized!(content in input);
            StructKind::Tuple
        };

        let fields = Self::parse_fields(&content, vkind)?;

        Ok(Self { name, fields })
    }
}

impl SwitchVariant {
    fn parse_fields(input: ParseStream, vkind: StructKind) -> parse::Result<Vec<String>> {
        let mut fields = Vec::<String>::new();
        let mut index = 0;
        while !input.is_empty() {
            if let Some((field_span, field)) = parse_field(input, index, vkind)? {
                if fields.contains(&field) {
                    return_syn_err!(
                        field_span,
                        "Cannot match on the same field twice in the same pattern",
                    );
                }
                fields.push(field);
            }
            if !input.is_empty() {
                let _: Token!(,) = input.parse()?;
            }
            index += 1;
        }
        Ok(fields)
    }
}

fn parse_field(
    input: ParseStream,
    index: usize,
    vkind: StructKind,
) -> parse::Result<Option<(Span, String)>> {
    match vkind {
        StructKind::Braced => {
            skip_ref(input)?;
            let field_name = input.parse::<IdentOrIndex>()?;
            if input.peek_parse(Token!(:))?.is_some() {
                if input.peek_parse(Token!(_))?.is_some() {
                    return Ok(None);
                } else {
                    skip_rest_of_field(input)?;
                }
            }
            Ok(Some((field_name.span(), field_name.to_string())))
        }
        StructKind::Tuple => {
            if input.peek_parse(Token!(_))?.is_some() {
                Ok(None)
            } else {
                skip_rest_of_field(input)?;
                Ok(Some((Span::call_site(), index.to_string())))
            }
        }
    }
}

fn skip_ref(input: ParseStream) -> parse::Result<()> {
    if let None = input.peek_parse(Token!(&))? {
        return Ok(());
    }
    if let None = input.peek_parse(Token!(mut))? {
        return Ok(());
    }
    if let None = input.peek_parse(Token!(mut))? {
        return Ok(());
    }
    Ok(())
}

fn skip_rest_of_field(input: ParseStream) -> parse::Result<()> {
    while !(input.is_empty() || input.peek(Token!(,))) {
        let _ = input.parse::<TokenTree2>()?;
    }
    Ok(())
}
