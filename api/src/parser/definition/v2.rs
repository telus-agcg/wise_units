use crate::v2::{behavior_traits::convert, type_traits};

use super::Definition;

impl type_traits::Definition<f64> for Definition {
    type Unit = Vec<crate::Term>;

    fn value(&self) -> &f64 {
        &self.value
    }

    fn unit(&self) -> Option<&Self::Unit> {
        Some(self.terms())
    }
}

impl convert::ToScalar<f64> for Definition {
    fn to_scalar(&self) -> f64 {
        self.terms.to_scalar()
    }
}

impl convert::ToMagnitude<f64> for Definition {
    fn to_magnitude(&self) -> f64 {
        self.terms.to_magnitude()
    }
}
