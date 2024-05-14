use crate::{unit, v2::convert::ToFraction, Measurement, Unit};

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

#[cfg(test)]
mod tests {
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
