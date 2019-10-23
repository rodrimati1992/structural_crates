use crate::{
    tokenizers::NamedModuleAndTokens,
};

use as_derive_utils::{
    gen_params_in::{GenParamsIn,InWhat},
};

#[allow(unused_imports)]
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

pub(crate) struct StructuralAliasFieldRef<'a>{
    pub(crate) access: Access,
    pub(crate) ident: IdentOrIndexRef<'a>,
    pub(crate) ty: &'a syn::Type,
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
        let access= input.parse()?;
        let ident= input.parse()?;
        let _:Token![:]= input.parse()?;
        let ty= input.parse()?;
        Ok(Self {
            access,
            ident,
            ty,
        })
    }
}

impl StructuralAliasField{
    fn borrowed(&self)->StructuralAliasFieldRef<'_>{
        StructuralAliasFieldRef{
            access: self.access,
            ident: self.ident.borrowed(),
            ty: &self.ty,
        }
    }
}


////////////////////////////////////////////////////////////////////////////////


/// Whether a field can be accessed by reference/mutable-reference/value.
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub(crate) enum Access{
    Shared,
    Mutable,
    Value
}

impl Default for Access{
    fn default()->Self{
        Access::Shared
    }
}


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


impl ToTokens for Access{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match *self {
            Access::Shared=>Ident::new("GetField",Span::call_site()),
            Access::Mutable=>Ident::new("GetFieldMut",Span::call_site()),
            Access::Value=>Ident::new("IntoField",Span::call_site()),
        }.to_tokens(tokens);
    }
}


////////////////////////////////////////////////////////////////////////////////


pub(crate) enum IdentOrIndex{
    Ident(Ident),
    Index(syn::LitInt),
}

impl IdentOrIndex{
    pub(crate) fn borrowed(&self)->IdentOrIndexRef<'_>{
        match self {
            IdentOrIndex::Ident(x) => IdentOrIndexRef::Ident(x),
            IdentOrIndex::Index(x) => IdentOrIndexRef::Index(x),
        }
    }
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


pub(crate) enum IdentOrIndexRef<'a>{
    Ident(&'a Ident),
    Index(&'a syn::LitInt),
}

impl ToTokens for IdentOrIndexRef<'_>{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            IdentOrIndexRef::Ident(x) => x.to_tokens(tokens),
            IdentOrIndexRef::Index(x) => x.to_tokens(tokens),
        }
    }
}

impl Display for IdentOrIndexRef<'_>{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        match self {
            IdentOrIndexRef::Ident(x) => Display::fmt(x,f),
            IdentOrIndexRef::Index(x) => Display::fmt(x,f),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn macro_impl(saf:StructuralAlias)->Result<TokenStream2,syn::Error> {
    let names_module_definition=NamedModuleAndTokens::new(
        &saf.ident,
        saf.fields.iter().map(|f| &f.ident )
    );
    
    for_delegation(
        &saf.attrs,
        format!("A trait alias for the following traits:\n"),
        &saf.vis,
        &saf.trait_token,
        &saf.ident,
        &saf.generics,
        &saf.supertraits,
        &names_module_definition,
        saf.fields.iter().map(StructuralAliasField::borrowed)
    )

}

/// This allows both `structural_alias` and `#[derive(Structural)]` to generate
/// the trait alias and its impl.
pub(crate) fn for_delegation<'a,A,I>(
    attrs: A,
    mut docs:String,
    vis: &syn::Visibility,
    trait_token: &Token!(trait),
    ident: &Ident,
    generics: &syn::Generics,
    supertraits: &Punctuated<syn::TypeParamBound, token::Add>,
    names_module_definition:&NamedModuleAndTokens,
    fields:I,
)->Result<TokenStream2,syn::Error> 
where
    A:IntoIterator,
    A::Item:ToTokens,
    I:IntoIterator<Item=StructuralAliasFieldRef<'a>>+Clone,
{
    let attrs=attrs.into_iter();

    let names_module=&names_module_definition.names_module;
    let alias_names=&names_module_definition.alias_names;

    let field_bounds={
        let x=fields.clone().into_iter()
            .zip(alias_names)
            .map(|(field,alias_names)|{
                let trait_=field.access;
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

    let _=writeln!(docs,);
    
    for field in fields.into_iter() {
        let (the_trait,access_desc)=match field.access {
            Access::Shared=>("GetField","shared"),
            Access::Mutable=>("GetFieldMut","shared and mutable"),
            Access::Value=>("IntoField","shared,mutable and by value"),
        };
        let _=writeln!(
            docs,
            "- `{0}<\"{1}\",Ty={2}>`\n:{3} access to a `{1}:{2}` field.",
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
