use crate::{v2::behavior_traits::ucum, Composable, Composition, Measurement};

impl ucum::Dimensionable for Measurement {
    type Output = Composition;

    fn dim(&self) -> Self::Output {
        Composable::composition(self)
    }
}
