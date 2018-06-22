/// Trait for allowing comparison of two things, but checking that their fields
/// are the same. Normally this would be handled in `PartialEq`, but we want
/// that allow for checking equality of units based on their reduced scalar
/// value (ex. "1km == 1000m") since that's a far more common use case.
///
pub trait FieldEq<'a, RHS = &'a Self> {
    fn field_eq(&self, other: RHS) -> bool;
}
