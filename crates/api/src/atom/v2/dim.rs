use crate::{
    v2::dim::{Dimension, IsCommensurableWith},
    Atom, Composable, Composition, IsCompatibleWith,
};

impl Dimension<Composition> for Atom {
    fn dimension(&self) -> Composition {
        Composable::composition(self)
    }
}

impl IsCommensurableWith<Composition> for Atom {
    fn is_commensurable_with(&self, rhs: &Self) -> bool {
        IsCompatibleWith::is_compatible_with(self, rhs)
    }
}
