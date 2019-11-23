/*!
Contains the Structural trait with info about the fields that have accessor trait impls.
*/


/// Indicates and provides information about the fields that implement accessor traits.
pub trait Structural{
    /// Information about fields that have accessor trait implemented for them.
    const FIELDS:&'static[FieldInfo];
}


/// An object-safe version of the `Structural` trait.
/// with information about the fields that implement accessor traits.
pub trait StructuralDyn{
    /// Information about fields that have accessor trait implemented for them.
    fn fields_info(&self)->&'static[FieldInfo];
}


////////////////////////////////////////////////////////////////////////////////


/// Information about a field with accessor trait impls.
pub struct FieldInfo{
    /// The original name of the field.
    pub original_name:&'static str,
    /// The name used in the accessor trait impls for the field.
    pub accessor_name:&'static str,
}


impl FieldInfo{
    /// Constructs a FieldInfo for a field which uses the same name in its accessor impl.
    pub const fn not_renamed(name:&'static str)->Self{
        Self{
            original_name:name,
            accessor_name:name,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////


/// An iterator over the name parameters for `T`'s accessor trait impls,
///
/// These may be different than the names of the fields because 
/// you can rename their accessor trait name parameter with `#[struc(raname="new_name")]`.
pub fn accessor_names<T>()->impl ExactSizeIterator<Item=&'static str>+Clone
where
    T:Structural
{
    T::FIELDS.iter().map(|f|f.accessor_name)
}


/// The names of `T`'s fields that have accessor trait impls.
pub fn field_names<T>()->impl ExactSizeIterator<Item=&'static str>+Clone
where
    T:Structural
{
    T::FIELDS.iter().map(|f|f.original_name)
}