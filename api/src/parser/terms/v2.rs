use crate::{v2::traits::convert, Term};

impl convert::Invert for Vec<Term> {
    fn invert(&mut self) {
        for term in self.iter_mut() {
            convert::Invert::invert(term);
        }
    }
}

impl convert::ToInverse for Vec<Term> {
    fn to_inverse(&self) -> Self {
        self.iter().map(convert::ToInverse::to_inverse).collect()
    }
}
