use crate::v2::ucum_symbol::{UcumDefinitionUnit, UcumDefinitionValue};

use super::Atom;

impl UcumDefinitionValue<f64> for Atom {
    fn definition_value(&self) -> f64 {
        self.definition().value()
    }
}

impl UcumDefinitionUnit for Atom {
    type Unit = crate::Unit;

    fn definition_unit(&self) -> <Self as UcumDefinitionUnit>::Unit {
        crate::Unit::new(self.definition().terms().clone())
    }
}
