mod convert;
mod ops;
mod ucum;
mod unit_conversion;

use std::borrow::Cow;

use crate::{v2::type_traits::Unit as TUnit, Composable, UcumUnit, Unit};

impl TUnit<'_, f64> for Unit {
    type InputString = str;
    type ParseError = crate::Error;
    type Expression = String;
    type Dimension = crate::Composition;

    fn parse(string: &Self::InputString) -> Result<Self, Self::ParseError> {
        use std::str::FromStr;

        Self::from_str(string)
    }

    fn expression(&self) -> Cow<'_, Self::Expression> {
        Cow::Owned(Self::expression(self))
    }

    fn dimension(&self) -> Self::Dimension {
        Composable::composition(self)
    }

    fn is_special(&self) -> bool {
        UcumUnit::is_special(self)
    }
}
