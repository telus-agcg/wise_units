use std::{cmp::Ordering, ops::Mul};

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

/// Similar to `std::cmp::PartialOrd`, but allows for checking if types are commensurable before
/// ordering. This makes room for types to use `std::cmp::PartialOrd` for strict object comparison
/// (particularly when considering hash-related operations; we want to have the option to have the
/// hash for, say, a `Term` that represents `1000m` to be different than a `Term` that represents
/// `1km`).
///
pub trait CommensurableOrd<D, Rhs = Self>: IsCommensurableWith<D, Rhs>
where
    Rhs: IsCommensurableWith<D>,
    D: PartialEq + Mul<i32, Output = D>,
{
    /// Similar to  `std::cmp::PartialOrd::partial_cmp()`, this is where you should a) check if
    /// `self` and `other` are commensurable (using `wise_units::v2::dim::IsCommensurableWith::is_commensurable_with()`),
    /// then proceed to determining the order of the two values.
    ///
    fn commensurable_cmp(&self, other: &Rhs) -> Option<Ordering>;

    /// Tests less than (for `self` and `other`).
    ///
    #[inline]
    fn lt(&self, other: &Rhs) -> bool {
        matches!(self.commensurable_cmp(other), Some(Ordering::Less))
    }

    /// Tests less than or equal to (for `self` and `other`).
    ///
    #[inline]
    fn le(&self, other: &Rhs) -> bool {
        matches!(
            self.commensurable_cmp(other),
            Some(Ordering::Less | Ordering::Equal)
        )
    }

    /// Tests greater than (for `self` and `other`).
    ///
    #[inline]
    fn gt(&self, other: &Rhs) -> bool {
        matches!(self.commensurable_cmp(other), Some(Ordering::Greater))
    }

    /// Tests greater than or equal to (for `self` and `other`).
    ///
    #[inline]
    fn ge(&self, other: &Rhs) -> bool {
        matches!(
            self.commensurable_cmp(other),
            Some(Ordering::Greater | Ordering::Equal)
        )
    }
}
