use crate::v2::traits::dim::Composable;

impl Composable for crate::Measurement {
    type Composition = crate::Composition;

    fn composition(&self) -> Self::Composition {
        // Just delegate to the old trait impl for now.
        crate::Composable::composition(self)
    }
}
