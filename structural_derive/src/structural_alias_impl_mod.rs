use crate::{
    arenas::Arenas,
    field_access::{Access, AccessAndIsOptional, IsOptional},
    ident_or_index::IdentOrIndexRef,
    parse_utils::ParsePunctuated,
    tokenizers::{tident_tokens, FullPathForChars, NamedModuleAndTokens, NamesModuleIndex},
};

use as_derive_utils::{
    gen_params_in::{GenParamsIn, InWhat},
    return_spanned_err, ToTokenFnMut,
};

#[allow(unused_imports)]
use core_extensions::SelfOps;

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};

use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, Generics, Ident, Token, TraitItem, Visibility,
};

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests;

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
    pub(crate) trait_token: Token!(trait),
    pub(crate) generics: syn::Generics,
    pub(crate) supertraits: Punctuated<syn::TypeParamBound, token::Add>,
    pub(crate) extra_items: Vec<TraitItem>,
    pub(crate) datatype: StructuralDataType<'a>,
}

#[derive(Debug)]
pub struct StructuralDataType<'a> {
    pub(crate) variants: Vec<StructuralVariant<'a>>,
    pub(crate) fields: Vec<StructuralField<'a>>,
}

#[derive(Debug, Clone)]
pub struct StructuralVariant<'a> {
    pub(crate) name: &'a Ident,
    pub(crate) alias_index: NamesModuleIndex,
    pub(crate) fields: Vec<StructuralField<'a>>,
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
    pub(crate) alias_index: NamesModuleIndex,
    pub(crate) inner_optionality: IsOptional,
    pub(crate) ty: FieldType<'a>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum FieldType<'a> {
    Ty(&'a syn::Type),
    Impl(&'a TypeParamBounds),
}

pub(crate) type TypeParamBounds = Punctuated<syn::TypeParamBound, syn::token::Add>;

#[derive(Debug, Copy, Clone)]
pub(crate) enum VariantIdent<'a> {
    Ident {
        ident: &'a Ident,
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

impl<'a> StructuralAliases<'a> {
    fn parse(arenas: &'a Arenas, input: ParseStream) -> Result<Self, syn::Error> {
        let mut list = Vec::<StructuralAlias>::new();
        while !input.is_empty() {
            list.push(StructuralAlias::parse(arenas, input)?);
        }
        Ok(Self { list })
    }
}

impl<'a> StructuralAlias<'a> {
    fn parse(arenas: &'a Arenas, input: ParseStream) -> Result<Self, syn::Error> {
        let mut extra_items = Vec::<TraitItem>::new();
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;

        let trait_token: Token![trait] = input.parse()?;

        let ident = arenas.alloc(input.parse::<Ident>()?);

        let mut names_mod = NamedModuleAndTokens::new(ident);
        let mut generics: Generics = input.parse()?;
        let colon_token: Option<Token![:]> = input.parse()?;

        let mut supertraits = Punctuated::new();
        if colon_token.is_some() {
            loop {
                supertraits.push_value(input.parse()?);
                if input.peek(Token![where]) || input.peek(token::Brace) {
                    break;
                }
                supertraits.push_punct(input.parse()?);
                if input.peek(Token![where]) || input.peek(token::Brace) {
                    break;
                }
            }
        }

        generics.where_clause = input.parse()?;

        // let equal:Token![=]= input.parse()?;

        let content;
        let braces = syn::braced!(content in input);

        let datatype =
            StructuralDataType::parse(&mut names_mod, &mut extra_items, arenas, &content)?;

        let span = trait_token
            .span
            .join(braces.span)
            .unwrap_or(trait_token.span);

        Ok(Self {
            names_mod,
            span,
            attrs,
            vis,
            trait_token,
            ident,
            generics,
            supertraits,
            extra_items,
            datatype,
        })
    }
}

impl<'a> StructuralDataType<'a> {
    fn parse(
        names_mod: &mut NamedModuleAndTokens,
        extra_items: &mut Vec<TraitItem>,
        arenas: &'a Arenas,
        input: ParseStream,
    ) -> Result<Self, syn::Error> {
        let mut variants = Vec::new();
        let mut fields = Vec::new();
        loop {
            if input.is_empty() {
                break;
            }

            let forked = input.fork();
            if let Ok(item) = forked.parse::<TraitItem>() {
                input.advance_to(&forked);
                extra_items.push(item);
                continue;
            }

            let access = input.parse::<Access>()?;
            if input.peek(syn::Ident) && input.peek2(token::Brace) {
                let ident = arenas.alloc(input.parse::<Ident>()?);
                let ident_index = names_mod.push_str(ident.into());

                let content;
                let _ = syn::braced!(content in input);
                input.parse::<Token![,]>()?;

                variants.push(StructuralVariant::parse(
                    names_mod,
                    access,
                    ident,
                    ident_index,
                    arenas,
                    &content,
                )?);
            } else {
                fields.push(StructuralField::parse_with_access(
                    names_mod, access, arenas, input,
                )?);
            }
        }
        Ok(Self { variants, fields })
    }
}

impl<'a> StructuralVariant<'a> {
    fn parse(
        names_mod: &mut NamedModuleAndTokens,
        access: Access,
        name: &'a Ident,
        alias_index: NamesModuleIndex,
        arenas: &'a Arenas,
        input: ParseStream,
    ) -> Result<Self, syn::Error> {
        let mut fields = Vec::<StructuralField<'a>>::new();
        while !input.is_empty() {
            let nested_access = Access::parse_optional(input)?;

            fields.push(StructuralField::parse_with_access(
                names_mod,
                nested_access.unwrap_or(access),
                arenas,
                input,
            )?);
        }
        Ok(Self {
            name,
            alias_index,
            fields,
            replace_bounds: None,
        })
    }
}

macro_rules! declare_structural_field_methods {
    () => (
        pub(crate) fn access_and_optionality(&self)-> AccessAndIsOptional {
            self.access.and_optionality( self.inner_optionality )
        }
    )
}

impl<'a> TinyStructuralField<'a> {
    pub(crate) fn parse(
        access: Access,
        arenas: &'a Arenas,
        input: ParseStream,
    ) -> Result<Self, syn::Error> {
        let ident = IdentOrIndexRef::parse(arenas, input)?;
        let _: Token![:] = input.parse()?;
        let inner_optionality = input.parse::<IsOptional>()?;
        let ty = FieldType::parse(arenas, input)?;

        Ok(Self {
            access,
            ident,
            inner_optionality,
            ty,
        })
    }

    declare_structural_field_methods! {}
}

impl<'a> StructuralField<'a> {
    fn parse_with_access(
        names_mod: &mut NamedModuleAndTokens,
        access: Access,
        arenas: &'a Arenas,
        input: ParseStream,
    ) -> Result<Self, syn::Error> {
        let TinyStructuralField {
            access: _,
            ident,
            inner_optionality,
            ty,
        } = TinyStructuralField::parse(access, arenas, input)?;
        input.parse::<Token![,]>()?;

        Ok(Self {
            access,
            ident,
            alias_index: names_mod.push_str(ident),
            inner_optionality,
            ty,
        })
    }

    declare_structural_field_methods! {}
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

    fn write_docs(&self, buffer: &mut String, variant_name: &Ident) {
        use std::fmt::Write;

        let doc_bounds = self
            .bounds
            .replace(Self::NEEDLE, &format!("TStr!({})", variant_name));

        let _ = writeln!(buffer, "Using these bounds for the variant: {}", doc_bounds);
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> TinyStructuralField<'a> {
    pub(crate) fn tokens(&self) -> TokenStream2 {
        let TinyStructuralField { ident, ty, .. } = *self;

        let the_trait = self.access_and_optionality().trait_tokens();
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
                structural::pmr::FieldPath<(#ident,)>,
                Ty=#ty,
            >
        )
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> FieldType<'a> {
    fn parse(arenas: &'a Arenas, input: ParseStream) -> Result<Self, syn::Error> {
        const ASSOC_TY_BOUNDS: bool = cfg!(feature = "impl_fields");

        use syn::Type;

        match input.parse::<syn::Type>()? {
            Type::ImplTrait(x) => {
                if ASSOC_TY_BOUNDS {
                    Ok(FieldType::Impl(arenas.alloc(x.bounds)))
                } else {
                    use syn::spanned::Spanned;
                    Err(syn::Error::new(
                        x.span(),
                        "\
                         Cannot use an `impl Trait` field without enabling the \
                         \"nightly_impl_fields\" or \"impl_fields\" feature.\
                         ",
                    ))
                }
            }
            x => Ok(FieldType::Ty(arenas.alloc(x))),
        }
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
    for saf in list {
        for_delegation(
            saf.span,
            &saf.attrs,
            format!(
                "A trait which aliases `structural` accessor traits,\
                 generated by the `structural_alias` macro.\n\n"
            ),
            &saf.vis,
            &saf.trait_token,
            &saf.ident,
            &saf.generics,
            &saf.supertraits,
            &saf.names_mod,
            &saf.extra_items,
            None,
            &saf.datatype,
        )?
        .piped(|x| out.append_all(x));

        saf.names_mod.to_tokens(&mut out);
    }
    // panic!("{}", out);
    Ok(out)
}

fn write_field_docs(
    docs: &mut String,
    left_padding: &str,
    variant: Option<VariantIdent<'_>>,
    field: &StructuralField<'_>,
) -> std::fmt::Result {
    use std::fmt::Write;

    use self::FieldType as FT;

    let the_trait = field.access_and_optionality().trait_name();

    let ident = match variant {
        Some(VariantIdent::Ident { ident, .. }) => format!("::{}.{}", ident, field.ident),
        Some(VariantIdent::StructVariantTrait { .. }) => {
            format!("::[__VariantName].{}", field.ident)
        }
        None => field.ident.to_string(),
    };

    let access_desc = match field.access {
        Access::Shared => "a shared reference accessor",
        Access::Mutable => "shared and mutable reference accessors",
        Access::Value => "shared reference, and by value accessors",
        Access::MutValue => "shared reference,mutable reference, and by value accessors",
    };
    let assoc_ty = match field.ty {
        FT::Ty(ty) => format!("Ty= {}", ty.to_token_stream()),
        FT::Impl(bounds) => format!("Ty: {}", bounds.to_token_stream()),
    };
    let field_ty = match field.ty {
        FT::Ty(ty) => format!("{}", ty.to_token_stream()),
        FT::Impl(bounds) => format!("impl {}", bounds.to_token_stream()),
    };
    writeln!(
        docs,
        "{LP}Bound:`{0}<FP!( {1} ),{2}>`\n<br>",
        the_trait,
        ident,
        assoc_ty,
        LP = left_padding,
    )?;
    writeln!(
        docs,
        "{LP}The &nbsp; `{0}: {1}` &nbsp; ",
        field.ident,
        field_ty,
        LP = left_padding,
    )?;
    match (variant, field.inner_optionality) {
        (
            Some(VariantIdent::Ident {
                ident: vari_name, ..
            }),
            _,
        ) => {
            write!(docs, "field in the `{}` variant", vari_name)?;
        }
        (Some(VariantIdent::StructVariantTrait { .. }), _) => {
            write!(docs, "field in the `[__VariantName]` variant")?;
        }
        (None, IsOptional::Yes) => docs.push_str("optional field"),
        (None, IsOptional::No) => docs.push_str("field"),
    }
    docs.push_str(", with ");
    docs.push_str(access_desc);
    docs.push_str("\n\n");
    Ok(())
}

fn process_field(
    field: &StructuralField<'_>,
    variant_ident: Option<VariantIdent<'_>>,
    names_mod: &NamedModuleAndTokens,
    field_bounds: &mut TokenStream2,
) {
    use self::FieldType as FT;

    let f_alias_name = names_mod.alias_name(field.alias_index);
    let names_mod_path = &names_mod.names_module;
    let aaoo = field.access_and_optionality();
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

            let vf_trait = aaoo.variant_field_trait_tokens();

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
                    structural::pmr::FieldPath1<#names_mod_path::#f_alias_name>,
                    #assoc_ty
                >+
            ));
        }
    }
}

/// This allows both `structural_alias` and `#[derive(Structural)]` to generate
/// the trait alias and its impl.
pub(crate) fn for_delegation<'a, A, I>(
    span: Span,
    attrs: A,
    mut docs: String,
    vis: &syn::Visibility,
    trait_token: &Token!(trait),
    mut ident: &'a Ident,
    generics: &syn::Generics,
    supertraits: &Punctuated<syn::TypeParamBound, token::Add>,
    names_mod: &NamedModuleAndTokens,
    trait_items: I,
    variant_trait: Option<&'a Ident>,
    datatype: &StructuralDataType<'_>,
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

        tokens.append_all(for_delegation(
            span,
            attrs,
            docs.clone(),
            vis,
            trait_token,
            ident,
            generics,
            supertraits,
            names_mod,
            trait_items,
            None,
            datatype,
        )?);

        ident = x;
    }

    let attrs = attrs.into_iter();

    let names_mod_path = &names_mod.names_module;

    let mut field_bounds = TokenStream2::new();

    const SPACES_X8: &'static str = "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;";

    if !datatype.variants.is_empty() {
        docs.push_str("### Variants\n\n");
        docs.push_str(
            "This trait aliases the `IsVariant<FP!( Variant )>` trait for \
             each of the variants below.\n\n\
             The accessor for every enum variant field is optional,\
             because the enum might not be that variant.\n\n",
        );
    }

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
                structural::pmr::FieldPath1<
                    #names_mod_path::#alias_ident
                >
            >
            +
        ));

        let _ = write!(docs, "Variant `{}` {{", variant.name,);
        docs.push_str(
            if variant.fields.is_empty() || variant.replace_bounds.is_some() {
                " "
            } else {
                "<br>"
            },
        );

        match &variant.replace_bounds {
            Some(replace_bounds) => {
                replace_bounds.to_tokens(&mut field_bounds, names_mod, alias_ident)?;

                replace_bounds.write_docs(&mut docs, variant.name);
            }
            None => {
                for field in &variant.fields {
                    process_field(field, variant_ident, names_mod, &mut field_bounds);

                    let _ = write_field_docs(&mut docs, SPACES_X8, variant_ident, field);
                }
            }
        }

        let _ = writeln!(docs, "}}\n");
    }

    if !datatype.fields.is_empty() {
        docs.push_str("### Fields\n\n");
    }
    for field in datatype.fields.iter() {
        process_field(field, top_variant_ident, names_mod, &mut field_bounds);
        let _ = write_field_docs(&mut docs, "", top_variant_ident, field);
    }

    if !supertraits.is_empty() {
        docs.push_str("### supertraits\n\n");

        for supertrait in supertraits {
            let _ = writeln!(docs, "- `{}`", supertrait.to_token_stream(),);
        }
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

    tokens.append_all(quote_spanned!(span=>
        #(#attrs)*
        #[doc=#docs]
        #vis
        #trait_token #ident <#decl_generics> :
            #( #supertraits_a+ )*
            #field_bounds
        where
            #(#where_preds_a,)*
        {
            #out_trait_items
        }


        impl<#impl_generics> #ident <#ty_generics>
        for __This
        where
            __This:
                ?Sized+
                #( #supertraits_b+ )*
                #field_bounds,
            #(#where_preds_b,)*
        {}
    ));

    Ok(tokens)
}
