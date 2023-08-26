use std::{borrow::Cow, cmp::Ordering};

use crate::v2::behavior_traits::convert::ToScalar;

use super::dimension::Dimension;

// where
//     &'a Self: Mul + Div,
// pub trait Unit<'a>: Sized + PartialEq + PartialOrd + ToScalar<f64> + 'a {
pub trait Unit: Sized {
    type InputString;
    type ParseError;
    type Expression: Clone;
    type Dimension: Dimension;

    /// Defines how to parse a string of unit symbols into a `Unit`.
    ///
    /// # Errors
    ///
    /// This should error if the `string` can't be parsed into a type that represents that
    /// combinations of units.
    ///
    fn parse(string: &Self::InputString) -> Result<Self, Self::ParseError>;

    /// This is the string that was either a) parsed to instantiate the object, or b) the canonical
    /// string that would represent the Unit (if it wasn't instantiated via parsing).
    ///
    fn expression(&self) -> Cow<'_, Self::Expression>;

    fn dimension(&self) -> Self::Dimension;

    fn equals<T>(&self, other: &T) -> bool;

    /// Is `other` of the same dimension?
    ///
    fn is_compatible_with<T>(&self, other: &T) -> bool
    where
        T: Dimension,
        Self::Dimension: PartialEq<<T as Dimension>::Output>,
    {
        self.dimension() == other.dimension()
    }

    fn is_commensurable_with<Rhs, S>(&self, rhs: &Rhs) -> bool
    where
        Rhs: Dimension + ToScalar<S>,
        Self::Dimension: PartialEq<<Rhs as Dimension>::Output>,
        S: PartialEq,
    {
        if !self.is_compatible_with(rhs) {
            return false;
        }

        self.to_scalar::<S>() == rhs.to_scalar()
    }

    /// `PartialOrd` depends on being in concert with `PartialEq`, which depends on being in concert
    /// it `Hash`, but only units that are strictly equal can fulfill this; units that are
    /// commensurable but not equal will not. This allows for the later case.
    ///
    fn commensurable_ord<Rhs, S>(&self, other: &Rhs) -> Option<Ordering>
    where
        Rhs: Dimension + ToScalar<S>,
        Self::Dimension: PartialEq<<Rhs as Dimension>::Output>,
        S: PartialOrd,
    {
        if !self.is_compatible_with(other) {
            return None;
        }

        self.to_scalar::<S>().partial_cmp(&other.to_scalar())
    }

    fn mul_ref<T, O>(&self, rhs: &T) -> O;
    fn div_ref<T, O>(&self, rhs: &T) -> O;
    fn to_scalar<T>(&self) -> T;
}
