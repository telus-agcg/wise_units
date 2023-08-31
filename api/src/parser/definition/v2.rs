use crate::v2::{behavior_traits::convert, type_traits};

use super::Definition;

impl<'a> type_traits::Definition<'a, f64> for Definition {
    type Unit = Vec<crate::Term>;

    fn value(&'a self) -> &'a f64 {
        &self.value
    }

    fn unit(&'a self) -> Option<&'a Self::Unit> {
        Some(self.terms())
    }
}

impl<'a> convert::ToScalar<'a, f64> for Definition {
    fn to_scalar(&'a self) -> f64 {
        self.terms.to_scalar()
    }
}

impl<'a> convert::ToMagnitude<'a, f64> for Definition {
    fn to_magnitude(&'a self) -> f64 {
        self.terms.to_magnitude()
    }
}
