use crate::{
    unit,
    v2::convert::{ToFraction, TryConvertTo},
    Error, Measurement, Unit,
};

impl ToFraction<Self, Option<Unit>> for Measurement {
    fn to_fraction(&self) -> (Self, Option<Unit>) {
        let unit_parts = self.unit.to_fraction();

        (
            Self {
                value: self.value,
                unit: unit_parts.0.unwrap_or(unit::UNITY),
            },
            unit_parts.1,
        )
    }

    fn to_numerator(&self) -> Self {
        Self {
            value: self.value,
            unit: self.unit.to_numerator().unwrap_or(unit::UNITY),
        }
    }

    fn to_denominator(&self) -> Option<Unit> {
        self.unit.to_denominator()
    }
}

impl<'a> TryConvertTo<&'a str> for Measurement {
    type Error = Error;

    fn try_convert_to(&self, rhs: &'a str) -> Result<Self, Self::Error> {
        // Delegate to old implementation for now.
        crate::Convertible::convert_to(self, rhs)
    }
}

impl<'a> TryConvertTo<&'a Unit> for Measurement {
    type Error = Error;

    fn try_convert_to(&self, rhs: &'a Unit) -> Result<Self, Self::Error> {
        // Delegate to old implementation for now.
        crate::Convertible::convert_to(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod to_fraction {
        use crate::{
            testing::const_units::{GRAM_METER, METER, METER_PER_SECOND, PER_SECOND, SECOND},
            unit::UNITY,
            Measurement,
        };

        use super::*;

        macro_rules! validate_fraction_parts {
            (
            $measurement:expr,
            expected_numerator => $expected_numerator:expr,
            expected_denominator => $expected_denominator:expr
        ) => {
                let fraction = $measurement.to_fraction();
                assert_eq!(&fraction.0, &$expected_numerator);
                assert_eq!(fraction.1, $expected_denominator);

                let numerator = $measurement.to_numerator();
                assert_eq!(numerator, $expected_numerator);

                let denominator = $measurement.to_denominator();
                assert_eq!(denominator, $expected_denominator);
            };
        }

        #[test]
        fn validate_one_numerator_term() {
            let measurement = Measurement::new(42.0, METER);

            validate_fraction_parts!(
                measurement,
                expected_numerator => measurement,
                expected_denominator => None
            );
        }

        #[test]
        fn validate_two_numerator_terms() {
            let measurement = Measurement::new(42.0, GRAM_METER);

            validate_fraction_parts!(
                measurement,
                expected_numerator => measurement,
                expected_denominator => None
            );
        }

        #[test]
        fn validate_one_numerator_term_one_denominator_term() {
            let measurement = Measurement::new(42.0, METER_PER_SECOND);

            validate_fraction_parts!(
                measurement,
                expected_numerator => Measurement::new(42.0, METER),
                expected_denominator => Some(SECOND)
            );
        }

        #[test]
        fn validate_one_denominator_term() {
            let measurement = Measurement::new(42.0, PER_SECOND);

            validate_fraction_parts!(
                measurement,
                expected_numerator => Measurement::new(42.0, UNITY),
                expected_denominator => Some(SECOND)
            );
        }
    }

    mod try_convert_to {
        use std::str::FromStr;

        use approx::{assert_relative_eq, assert_ulps_eq};

        use crate::testing::const_units::{KILOMETER, METER};

        use super::*;

        macro_rules! try_convert_test {
            ($subject:expr, $unit:expr, $expected_value:expr) => {
                let converted = $subject.try_convert_to($unit).unwrap();
                assert_eq!(converted, $subject);
                assert_relative_eq!(converted.value, $expected_value);
                assert_ulps_eq!(converted.value, $expected_value);
            };
        }

        #[test]
        fn try_convert_to_str_test() {
            let meter = Measurement::new(1.0, METER);
            try_convert_test!(meter, "m", 1.0);
            try_convert_test!(meter, "2m", 0.5);
            try_convert_test!(meter, "m2/m", 1.0);
            // This seems like it should pass, but see docs in term/is_compatible_with.rs as to why
            // it's not.
            assert!(meter.try_convert_to("m{meow}").is_err());

            let meter = Measurement::new(1000.0, METER);
            try_convert_test!(meter, "km", 1.0);
            try_convert_test!(meter, "2km", 0.5);
            try_convert_test!(meter, "2km2/2km", 0.5);
            assert!(meter.try_convert_to("g").is_err());
        }

        #[test]
        fn try_convert_to_unit_test() {
            {
                let meter = Measurement::new(1.0, METER);
                try_convert_test!(meter, &METER, 1.0);
                try_convert_test!(meter, &KILOMETER, 0.001);
            }

            {
                let meter = Measurement::new(1000.0, METER);
                try_convert_test!(meter, &KILOMETER, 1.0);
                try_convert_test!(meter, &METER, 1000.0);
            }

            {
                let m1 = Measurement::try_new(10.0, "[acr_us]/{tree}").unwrap();

                let u1 = Unit::from_str("m2/{tree}").unwrap();
                let u2 = Unit::from_str("m2/{vine}").unwrap();

                try_convert_test!(m1, &u1, 40_468.726_098_742_52);
                assert!(m1.try_convert_to(&u2).is_err());
            }

            {
                let m1 = Measurement::try_new(10.0, "{tree}").unwrap();

                let u1 = Unit::from_str("{tree}").unwrap();
                let u2 = Unit::from_str("{vine}").unwrap();

                try_convert_test!(m1, &u1, 10.0);
                assert!(m1.try_convert_to(&u2).is_err());
            }
        }
    }
}
