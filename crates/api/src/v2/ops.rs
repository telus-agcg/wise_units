use std::ops::Mul;

use super::dim::IsCommensurableWith;

/// Trait for comparisons that checks for equality first. Equality checks must only be done with
/// types that are commensurable. Return `None` if types are _not_ commensurable, otherwise
/// `Some(result)`.
///
/// This trait should be (well, should become) the de facto method for checking equality between
/// measurements (notice lower-case "m" there, indicating objects that can measure, not necessarily
/// only `Measurement`), etc. Historically, we've used `PartialEq` implementations, but those
/// should've been reserved for object equality, not semantic equality (this trait checks the
/// latter, not the former).
///
pub trait CommensurableEq<D, Rhs = Self>: IsCommensurableWith<D, Rhs>
where
    Rhs: IsCommensurableWith<D>,
    D: PartialEq + Mul<i32, Output = D>,
{
    /// This method tests the commensurability between `self` and `other`, then checks if their
    /// scalar values are equal.
    ///
    fn commensurable_eq(&self, other: &Rhs) -> Option<bool>;

    /// This method tests the commensurability between `self` and `other`, then checks if their
    /// scalar values are _not_ equal.
    ///
    #[inline]
    fn commensurable_ne(&self, other: &Rhs) -> Option<bool> {
        self.commensurable_eq(other).map(|r| !r)
    }
}
