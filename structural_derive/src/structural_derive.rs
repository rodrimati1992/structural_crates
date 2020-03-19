use crate::{
    arenas::Arenas,
    datastructure::StructOrEnum,
    ident_or_index::IdentOrIndexRef,
    structural_alias_impl_mod::{
        Exhaustiveness, FieldType, StructuralAliasParams, StructuralDataType, StructuralField,
        StructuralVariant, VariantIdent,
    },
    tokenizers::tident_tokens,
    write_docs::{self, DocsFor},
};

use as_derive_utils::{
    datastructure::{DataStructure, DataVariant, Field, Struct},
    gen_params_in::{GenParamsIn, InWhat},
    return_syn_err,
};

use core_extensions::SelfOps;

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, ToTokens, TokenStreamExt};

use syn::{punctuated::Punctuated, DeriveInput, Ident, Visibility};

mod attribute_parsing;

mod delegation;

#[cfg(test)]
mod tests;

use self::{attribute_parsing::StructuralOptions, delegation::DelegateTo};

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
        delegation_params,
        bounds,
        mut_bounds,
        move_bounds,
    } = delegate_to;

    use std::fmt::Write;

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

    let mut docs = format!("`{}` delegates all its accessor trait impls to ", tyname);
    let ty_tokens = field.ty.to_token_stream();
    let _ = match field.vis {
        Visibility::Public { .. } => write!(docs, "the `{}: {}` field", field.ident, ty_tokens),
        _ => write!(docs, "a private `{}` field", ty_tokens),
    };

    quote!(::structural::unsafe_delegate_structural_with! {
        #[doc=#docs]
        impl[#impl_generics] #tyname #ty_generics
        where[
            #(#where_preds,)*
            #(#bounds,)*
        ]

        self_ident=this;
        #delegation_params
        delegating_to_type= #fieldty;

        GetField { &this.#the_field }


        GetFieldMut
        where[ #(#mut_bounds,)* ]
        { &mut this.#the_field }

        as_delegating_raw{
            &mut (*this).#the_field as *mut #fieldty
        }


        IntoField
        where[ #(#move_bounds,)* ]
        { this.#the_field }
    })
    .piped(Ok)
}

fn deriving_structural<'a>(
    ds: &'a DataStructure<'a>,
    options: &'a StructuralOptions<'a>,
    _arenas: &'a Arenas,
) -> Result<TokenStream2, syn::Error> {
    let StructuralOptions {
        fields: config_fields,
        with_trait_alias,
        ..
    } = options;

    let struct_ = &ds.variants[0];

    let vis = ds.vis;

    let tyname = ds.name;

    let struct_or_enum = match ds.data_variant {
        DataVariant::Struct => StructOrEnum::Struct,
        DataVariant::Enum => StructOrEnum::Enum,
        DataVariant::Union => unreachable!(),
    };

    let make_fields = |variant: &'a Struct<'a>| {
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

                Some(StructuralField {
                    access: config_f.access,
                    ident,
                    pub_field_rename: if field.is_public() && config_f.renamed.is_some() {
                        Some((&field.ident).into())
                    } else {
                        None
                    },
                    ty: match &config_f.is_impl {
                        Some(yes) => FieldType::Impl(yes),
                        None => FieldType::Ty(field.ty),
                    },
                })
            })
            .collect::<Vec<StructuralField<'a>>>()
    };

    let sdt = match struct_or_enum {
        StructOrEnum::Struct => StructuralDataType {
            type_name: Some(ds.name),
            fields: make_fields(struct_),
            variants: Vec::new(),
        },
        StructOrEnum::Enum => StructuralDataType {
            type_name: Some(ds.name),
            fields: Vec::new(),
            variants: ds
                .variants
                .iter()
                .enumerate()
                .map(|(vari, variant)| {
                    let config_v = &options.variants[vari];

                    let name: IdentOrIndexRef<'a> = match &config_v.renamed {
                        Some(x) => x.borrowed(),
                        None => variant.name.into(),
                    };

                    StructuralVariant {
                        name: VariantIdent::Ident(name),
                        pub_vari_rename: if options.generate_docs && config_v.renamed.is_some() {
                            Some(variant.name.into())
                        } else {
                            None
                        },
                        fields: make_fields(variant),
                        is_newtype: config_v.is_newtype,
                        replace_bounds: config_v.replace_bounds.as_ref(),
                    }
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

        let docs = if options.generate_docs {
            Some(format!(
                "A trait aliasing the accessor impls for \
                 [{tyname}](./{soe_str}.{tyname}.html) fields\n\
                 \n\
                 This trait also has all the constraints(where clause and generic parameter bounds)
                 of [the same type](./{soe_str}.{tyname}.html).\n\n\
                 ### Accessor traits\n\
                 These are the accessor traits this aliases:\n\
                ",
                tyname = tyname,
                soe_str = soe_str,
            ))
        } else {
            None
        };

        let struct_variant_trait = match struct_or_enum {
            StructOrEnum::Struct => Some(Ident::new(&format!("{}_VSI", tyname), Span::call_site())),
            StructOrEnum::Enum => None,
        };

        let sop = StructuralAliasParams {
            span: tyname.span(),
            attrs: None::<&Ident>,
            docs,
            vis,
            ident: &trait_ident,
            generics: ds.generics,
            extra_where_preds: &options.bounds,
            supertraits: &Punctuated::new(),
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

    let mut config_variants = options.variants.iter();

    let tuple =
        match struct_or_enum {
            StructOrEnum::Struct => {
                let fields = struct_
                    .fields
                    .iter()
                    .filter(|&f| config_fields[f].is_pub)
                    .collect::<Vec<&Field<'_>>>();

                let getter_trait = sdt
                    .fields
                    .iter()
                    .map(|f| f.access.compute_trait(StructOrEnum::Struct));

                let field_names = fields.iter().map(|f| &f.ident);

                let field_name_tstrs = sdt.fields.iter().map(|f| f.ident.tstr_tokens());

                let field_tys = fields.iter().map(|f| f.ty);

                let renamed_field_names =
                    fields
                        .iter()
                        .map(|&field| match &config_fields[field].renamed {
                            Some(x) => x.to_string(),
                            None => field.ident.to_string(),
                        });

                (
                    quote!(_private_impl_getters_for_derive_struct),
                    quote!(),
                    quote!(
                        #((
                            #getter_trait<
                                #field_names : #field_tys ,
                                #field_name_tstrs,
                                #renamed_field_names,
                            >
                        ))*
                    ),
                )
            }
            StructOrEnum::Enum => {
                let variants =
                    ds.variants
                        .iter()
                        .zip(&sdt.variants)
                        .map(|(variant, sdt_variant)| {
                            let fields = variant
                                .fields
                                .iter()
                                .filter(|&f| config_fields[f].is_pub)
                                .collect::<Vec<&Field<'_>>>();

                            let config_v = config_variants.next().unwrap();

                            let variant_kind = if config_v.is_newtype {
                                quote!(newtype)
                            } else {
                                quote!(regular)
                            };

                            let field_tokens = fields.iter().zip(&sdt_variant.fields).map(
                                |(&field, sdt_field)| {
                                    let access = sdt_field.access.compute_trait(StructOrEnum::Enum);
                                    let fname = &field.ident;
                                    let fty = field.ty;
                                    let f_tstr = sdt_field.ident.tstr_tokens();
                                    quote!(
                                        #access,
                                        #fname:#fty,
                                        #f_tstr,
                                    )
                                },
                            );

                            let variant_name = variant.name;
                            let variant_str = sdt_variant.name.tokens();
                            quote!(
                                #variant_name,
                                #variant_str,
                                kind=#variant_kind,
                                fields( #( (#field_tokens) )* )
                            )
                        });

                let enum_ = ds.name;
                let variant_count = tident_tokens(ds.variants.len().to_string());

                let variant_count_tokens = if options.make_variant_count_alias {
                    let variant_count_ident_str = format!("{}_VC", ds.name);
                    let variant_count_docs = format!(
                        "\
                        The amount of variants in the [{0} enum](./enum.{0}.html)\n\
                        \n\
                        This is a structural::TStr,\
                        which can be instantiated with {1}::NEW.\n\
                    ",
                        ds.name, variant_count_ident_str,
                    );
                    let variant_count_type =
                        syn::Ident::new(&variant_count_ident_str, Span::call_site());

                    quote!(
                        #[doc=#variant_count_docs]
                        #vis type #variant_count_type=#variant_count;
                    )
                } else {
                    quote!()
                };

                (
                    quote!(_private_impl_getters_for_derive_enum),
                    variant_count_tokens,
                    quote! {
                        enum=#enum_
                        variant_count=#variant_count,
                        #((#variants))*
                    },
                )
            }
        };

    let mut impl_docs = String::new();
    if options.generate_docs {
        write_docs::write_datatype_docs(&mut impl_docs, DocsFor::Type, &sdt)?;
    }

    let (which_macro, soe_specific_out, soe_specific_in) = tuple;
    let extra_where_preds = options.bounds.iter();

    quote!(
        #structural_alias_trait

        #soe_specific_out

        ::structural::#which_macro!{
            #[doc=#impl_docs]
            impl[#impl_generics] #tyname #ty_generics
            where[
                #(#where_preds,)*
                #(#extra_where_preds,)*
            ]
            {#soe_specific_in}
        }
    )
    .piped(Ok)
}
