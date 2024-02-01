use crate::{v2::behavior_traits::convert, Term};

impl convert::ToScalar<f64> for Term {
    fn to_scalar(&self) -> f64 {
        // Just delegate to existing impl for now.
        crate::UcumUnit::scalar(self)
    }
}

impl convert::ToMagnitude<f64> for Term {
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
    fn to_scalar_test() {
        use convert::ToScalar;

        let term = term!(Kilo, Meter);
        assert_ulps_eq!(1000.0, term.to_scalar());
    }
}
