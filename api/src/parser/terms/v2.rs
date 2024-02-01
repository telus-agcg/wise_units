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

impl convert::ToScalar<f64> for Vec<Term> {
    fn to_scalar(&self) -> f64 {
        self.iter()
            .map(convert::ToScalar::<f64>::to_scalar)
            .product()
    }
}

impl convert::ToMagnitude<f64> for Vec<Term> {
    fn to_magnitude(&self) -> f64 {
        self.iter()
            .map(convert::ToMagnitude::to_magnitude)
            .product()
    }
}
