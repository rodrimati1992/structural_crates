use crate::{
    tokenizers::NamedModuleAndTokens,
    structural_alias_impl::{FieldTypeRef,StructuralAliasFieldRef},
    ident_or_index::IdentOrIndexRef,
};

use as_derive_utils::{
    datastructure::{DataStructure,DataVariant,Field},
    gen_params_in::{GenParamsIn,InWhat},
    return_syn_err,
};

use core_extensions::SelfOps;

use proc_macro2::{
    TokenStream as TokenStream2,
    Span,
};

use quote::quote;

use syn::{
    punctuated::Punctuated,
    DeriveInput,
    Ident,
    Token,
};


mod attribute_parsing;

#[cfg(test)]
mod tests;

use self::attribute_parsing::StructuralOptions;


#[cfg(test)]
fn derive_from_str(string:&str) -> Result<TokenStream2,syn::Error> {
    syn::parse_str(string).and_then(derive)
}

pub fn derive(data: DeriveInput) -> Result<TokenStream2,syn::Error> {
    let ds = &DataStructure::new(&data);

    match ds.data_variant {
        DataVariant::Enum=>
            return_syn_err!(Span::call_site(),"Cannot derive Structural on an enum"),
        DataVariant::Union=>
            return_syn_err!(Span::call_site(),"Cannot derive Structural on an union"),
        DataVariant::Struct=>{}            
    }

    let options=attribute_parsing::parse_attrs_for_structural(ds)?;
    let debug_print=options.debug_print;

    match options.delegate_to {
        Some(to)=>delegating_structural(ds,options,to),
        None=>deriving_structural(ds,options),
    }?
    .observe(|tokens|{
        if debug_print{
            panic!("\n\n\n{}\n\n\n",tokens);
        }
    })
    .piped(Ok)
    
}    


fn delegating_structural<'a>(
    ds:&'a DataStructure<'a>,
    _options:StructuralOptions<'a>,
    delegate_to:&'a Field<'a>,
)-> Result<TokenStream2,syn::Error> {
    let (_, ty_generics, where_clause) = ds.generics.split_for_impl();

    let impl_generics=GenParamsIn::new(ds.generics,InWhat::ImplHeader);

    let tyname=ds.name;

    let the_field=delegate_to.ident();
    let fieldty=delegate_to.ty;

    let empty_preds=Punctuated::new();
    let where_preds=where_clause.as_ref().map_or(&empty_preds,|x|&x.predicates).into_iter();


    quote!(
        ::structural::z_delegate_structural_with!{
            impl[#impl_generics] #tyname #ty_generics
            where[ #(#where_preds,)* ] 

            self_ident=this;
            delegating_to_type= #fieldty;
            field_name_param=( _field_name : __FieldName );

            GetField { &this.#the_field }

            unsafe GetFieldMut { &mut this.#the_field }

            as_delegating_raw{
                &mut (*this).#the_field as *mut #fieldty 
            }
            IntoField { this.#the_field }
        }
    ).piped(Ok)
}


fn deriving_structural<'a>(
    ds:&'a DataStructure<'a>,
    options:StructuralOptions<'a>,
)-> Result<TokenStream2,syn::Error> {
    let StructuralOptions{
        fields:ref config_fields,
        with_trait_alias,
        ..
    }=options;

    let struct_=&ds.variants[0];

    let fields=config_fields.values()
        .zip( struct_.fields.iter() )
        .filter(|(f_cond,_)| f_cond.is_pub )
        .map(|(_,f)| f )
        .collect::<Vec<&Field<'_>>>();
    
    let config_fields=&config_fields.values().filter(|f| f.is_pub ).collect::<Vec<_>>();

    let renamed_field_names=config_fields.iter()
        .zip( fields.iter().cloned() )
        .map(|(field_conf,field)|{
            match &field_conf.renamed {
                Some(x) => x.to_string(),
                None => field.ident.to_string(),
            }
        })
        .collect::<Vec<_>>();

    let names_module_definition=NamedModuleAndTokens::new(
        ds.name,
        &renamed_field_names
    );
    
    let names_module=&names_module_definition.names_module;
    let alias_names=&names_module_definition.alias_names;
    

    let field_tys=fields.iter().cloned().map(|f| &f.ty );

    let impl_generics=GenParamsIn::new(ds.generics,InWhat::ImplHeader);

    let (_, ty_generics, where_clause) = ds.generics.split_for_impl();

    let tyname=ds.name;

    let empty_preds=Punctuated::new();
    let where_preds=where_clause.as_ref().map_or(&empty_preds,|x|&x.predicates).into_iter();


    let getter_trait=config_fields.iter().map(|f| f.access );

    // dbg!(field_names.clone().for_each(|x|{ dbg!(x.to_token_stream().to_string()); }));
    // dbg!(&field_tys);
    // dbg!(alias_names.iter().for_each(|x|{ dbg!(x); }));

    let docs=format!(
        "A trait aliasing the accessor impls for \
         [{struct_}](./struct.{struct_}.html) fields\n\
         \n\
         This trait also has all the constraints(where clause and generic parametr bounds)
         of [the same struct](./struct.{struct_}.html).\n\n\
         ### Accessor traits\n\
         These are the accessor traits this aliases:\n\
        ",
        struct_=tyname,
    );

    let structural_alias_trait;
    let opt_names_module_definition;

    if with_trait_alias {
        structural_alias_trait=crate::structural_alias_impl::for_delegation(
            tyname.span(),
            std::iter::empty::<Ident>(),
            docs,
            ds.vis,
            &<Token!(trait)>::default(),
            &Ident::new(&format!("{}_SI",tyname),Span::call_site()),
            ds.generics,
            &Punctuated::new(),
            &names_module_definition,
            fields.iter()
                .zip(config_fields.iter())
                .map(|( field, field_config )|{
                    StructuralAliasFieldRef{
                        access:field_config.access,
                        ident:field.ident().piped(IdentOrIndexRef::Ident),
                        ty:match &field_config.is_impl {
                            Some(yes)=>FieldTypeRef::Impl(yes),
                            None=>FieldTypeRef::Ty(&field.ty),
                        },
                    }
                }),
        )?.piped(Some);
        opt_names_module_definition=None;
    }else{
        structural_alias_trait=None;
        opt_names_module_definition=Some(&names_module_definition);
    };

    let structural_alias_trait=structural_alias_trait.into_iter();
    let opt_names_module_definition=opt_names_module_definition.into_iter();


    let field_names=fields.iter().map(|f| &f.ident );
    
    quote!(
        #(#structural_alias_trait)*

        #(#opt_names_module_definition)*

        ::structural::impl_getters_for_derive!{
            impl[#impl_generics] #tyname #ty_generics
            where[ #(#where_preds,)* ] 
            {
                #((
                    #getter_trait< 
                        #field_names : #field_tys , 
                        #names_module::#alias_names,
                        #renamed_field_names,
                    > 
                ))*
            }
        }
    ).piped(Ok)
}
