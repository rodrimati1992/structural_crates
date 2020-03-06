use crate::{
    datastructure::StructOrEnum,
    field_access::{Access, IsOptional},
    structural_alias_impl_mod::{FieldType, StructuralDataType, StructuralField, VariantIdent},
    tokenizers::NamedModuleAndTokens,
};

use quote::ToTokens;

use std::fmt::Write;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DocsFor {
    Type,
    Trait,
}

fn try_opt2<A, B>(a: Option<A>, b: Option<B>) -> Option<(A, B)> {
    match (a, b) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

pub(crate) fn write_datatype_docs(
    buff: &mut String,
    docs_for: DocsFor,
    datatype: &StructuralDataType<'_>,
    names_mod: &NamedModuleAndTokens,
    top_variant_ident: Option<VariantIdent<'_>>,
) {
    if !datatype.variants.is_empty() {
        buff.push_str("### Variants\n\n");
        buff.push_str(match docs_for {
            DocsFor::Type => "This type implements",
            DocsFor::Trait => "This trait aliases",
        });
        buff.push_str(
            " the `IsVariant<TS!( NameOfVariant )>` trait for each of the variants below.\n\n",
        );
    }

    let type_name = datatype.type_name.filter(|_| docs_for == DocsFor::Type);

    for variant in &datatype.variants {
        let alias_ident = names_mod.alias_name(variant.alias_index);
        let variant_ident = Some(VariantIdent::Ident {
            ident: variant.name,
            alias_ident,
        });

        let _ = write!(
            buff,
            "Variant `{}`{} {{",
            variant.name,
            try_opt2(type_name, variant.pub_vari_rename).map_or(String::new(), |(t, v)| format!(
                "(named `{}` in `{}`)",
                v, t
            )),
        );
        buff.push_str(
            if variant.fields.is_empty() || variant.replace_bounds.is_some() {
                " "
            } else {
                "<br>"
            },
        );

        match (&variant.replace_bounds, docs_for) {
            (Some(replace_bounds), DocsFor::Trait) => {
                replace_bounds.write_docs(buff, variant.name);
            }
            _ => {
                for field in &variant.fields {
                    let _ = write_field_docs(buff, SPACES_X8, type_name, variant_ident, field);
                }
            }
        }

        let _ = writeln!(buff, "}}\n");
    }

    if !datatype.fields.is_empty() {
        buff.push_str("### Fields\n\n");
    }
    for field in datatype.fields.iter() {
        let _ = write_field_docs(buff, "", type_name, top_variant_ident, field);
    }
}

fn write_field_docs(
    buff: &mut String,
    left_padding: &str,
    type_name: Option<&syn::Ident>,
    variant: Option<VariantIdent<'_>>,
    field: &StructuralField<'_>,
) -> std::fmt::Result {
    use self::FieldType as FT;

    let soe = StructOrEnum::from(variant);

    let the_trait = field.compute_trait(soe).trait_name();

    let ident = match variant {
        Some(VariantIdent::Ident { ident, .. }) => format!(
            "TS!({}), TS!({})",
            ident.to_token_stream(),
            field.ident.to_token_stream()
        ),
        Some(VariantIdent::StructVariantTrait { .. }) => {
            format!("<__VariantName>, TS!({})", field.ident.to_token_stream())
        }
        None => format!("FP!({})", field.ident.to_token_stream()),
    };

    let access_desc = match field.access {
        Access::Shared => "a shared accessor",
        Access::Mutable => "shared and mutable accessors",
        Access::Value => "shared, and by value accessors",
        Access::MutValue => "shared, mutable, and by value accessors",
    };
    let assoc_ty = match field.ty {
        FT::Ty(ty) => format!("Ty= {}", ty.to_token_stream()),
        FT::Impl(bounds) => format!("Ty: {}", bounds.to_token_stream()),
    };
    let field_ty = match field.ty {
        FT::Ty(ty) => format!("{}", ty.to_token_stream()),
        FT::Impl(bounds) => format!("impl {}", bounds.to_token_stream()),
    };
    writeln!(
        buff,
        "{LP}Bound:`{0}<{1},{2}>`\n<br>",
        the_trait,
        ident,
        assoc_ty,
        LP = left_padding,
    )?;
    writeln!(
        buff,
        "{LP}The &nbsp; `{0}: {1}` {2} {3} &nbsp; ",
        field.ident,
        field_ty,
        match field.inner_optionality {
            IsOptional::Yes => "optional field",
            IsOptional::No => "field",
        },
        try_opt2(type_name, field.pub_field_rename).map_or(String::new(), |(t, f)| format!(
            "(named `{}` in `{}`)",
            f, t
        )),
        LP = left_padding,
    )?;
    match variant {
        Some(VariantIdent::Ident {
            ident: vari_name, ..
        }) => {
            write!(buff, " in the `{}` variant", vari_name)?;
        }
        Some(VariantIdent::StructVariantTrait { .. }) => {
            write!(buff, " in the `[__VariantName]` variant")?;
        }
        None => {}
    }

    buff.push_str(", with ");
    buff.push_str(access_desc);
    buff.push_str("\n\n");
    Ok(())
}

const SPACES_X8: &'static str = "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;";
