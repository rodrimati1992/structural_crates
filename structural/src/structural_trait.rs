/*!
Contains the Structural trait with info about the fields that have accessor trait impls.
*/

use std_::marker::PhantomData;
    




/// Indicates and provides information about the fields that implement accessor traits.
pub trait Structural{
    /// Information about fields that have accessor trait implemented for them.
    const FIELDS:&'static[FieldInfo];
}


/// An object-safe version of the `Structural` trait.
/// with information about the fields that implement accessor traits.
///
/// This trait has a blanket implementation for types that implement `Structural`,
/// and cannot be implemented outside this module.
pub trait StructuralDyn{
    /// Information about fields that have accessor trait implemented for them.
    fn fields_info(&self)->&'static[FieldInfo];

    // This is to ensure that the trait is only implemented by types that implement Structural.
    //
    // Adding Structural as a supertrait would make this not object safe,
    // so this is the best alternative.
    #[doc(hidden)]
    fn unimplementable_outside_the_structural_trait_module()->BlanketImpl<Self>
    where
        Self:Sized;
}


impl<This> StructuralDyn for This
where
    This:Structural
{    
    fn fields_info(&self)->&'static[FieldInfo]{
        &<This as Structural>::FIELDS
    }

    fn unimplementable_outside_the_structural_trait_module()->BlanketImpl<Self>
    where
        Self:Sized
    {
        BlanketImpl(PhantomData)
    }
}


mod blanket_impl{
    use super::*;
    pub struct BlanketImpl<T>(pub(super) PhantomData<T>);
}

use blanket_impl::BlanketImpl;


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