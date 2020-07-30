use crate::{
    field_access::Access,
    ident_or_index::{IdentOrIndex, IdentOrIndexRef},
    structural_alias_impl_mod::{ReplaceBounds, TypeParamBounds},
};

use super::from_structural::InitWith;

use as_derive_utils::datastructure::Field;

#[derive(Debug, Clone)]
pub(crate) struct NewtypeConfig {
    pub(crate) replace_bounds: Option<ReplaceBounds>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct VariantConfig {
    pub(crate) renamed: Option<IdentOrIndex>,
    pub(crate) is_newtype: bool,
    pub(crate) replace_bounds: Option<ReplaceBounds>,
}

#[derive(Debug)]
pub(crate) struct FieldConfig<'a> {
    pub(crate) field: &'a Field<'a>,
    pub(crate) access: Access,
    pub(crate) renamed: Option<IdentOrIndex>,
    /// Whether the type is replaced with bounds in the `<deriving_type>_SI` trait.
    pub(crate) is_impl: Option<TypeParamBounds>,

    /// Determines whether the field is considered public.
    ///
    /// `false`: means that the field does not get an accessor.
    /// `true`: means that the field gets an accessor.
    pub(crate) is_pub: bool,

    /// How to initialize the field in the FromStructural implementation.
    pub(crate) init_with: Option<InitWith>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DropParams {
    pub(crate) pre_post_drop_fields: bool,
    pub(crate) pre_move: Option<syn::Path>,
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> FieldConfig<'a> {
    pub(crate) fn renamed_ident(&self) -> IdentOrIndexRef<'_> {
        match &self.renamed {
            Some(x) => x.borrowed(),
            None => IdentOrIndexRef::from(&self.field.ident),
        }
    }
}
