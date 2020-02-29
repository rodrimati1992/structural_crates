/*!
Contains the Structural trait with info about the fields that have accessor trait impls.
*/

/// Provides information about the accessor trait impls for the type.
pub trait Structural {
    /// Information about fields/variants that have accessor trait implemented for them.
    const FIELDS: &'static FieldInfos;
}

////////////////////////////////////////////////////////////////////////////////

/// Information about the accessor traits implemented by a type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FieldInfos {
    /// This is an `Option<T>`.
    Option(&'static FieldInfos),
    /// This is an enum,with these variants.
    Enum(&'static [VariantInfo]),
    /// This is a struct,with these fields.
    Struct(&'static [FieldInfo]),
}

////////////////////////////////////////////////////////////////////////////////

/// Information about a field with accessor trait impls.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VariantInfo {
    /// The name of the variant,both the original and in the accessor impls.
    pub name: Name,
}

impl VariantInfo {
    /// Constructs a VariantInfo for a variant which uses the same name in its accessor impl.
    pub const fn not_renamed(name: &'static str) -> Self {
        Self {
            name: Name::not_renamed(name),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Information about a field with accessor trait impls.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FieldInfo {
    /// The name of the field,both the original and in the accessor impls.
    pub name: Name,
    ///
    /// The `Structural` derive macro does no special handling of `Option` fields,
    /// so this will be `false` for those fields.
    pub optionality: IsOptional,
}

impl FieldInfo {
    /// Constructs a FieldInfo for a field which uses the same name in its accessor impl.
    pub const fn not_renamed(name: &'static str) -> Self {
        Self {
            name: Name::not_renamed(name),
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
pub struct Name {
    /// The original name of the thing.
    pub original: &'static str,
    /// The name used in the accessor trait impls.
    pub accessor: &'static str,
}

impl Name {
    /// Constructs a Name which is the same in the accessor impl.
    pub const fn not_renamed(name: &'static str) -> Self {
        Self {
            original: name,
            accessor: name,
        }
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

#[cfg(feature = "alloc")]
pub fn names<T>() -> crate::alloc::vec::Vec<Name>
where
    T: Structural,
{
    names_inner(T::FIELDS)
}

#[cfg(feature = "alloc")]
fn names_inner(infos: &FieldInfos) -> crate::alloc::vec::Vec<Name> {
    match infos {
        FieldInfos::Option(x) => names_inner(x),
        FieldInfos::Enum(x) => x.iter().map(|x| x.name).collect(),
        FieldInfos::Struct(x) => x.iter().map(|x| x.name).collect(),
    }
}

#[cfg(feature = "alloc")]
pub fn accessor_names<T>() -> impl ExactSizeIterator<Item = &'static str> + Clone
where
    T: Structural,
{
    names::<T>().into_iter().map(|f| f.accessor)
}

#[cfg(feature = "alloc")]
pub fn field_names<T>() -> impl ExactSizeIterator<Item = &'static str> + Clone
where
    T: Structural,
{
    names::<T>().into_iter().map(|f| f.original)
}
