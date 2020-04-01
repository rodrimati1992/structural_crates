use crate::{
    datastructure::StructOrEnum,
    field_access::{Access, ComputeTrait},
    ident_or_index::IdentOrIndexRef,
    parse_utils::ParsePunctuated,
    tokenizers::tstr_tokens,
    utils::SpanExt,
    write_docs::DocsFor,
};

use as_derive_utils::{
    gen_params_in::{GenParamsIn, InWhat},
    return_spanned_err,
};

#[allow(unused_imports)]
use core_extensions::{matches, SelfOps};

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, Ident, TraitItem,
};

use std::fmt::{self, Display};

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests;

mod attribute_parsing;
mod parsing;

use self::attribute_parsing::StructuralAliasOptions;

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct StructuralAliasesHack {
    pub(crate) tokens: TokenStream2,
}

pub(crate) struct StructuralAliases<'a> {
    pub(crate) list: Vec<StructuralAlias<'a>>,
}

pub(crate) struct StructuralAlias<'a> {
    pub(crate) span: Span,
    pub(crate) ident: &'a syn::Ident,
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) vis: syn::Visibility,
    pub(crate) generics: syn::Generics,
    pub(crate) supertraits: Punctuated<syn::TypeParamBound, token::Add>,
    pub(crate) extra_items: Vec<TraitItem>,
    pub(crate) datatype: StructuralDataType<'a>,
    pub(crate) options: StructuralAliasOptions<'a>,
}

#[derive(Debug)]
pub struct StructuralDataType<'a> {
    /// The name of the type this was created from
    pub(crate) type_name: Option<&'a syn::Ident>,
    pub(crate) variants: Vec<StructuralVariant<'a>>,
    pub(crate) fields: Vec<StructuralField<'a>>,
}

#[derive(Debug, Clone)]
pub struct StructuralVariant<'a> {
    pub(crate) name: VariantIdent<'a>,
    /// The name of the original variant.
    /// This is Some if all these conditions are true:
    /// - this was derived from a type definition.
    /// - the enum is public
    /// - the variant has a `#[struc(rename="")]` attribute
    pub(crate) pub_vari_rename: Option<IdentOrIndexRef<'a>>,
    pub(crate) fields: Vec<StructuralField<'a>>,
    pub(crate) is_newtype: bool,
    pub(crate) replace_bounds: Option<&'a ReplaceBounds>,
}

/// A smaller version of StructuralField
#[derive(Debug, Copy, Clone)]
pub(crate) struct TinyStructuralField<'a> {
    pub(crate) access: Access,
    pub(crate) ident: IdentOrIndexRef<'a>,
    pub(crate) ty: FieldType<'a>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct StructuralField<'a> {
    pub(crate) access: Access,
    pub(crate) ident: IdentOrIndexRef<'a>,
    /// The name of the original field.
    /// This is Some if all these conditions are true:
    /// - this was derived from a type definition.
    /// - the field is public
    /// - the field has a `#[struc(rename="")]` attribute
    pub(crate) pub_field_rename: Option<IdentOrIndexRef<'a>>,
    pub(crate) ty: FieldType<'a>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum FieldType<'a> {
    Ty(&'a syn::Type),
    Impl(&'a TypeParamBounds),
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub(crate) enum Exhaustiveness<'a> {
    Nonexhaustive,
    Exhaustive,
    AndExhaustive { name: &'a Ident },
}

pub(crate) type TypeParamBounds = Punctuated<syn::TypeParamBound, syn::token::Add>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum VariantIdent<'a> {
    Ident(IdentOrIndexRef<'a>),
    /// The variant name is determined by a generic parameter
    Generic(&'a Ident),
}

// A hack to allow borrowing from the arena inside a parser
impl<'a> Parse for StructuralAliasesHack {
    fn parse(input: ParseStream<'_>) -> Result<Self, syn::Error> {
        let arenas = crate::arenas::Arenas::default();

        let sa = StructuralAliases::parse(&arenas, input)?;

        Ok(StructuralAliasesHack {
            tokens: self::macro_impl(sa)?,
        })
    }
}

macro_rules! declare_structural_field_methods {
    () => {
        pub(crate) fn compute_trait(&self, soe: StructOrEnum) -> ComputeTrait {
            self.access.compute_trait(soe)
        }
    };
}
impl<'a> TinyStructuralField<'a> {
    declare_structural_field_methods! {}
}

impl<'a> StructuralField<'a> {
    declare_structural_field_methods! {}
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> Display for VariantIdent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariantIdent::Ident(ident) => Display::fmt(&ident.tstr_tokens(), f),
            VariantIdent::Generic(ident) => Display::fmt(ident, f),
        }
    }
}

impl<'a> ToTokens for VariantIdent<'a> {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        match self {
            VariantIdent::Ident(ident) => ts.append_all(ident.tstr_tokens()),
            VariantIdent::Generic(ident) => ident.to_tokens(ts),
        }
    }
}

impl<'a> VariantIdent<'a> {
    pub(crate) fn span(&self) -> Span {
        match *self {
            VariantIdent::Ident(ident) => ident.span(),
            VariantIdent::Generic(ident) => ident.span(),
        }
    }

    pub(crate) fn tokens(&self) -> TokenStream2 {
        match self {
            VariantIdent::Ident(ident) => ident.tstr_tokens(),
            VariantIdent::Generic(ident) => ident.to_token_stream(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl From<Option<VariantIdent<'_>>> for StructOrEnum {
    fn from(opt: Option<VariantIdent<'_>>) -> Self {
        match opt {
            Some(_) => StructOrEnum::Enum,
            None => StructOrEnum::Struct,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub(crate) struct ReplaceBounds {
    pub(crate) bounds: String,
    pub(crate) span: Span,
}

impl ReplaceBounds {
    pub(crate) const NEEDLE: &'static str = "@variant";

    pub(crate) fn to_tokens(
        &self,
        field_bounds: &mut TokenStream2,
        variant_name: VariantIdent<'_>,
    ) -> Result<(), syn::Error> {
        let bounds_str = syn::LitStr::new(
            &self.bounds.replace(Self::NEEDLE, &variant_name.to_string()),
            Span::call_site(),
            // For some reason the docs show the contents of `self.bounds`
            // as the value of the replacement TStr parameter (when using const generics)
            // if I use `self.span` instead of the call site span.
            // self.span,
        );

        let bounds: TypeParamBounds = bounds_str.parse::<ParsePunctuated<_, _>>()?.list;

        let plus = <syn::Token!(+)>::default();
        for bound in bounds {
            bound.to_tokens(field_bounds);
            plus.to_tokens(field_bounds);
        }
        Ok(())
    }

    pub(crate) fn get_docs(&self, variant_name: VariantIdent<'_>) -> String {
        let replacement = match variant_name {
            VariantIdent::Ident(ident) => format!("TS!({})", ident),
            VariantIdent::Generic(ident) => ident.to_string(),
        };
        self.bounds.replace(Self::NEEDLE, &replacement)
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> FieldType<'a> {
    pub(crate) fn span(&self) -> Span {
        match self {
            FieldType::Ty(x) => syn::spanned::Spanned::span(x),
            FieldType::Impl(x) => syn::spanned::Spanned::span(x),
        }
    }
}

impl<'a> ToTokens for FieldType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            FieldType::Ty(x) => x.to_tokens(tokens),
            FieldType::Impl(x) => {
                <syn::Token!(impl)>::default().to_tokens(tokens);
                x.to_tokens(tokens);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> TinyStructuralField<'a> {
    pub(crate) fn tokens(&self, soe: StructOrEnum) -> TokenStream2 {
        let TinyStructuralField { ident, ty, .. } = *self;

        let the_trait = self.compute_trait(soe).trait_tokens();
        let ident = ident.tstr_tokens();

        quote!(
            structural::pmr::#the_trait<
                #ident,
                Ty=#ty,
            >
        )
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub(crate) fn derive_from_str(saf: &str) -> Result<TokenStream2, syn::Error> {
    syn::parse_str::<StructuralAliasesHack>(saf).map(|x| x.tokens)
}

pub(crate) fn macro_impl(aliases: StructuralAliases<'_>) -> Result<TokenStream2, syn::Error> {
    let list = aliases.list;

    if list.is_empty() {
        return Ok(quote!());
    }

    let mut out = TokenStream2::new();
    let extra_where_preds = Punctuated::default();
    for saf in list {
        let trait_docs =
            if saf.options.generate_docs && matches!(syn::Visibility::Public{..} = saf.vis) {
                Some(
                    ("A trait which aliases `structural` accessor traits,\
                     generated by the `structural_alias` macro.\n\n")
                        .to_string(),
                )
            } else {
                None
            };
        let tokens = StructuralAliasParams {
            span: saf.span,
            attrs: &saf.attrs,
            docs: trait_docs.clone(),
            vis: &saf.vis,
            ident: &saf.ident,
            generics: &saf.generics,
            extra_where_preds: &extra_where_preds,
            supertraits: &saf.supertraits,
            trait_items: &saf.extra_items,
            variant_trait: None,
            enum_exhaustiveness: saf.options.enum_exhaustiveness,
            datatype: &saf.datatype,
        }
        .tokens()?;

        if saf.options.debug_print {
            println!("\n\n\n{}\n\n\n", tokens);
        }

        out.append_all(tokens);
    }
    // panic!("{}", out);
    Ok(out)
}

fn process_field(
    field: &StructuralField<'_>,
    variant_ident: Option<VariantIdent<'_>>,
    field_bounds: &mut TokenStream2,
) {
    use self::FieldType as FT;

    let soe = StructOrEnum::from(variant_ident);

    let span = field.ident.span().combine_span(field.ty.span());

    let field_name = field.ident.tstr_tokens();
    let aaoo = field.compute_trait(soe);
    let assoc_ty = match &field.ty {
        FT::Ty(ty) => quote!(Ty=#ty),
        FT::Impl(bounds) => quote!(Ty:#bounds),
    };

    match variant_ident {
        Some(variant_ident) => {
            let variant_name_param = variant_ident.tokens();

            let vf_trait = ComputeTrait {
                struct_or_enum: StructOrEnum::Enum,
                ..aaoo
            };

            field_bounds.append_all(quote_spanned!(span=>
                structural::pmr::#vf_trait<
                    #variant_name_param,
                    #field_name,
                    #assoc_ty
                >+
            ));
        }
        None => {
            let trait_ = aaoo.trait_tokens();
            field_bounds.append_all(quote_spanned!(span=>
                structural::#trait_<
                    #field_name,
                    #assoc_ty
                >+
            ));
        }
    }
}

pub(crate) struct StructuralAliasParams<'a, A, I> {
    pub(crate) span: Span,
    pub(crate) attrs: A,
    pub(crate) docs: Option<String>,
    pub(crate) vis: &'a syn::Visibility,
    pub(crate) ident: &'a Ident,
    pub(crate) generics: &'a syn::Generics,
    pub(crate) extra_where_preds: &'a Punctuated<syn::WherePredicate, syn::Token!(,)>,
    pub(crate) supertraits: &'a Punctuated<syn::TypeParamBound, token::Add>,
    pub(crate) trait_items: I,
    pub(crate) variant_trait: Option<&'a Ident>,
    pub(crate) enum_exhaustiveness: Exhaustiveness<'a>,
    pub(crate) datatype: &'a StructuralDataType<'a>,
}

impl<'a, A, I> StructuralAliasParams<'a, A, I> {
    pub(crate) fn tokens(self) -> Result<TokenStream2, syn::Error>
    where
        A: IntoIterator + Copy,
        A::Item: ToTokens,
        I: IntoIterator<Item = &'a TraitItem> + Copy,
    {
        for_delegation(self)
    }
}

/// This allows both `structural_alias` and `#[derive(Structural)]` to generate
/// the trait alias and its impl.
//
// At some point it would be a good idea to split this into multiple functions
#[allow(clippy::cognitive_complexity)]
pub(crate) fn for_delegation<'a, A, I>(
    StructuralAliasParams {
        span,
        attrs,
        mut docs,
        vis,
        mut ident,
        generics,
        extra_where_preds,
        supertraits,
        trait_items,
        variant_trait,
        enum_exhaustiveness,
        datatype,
    }: StructuralAliasParams<'a, A, I>,
) -> Result<TokenStream2, syn::Error>
where
    A: IntoIterator + Copy,
    A::Item: ToTokens,
    I: IntoIterator<Item = &'a TraitItem> + Copy,
{
    use std::fmt::Write;

    let mut tokens = TokenStream2::new();

    let variant_name_generic;
    let owned_datatype;
    let mut borrowed_datatype = datatype;
    if let Some(x) = variant_trait {
        variant_name_generic = Ident::new("__VariantName", x.span());
        owned_datatype = StructuralDataType {
            type_name: datatype.type_name,
            fields: Vec::new(),
            variants: vec![StructuralVariant {
                name: VariantIdent::Generic(&variant_name_generic),
                pub_vari_rename: None,
                fields: datatype.fields.clone(),
                is_newtype: false,
                replace_bounds: None,
            }],
        };
        borrowed_datatype = &owned_datatype;

        let sap = StructuralAliasParams {
            span,
            attrs,
            docs: docs.clone(),
            vis,
            ident,
            generics,
            extra_where_preds,
            supertraits,
            trait_items,
            variant_trait: None,
            enum_exhaustiveness: Exhaustiveness::Nonexhaustive,
            datatype,
        };

        tokens.append_all(sap.tokens()?);

        ident = x;
    }
    let datatype = borrowed_datatype;

    if let Some(docs) = &mut docs {
        let docs_for = if variant_trait.is_some() {
            DocsFor::VsiTrait
        } else {
            DocsFor::Trait
        };
        crate::write_docs::write_datatype_docs(docs, docs_for, datatype)?;
        if !supertraits.is_empty() {
            docs.push_str("### supertraits\n\n");

            for supertrait in supertraits {
                let _ = writeln!(docs, "- `{}`", supertrait.to_token_stream(),);
            }
        }
    }

    let attrs = attrs;

    let mut field_bounds = TokenStream2::new();

    let mut out_trait_items = TokenStream2::new();
    for item in trait_items {
        let (is_defaulted, item_name) = match item {
            TraitItem::Const(x) => {
                x.to_tokens(&mut out_trait_items);

                (x.default.is_some(), "associated constant")
            }
            TraitItem::Method(x) => {
                x.to_tokens(&mut out_trait_items);

                (x.default.is_some(), "associated function")
            }
            _ => return_spanned_err!(
                item,
                "Only defaulted associated constant/function are supported.",
            ),
        };
        if !is_defaulted {
            return_spanned_err!(item, "Expected this {} to be defaulted", item_name)
        }
    }

    for variant in &datatype.variants {
        let span = variant.name.span();
        let variant_name = variant.name;
        let variant_ident = Some(variant.name);

        field_bounds.append_all(quote_spanned!(span=>
            structural::pmr::IsVariant<#variant_name>+
        ));

        match &variant.replace_bounds {
            Some(replace_bounds) => {
                replace_bounds.to_tokens(&mut field_bounds, variant.name)?;
            }
            None if !variant.is_newtype => {
                for field in &variant.fields {
                    process_field(field, variant_ident, &mut field_bounds);
                }
            }
            None => {}
        }
    }

    for field in datatype.fields.iter() {
        process_field(field, None, &mut field_bounds);
    }

    let supertraits_a = supertraits.into_iter();
    let supertraits_b = supertraits.into_iter();

    let variant_generic_param = match variant_trait {
        Some(_) => quote!(__VariantName,),
        None => TokenStream2::new(),
    };

    let impl_generics = {
        let after_types = quote!(__This,#variant_generic_param);
        GenParamsIn::with_after_types(generics, InWhat::ImplHeader, after_types)
    };

    let decl_generics =
        GenParamsIn::with_after_types(generics, InWhat::ItemDecl, &variant_generic_param);

    let ty_generics =
        GenParamsIn::with_after_types(generics, InWhat::ItemUse, &variant_generic_param);

    let (_, _, where_clause) = generics.split_for_impl();

    let empty_preds = Punctuated::new();
    let where_preds = where_clause
        .as_ref()
        .map_or(&empty_preds, |x| &x.predicates);
    let where_preds_a = where_preds.into_iter();
    let where_preds_b = where_preds.into_iter();

    let mut exhaustive_bound = None;

    if let Exhaustiveness::Exhaustive | Exhaustiveness::AndExhaustive { .. } = enum_exhaustiveness {
        let variant_count_str = tstr_tokens(datatype.variants.len().to_string(), ident.span());
        let count_bound = quote!( ::structural::pmr::VariantCount<Count=#variant_count_str>+  );

        let attrs = attrs.into_iter();

        let where_preds_a = where_preds.into_iter();
        let where_preds_b = where_preds.into_iter();

        let extra_where_preds_a = extra_where_preds.iter();
        let extra_where_preds_b = extra_where_preds.iter();

        match enum_exhaustiveness {
            Exhaustiveness::Nonexhaustive => unreachable!(),
            Exhaustiveness::Exhaustive => {
                exhaustive_bound = Some(count_bound);
            }
            Exhaustiveness::AndExhaustive { name: exhaus_ident } => {
                let exh_docs = format!(
                    "A subtrait of [{NE}](./trait.{NE}.html) with the additional requirement \
                     that the names and amount of variants must match exactly.",
                    NE = ident,
                );

                tokens.append_all(quote_spanned!(span=>
                    #(#attrs)*
                    #[doc=#exh_docs]
                    #[allow(non_camel_case_types)]
                    #vis trait #exhaus_ident <#decl_generics> :
                        #ident <#ty_generics>+
                        #count_bound
                    where
                        #(#where_preds_a,)*
                        #(#extra_where_preds_a,)*
                    {}

                    impl<#impl_generics> #exhaus_ident <#ty_generics>
                    for __This
                    where
                        #(#where_preds_b,)*
                        #(#extra_where_preds_b,)*
                        __This:
                            ?Sized+
                            #ident <#ty_generics>+
                            #count_bound
                    {}
                ));
            }
        }
    }

    let attrs = attrs.into_iter();

    let extra_where_preds_a = extra_where_preds.iter();
    let extra_where_preds_b = extra_where_preds.iter();

    let docs = docs.into_iter();

    tokens.append_all(quote_spanned!(span=>
        #(#attrs)*
        #( #[doc=#docs] )*
        #vis trait #ident <#decl_generics> :
            #( #supertraits_a+ )*
            #exhaustive_bound
            #field_bounds
        where
            #(#where_preds_a,)*
            #(#extra_where_preds_a,)*
        {
            #out_trait_items
        }


        impl<#impl_generics> #ident <#ty_generics>
        for __This
        where
            __This:
                ?Sized+
                #( #supertraits_b+ )*
                #exhaustive_bound
                #field_bounds,
            #(#where_preds_b,)*
            #(#extra_where_preds_b,)*
        {}
    ));

    Ok(tokens)
}
