mod convert;
mod ops;
mod ucum;
mod unit_conversion;

use std::borrow::Cow;

use crate::{v2::traits::Unit as V2Unit, Unit};

impl V2Unit for Unit {
    type Term = crate::Term;
    type UcumString = str;
    type ParseError = crate::Error;

    fn parse_ucum_str(ucum_str: &str) -> Result<Self, Self::ParseError> {
        std::str::FromStr::from_str(ucum_str)
    }

    fn expression(&self) -> Cow<'_, str> {
        Cow::Owned(Self::expression(self))
    }

    fn terms(&self) -> &[Self::Term] {
        &self.terms
    }

    fn terms_mut(&mut self) -> &mut [Self::Term] {
        &mut self.terms
    }
}
