use crate::{
    tokenizers::names_modules,
};

use as_derive_utils::{
    gen_params_in::{GenParamsIn,InWhat},
};

use core_extensions::SelfOps;

use proc_macro2::{
    TokenStream as TokenStream2,
    Span,
};

use quote::{quote,ToTokens};

use syn::{
    parse::{Parse,ParseStream},
    punctuated::Punctuated,
    Attribute,
    Generics,
    Token,
    token,
    Ident,
    Visibility
};

use std::fmt::{self,Display};


pub(crate) struct StructuralAlias {
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) vis: syn::Visibility,
    pub(crate) trait_token: Token!(trait),
    pub(crate) ident: Ident,
    pub(crate) generics: syn::Generics,
    pub(crate) supertraits: Punctuated<syn::TypeParamBound, token::Add>,
    pub(crate) fields: Vec<StructuralAliasField>,
}


pub(crate) struct StructuralAliasField{
    pub(crate) access: Access,
    pub(crate) ident: IdentOrIndex,
    pub(crate) ty: syn::Type,
}


/// Whether a field can be accessed by reference/mutable-reference/value.
pub(crate) enum Access{
    Shared,
    Mutable,
    Value
}



impl Parse for StructuralAlias {
    fn parse(input: ParseStream) -> Result<Self,syn::Error> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let trait_token: Token![trait] = input.parse()?;
        let ident: Ident = input.parse()?;
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
        let _ = syn::braced!(content in input);
        let mut fields = Vec::<StructuralAliasField>::new();
        while !content.is_empty() {
            fields.push(content.parse()?);
            let _:Result<Token![,],syn::Error>=content.parse();
        }
        
        // let equal:Token![;]= input.parse()?;

        Ok(Self {
            attrs,
            vis,
            trait_token,
            ident,
            generics,
            supertraits,
            fields,
        })
    }
}



impl Parse for StructuralAliasField {
    /// Parses a named (braced struct) field.
    fn parse(input: ParseStream) -> Result<Self,syn::Error> {
        Ok(Self {
            access: input.parse()?,
            ident: input.parse()?,
            ty: input.parse()?,
        })
    }
}


////////////////////////////////////////////////////////////////////////////////

impl Parse for Access {
    fn parse(input: ParseStream) -> Result<Self,syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![ref]) {
            let _:Result<Token![ref],_>=input.parse();
            Ok(Access::Shared)
        } else if lookahead.peek(Token![mut]) {
            let _:Result<Token![mut],_>=input.parse();
            Ok(Access::Mutable)
        } else if lookahead.peek(Token![move]) {
            let _:Result<Token![move],_>=input.parse();
            Ok(Access::Value)
        } else {
            Ok(Access::Shared)
        }
    }
}



////////////////////////////////////////////////////////////////////////////////


pub(crate) enum IdentOrIndex{
    Ident(Ident),
    Index(syn::LitInt),
}


impl Parse for IdentOrIndex{
    fn parse(input: ParseStream) -> Result<Self,syn::Error> {
        let lookahead=input.lookahead1();
        if lookahead.peek(syn::Ident) {
            Ok(IdentOrIndex::Ident(input.parse()?))
        } else if lookahead.peek(syn::LitInt) {
            Ok(IdentOrIndex::Index(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for IdentOrIndex{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            IdentOrIndex::Ident(x) => x.to_tokens(tokens),
            IdentOrIndex::Index(x) => x.to_tokens(tokens),
        }
    }
}

impl Display for IdentOrIndex{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        match self {
            IdentOrIndex::Ident(x) => Display::fmt(x,f),
            IdentOrIndex::Index(x) => Display::fmt(x,f),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn macro_impl(saf:StructuralAlias)->Result<TokenStream2,syn::Error> {
    let StructuralAlias{
        attrs,
        vis,
        trait_token,
        ident,
        generics,
        supertraits,
        fields,
        ..
    }=&saf;

    let names_module=Ident::new(&format!("{}_names_module",ident),Span::call_site());

    let alias_names=fields.iter()
        .map(|f| Ident::new(&format!("STR_{}",f.ident),Span::call_site()) )
        .collect::<Vec<Ident>>();

    let field_name_strs=fields.iter()
        .map(|f| f.ident.to_string() )
        .collect::<Vec<String>>();

    let names_module_definition=alias_names.iter()
        .zip( field_name_strs.iter().map(|s|s.as_str()) )
        .piped(|i| names_modules(&names_module,i) );

    let field_bounds={
        let x=fields.iter()
            .zip(&alias_names)
            .map(|(field,alias_names)|{
                let trait_=match field.access {
                    Access::Shared=>quote!(GetField),
                    Access::Mutable=>quote!(GetFieldMut),
                    Access::Value=>quote!(IntoField),
                };
                let field_ty=&field.ty;
                quote!(
                    structural::#trait_<
                        #names_module::#alias_names,
                        Ty= #field_ty,
                    >
                )
            });

        quote!( #( #x+ )* )
    };

    use std::fmt::Write;
    let mut docs=format!("A trait alias for the following traits:\n");
    
    for field in fields {
        let (the_trait,access_desc)=match field.access {
            Access::Shared=>("GetField","shared"),
            Access::Mutable=>("GetFieldMut","shared and mutable"),
            Access::Value=>("IntoField","shared,mutable and by value"),
        };
        let _=writeln!(
            docs,
            "- `{0}<\"{1}\",{2}>`\n:{3} access to a `{1}:{2}` field.",
            the_trait,
            field.ident,
            field.ty.to_token_stream(),
            access_desc,
        );
    }
    
    if !supertraits.is_empty() {
        for supertrait in supertraits {
            let _=writeln!(
                docs,
                "- `{}`",
                supertrait.to_token_stream(),
            );
        }
    }


    let supertraits_a=supertraits.into_iter();
    let supertraits_b=supertraits.into_iter();

    let impl_generics=GenParamsIn::with_after_types(
        generics,
        InWhat::ImplHeader,
        quote!(__This:?Sized,),
    );

    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let empty_preds=Punctuated::new();
    let where_preds=where_clause.as_ref().map_or(&empty_preds,|x|&x.predicates).into_iter();

    Ok(quote!(
        #names_module_definition

        #(#attrs)*
        #[doc=#docs]
        #vis
        #trait_token #ident #generics : 
            #( #supertraits_a+ )* 
            #field_bounds
        #where_clause
        {}


        impl<#impl_generics> #ident #ty_generics 
        for __This
        where
            __This:
                #( #supertraits_b+ )* 
                #field_bounds,
            #(#where_preds,)*
        {}
    ))

}