use crate::v2::traits::ucum;

use super::Atom;

impl ucum::DefinitionValue<f64> for Atom {
    fn definition_value(&self) -> f64 {
        // Just delegate to the original impl for now.
        crate::UcumSymbol::definition_value(self)
    }
}

impl ucum::DefinitionUnit for Atom {
    type Unit = crate::Unit;

    fn definition_unit(&self) -> <Self as ucum::DefinitionUnit>::Unit {
        // Just delegate to the original impl for now.
        crate::UcumSymbol::definition_unit(self)
    }
}

impl ucum::Dim<crate::Composition> for Atom {
    fn dim(&self) -> crate::Composition {
        // Just delegate to the original impl for now.
        crate::Composable::composition(self)
    }
}
