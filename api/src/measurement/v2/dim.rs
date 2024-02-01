use crate::{v2::behavior_traits::dim, Composable, Composition, Measurement};

impl dim::Dimensionable for Measurement {
    type Output = Composition;

    fn dim(&self) -> Self::Output {
        Composable::composition(self)
    }
}
