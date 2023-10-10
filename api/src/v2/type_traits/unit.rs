use crate::v2::behavior_traits::{convert, ops};

pub trait Unit<'a, V>:
    Sized
    + convert::Invert
    + convert::ToFraction
    + convert::ToInverse
    + convert::ToMagnitude<V>
    + convert::ToScalar<V>
    + ops::DimEq
    + ops::TryDivRef
    + ops::TryMulRef
where
    V: PartialOrd,
{
    type InputString: ?Sized;
    type ParseError;
    type Expression: Clone;

    /// Defines how to parse a string of unit symbols into a `Unit`.
    ///
    /// # Errors
    ///
    /// This should error if the `string` can't be parsed into a type that represents that
    /// combinations of units.
    ///
    fn parse(string: Self::InputString) -> Result<Self, Self::ParseError>;

    /// This is the string that was either a) parsed to instantiate the object, or b) the canonical
    /// string that would represent the Unit (if it wasn't instantiated via parsing).
    ///
    fn expression(&'a self) -> Cow<'a, Self::Expression>;

    fn is_special(&self) -> bool;
}
