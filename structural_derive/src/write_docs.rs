use crate::{
    datastructure::StructOrEnum,
    field_access::Access,
    structural_alias_impl_mod::{FieldType, IdentType, StructuralDataType, StructuralField},
    utils::StrExt,
};

use quote::ToTokens;

use std::fmt::Write;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DocsFor {
    Type,
    Trait,
    VsiTrait,
}

struct FieldsState {
    has_generic_field_name: bool,
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

    let fields_state = &mut FieldsState {
        has_generic_field_name: false,
    };

    for variant in &datatype.variants {
        let variant_name_clarification_string;
        let printed_variant_name;
        let variant_name_clarification: &str;

        match variant.name {
            IdentType::Generic { .. } | IdentType::SomeType { .. } => {
                has_generic_variant_name = true;
                let (the_tstr_type, type_origin) = match variant.name {
                    IdentType::Generic(generic) => (generic.to_string(), " parameter"),
                    IdentType::SomeType(ty) => (ty.to_token_stream().to_string(), ""),
                    _ => unreachable!(),
                };
                printed_variant_name = the_tstr_type.surrounded_with("<", ">");

                variant_name_clarification_string = format!(
                    "{SP}\
                     The name of this variant is determined by the `{}` type{}\
                     (a `TStr`),<br>",
                    the_tstr_type,
                    type_origin,
                    SP = SPACES_X8,
                );
                variant_name_clarification = &variant_name_clarification_string;
            }
            IdentType::Ident(generic) => {
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
                v.display(),
                t
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
                    let _ = write_field_docs(
                        buff,
                        fields_state,
                        SPACES_X8,
                        type_name,
                        variant_ident,
                        field,
                    );
                }
            }
        }

        let _ = writeln!(buff, "}}\n");
    }

    if !datatype_fields.is_empty() {
        buff.push_str("### Fields\n\n");
    }
    for field in datatype_fields.iter() {
        let _ = write_field_docs(buff, fields_state, "", type_name, None, field);
    }
    if has_generic_variant_name || fields_state.has_generic_field_name {
        buff.push_str(if datatype.variants.is_empty() {
            GENERIC_STRUCT_NAME_DOCS
        } else {
            GENERIC_ENUM_NAME_DOCS
        })
    }
    Ok(())
}

fn write_field_docs(
    buff: &mut String,
    fields_state: &mut FieldsState,
    left_padding: &str,
    type_name: Option<&syn::Ident>,
    variant: Option<IdentType<'_>>,
    field: &StructuralField<'_>,
) -> std::fmt::Result {
    use self::FieldType as FT;

    let soe = StructOrEnum::from(variant);

    let the_trait = field.compute_trait(soe).trait_name();

    // This intentionally does NOT have a default case (else,or `_=>{}`)
    let is_generic_field_name = match field.ident {
        IdentType::Generic { .. } | IdentType::SomeType { .. } => true,
        IdentType::Ident { .. } => false,
    };
    fields_state.has_generic_field_name |= is_generic_field_name;

    let field_ident_tstr = match field.ident {
        IdentType::Ident(ident) => format!("FP!({})", ident.to_token_stream()),
        IdentType::Generic(generic) => generic.to_string(),
        IdentType::SomeType(ty) => format!("{}", ty.to_token_stream()),
    };

    let (field_ident_in_desc, type_origin) = match field.ident {
        IdentType::Ident(ident) => (ident.to_string(), ""),
        IdentType::Generic(generic) => (format!("<{}>", generic), " parameter"),
        IdentType::SomeType(ty) => (format!("<{}>", ty.to_token_stream()), ""),
    };

    let path_param = match variant {
        Some(IdentType::Ident(ident)) => {
            format!("TS!({}), {}", ident.to_token_stream(), field_ident_tstr,)
        }
        Some(IdentType::Generic(generic)) => format!("{}, {}", generic, field_ident_tstr),
        Some(IdentType::SomeType(ty)) => format!("{}, {}", ty.to_token_stream(), field_ident_tstr),
        None => field_ident_tstr.clone(),
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
        path_param,
        assoc_ty,
        LP = left_padding,
    )?;
    writeln!(
        buff,
        "{LP}The &nbsp; `{0}: {1}` field {2} &nbsp; ",
        field_ident_in_desc,
        field_ty,
        try_opt2(type_name, field.pub_field_rename).map_or(String::new(), |(t, f)| format!(
            "(named `{}` in `{}`)",
            f.display(),
            t
        )),
        LP = left_padding,
    )?;
    match variant {
        Some(IdentType::Ident(vari_name)) => {
            write!(buff, " in the `{}` variant", vari_name.display())?;
        }
        Some(IdentType::Generic(generic)) => {
            write!(buff, " in the `<{}>` variant", generic)?;
        }
        Some(IdentType::SomeType(ty)) => {
            write!(buff, " in the `<{}>` variant", ty.to_token_stream())?;
        }
        None => {}
    }

    buff.push_str(", with ");
    buff.push_str(access_desc);
    if is_generic_field_name {
        write!(
            buff,
            "<br>{LP}The `{}` type {} (a `TStr`) determines the field name.",
            field_ident_tstr,
            type_origin,
            LP = left_padding,
        )?;
    }
    buff.push_str("\n\n");
    Ok(())
}

const SPACES_X8: &str = "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;";

const GENERIC_ENUM_NAME_DOCS: &str = "
# Generic Names

In the general case,
`<Foo>` as the name of a variant/field in the generated documentation means that the 
name of the variant/field is determined by the `Foo` type.<br>
If `Foo` is the `TS!(Bar)` type,then the variant/field is named `Bar`.

";

const GENERIC_STRUCT_NAME_DOCS: &str = "
# Generic Field Names

In the general case,
`<Foo>` as the name of a field in the generated documentation means that the 
name of the field is determined by the `Foo` type.<br>
If `Foo` is the `TS!(Bar)` type,then the field is named `Bar`.

";
