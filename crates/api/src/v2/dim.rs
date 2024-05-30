//! Types here deal with core aspects of dimensional analysis of other types.
//!

use std::ops::Mul;

/// A type that can represent the dimension of another type must implement this.
///
pub trait Dimension<D>
where
    D: PartialEq + Mul<i32, Output = D>,
{
    /// Returns the dimension of `self`, which may be a combination of more than one dimension ()
    ///
    fn dimension(&self) -> D;
}

/// Trait for determining if two things are dimensionally equal, aka commensurable. Per [Dimensional analysis#Dimensional homogeneity
/// (commensurability)](https://en.wikipedia.org/wiki/Dimensional_analysis#Dimensional_homogeneity_(commensurability):
///
/// > The most basic rule of dimensional analysis is that of dimensional homogeneity.[6]
/// >   Only commensurable quantities (physical quantities having the same dimension) may be
/// >   compared, equated, added, or subtracted.
///
/// Will replace `crate::IsCompatibleWith`.
///
pub trait IsCommensurableWith<D, Rhs = Self>: Dimension<D>
where
    Rhs: IsCommensurableWith<D>,
    D: PartialEq + Mul<i32, Output = D>,
{
    #[inline]
    fn is_commensurable_with(&self, rhs: &Rhs) -> bool {
        self.dimension() == rhs.dimension()
    }
}
