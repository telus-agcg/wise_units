mod convert;
mod ops;
mod ucum;
mod unit_conversion;

use std::borrow::Cow;

use crate::{
    v2::type_traits::{term_unit::TermUnit, unit_old::Unit as V2Unit},
    Unit,
};

impl V2Unit for Unit {
    type UcumString = str;
    type ParseError = crate::Error;

    fn parse_ucum_str(ucum_str: &str) -> Result<Self, Self::ParseError> {
        std::str::FromStr::from_str(ucum_str)
    }

    fn expression(&self) -> Cow<'_, str> {
        Cow::Owned(Self::expression(self))
    }

    fn is_unity(&self) -> bool {
        todo!()
    }

    fn equals<T>(&self, rhs: &T) -> bool {
        todo!()
    }

    fn dim<D>(&self) -> D {
        todo!()
    }

    fn commensurable_ord<T>(&self, rhs: &T) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl TermUnit for Unit {
    type Term = crate::Term;

    fn terms(&self) -> &[Self::Term] {
        &self.terms
    }

    fn terms_mut(&mut self) -> &mut [Self::Term] {
        &mut self.terms
    }
}
