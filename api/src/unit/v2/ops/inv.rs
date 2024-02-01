use crate::{v2::behavior_traits::ops::inv, Unit};

impl inv::InvertMut for Unit {
    fn invert_mut(&mut self) {
        inv::InvertMut::invert_mut(&mut self.terms);
    }
}

impl inv::ToInverse for Unit {
    fn to_inverse(&self) -> Self {
        Self::new(inv::ToInverse::to_inverse(&self.terms))
    }
}
