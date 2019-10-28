/*!
Contains the Structural trait with info about the fields that have accessor trait impls.
*/

/// Indicates that the type derives Structural,
/// and provides information about the fields that impl accessor traits.
pub trait Structural{
    /// The fields that have accessor trait implemented for them.
    ///
    /// Note that,if you rename the fields in their accessors,
    /// their name in this slice won't match the name used by the 
    /// accessor traits for the field.
    ///
    const FIELDS:&'static[&'static str];
}