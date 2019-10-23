use crate::{
    tokenizers::NamedModuleAndTokens,
};

use as_derive_utils::{
    datastructure::{DataStructure,DataVariant,Field},
    gen_params_in::{GenParamsIn,InWhat},
    return_syn_err,
    ToTokenFnMut,
};

use core_extensions::SelfOps;

use proc_macro2::{
    TokenStream as TokenStream2,
    Span,
};

use quote::{quote,ToTokens};

use syn::{
    punctuated::Punctuated,
    DeriveInput,
};


mod attribute_parsing;


pub fn derive(data: DeriveInput) -> Result<TokenStream2,syn::Error> {
    let ds = &DataStructure::new(&data);

    match ds.data_variant {
        DataVariant::Enum=>
            return_syn_err!(Span::call_site(),"Cannot derive Structural on an enum"),
        DataVariant::Union=>
            return_syn_err!(Span::call_site(),"Cannot derive Structural on an union"),
        DataVariant::Struct=>{}            
    }

    let config=&attribute_parsing::parse_attrs_for_structural(ds)?;
    
    let struct_=&ds.variants[0];

    let fields=struct_.fields.iter()
        .filter(|f| f.is_public() )
        .collect::<Vec<&Field<'_>>>();
    
    let names_module_definition=NamedModuleAndTokens::new(
        ds.name,
        fields.iter().cloned().map(|f| &f.ident )
    );
    
    let names_module=&names_module_definition.names_module;
    let alias_names=&names_module_definition.alias_names;
    
    let field_names=config.renamed_fields.iter()
        .zip( fields.iter().cloned() )
        .map(|((_,rename),field)|{
            ToTokenFnMut::new(move|ts|{
                match rename {
                    Some(x) => x.to_tokens(ts),
                    None => field.ident().to_tokens(ts),
                }
            })
        });

    let field_tys=fields.iter().cloned().map(|f| &f.ty );

    let impl_generics=GenParamsIn::with_after_types(
        ds.generics,
        InWhat::ImplHeader,
        quote!(),
    );

    let (_, ty_generics, where_clause) = ds.generics.split_for_impl();

    let tyname=ds.name;

    let empty_preds=Punctuated::new();
    let where_preds=where_clause.as_ref().map_or(&empty_preds,|x|&x.predicates).into_iter();

    let getter_trait=config.field_access.values();

    // dbg!(field_names.clone().for_each(|x|{ dbg!(x.to_token_stream().to_string()); }));
    // dbg!(&field_tys);
    // dbg!(alias_names.iter().for_each(|x|{ dbg!(x); }));

    quote!(
        #names_module_definition

        impl_getters_for_derive!{
            impl[#impl_generics] #tyname #ty_generics
            where[ #(#where_preds)* ] 
            {
                #((
                    #getter_trait< 
                        #field_names : #field_tys , 
                        #names_module::#alias_names> 
                ))*
            }
        }
    )
    .observe(|tokens|{
        if config.debug_print{
            panic!("\n\n\n{}\n\n\n",tokens);
        }
    })
    .piped(Ok)
}
