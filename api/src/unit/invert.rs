use crate::invert::{IntoInverse, Invert};
use super::Unit;

impl Invert for &mut Unit {
    #[inline]
    fn invert(self) {
        self.terms.invert();
    }
}

impl IntoInverse for Unit {
    #[inline]
    fn into_inverse(&self) -> Unit {
        let inverted_terms = self.terms.into_inverse();

        Unit { terms: inverted_terms }
    }
}

#[cfg(test)]
mod tests {
    mod invert {
        use crate::{Invert, Unit};
        use std::str::FromStr;

        #[test]
        fn validate_numerator_no_exponent() {
            let mut unit = Unit::from_str("m").unwrap();
            unit.invert();
            assert_eq!(unit, Unit::from_str("m-1").unwrap());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let mut unit = Unit::from_str("m1").unwrap();
            unit.invert();
            assert_eq!(unit, Unit::from_str("m-1").unwrap());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            let mut unit = Unit::from_str("m-1").unwrap();
            unit.invert();
            assert_eq!(unit, Unit::from_str("m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let mut unit = Unit::from_str("m2/s2").unwrap();
            unit.invert();
            assert_eq!(unit, Unit::from_str("s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let mut unit = Unit::from_str("m2/s2.g4/km4/har5").unwrap();
            unit.invert();
            assert_eq!(unit, Unit::from_str("s2.g4.har5/m2.km4").unwrap());
        }
    }

    mod into_inverse {
        use crate::{IntoInverse, Unit};
        use std::str::FromStr;

        #[test]
        fn validate_numerator_no_exponent() {
            let unit = Unit::from_str("m").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("m-1").unwrap());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let unit = Unit::from_str("m1").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("m-1").unwrap());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            let unit = Unit::from_str("m-1").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let unit = Unit::from_str("m2/s2").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let unit = Unit::from_str("m2/s2.g4/km4/har5").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Unit::from_str("s2.g4.har5/m2.km4").unwrap());
        }
    }
}
