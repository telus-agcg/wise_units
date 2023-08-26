use std::{borrow::Cow, cmp::Ordering};

use crate::IsCompatibleWith;

use crate::v2::behavior_traits::{
    convert::{Invert, ToInverse, ToMagnitude, ToScalar},
    ops::IsCommensurableWith,
};

pub trait Unit:
    Sized
    + Invert
    + IsCommensurableWith
    + IsCompatibleWith
    + PartialEq
    + ToInverse
    + ToScalar<f64>
    + ToMagnitude<f64>
{
    type UcumString: ?Sized;
    type ParseError;

    /// Defines how to parse a string of unit symbols into a `Unit`.
    ///
    /// # Errors
    ///
    /// This should error if the `ucum_str` can't be parsed into a type that represents that
    /// combinations of units.
    ///
    fn parse_ucum_str(ucum_str: &Self::UcumString) -> Result<Self, Self::ParseError>;
    fn expression(&self) -> Cow<'_, str>;
    fn is_unity(&self) -> bool;

    /// Here, "1 kilometer != 1000 meters"; "1 kilometer" only equals "1 kilometer".
    /// Typically this would/could delegate to a `PartialEq` implementation.
    ///
    fn equals<T>(&self, rhs: &T) -> bool;

    fn dim<D>(&self) -> D;

    /// `PartialOrd` depends on being in concert with `PartialEq`, which depends on being in concert
    /// it `Hash`, but only units that are strictly equal can fulfill this; units that are
    /// commensurable but not equal will not. This allows for the later case.
    ///
    fn commensurable_ord<T>(&self, rhs: &T) -> Option<Ordering>;
}
