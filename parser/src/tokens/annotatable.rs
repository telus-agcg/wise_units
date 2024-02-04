use super::SimpleUnit;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Annotatable {
    pub(crate) simple_unit: SimpleUnit,
    pub(crate) exponent: Option<i8>,
}

impl Annotatable {
    #[must_use]
    pub const fn new(simple_unit: SimpleUnit, exponent: Option<i8>) -> Self {
        Self {
            simple_unit,
            exponent,
        }
    }

    #[must_use]
    pub const fn simple_unit(&self) -> &SimpleUnit {
        &self.simple_unit
    }

    #[must_use]
    pub const fn exponent(&self) -> Option<i8> {
        self.exponent
    }
}
