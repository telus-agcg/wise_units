use crate::{
    v2::dim::{Dimension, IsCommensurableWith},
    Composable, Composition, IsCompatibleWith, Measurement, Unit,
};

impl Dimension<Composition> for Unit {
    fn dimension(&self) -> Composition {
        Composable::composition(self)
    }
}

impl IsCommensurableWith<Composition> for Unit {
    fn is_commensurable_with(&self, rhs: &Self) -> bool {
        IsCompatibleWith::is_compatible_with(self, rhs)
    }
}

impl IsCommensurableWith<Composition, Measurement> for Unit {
    fn is_commensurable_with(&self, rhs: &Measurement) -> bool {
        IsCompatibleWith::is_compatible_with(self, rhs)
    }
}
