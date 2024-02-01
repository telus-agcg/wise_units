//! These traits are a progression from the existing `crate::UcumSymbol` trait, allowing the same
//! functionality, but also allowing downstream crates to implement for wrapper types.
//!

use crate::v2::type_traits::dimension::Dimension;

/// Trait for determining if two things are dimensionally equal. This is a required check for many
/// operations, such as adding, subtracting, and commensurability checking. Really, any type that
/// will be involved in comparing for commensurability should implement this.
///
pub trait Dimensionable {
    type Output: Dimension;

    fn dim(&self) -> Self::Output;

    #[inline]
    fn dim_eq<Rhs>(&self, rhs: &Rhs) -> bool
    where
        Rhs: Dimensionable<Output = Self::Output>,
    {
        self.dim() == rhs.dim()
    }
}
