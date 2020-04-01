use crate::{
    datastructure::StructOrEnum,
    field_access::Access,
    structural_alias_impl_mod::{FieldType, StructuralDataType, StructuralField, VariantIdent},
};

use quote::ToTokens;

use std::fmt::Write;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DocsFor {
    Type,
    Trait,
    VsiTrait,
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
) -> Result<(), syn::Error> {
    if !datatype.variants.is_empty() {
        buff.push_str("### Variants\n\n");
        buff.push_str(match docs_for {
            DocsFor::Type => "This type implements",
            DocsFor::Trait | DocsFor::VsiTrait => "This trait aliases",
        });
        buff.push_str(
            " the `IsVariant<TS!( NameOfVariant )>` trait for each of the variants below.\n\n",
        );
    }

    let type_name = datatype.type_name.filter(|_| docs_for == DocsFor::Type);

    let datatype_fields = match docs_for {
        DocsFor::Type | DocsFor::Trait => &datatype.fields[..],
        DocsFor::VsiTrait => &[],
    };

    let mut has_generic_variant_name = false;

    for variant in &datatype.variants {
        let variant_name_clarification_string;
        let printed_variant_name;
        let variant_name_clarification: &str;
        match variant.name {
            VariantIdent::Generic(generic) => {
                has_generic_variant_name = true;
                printed_variant_name = format!("<{}>", generic);

                variant_name_clarification_string = format!(
                    "{SP}\
                     The name of this variant is determined by the `{}` type parameter\
                     (a `structural::TStr`),<br>",
                    generic,
                    SP = SPACES_X8,
                );
                variant_name_clarification = &variant_name_clarification_string;
            }
            VariantIdent::Ident(generic) => {
                printed_variant_name = generic.to_string();
                variant_name_clarification = "";
            }
        };

        let _ = write!(
            buff,
            "Variant `{}`{} {{<br>{}",
            printed_variant_name,
            try_opt2(type_name, variant.pub_vari_rename).map_or(String::new(), |(t, v)| format!(
                "(named `{}` in `{}`)",
                v, t
            )),
            variant_name_clarification,
        );
        buff.push_str(
            if variant.fields.is_empty() || variant.replace_bounds.is_some() {
                " "
            } else {
                "<br>"
            },
        );

        match (&variant.replace_bounds, docs_for) {
            (Some(replace_bounds), _) => {
                let _ = writeln!(
                    buff,
                    "{SP}This {}variant is represented using these traits:<br>{SP}`{}`<br>",
                    if variant.is_newtype { "newtype " } else { "" },
                    replace_bounds.get_docs(variant.name),
                    SP = SPACES_X8,
                );
            }
            (None, DocsFor::Type) if variant.is_newtype => {
                let _ = writeln!(
                    buff,
                    "{SP}Delegates the field accessor traits of the variant to the `{}` field.",
                    variant.fields[0].ty.to_token_stream(),
                    SP = SPACES_X8,
                );
            }
            (None, _) => {
                let variant_ident = Some(variant.name);
                for field in &variant.fields {
                    let _ = write_field_docs(buff, SPACES_X8, type_name, variant_ident, field);
                }
            }
        }

        let _ = writeln!(buff, "}}\n");
    }

    if !datatype_fields.is_empty() {
        buff.push_str("### Fields\n\n");
    }
    for field in datatype_fields.iter() {
        let _ = write_field_docs(buff, "", type_name, None, field);
    }
    if has_generic_variant_name {
        buff.push_str(GENERIC_VARIANT_NAME_DOCS)
    }
    Ok(())
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
        Some(VariantIdent::Ident(ident)) => format!(
            "TS!({}), TS!({})",
            ident.to_token_stream(),
            field.ident.to_token_stream()
        ),
        Some(VariantIdent::Generic(generic)) => {
            format!("{}, TS!({})", generic, field.ident.to_token_stream())
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
        "{LP}The &nbsp; `{0}: {1}` field {2} &nbsp; ",
        field.ident,
        field_ty,
        try_opt2(type_name, field.pub_field_rename).map_or(String::new(), |(t, f)| format!(
            "(named `{}` in `{}`)",
            f, t
        )),
        LP = left_padding,
    )?;
    match variant {
        Some(VariantIdent::Ident(vari_name)) => {
            write!(buff, " in the `{}` variant", vari_name)?;
        }
        Some(VariantIdent::Generic(generic)) => {
            write!(buff, " in the `{}` variant", generic)?;
        }
        None => {}
    }

    buff.push_str(", with ");
    buff.push_str(access_desc);
    buff.push_str("\n\n");
    Ok(())
}

const SPACES_X8: &str = "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;";

const GENERIC_VARIANT_NAME_DOCS: &str = "
# Generic Variant Names

In the general case,
`<Foo>` as the name of a variant in the generated documentation means that the 
name of the variant is determined by the `Foo` type parameter.<br>
If `TS!(Bar)` is passed as the `Foo` type argument,then the variant is named `Bar`.

";
