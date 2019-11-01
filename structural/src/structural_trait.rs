/*!
Contains the Structural trait with info about the fields that have accessor trait impls.
*/

use std_::marker::PhantomData;



/// Indicates and provides information about the fields that implement accessor traits.
pub trait Structural{
    /// Information about fields that have accessor trait implemented for them.
    const FIELDS:&'static[FieldInfo];

    /// A type-level list of field name,field type pairs.
    ///
    /// Those lists can be manually constructed like this:
    ///
    /// ```
    /// use structural::{
    ///     TList,TStr,
    ///     structural_trait::TField,
    /// };
    /// 
    /// type TheList=TList![
    ///     TField<TStr!(f o o),String>,
    ///     TField<TStr!(b a r),Vec<u8>>,
    /// ];
    /// 
    /// ```
    type Fields;
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


/// A type-level value representing a field's name and type.
pub struct TField<Name,Ty>(PhantomData<(Name,Ty)>);


////////////////////////////////////////////////////////////////////////////////


/// The names that `T`'s fields have in their accessor trait impls.
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