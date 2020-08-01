use crate::tokenizers::tstr_tokens;

use super::attribute_parsing::StructuralOptions;

use as_derive_utils::{
    datastructure::{DataStructure, DataVariant, Field},
    gen_params_in::{GenParamsIn, InWhat},
    spanned_err,
    utils::{LinearResult, SynResultExt},
};

use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens, TokenStreamExt};

use syn::{punctuated::Punctuated, WherePredicate};

#[derive(Debug, Clone)]
pub(crate) struct FromStructuralConfig {
    pub(crate) bounds: Punctuated<WherePredicate, syn::Token!(,)>,
}

#[derive(Debug)]
pub(crate) struct FieldsInit<'a> {
    /// Fields constructed fields from another type.
    pub(crate) from: Vec<FieldFrom<'a>>,
    pub(crate) init: Vec<FieldAndInit<'a>>,
}

/// A field constructed with a field from another type.
#[derive(Debug, Clone)]
pub(crate) struct FieldFrom<'a> {
    pub(crate) field: &'a Field<'a>,
    /// The TStr type for the field in the accessor.
    pub(crate) accessor_name: TokenStream2,
}

#[derive(Debug)]
pub(crate) struct FieldAndInit<'a> {
    pub(crate) field: &'a Field<'a>,
    pub(crate) init: &'a InitWith,
}

#[derive(Debug)]
pub(crate) enum InitWith {
    Default,
    Fn(syn::Expr),
    Lit(syn::Lit),
    Val(syn::Expr),
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> ToTokens for FieldFrom<'a> {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        self.field.ident.to_tokens(ts);
        <syn::Token!(:)>::default().to_tokens(ts);
        self.field.ident().to_tokens(ts);
        <syn::Token!(,)>::default().to_tokens(ts);
    }
}

impl ToTokens for InitWith {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        match self {
            InitWith::Default => ts.append_all(quote!(Default::default())),
            InitWith::Fn(expr) => {
                let paren = syn::token::Paren::default();
                paren.surround(ts, |ts| expr.to_tokens(ts));
                paren.surround(ts, |_| ());
            }
            InitWith::Lit(lit) => lit.to_tokens(ts),
            InitWith::Val(val) => val.to_tokens(ts),
        }
    }
}

impl<'a> ToTokens for FieldAndInit<'a> {
    fn to_tokens(&self, ts: &mut TokenStream2) {
        self.field.ident.to_tokens(ts);
        <syn::Token!(:)>::default().to_tokens(ts);
        self.init.to_tokens(ts);
        <syn::Token!(,)>::default().to_tokens(ts);
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn deriving_from_structural<'a>(
    ds: &'a DataStructure<'a>,
    options: &'a StructuralOptions<'a>,
    from_opts: &'a FromStructuralConfig,
) -> Result<TokenStream2, syn::Error> {
    let mut res = LinearResult::ok(());

    let mut init_type_with = Vec::new();
    init_type_with.resize_with(ds.variants.len(), || FieldsInit {
        from: Vec::new(),
        init: Vec::new(),
    });

    for (vari, fields_init) in ds.variants.iter().zip(init_type_with.iter_mut()) {
        for field in &vari.fields {
            let f_options = &options.fields[field];
            match &f_options.init_with {
                None if f_options.is_pub => {
                    let ff = FieldFrom {
                        field,
                        accessor_name: f_options.renamed_ident().tstr_tokens(),
                    };
                    fields_init.from.push(ff);
                }
                None => {
                    const TRAILING_MSG: &str = "\n\
                        Eg: `#[struc(init_with = \"some_function\")]`\n\
                        Eg: `#[struc(init_with_lit = \"initial_value\")]`\n\
                        Eg: `#[struc(init_with_lit = 0)]`\n\
                        Eg: `#[struc(init_with_default)]`\n\
                        \n\
                        This is required by the `#[struc(from_structural)]` attribute.\n\
                    ";

                    let first_err = res.is_ok();
                    res.push_err(spanned_err!(
                        field.ident(),
                        "Private fields must have an explicit initialization attribute \
                         to derive FromStructural.\
                         {}
                        ",
                        if first_err { TRAILING_MSG } else { "" },
                    ))
                }
                Some(init_with) => {
                    let fi = FieldAndInit {
                        field,
                        init: init_with,
                    };
                    fields_init.init.push(fi);
                }
            }
        }
    }

    // Early return after post-processing
    res.take()?;

    let tokens = match ds.data_variant {
        DataVariant::Struct => deriving_from_structural_struct(ds, &init_type_with[0], from_opts),
        DataVariant::Enum => deriving_from_structural_enum(ds, &init_type_with, from_opts),
        DataVariant::Union => unreachable!("unions can't derive FromStructural"),
    };
    Ok(tokens)
}

fn deriving_from_structural_struct<'a>(
    ds: &DataStructure<'a>,
    fields_init: &FieldsInit<'a>,
    from_opts: &'a FromStructuralConfig,
) -> TokenStream2 {
    let from_tys = fields_init.from.iter().map(|f| f.field.ty);

    let from_names_str_a = fields_init.from.iter().map(|ff| &ff.accessor_name);
    let from_names_str_b = from_names_str_a.clone();
    let from_fields = fields_init.from.iter().map(|ff| ff.field.ident());
    let into_fields = fields_init.from.iter();

    let impl_generics =
        GenParamsIn::with_after_types(ds.generics, InWhat::ImplHeader, quote!(__Struc_From,));

    let tyname = ds.name;
    let (_, ty_generics, where_clause) = ds.generics.split_for_impl();

    let empty_preds = Punctuated::new();
    let where_preds = where_clause
        .as_ref()
        .map_or(&empty_preds, |x| &x.predicates)
        .into_iter();

    let init_fields = fields_init.init.iter();

    let extra_bounds = from_opts.bounds.iter();

    quote!(::structural::z_impl_from_structural! {
        impl[#impl_generics] FromStructural<__Struc_From> for #tyname #ty_generics
        where [
            #(#where_preds,)*
            #(__Struc_From: ::structural::IntoField<#from_names_str_a, Ty = #from_tys>,)*
            #(#extra_bounds,)*
        ] {
            fn from_structural(from){
                unsafe{
                    let mut from = ::structural::field::ownership::IntoFieldsWrapper::new(from);

                    let (from, moved_out) = from.inner_and_moved_mut();

                    let (#(#from_fields,)*) = (
                        #(
                            ::structural::IntoField::move_out_field_(
                                from,
                                <#from_names_str_b>::NEW,
                                moved_out,
                            ),
                        )*
                    );

                    Self{
                        #(#into_fields)*
                        #(#init_fields)*
                    }
                }
            }
        }
    })
}

fn deriving_from_structural_enum<'a>(
    ds: &DataStructure<'a>,
    fields_init: &[FieldsInit<'a>],
    from_opts: &'a FromStructuralConfig,
) -> TokenStream2 {
    let variant_str_tokens = ds
        .variants
        .iter()
        .map(|v| tstr_tokens(v.name.to_string(), v.name.span()))
        .collect::<Vec<_>>();

    let variant_str_tokens_a = variant_str_tokens.iter();

    let into_field_bounds = fields_init.iter().flat_map(|fi| fi.from.iter()).map(|ff| {
        let field = ff.field;
        let accessor_name = &ff.accessor_name;
        let variant_name_str = &variant_str_tokens[field.index.variant];
        let fty = field.ty;
        quote!(
            ::structural::IntoVariantField<#variant_name_str, #accessor_name, Ty = #fty>
        )
    });

    let variant_strs = variant_str_tokens.iter();

    let impl_generics =
        GenParamsIn::with_after_types(ds.generics, InWhat::ImplHeader, quote!(__Struc_From,));

    let tyname = ds.name;
    let (_, ty_generics, where_clause) = ds.generics.split_for_impl();

    let empty_preds = Punctuated::new();
    let where_preds = where_clause
        .as_ref()
        .map_or(&empty_preds, |x| &x.predicates)
        .into_iter();

    let variant_constructors = fields_init.iter().enumerate().map(|(index, fi)| {
        let variant_str = &variant_str_tokens[index];
        let variant_name = &ds.variants[index].name;
        let from_names_str = fi.from.iter().map(|ff| &ff.accessor_name);
        let from_fields = fi.from.iter().map(|ff| ff.field.ident());
        let into_fields = fi.from.iter();
        let init_fields = fi.init.iter();

        quote!({
            let mut from = ::structural::field::ownership::IntoFieldsWrapper::new(from);

            let (from, moved_out) = from.inner_and_moved_mut();

            let (#(#from_fields,)*) = (
                #(
                    ::structural::IntoVariantField::move_out_vfield_unchecked_(
                        from,
                        <#variant_str>::NEW,
                        <#from_names_str>::NEW,
                        moved_out,
                    ),
                )*
            );

            Self::#variant_name{
                #(#into_fields)*
                #(#init_fields)*
            }
        })
    });

    let extra_bounds = from_opts.bounds.iter();

    quote!(::structural::z_impl_try_from_structural_for_enum! {
        impl[#impl_generics] TryFromStructural<__Struc_From> for #tyname #ty_generics
        where[
            #(#where_preds,)*
            #(#extra_bounds,)*
            #(__Struc_From: #into_field_bounds,)*
            #(__Struc_From: ::structural::enums::IsVariant<#variant_strs>,)*
        ]{
            type Error = ::structural::convert::EmptyTryFromError;

            fn try_from_structural(from) {
                let ret = unsafe{
                    #(
                        if ({
                            let vp = <#variant_str_tokens_a>::NEW;
                            ::structural::pmr::IsVariant::is_variant_(&from, vp)
                        })
                            #variant_constructors
                        else
                    )*
                    {
                        use ::structural::convert::TryFromError;
                        return Err(TryFromError::with_empty_error(from));
                    }
                };
                Ok(ret)
            }
        }

        // `Variants_ESI` is like `Variants_SI` with the additional requirement that `F`
        // only has the `Foo`,`Bar`,and `Baz` variants.
        FromStructural
        where[]
    })
}
