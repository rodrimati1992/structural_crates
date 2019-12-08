/*!
Contains the Structural trait with info about the fields that have accessor trait impls.
*/

/// Indicates and provides information about the fields that implement accessor traits.
pub trait Structural {
    /// Information about fields that have accessor trait implemented for them.
    const FIELDS: &'static [FieldInfo];
}

/// An object-safe version of the `Structural` trait.
/// with information about the fields that implement accessor traits.
pub trait StructuralDyn {
    /// Information about fields that have accessor trait implemented for them.
    fn fields_info(&self) -> &'static [FieldInfo];
}

////////////////////////////////////////////////////////////////////////////////

/// Information about a field with accessor trait impls.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FieldInfo {
    /// The original name of the field.
    pub original_name: &'static str,
    /// The name used in the accessor trait impls for the field.
    pub accessor_name: &'static str,
    /// Whether the field is optional.
    ///
    /// The `Structural` derive macro does no special handling of `Option` fields,
    /// so this will be `false` for those fields.
    pub optionality: IsOptional,
}

impl FieldInfo {
    /// Constructs a FieldInfo for a field which uses the same name in its accessor impl.
    pub const fn not_renamed(name: &'static str) -> Self {
        Self {
            original_name: name,
            accessor_name: name,
            optionality: IsOptional::No,
        }
    }
    /// Sets the value of the `optionality` field,returning the mutated `self` back.
    ///
    /// This is intended to be used in method chains.
    pub const fn set_optionality(mut self, value: IsOptional) -> Self {
        self.optionality = value;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IsOptional {
    No,
    Yes,
}

impl IsOptional {
    pub const fn new(v: bool) -> IsOptional {
        [IsOptional::No, IsOptional::Yes][v as usize]
    }
}

////////////////////////////////////////////////////////////////////////////////

/// An iterator over the name parameters for `T`'s accessor trait impls,
///
/// These may be different than the names of the fields because
/// you can rename their accessor trait name parameter with `#[struc(raname="new_name")]`.
pub fn accessor_names<T>() -> impl ExactSizeIterator<Item = &'static str> + Clone
where
    T: Structural,
{
    T::FIELDS.iter().map(|f| f.accessor_name)
}

/// The names of `T`'s fields that have accessor trait impls.
pub fn field_names<T>() -> impl ExactSizeIterator<Item = &'static str> + Clone
where
    T: Structural,
{
    T::FIELDS.iter().map(|f| f.original_name)
}
