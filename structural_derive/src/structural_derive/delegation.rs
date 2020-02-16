use as_derive_utils::datastructure::Field;

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens};

use syn::{punctuated::Punctuated, NestedMeta, WherePredicate};

#[derive(Debug, Clone)]
pub(crate) struct DelegateTo<'a> {
    pub(crate) field: &'a Field<'a>,
    pub(crate) delegation_params: RawMutImplParam,
    pub(crate) bounds: Vec<WherePredicate>,
    pub(crate) mut_bounds: Vec<WherePredicate>,
    pub(crate) move_bounds: Vec<WherePredicate>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum RawMutImplParam {
    Sized_,
    Unsized,
    SpecializeCfg(Punctuated<NestedMeta, syn::Token!(,)>),
    Unspecified,
}

impl ToTokens for RawMutImplParam {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        match self {
            RawMutImplParam::Sized_ => quote!(raw_mut_impl(Sized)),
            RawMutImplParam::Unsized => quote!(raw_mut_impl(?Sized)),
            RawMutImplParam::SpecializeCfg(list) => quote!( raw_mut_impl( specialize_cfg(#list) ) ),
            RawMutImplParam::Unspecified => {
                return;
            }
        }
        .to_tokens(ts);
        <syn::Token!(;)>::default().to_tokens(ts);
    }
}
