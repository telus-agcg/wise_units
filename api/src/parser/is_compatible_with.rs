use parser::Composable;

/// A simple trait for defining how a type should be compared to another type to see if they're
/// compatible. Typically this comparison is made via each type's `Composition`, but the trait
/// does not enforce that.
///
pub trait IsCompatibleWith<RHS=Self> {
    fn is_compatible_with(self, rhs: RHS) -> bool;
}

/// Implements `IsCompatibleWith` for all types that are `Composable`.
///
impl<T, U> IsCompatibleWith<U> for T
where
    T: Composable,
    U: Composable,
{
    #[inline]
    fn is_compatible_with(self, rhs: U) -> bool {
        self.composition() == rhs.composition()
    }
}
