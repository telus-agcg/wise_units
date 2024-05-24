use crate::{v2::dim::Dimension, Atom, Composable, Composition};

impl Dimension<Composition> for Atom {
    fn dimension(&self) -> Composition {
        Composable::composition(self)
    }
}
