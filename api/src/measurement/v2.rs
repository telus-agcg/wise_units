mod convert;
mod ops;
mod ucum;
mod unit_conversion;

use crate::{v2::type_traits, Measurement};

impl<'a> type_traits::Measurement<'a, f64> for Measurement {
    type Unit = crate::Unit;

    fn value(&'a self) -> &'a f64 {
        &self.value
    }

    fn unit(&'a self) -> &'a Self::Unit {
        &self.unit
    }
}
