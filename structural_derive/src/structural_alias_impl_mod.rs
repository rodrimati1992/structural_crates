use crate::{
    datastructure::StructOrEnum,
    field_access::{Access, ComputeTrait, IsOptional},
    ident_or_index::IdentOrIndexRef,
    parse_utils::ParsePunctuated,
    tokenizers::{tident_tokens, FullPathForChars, NamedModuleAndTokens, NamesModuleIndex},
    write_docs::DocsFor,
};

use as_derive_utils::{
    gen_params_in::{GenParamsIn, InWhat},
    return_spanned_err, ToTokenFnMut,
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
    pub(crate) names_mod: NamedModuleAndTokens,
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
    pub(crate) name: IdentOrIndexRef<'a>,
    /// The name of the original variant.
    /// This is Some if all these conditions are true:
    /// - this was derived from a type definition.
    /// - the enum is public
    /// - the variant has a `#[struc(rename="")]` attribute
    pub(crate) pub_vari_rename: Option<IdentOrIndexRef<'a>>,
    pub(crate) alias_index: NamesModuleIndex,
    pub(crate) fields: Vec<StructuralField<'a>>,
    pub(crate) is_newtype: bool,
    pub(crate) replace_bounds: Option<&'a ReplaceBounds>,
}

/// A smaller version of StructuralField
#[derive(Debug, Copy, Clone)]
pub(crate) struct TinyStructuralField<'a> {
    pub(crate) access: Access,
    pub(crate) ident: IdentOrIndexRef<'a>,
    pub(crate) inner_optionality: IsOptional,
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
    pub(crate) alias_index: NamesModuleIndex,
    pub(crate) inner_optionality: IsOptional,
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

#[derive(Debug, Copy, Clone)]
pub(crate) enum VariantIdent<'a> {
    Ident {
        ident: IdentOrIndexRef<'a>,
        alias_ident: &'a Ident,
    },
    StructVariantTrait {
        ident: &'a Ident,
    },
}

// A hack to allow borrowing from the arena inside a parser
impl<'a> Parse for StructuralAliasesHack {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
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
            self.access.compute_trait(self.inner_optionality, soe)
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

    fn to_tokens(
        &self,
        field_bounds: &mut TokenStream2,
        names_mod: &NamedModuleAndTokens,
        alias_ident: &Ident,
    ) -> Result<(), syn::Error> {
        let nma = format!("{}::{}", names_mod.names_module, alias_ident);
        let bounds_str = syn::LitStr::new(&self.bounds.replace(Self::NEEDLE, &nma), self.span);

        let bounds: TypeParamBounds = bounds_str.parse::<ParsePunctuated<_, _>>()?.list;

        let plus = <syn::Token!(+)>::default();
        for bound in bounds {
            bound.to_tokens(field_bounds);
            plus.to_tokens(field_bounds);
        }
        Ok(())
    }

    pub(crate) fn write_docs(&self, buffer: &mut String, variant_name: IdentOrIndexRef<'_>) {
        use std::fmt::Write;

        let doc_bounds = self
            .bounds
            .replace(Self::NEEDLE, &format!("TS!({})", variant_name));

        let _ = writeln!(buffer, "Using these bounds for the variant: {}", doc_bounds);
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> TinyStructuralField<'a> {
    pub(crate) fn tokens(&self, soe: StructOrEnum) -> TokenStream2 {
        let TinyStructuralField { ident, ty, .. } = *self;

        let the_trait = self.compute_trait(soe).trait_tokens();
        let ident = tident_tokens(ident.to_string(), FullPathForChars::Yes);
        let ty = ToTokenFnMut::new(|tokens| match ty {
            FieldType::Ty(x) => x.to_tokens(tokens),
            FieldType::Impl(x) => {
                <syn::Token!(impl)>::default().to_tokens(tokens);
                x.to_tokens(tokens);
            }
        });

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

pub(crate) fn macro_impl<'a>(aliases: StructuralAliases<'a>) -> Result<TokenStream2, syn::Error> {
    let list = aliases.list;

    if list.is_empty() {
        return Ok(quote!());
    }

    let mut out = TokenStream2::new();
    let extra_where_preds = Punctuated::default();
    for saf in list {
        let trait_docs =
            if saf.options.generate_docs && matches!(syn::Visibility::Public{..} = saf.vis) {
                Some(format!(
                    "A trait which aliases `structural` accessor traits,\
                     generated by the `structural_alias` macro.\n\n"
                ))
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
            names_mod: &saf.names_mod,
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

        saf.names_mod.to_tokens(&mut out);
    }
    // panic!("{}", out);
    Ok(out)
}

fn process_field(
    field: &StructuralField<'_>,
    variant_ident: Option<VariantIdent<'_>>,
    names_mod: &NamedModuleAndTokens,
    field_bounds: &mut TokenStream2,
) {
    use self::FieldType as FT;

    let soe = StructOrEnum::from(variant_ident);

    let f_alias_name = names_mod.alias_name(field.alias_index);
    let names_mod_path = &names_mod.names_module;
    let aaoo = field.compute_trait(soe);
    let assoc_ty = match &field.ty {
        FT::Ty(ty) => quote!(Ty=#ty),
        FT::Impl(bounds) => quote!(Ty:#bounds),
    };

    match variant_ident {
        Some(variant_ident) => {
            let variant_name_param = match variant_ident {
                VariantIdent::Ident { alias_ident, .. } => quote!(#names_mod_path::#alias_ident),
                VariantIdent::StructVariantTrait { .. } => quote!(__VariantName),
            };

            let vf_trait = ComputeTrait {
                struct_or_enum: StructOrEnum::Enum,
                ..aaoo
            };

            field_bounds.append_all(quote!(
                structural::pmr::#vf_trait<
                    #variant_name_param,
                    #names_mod_path::#f_alias_name,
                    #assoc_ty
                >+
            ));
        }
        None => {
            let trait_ = aaoo.trait_tokens();
            field_bounds.append_all(quote!(
                structural::#trait_<
                    #names_mod_path::#f_alias_name,
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
    pub(crate) names_mod: &'a NamedModuleAndTokens,
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
        names_mod,
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

    let mut top_variant_ident = None::<VariantIdent<'a>>;

    let mut tokens = TokenStream2::new();

    if let Some(x) = variant_trait {
        top_variant_ident = Some(VariantIdent::StructVariantTrait { ident: x });

        let sap = StructuralAliasParams {
            span,
            attrs,
            docs: docs.clone(),
            vis,
            ident,
            generics,
            extra_where_preds,
            supertraits,
            names_mod,
            trait_items,
            variant_trait: None,
            enum_exhaustiveness: Exhaustiveness::Nonexhaustive,
            datatype,
        };

        tokens.append_all(sap.tokens()?);

        ident = x;
    }

    if let Some(docs) = &mut docs {
        crate::write_docs::write_datatype_docs(
            docs,
            DocsFor::Trait,
            datatype,
            &names_mod,
            top_variant_ident,
        );
        if !supertraits.is_empty() {
            docs.push_str("### supertraits\n\n");

            for supertrait in supertraits {
                let _ = writeln!(docs, "- `{}`", supertrait.to_token_stream(),);
            }
        }
    }

    let attrs = attrs;

    let names_mod_path = &names_mod.names_module;

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
        let alias_ident = names_mod.alias_name(variant.alias_index);
        let variant_ident = Some(VariantIdent::Ident {
            ident: variant.name,
            alias_ident,
        });

        field_bounds.append_all(quote!(
            structural::pmr::IsVariant<
                #names_mod_path::#alias_ident
            >
            +
        ));

        match &variant.replace_bounds {
            Some(replace_bounds) => {
                replace_bounds.to_tokens(&mut field_bounds, names_mod, alias_ident)?;
            }
            None if variant.is_newtype => {
                for field in &variant.fields {
                    process_field(field, variant_ident, names_mod, &mut field_bounds);
                }
            }
            None => {}
        }
    }

    for field in datatype.fields.iter() {
        process_field(field, top_variant_ident, names_mod, &mut field_bounds);
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
        let variant_count_str =
            tident_tokens(datatype.variants.len().to_string(), FullPathForChars::Yes);
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
                tokens.append_all(quote_spanned!(span=>
                    #(#attrs)*
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
