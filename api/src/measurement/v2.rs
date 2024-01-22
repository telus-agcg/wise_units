mod convert;
mod ops;
mod ucum;

use crate::{v2::type_traits, Measurement};

impl<'a> type_traits::Measurement<'a, f64> for Measurement {
    type Unit = crate::Unit;

    fn value(&self) -> &f64 {
        &self.value
    }

    fn unit(&self) -> &Self::Unit {
        &self.unit
    }
}
