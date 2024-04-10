// TODO: Remove in 1.0.0 release.
#![allow(deprecated)]

use crate::{
    invert::{Invert, ToInverse},
    Error,
};

use super::Measurement;

impl Invert for &mut Measurement {
    #[inline]
    fn invert(self) {
        let _ = num_traits::Inv::inv(self);
    }
}

// TODO: Remove the check for divide by 0 in the future.
//
impl ToInverse for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn to_inverse(&self) -> Self::Output {
        let m = num_traits::Inv::inv(self.clone());

        if m.value.is_infinite() {
            return Err(Error::DivideByZero);
        }

        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    mod invert {
        use crate::{invert::Invert, Measurement};

        #[test]
        fn validate_numerator_no_exponent() {
            let mut measurement = Measurement::try_new(10.0, "m").unwrap();
            measurement.invert();
            assert_eq!(measurement, Measurement::try_new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let mut measurement = Measurement::try_new(10.0, "m1").unwrap();
            measurement.invert();
            assert_eq!(measurement, Measurement::try_new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            let mut measurement = Measurement::try_new(10.0, "m-1").unwrap();
            measurement.invert();
            assert_eq!(measurement, Measurement::try_new(0.1, "m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let mut measurement = Measurement::try_new(10.0, "m2/s2").unwrap();
            measurement.invert();
            assert_eq!(measurement, Measurement::try_new(0.1, "s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let mut measurement = Measurement::try_new(10.0, "m2/s2.g4/km4/har5").unwrap();
            measurement.invert();
            assert_eq!(
                measurement,
                Measurement::try_new(0.1, "s2.g4.har5/m2.km4").unwrap()
            );
        }
    }

    mod to_inverse {
        use crate::{invert::ToInverse, Measurement};

        #[test]
        fn validate_numerator_no_exponent() {
            let measurement = Measurement::try_new(10.0, "m").unwrap();
            let new_measurement = measurement.to_inverse().unwrap();
            assert_eq!(new_measurement, Measurement::try_new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let measurement = Measurement::try_new(10.0, "m1").unwrap();
            let new_measurement = measurement.to_inverse().unwrap();
            assert_eq!(new_measurement, Measurement::try_new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            let measurement = Measurement::try_new(10.0, "m-1").unwrap();
            let new_measurement = measurement.to_inverse().unwrap();
            assert_eq!(new_measurement, Measurement::try_new(0.1, "m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let measurement = Measurement::try_new(10.0, "m2/s2").unwrap();
            let new_measurement = measurement.to_inverse().unwrap();
            assert_eq!(new_measurement, Measurement::try_new(0.1, "s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let measurement = Measurement::try_new(10.0, "m2/s2.g4/km4/har5").unwrap();
            let new_measurement = measurement.to_inverse().unwrap();
            assert_eq!(
                new_measurement,
                Measurement::try_new(0.1, "s2.g4.har5/m2.km4").unwrap()
            );
        }

        #[test]
        fn validate_zero_value() {
            let measurement = Measurement::try_new(0.0, "m2/s2.g4/km4/har5").unwrap();
            let new_measurement = measurement.to_inverse();
            assert_eq!(new_measurement, Err(crate::Error::DivideByZero));
        }
    }

    mod into_inverse {
        use crate::{invert::IntoInverse, Measurement};

        #[test]
        fn validate_numerator_no_exponent() {
            let measurement = Measurement::try_new(10.0, "m").unwrap();
            let new_measurement = measurement.into_inverse().unwrap();
            assert_eq!(new_measurement, Measurement::try_new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_numerator_with_exponent_1() {
            let measurement = Measurement::try_new(10.0, "m1").unwrap();
            let new_measurement = measurement.into_inverse().unwrap();
            assert_eq!(new_measurement, Measurement::try_new(0.1, "m-1").unwrap());
        }

        #[test]
        fn validate_denominator_with_exponent_minus_1() {
            let measurement = Measurement::try_new(10.0, "m-1").unwrap();
            let new_measurement = measurement.into_inverse().unwrap();
            assert_eq!(new_measurement, Measurement::try_new(0.1, "m").unwrap());
        }

        #[test]
        fn validate_numerator_and_denominator() {
            let measurement = Measurement::try_new(10.0, "m2/s2").unwrap();
            let new_measurement = measurement.into_inverse().unwrap();
            assert_eq!(new_measurement, Measurement::try_new(0.1, "s2/m2").unwrap());
        }

        #[test]
        fn validate_numerators_and_denominators_mixed() {
            let measurement = Measurement::try_new(10.0, "m2/s2.g4/km4/har5").unwrap();
            let new_measurement = measurement.into_inverse().unwrap();
            assert_eq!(
                new_measurement,
                Measurement::try_new(0.1, "s2.g4.har5/m2.km4").unwrap()
            );
        }
    }
}
