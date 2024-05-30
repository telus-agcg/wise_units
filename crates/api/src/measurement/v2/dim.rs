use crate::{
    v2::dim::{Dimension, IsCommensurableWith},
    Composable, Composition, IsCompatibleWith, Measurement, Unit,
};

impl Dimension<Composition> for Measurement {
    fn dimension(&self) -> Composition {
        Composable::composition(self)
    }
}

impl IsCommensurableWith<Composition> for Measurement {
    fn is_commensurable_with(&self, rhs: &Self) -> bool {
        IsCompatibleWith::is_compatible_with(self, rhs)
    }
}

impl IsCommensurableWith<Composition, Unit> for Measurement {
    fn is_commensurable_with(&self, rhs: &Unit) -> bool {
        IsCompatibleWith::is_compatible_with(self, rhs)
    }
}
