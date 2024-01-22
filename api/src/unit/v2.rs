mod convert;
mod ops;
mod ucum;

use std::borrow::Cow;

use crate::{v2::type_traits::Unit as TUnit, UcumUnit, Unit};

impl<'a> TUnit<'a, f64> for Unit {
    type InputString = &'a str;
    type ParseError = crate::Error;
    type Expression = Cow<'a, str>;

    fn parse(string: Self::InputString) -> Result<Self, Self::ParseError> {
        use std::str::FromStr;

        Self::from_str(string)
    }

    fn expression(&'a self) -> Self::Expression {
        self.as_str()
    }

    fn is_special(&self) -> bool {
        UcumUnit::is_special(self)
    }
}
