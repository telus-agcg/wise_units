use crate::v2::ucum_symbol::UcumDefinitionUnit;

use super::Atom;

impl UcumDefinitionUnit for Atom {
    type Unit = crate::Unit;

    fn definition_unit(&self) -> <Self as UcumDefinitionUnit>::Unit {
        crate::Unit::new(self.definition().terms().clone())
    }
}