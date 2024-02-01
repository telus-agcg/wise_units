use super::{Commensurability, Compare, MulDiv, TryToNumber, Ucum};

// Sized
// + convert::InvertMut
// + convert::ToFraction
// + convert::ToInverse
// + ops::IsCommensurableWith<'a, V>
// + ops::CommensurableOrd
// + ops::TryMulRef<'a>
// + ops::TryDivRef<'a>
// where
//     V: PartialOrd,
// Sized + convert::TryToMagnitude<V> + convert::TryToScalar<V> + ucum::Dimensionable
pub trait WrapperUnit: Ucum + TryToNumber + Commensurability + Compare + MulDiv {
    type InputString: ?Sized;
    type ParseError;
    type Expression: ?Sized;

    /// Defines how to parse a string of unit symbols into a `Unit`.
    ///
    /// # Errors
    ///
    /// This should error if the `string` can't be parsed into a type that represents that
    /// combinations of units.
    ///
    fn parse(string: Self::InputString) -> Result<Self, Self::ParseError>
    where
        Self: Sized;

    /// This is the string that was either a) parsed to instantiate the object, or b) the canonical
    /// string that would represent the Unit (if it wasn't instantiated via parsing).
    ///
    fn expression(&self) -> Self::Expression;
}
