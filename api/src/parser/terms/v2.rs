use crate::{v2::behavior_traits::convert, Term};

impl convert::Invert for Vec<Term> {
    fn invert(&mut self) {
        for term in &mut *self {
            convert::Invert::invert(term);
        }
    }
}

impl convert::ToInverse for Vec<Term> {
    fn to_inverse(&self) -> Self {
        self.iter().map(convert::ToInverse::to_inverse).collect()
    }
}

impl<'a> convert::ToScalar<'a, f64> for Vec<Term> {
    fn to_scalar(&'a self) -> f64 {
        self.iter().map(convert::ToScalar::to_scalar).product()
    }
}

impl<'a> convert::ToMagnitude<'a, f64> for Vec<Term> {
    fn to_magnitude(&'a self) -> f64 {
        self.iter()
            .map(convert::ToMagnitude::to_magnitude)
            .product()
    }
}
