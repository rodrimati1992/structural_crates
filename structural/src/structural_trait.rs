/// Indicates that the type derives Structural,
/// and provides information about the fields that impl accessor traits.
pub trait Structural{
    /// The fields that have accessor trait implemented for them.
    ///
    /// Note that,because it's possible to rename the fields in their accessors,
    /// those names won't necessarily match.
    const FIELDS:&'static[&'static str];
}