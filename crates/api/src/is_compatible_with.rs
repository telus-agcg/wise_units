use crate::Composable;

/// A simple trait for defining how a type should be compared to another type to see if they're
/// compatible. Typically this comparison is made via each type's `Composition`, but the trait
/// does not enforce that.
///
pub trait IsCompatibleWith<RHS = Self> {
    fn is_compatible_with(&self, rhs: &RHS) -> bool;
}

/// Marker trait to allow for auto-implementing `IsCompatibleWith` using the default implementation.
///
pub trait DefaultCompatibility {}

/// Implements `IsCompatibleWith` for all types that are `Composable` and `DefaultCompatibility`.
///
impl<T, U> IsCompatibleWith<U> for T
where
    T: Composable + DefaultCompatibility,
    U: Composable + DefaultCompatibility,
{
    #[inline]
    fn is_compatible_with(&self, rhs: &U) -> bool {
        self.composition() == rhs.composition()
    }
}
