use crate::{
    datastructure::StructOrEnum,
    field_access::IsOptional,
    ident_or_index::IdentOrIndexRef,
    structural_alias_impl_mod::{
        FieldType, StructuralDataType, StructuralField, StructuralVariant,
    },
    tokenizers::NamedModuleAndTokens,
};

use as_derive_utils::{
    datastructure::{DataStructure, DataVariant, Field, Struct, StructKind},
    gen_params_in::{GenParamsIn, InWhat},
    return_syn_err,
};

use core_extensions::SelfOps;

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::quote;

use syn::{punctuated::Punctuated, DeriveInput, Ident, Token};

mod attribute_parsing;

#[cfg(test)]
mod tests;

use self::attribute_parsing::StructuralOptions;

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

    match options.delegate_to {
        Some(to) => delegating_structural(ds, &options, to),
        None => deriving_structural(ds, &options),
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
    delegate_to: &'a Field<'a>,
) -> Result<TokenStream2, syn::Error> {
    let (_, ty_generics, where_clause) = ds.generics.split_for_impl();

    let impl_generics = GenParamsIn::new(ds.generics, InWhat::ImplHeader);

    let tyname = ds.name;

    let the_field = &delegate_to.ident;
    let fieldty = delegate_to.ty;

    let empty_preds = Punctuated::new();
    let where_preds = where_clause
        .as_ref()
        .map_or(&empty_preds, |x| &x.predicates)
        .into_iter();

    quote!(::structural::z_delegate_structural_with! {
        impl[#impl_generics] #tyname #ty_generics
        where[ #(#where_preds,)* ]

        self_ident=this;
        delegating_to_type= #fieldty;
        field_name_param=( _field_name : __FieldName );

        GetFieldImpl { &this.#the_field }

        unsafe GetFieldMutImpl { &mut this.#the_field }

        as_delegating_raw{
            &mut (*this).#the_field as *mut #fieldty
        }
        IntoFieldImpl { this.#the_field }
    })
    .piped(Ok)
}

fn deriving_structural<'a>(
    ds: &'a DataStructure<'a>,
    options: &'a StructuralOptions<'a>,
) -> Result<TokenStream2, syn::Error> {
    let StructuralOptions {
        fields: config_fields,
        with_trait_alias,
        ..
    } = options;

    let struct_ = &ds.variants[0];

    let mut names_module = NamedModuleAndTokens::new(ds.name);

    let tyname = ds.name;

    let struct_or_enum = match ds.data_variant {
        DataVariant::Struct => StructOrEnum::Struct,
        DataVariant::Enum => StructOrEnum::Enum,
        DataVariant::Union => unreachable!(),
    };

    let make_fields = |names_module: &mut NamedModuleAndTokens, variant: &'a Struct<'a>| {
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

                let alias_index = match struct_or_enum {
                    StructOrEnum::Struct => names_module.push_path(ident),
                    StructOrEnum::Enum => names_module.push_str(ident),
                };

                Some(StructuralField {
                    access: config_f.access,
                    optionality: IsOptional::No,
                    ident,
                    alias_index,
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
            fields: make_fields(&mut names_module, struct_),
            variants: Vec::new(),
        },
        StructOrEnum::Enum => StructuralDataType {
            fields: Vec::new(),
            variants: ds
                .variants
                .iter()
                .map(|variant| StructuralVariant {
                    name: &variant.name,
                    alias_index: names_module.push_str(variant.name.into()),
                    fields: make_fields(&mut names_module, variant),
                })
                .collect(),
        },
    };

    // dbg!(field_names.clone().for_each(|x|{ dbg!(x.to_token_stream().to_string()); }));
    // dbg!(&field_tys);
    // dbg!(alias_names.iter().for_each(|x|{ dbg!(x); }));

    let structural_alias_trait;
    let opt_names_module;

    if *with_trait_alias {
        let docs = format!(
            "A trait aliasing the accessor impls for \
             [{struct_}](./struct.{struct_}.html) fields\n\
             \n\
             This trait also has all the constraints(where clause and generic parametr bounds)
             of [the same struct](./struct.{struct_}.html).\n\n\
             ### Accessor traits\n\
             These are the accessor traits this aliases:\n\
            ",
            struct_ = tyname,
        );
        structural_alias_trait = crate::structural_alias_impl_mod::for_delegation(
            tyname.span(),
            std::iter::empty::<Ident>(),
            docs,
            ds.vis,
            &<Token!(trait)>::default(),
            &Ident::new(&format!("{}_SI", tyname), Span::call_site()),
            ds.generics,
            &Punctuated::new(),
            &names_module,
            &sdt,
        )?
        .piped(Some);
        opt_names_module = None;
    } else {
        structural_alias_trait = None;
        opt_names_module = Some(&names_module);
    };

    let impl_generics = GenParamsIn::new(ds.generics, InWhat::ImplHeader);

    let (_, ty_generics, where_clause) = ds.generics.split_for_impl();

    let empty_preds = Punctuated::new();
    let where_preds = where_clause
        .as_ref()
        .map_or(&empty_preds, |x| &x.predicates)
        .into_iter();

    let structural_alias_trait = structural_alias_trait.into_iter();
    let opt_names_module = opt_names_module.into_iter();

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

            let field_tys = fields.iter().cloned().map(|f| &f.ty);

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
                #(#structural_alias_trait)*

                #(#opt_names_module)*

                ::structural::impl_getters_for_derive_struct!{
                    impl[#impl_generics] #tyname #ty_generics
                    where[ #(#where_preds,)* ]
                    {
                        #((
                            #getter_trait<
                                #field_names : #field_tys ,
                                #names_module_path::#alias_names,
                                #renamed_field_names,
                                opt=false,
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
                                let fty = field.ty;
                                let f_tstr = names_module.alias_name(sdt_field.alias_index);
                                quote!(
                                    #access,
                                    #fname:#fty,
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

            let enum_ = ds.name;
            let proxy_ = syn::Ident::new(&format!("{}_VariantProxy", ds.name), Span::call_site());

            quote!(
                #(#structural_alias_trait)*

                #(#opt_names_module)*

                ::structural::declare_variant_proxy!{
                    #proxy_
                }

                ::structural::impl_getters_for_derive_enum!{
                    impl[#impl_generics] #tyname #ty_generics
                    where[ #(#where_preds,)* ]
                    {
                        enum=#enum_
                        proxy=#proxy_
                        #((#variants))*
                    }
                }
            )
        }
    }
    .piped(Ok)
}
