use crate::{v2::behavior_traits::ops::inv, Term};

impl inv::InvertMut for Vec<Term> {
    fn invert_mut(&mut self) {
        for term in &mut *self {
            inv::InvertMut::invert_mut(term);
        }
    }
}

impl inv::ToInverse for Vec<Term> {
    fn to_inverse(&self) -> Self {
        self.iter().map(inv::ToInverse::to_inverse).collect()
    }
}
