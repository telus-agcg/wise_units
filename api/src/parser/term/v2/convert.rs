use crate::{v2::behavior_traits::convert, Term};

impl convert::Invert for Term {
    fn invert(&mut self) {
        // Just delegate to existing impl for now.
        crate::invert::Invert::invert(self);
    }
}

impl convert::ToInverse for Term {
    #[must_use]
    fn to_inverse(&self) -> Self {
        // Just delegate to existing impl for now.
        crate::invert::ToInverse::to_inverse(self)
    }
}

impl convert::ToScalar<f64> for Term {
    fn to_scalar(&self) -> f64 {
        // Just delegate to existing impl for now.
        crate::UcumUnit::scalar(self)
    }
}

impl convert::ToMagnitude<'_, f64> for Term {
    fn to_magnitude(&self) -> f64 {
        // Just delegate to existing impl for now.
        crate::UcumUnit::magnitude(self)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;

    use super::*;
    use crate::parser::{Atom, Prefix};

    #[test]
    fn invert_test() {
        let mut term = term!(Meter, exponent: 1);
        convert::Invert::invert(&mut term);
        assert_eq!(term, term!(Meter, exponent: -1));
    }

    #[test]
    fn to_inverse_test() {
        let term = term!(Meter);
        let new_term = convert::ToInverse::to_inverse(&term);
        assert_eq!(new_term, term!(Meter, exponent: -1));
    }

    #[test]
    fn to_scalar_test() {
        use convert::ToScalar;

        let term = term!(Kilo, Meter);
        assert_ulps_eq!(1000.0, term.to_scalar());
    }
}
