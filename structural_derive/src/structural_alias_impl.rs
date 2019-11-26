use crate::{
    field_access::Access,
    tokenizers::NamedModuleAndTokens,
    ident_or_index::{IdentOrIndex,IdentOrIndexRef},
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

use quote::{quote,quote_spanned,ToTokens,TokenStreamExt};

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



////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests;


////////////////////////////////////////////////////////////////////////////////

pub(crate) struct StructuralAliases{
    pub(crate) list: Vec<StructuralAlias>,
}

pub(crate) struct StructuralAlias {
    pub(crate) span:Span,
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
    pub(crate) ty: FieldType,
}

pub(crate) struct StructuralAliasFieldRef<'a>{
    pub(crate) access: Access,
    pub(crate) ident: IdentOrIndexRef<'a>,
    pub(crate) ty: FieldTypeRef<'a>,
}

pub(crate) enum FieldType{
    Ty(syn::Type),
    Impl(TypeParamBounds)
}

pub(crate) enum FieldTypeRef<'a>{
    Ty(&'a syn::Type),
    Impl(&'a TypeParamBounds)
}

pub(crate) type TypeParamBounds=
    Punctuated<syn::TypeParamBound, token::Add>;


impl Parse for StructuralAliases {
    fn parse(input: ParseStream) -> Result<Self,syn::Error> {
        let mut list=Vec::<StructuralAlias>::new();
        while !input.is_empty() {
            list.push(input.parse()?);
        }
        Ok(Self{list})
    }
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
        let braces = syn::braced!(content in input);
        let mut fields = Vec::<StructuralAliasField>::new();
        while !content.is_empty() {
            fields.push(content.parse()?);
            let _:Result<Token![,],syn::Error>=content.parse();
        }
        
        // let equal:Token![;]= input.parse()?;

        let span:Span=trait_token.span.join(braces.span).unwrap_or(trait_token.span);

        Ok(Self {
            span,
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
        let access= input.parse::<Access>()?;
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
            ty: self.ty.borrowed(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////


impl Parse for FieldType {
    fn parse(input: ParseStream) -> Result<Self,syn::Error> {
        const ASSOC_TY_BOUNDS:bool=cfg!(feature="impl_fields");

        use syn::Type;
        
        match input.parse::<syn::Type>()? {
            Type::ImplTrait(x)=>
                if ASSOC_TY_BOUNDS {
                    Ok(FieldType::Impl(x.bounds))
                }else{
                    use syn::spanned::Spanned;
                    Err(syn::Error::new(
                        x.span(),
                        "\
                            Cannot use an `impl Trait` field without enabling the \
                            \"nightly_impl_fields\" or \"impl_fields\" feature.\
                        ",
                    ))
                },
            x=>Ok(FieldType::Ty(x)),
        }
    }
}

impl FieldType{
    fn borrowed(&self)->FieldTypeRef<'_>{
        match self {
            FieldType::Ty(x)=>FieldTypeRef::Ty(x),
            FieldType::Impl(x)=>FieldTypeRef::Impl(x),
        }
    }
}


////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub(crate) fn derive_from_str(saf:&str)->Result<TokenStream2,syn::Error> {
    syn::parse_str(saf).and_then(macro_impl)
}



pub(crate) fn macro_impl(aliases:StructuralAliases)->Result<TokenStream2,syn::Error> {

    let list=aliases.list;
    
    if list.is_empty() {
        return Ok(quote!());
    }

    
    let mut out=TokenStream2::new();
    for saf in list {
        let names_module_definition=NamedModuleAndTokens::new(
            &saf.ident,
            saf.fields.iter().map(|f| &f.ident ),
        );

        for_delegation(
            saf.span,
            &saf.attrs,
            format!("A trait alias generated by `structural::structural_alias`,for the following traits:\n"),
            &saf.vis,
            &saf.trait_token,
            &saf.ident,
            &saf.generics,
            &saf.supertraits,
            &names_module_definition,
            saf.fields.iter().map(StructuralAliasField::borrowed)
        )?.piped(|x|out.append_all(x));
    }
    // panic!("{}", out);
    Ok(out)
}

/// This allows both `structural_alias` and `#[derive(Structural)]` to generate
/// the trait alias and its impl.
pub(crate) fn for_delegation<'a,A,I>(
    span:Span,
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
    use self::FieldTypeRef as FTR;

    let attrs=attrs.into_iter();

    let names_module=&names_module_definition.names_module;
    let alias_names=&names_module_definition.alias_names;

    let field_bounds={
        let x=fields.clone().into_iter()
            .zip(alias_names)
            .map(|(field,alias_names)|{
                let trait_=field.access;
                let assoc_ty=match field.ty {
                    FTR::Ty(ty)=>quote!(Ty=#ty),
                    FTR::Impl(bounds)=>quote!(Ty:#bounds),
                };
                quote!(
                    structural::#trait_<
                        #names_module::#alias_names,
                        #assoc_ty
                    >
                )
            });

        quote!( #( #x+ )* )
    };
    
    let assoc_ty_bounds={
        let x=fields.clone().into_iter()
            .zip(alias_names)
            .filter_map(|(field,alias_names)|{
                let bounds=match field.ty {
                    FTR::Ty(_)=>return None,
                    FTR::Impl(bounds)=>bounds,
                };
                Some(quote!(
                    ::structural::GetFieldType<Self,#names_module::#alias_names>:
                        #bounds,
                ))
            });
        
        quote!( #( #x )* )
    };

    use std::fmt::Write;

    let _=writeln!(docs,);
    
    for field in fields.into_iter() {
        let (the_trait,access_desc)=match field.access {
            Access::Shared=>("GetField","shared"),
            Access::Mutable=>("GetFieldMut","shared and mutable"),
            Access::Value=>("IntoField","shared, and by value"),
            Access::MutValue=>("IntoFieldMut","shared,mutable and by value"),
        };
        let assoc_ty=match field.ty {
            FTR::Ty(ty)=>format!("Ty= {}",ty.to_token_stream()),
            FTR::Impl(bounds)=>format!("Ty: {}",bounds.to_token_stream()),
        };
        let field_ty=match field.ty {
            FTR::Ty(ty)=>format!("{}",ty.to_token_stream()),
            FTR::Impl(bounds)=>format!("impl {}",bounds.to_token_stream()),
        };
        let _=writeln!(
            docs,
            "`{0}<FP!( {1} ),{2}>`\n<br>\
             The &nbsp; `{1}: {3}` &nbsp; field,with {4} access.
            \n",
            the_trait,
            field.ident,
            assoc_ty,
            field_ty,
            access_desc,
        );
    }
    
    if !supertraits.is_empty() {
        for supertrait in supertraits {
            let _=writeln!(
                docs,
                "`{}`",
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
    let where_preds=where_clause.as_ref().map_or(&empty_preds,|x|&x.predicates);
    let where_preds_a=where_preds.into_iter();
    let where_preds_b=where_preds.into_iter();

    Ok(quote_spanned!(span=>
        #names_module_definition

        #[doc=#docs]
        #(#attrs)*
        #vis
        #trait_token #ident #generics : 
            #( #supertraits_a+ )* 
            #field_bounds
        where
            #(#where_preds_a,)*
            #assoc_ty_bounds
        {}


        impl<#impl_generics> #ident #ty_generics 
        for __This
        where
            __This:
                #( #supertraits_b+ )* 
                #field_bounds,
            #assoc_ty_bounds
            #(#where_preds_b,)*
        {}
    ))

}
