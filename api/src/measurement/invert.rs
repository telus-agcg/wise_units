use super::Measurement;
use crate::invert::{Invert, ToInverse};

impl Invert for &mut Measurement {
    #[inline]
    fn invert(self) {
        self.value = 1.0 / self.value;
        self.unit.invert();
    }
}

impl ToInverse for Measurement {
    #[inline]
    fn to_inverse(&self) -> Measurement {
        let new_value = 1.0 / self.value;

        Self {
            value: new_value,
            unit: self.unit.to_inverse(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod invert {
        use crate::{invert::Invert, Measurement};

        #[test]
        fn validate_numerator_no_exponent() {
            let mut unit = Measurement::new(10.0, "m").unwrap();
            unit.invert();
            assert_eq!(unit, Measurement::new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let mut unit = Measurement::new(10.0, "m1").unwrap();
            unit.invert();
            assert_eq!(unit, Measurement::new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            let mut unit = Measurement::new(10.0, "m-1").unwrap();
            unit.invert();
            assert_eq!(unit, Measurement::new(0.1, "m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let mut unit = Measurement::new(10.0, "m2/s2").unwrap();
            unit.invert();
            assert_eq!(unit, Measurement::new(0.1, "s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let mut unit = Measurement::new(10.0, "m2/s2.g4/km4/har5").unwrap();
            unit.invert();
            assert_eq!(unit, Measurement::new(0.1, "s2.g4.har5/m2.km4").unwrap());
        }
    }

    mod to_inverse {
        use crate::{invert::ToInverse, Measurement};

        #[test]
        fn validate_numerator_no_exponent() {
            let unit = Measurement::new(10.0, "m").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Measurement::new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let unit = Measurement::new(10.0, "m1").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Measurement::new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            let unit = Measurement::new(10.0, "m-1").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Measurement::new(0.1, "m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let unit = Measurement::new(10.0, "m2/s2").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Measurement::new(0.1, "s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let unit = Measurement::new(10.0, "m2/s2.g4/km4/har5").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Measurement::new(0.1, "s2.g4.har5/m2.km4").unwrap());
        }

        #[test]
        fn validate_zero_value() {
            let unit = Measurement::new(0.0, "m2/s2.g4/km4/har5").unwrap();
            let new_unit = unit.to_inverse();
            assert_eq!(new_unit, Measurement::new(std::f64::INFINITY, "s2.g4.har5/m2.km4").unwrap());
        }
    }

    mod into_inverse {
        use crate::{invert::IntoInverse, Measurement};

        #[test]
        fn validate_numerator_no_exponent() {
            let unit = Measurement::new(10.0, "m").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Measurement::new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let unit = Measurement::new(10.0, "m1").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Measurement::new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            let unit = Measurement::new(10.0, "m-1").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Measurement::new(0.1, "m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let unit = Measurement::new(10.0, "m2/s2").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Measurement::new(0.1, "s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let unit = Measurement::new(10.0, "m2/s2.g4/km4/har5").unwrap();
            let new_unit = unit.into_inverse();
            assert_eq!(new_unit, Measurement::new(0.1, "s2.g4.har5/m2.km4").unwrap());
        }
    }
}
