use crate::{
    arenas::Arenas,
    datastructure::StructOrEnum,
    field_access::IsOptional,
    ident_or_index::IdentOrIndexRef,
    parse_utils::extract_option_parameter,
    structural_alias_impl_mod::{
        Exhaustiveness, FieldType, StructuralAliasParams, StructuralDataType, StructuralField,
        StructuralVariant,
    },
    tokenizers::{tident_tokens, FullPathForChars, NamedModuleAndTokens},
};

use as_derive_utils::{
    datastructure::{DataStructure, DataVariant, Field, FieldMap, Struct, StructKind},
    gen_params_in::{GenParamsIn, InWhat},
    return_syn_err,
};

use core_extensions::SelfOps;

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, TokenStreamExt};

use syn::{punctuated::Punctuated, DeriveInput, Ident};

mod attribute_parsing;

mod delegation;

#[cfg(test)]
mod tests;

use self::{
    attribute_parsing::{FieldConfig, StructuralOptions},
    delegation::DelegateTo,
};

#[cfg(test)]
fn derive_from_str(string: &str) -> Result<TokenStream2, syn::Error> {
    syn::parse_str(string).and_then(derive)
}

pub fn derive(data: DeriveInput) -> Result<TokenStream2, syn::Error> {
    let ds = &DataStructure::new(&data);

    match ds.data_variant {
        DataVariant::Enum => {}
        DataVariant::Union => {
            return_syn_err!(Span::call_site(), "Cannot derive Structural on an union")
        }
        DataVariant::Struct => {}
    }

    let options = attribute_parsing::parse_attrs_for_structural(ds)?;
    let debug_print = options.debug_print;

    match &options.delegate_to {
        Some(to) => delegating_structural(ds, &options, to),
        None => {
            let arenas = Arenas::default();
            deriving_structural(ds, &options, &arenas)
        }
    }?
    .observe(|tokens| {
        if debug_print {
            panic!("\n\n\n{}\n\n\n", tokens);
        }
    })
    .piped(Ok)
}

fn delegating_structural<'a>(
    ds: &'a DataStructure<'a>,
    _options: &'a StructuralOptions<'a>,
    delegate_to: &'a DelegateTo<'a>,
) -> Result<TokenStream2, syn::Error> {
    let DelegateTo {
        field,
        raw_mut_impl_param,
        bounds,
        mut_bounds,
        move_bounds,
    } = delegate_to;

    let (_, ty_generics, where_clause) = ds.generics.split_for_impl();

    let impl_generics = GenParamsIn::new(ds.generics, InWhat::ImplHeader);

    let tyname = ds.name;

    let the_field = &field.ident;
    let fieldty = field.ty;

    let empty_preds = Punctuated::new();
    let where_preds = where_clause
        .as_ref()
        .map_or(&empty_preds, |x| &x.predicates)
        .into_iter();

    quote!(::structural::unsafe_delegate_structural_with! {
        impl[#impl_generics] #tyname #ty_generics
        where[
            #(#where_preds,)*
            #(#bounds,)*
        ]

        self_ident=this;
        delegating_to_type= #fieldty;
        field_name_param=( _field_name : __FieldName );

        GetFieldImpl { &this.#the_field }


        unsafe GetFieldMutImpl
        where[ #(#mut_bounds,)* ]
        { &mut this.#the_field }

        as_delegating_raw{
            &mut (*this).#the_field as *mut #fieldty
        }
        #raw_mut_impl_param


        IntoFieldImpl
        where[ #(#move_bounds,)* ]
        { this.#the_field }
    })
    .piped(Ok)
}

fn get_optionality<'a>(
    implicit_optionality: bool,
    field: &'_ Field<'a>,
    config_f: &FieldConfig,
    arenas: &'a Arenas,
) -> Option<&'a syn::Type> {
    let optionality_override = config_f.optionality_override;

    match (implicit_optionality, optionality_override) {
        (_, Some(IsOptional::No)) => return None,
        (_, Some(IsOptional::Yes)) => {}
        (false, None) => return None,
        (true, None) => {}
    }

    let ty = field.ty;

    let extracted = extract_option_parameter(ty);

    match (optionality_override.is_some(), extracted) {
        (_, Some(extracted)) => Some(extracted),
        (false, None) => None,
        (true, None) => {
            let opt_ty: syn::Type = syn::parse_quote!( structural::pmr::OptionParam<#ty> );

            Some(arenas.alloc(opt_ty))
        }
    }
}

fn deriving_structural<'a>(
    ds: &'a DataStructure<'a>,
    options: &'a StructuralOptions<'a>,
    arenas: &'a Arenas,
) -> Result<TokenStream2, syn::Error> {
    let StructuralOptions {
        fields: config_fields,
        with_trait_alias,
        implicit_optionality,
        ..
    } = options;
    let &implicit_optionality = implicit_optionality;

    let struct_ = &ds.variants[0];

    let mut names_module = NamedModuleAndTokens::new(ds.name);

    let vis = ds.vis;

    let tyname = ds.name;

    let struct_or_enum = match ds.data_variant {
        DataVariant::Struct => StructOrEnum::Struct,
        DataVariant::Enum => StructOrEnum::Enum,
        DataVariant::Union => unreachable!(),
    };

    let mut field_types = FieldMap::with(ds, |f| f.ty);

    let mut make_fields = |names_module: &mut NamedModuleAndTokens, variant: &'a Struct<'a>| {
        variant
            .fields
            .iter()
            .filter_map(|field| {
                let config_f = &config_fields[field.index];

                if !config_f.is_pub {
                    return None;
                }

                let ident: IdentOrIndexRef<'a> = match &config_f.renamed {
                    Some(x) => x.borrowed(),
                    None => (&field.ident).into(),
                };

                let alias_index = names_module.push_str(ident);

                let optionality_ty = get_optionality(implicit_optionality, field, config_f, arenas);

                let fty = &mut field_types[field];

                if let Some(x) = optionality_ty {
                    *fty = x;
                }

                Some(StructuralField {
                    access: config_f.access,
                    inner_optionality: match optionality_ty {
                        Some(_) => IsOptional::Yes,
                        None => IsOptional::No,
                    },
                    ident,
                    alias_index,
                    ty: match &config_f.is_impl {
                        Some(yes) => FieldType::Impl(yes),
                        None => FieldType::Ty(*fty),
                    },
                })
            })
            .collect::<Vec<StructuralField<'a>>>()
    };

    let sdt = match struct_or_enum {
        StructOrEnum::Struct => StructuralDataType {
            fields: make_fields(&mut names_module, struct_),
            variants: Vec::new(),
        },
        StructOrEnum::Enum => StructuralDataType {
            fields: Vec::new(),
            variants: ds
                .variants
                .iter()
                .enumerate()
                .map(|(vari, variant)| StructuralVariant {
                    name: &variant.name,
                    alias_index: names_module.push_str(variant.name.into()),
                    fields: make_fields(&mut names_module, variant),
                    replace_bounds: options.variants[vari].replace_bounds.as_ref(),
                })
                .collect(),
        },
    };

    let mut structural_alias_trait = TokenStream2::new();

    if *with_trait_alias {
        let trait_ident = Ident::new(&format!("{}_SI", tyname), Span::call_site());
        let soe_str = match struct_or_enum {
            StructOrEnum::Struct => "struct",
            StructOrEnum::Enum => "enum",
        };

        let exhaustive_ident = Ident::new(&format!("{}_ESI", tyname), Span::call_site());

        let enum_exhaustiveness = match struct_or_enum {
            StructOrEnum::Struct => Exhaustiveness::Nonexhaustive,
            StructOrEnum::Enum => Exhaustiveness::AndExhaustive {
                name: &exhaustive_ident,
            },
        };

        Ident::new(&format!("{}_SI", tyname), Span::call_site());

        let docs = format!(
            "A trait aliasing the accessor impls for \
             [{tyname}](./{soe_str}.{tyname}.html) fields\n\
             \n\
             This trait also has all the constraints(where clause and generic parametr bounds)
             of [the same type](./{soe_str}.{tyname}.html).\n\n\
             ### Accessor traits\n\
             These are the accessor traits this aliases:\n\
            ",
            tyname = tyname,
            soe_str = soe_str,
        );

        let struct_variant_trait = match struct_or_enum {
            StructOrEnum::Struct => Some(Ident::new(&format!("{}_VSI", tyname), Span::call_site())),
            StructOrEnum::Enum => None,
        };

        let sop = StructuralAliasParams {
            span: tyname.span(),
            attrs: None::<&Ident>,
            docs: docs,
            vis: vis,
            ident: &trait_ident,
            generics: ds.generics,
            supertraits: &Punctuated::new(),
            names_mod: &names_module,
            trait_items: &[],
            variant_trait: struct_variant_trait.as_ref(),
            enum_exhaustiveness,
            datatype: &sdt,
        };

        structural_alias_trait.append_all(sop.tokens()?);
    }

    let impl_generics = GenParamsIn::new(ds.generics, InWhat::ImplHeader);

    let (_, ty_generics, where_clause) = ds.generics.split_for_impl();

    let empty_preds = Punctuated::new();
    let where_preds = where_clause
        .as_ref()
        .map_or(&empty_preds, |x| &x.predicates)
        .into_iter();

    let names_module_path = &names_module.names_module;

    match struct_or_enum {
        StructOrEnum::Struct => {
            let fields = struct_
                .fields
                .iter()
                .filter(|&f| config_fields[f].is_pub)
                .collect::<Vec<&Field<'_>>>();

            let getter_trait = sdt.fields.iter().map(|f| f.access);

            let field_names = fields.iter().map(|f| &f.ident);

            let field_tys = fields.iter().map(|f| field_types[*f]);

            let inner_optionality = sdt.fields.iter().map(|f| f.inner_optionality.derive_arg());

            let renamed_field_names =
                fields
                    .iter()
                    .map(|&field| match &config_fields[field].renamed {
                        Some(x) => x.to_string(),
                        None => field.ident.to_string(),
                    });

            let alias_names = sdt
                .fields
                .iter()
                .map(|f| names_module.alias_name(f.alias_index));

            quote!(
                #structural_alias_trait

                #names_module

                ::structural::impl_getters_for_derive_struct!{
                    impl[#impl_generics] #tyname #ty_generics
                    where[ #(#where_preds,)* ]
                    {
                        #((
                            #getter_trait<
                                #field_names : #field_tys ,
                                structural::pmr::FieldPath1<#names_module_path::#alias_names>,
                                opt=#inner_optionality,
                                #renamed_field_names,
                            >
                        ))*
                    }
                }
            )
        }
        StructOrEnum::Enum => {
            let variants = ds
                .variants
                .iter()
                .zip(&sdt.variants)
                .map(|(variant, sdt_variant)| {
                    let fields = variant
                        .fields
                        .iter()
                        .filter(|&f| config_fields[f].is_pub)
                        .collect::<Vec<&Field<'_>>>();

                    let variant_kind = match (variant.kind, variant.fields.len()) {
                        (StructKind::Tuple, 0) => quote!(unit),
                        (StructKind::Tuple, 1) => quote!(newtype),
                        _ => quote! { regular },
                    };

                    let field_tokens =
                        fields
                            .iter()
                            .zip(&sdt_variant.fields)
                            .map(|(&field, sdt_field)| {
                                let access = sdt_field.access;
                                let fname = &field.ident;
                                let fty = field_types[field];
                                let inner_optionality = sdt_field.inner_optionality.derive_arg();
                                let f_tstr = names_module.alias_name(sdt_field.alias_index);
                                quote!(
                                    #access,
                                    #fname:#fty,
                                    #inner_optionality,
                                    #names_module_path::#f_tstr,
                                )
                            });

                    let variant_name = variant.name;
                    let variant_str = names_module.alias_name(sdt_variant.alias_index);

                    quote!(
                        #variant_name,
                        #names_module_path::#variant_str,
                        kind=#variant_kind,
                        fields( #( (#field_tokens) )* )
                    )
                });

            let variant_count_docs = format!("The amount of variants in the {} enum", ds.name);
            let variant_count_type =
                syn::Ident::new(&format!("{}_VariantCount", ds.name), Span::call_site());

            let variant_count = tident_tokens(ds.variants.len().to_string(), FullPathForChars::Yes);
            let enum_ = ds.name;

            quote!(
                #structural_alias_trait

                #names_module

                #[doc=#variant_count_docs]
                #vis type #variant_count_type=#variant_count;

                ::structural::impl_getters_for_derive_enum!{
                    impl[#impl_generics] #tyname #ty_generics
                    where[ #(#where_preds,)* ]
                    {
                        enum=#enum_
                        variant_count=#variant_count_type,
                        #((#variants))*
                    }
                }
            )
        }
    }
    .piped(Ok)
}
