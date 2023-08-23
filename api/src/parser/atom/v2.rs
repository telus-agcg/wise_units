use crate::v2::traits::ucum::DefinitionUnit;

use super::Atom;

impl DefinitionUnit for Atom {
    type Unit = crate::Unit;

    fn definition_unit(&self) -> <Self as DefinitionUnit>::Unit {
        crate::Unit::new(self.definition().terms().clone())
    }
}
