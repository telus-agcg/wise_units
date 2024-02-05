use crate::{measurement::Measurement, reducible::Reducible, ucum_unit::UcumUnit};

impl Reducible<f64> for Measurement {
    fn reduce_value(&self, value: f64) -> f64 {
        if self.is_special() {
            self.unit.reduce_value(value)
        } else {
            value * self.unit.reduce_value(1.0)
        }
    }

    fn calculate_magnitude(&self, value: f64) -> f64 {
        if self.is_special() {
            self.unit.calculate_magnitude(self.scalar())
        } else {
            value * self.unit.calculate_magnitude(1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{measurement::Measurement, reducible::Reducible};
    use approx::assert_relative_eq;

    macro_rules! validate_reduce_value {
        ($test_name:ident, $measurement_value:expr, $unit_str:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let measurement = Measurement::try_new($measurement_value, $unit_str).unwrap();
                assert_relative_eq!(measurement.reduce_value(1.0), $expected_value);
            }
        };
    }

    macro_rules! validate_calculate_magnitude {
        ($test_name:ident, $measurement_value:expr, $unit_str:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let measurement = Measurement::try_new($measurement_value, $unit_str).unwrap();
                assert_relative_eq!(measurement.calculate_magnitude(1.0), $expected_value);
            }
        };
    }

    // reduce_value tests
    validate_reduce_value!(validate_reduce_value_m, 1.0, "m", 1.0);
    validate_reduce_value!(validate_reduce_value_km, 1.0, "km", 1000.0);
    validate_reduce_value!(validate_reduce_value_meter_minus1, 1.0, "m-1", 1.0);
    validate_reduce_value!(validate_reduce_value_meter_factor, 1.0, "10m", 10.0);
    validate_reduce_value!(
        validate_reduce_value_kilometer_factor,
        1.0,
        "10km",
        10_000.0
    );
    validate_reduce_value!(
        validate_reduce_value_kilometer_factor_exponent,
        1.0,
        "10km-1",
        0.0001
    );
    validate_reduce_value!(validate_reduce_value_liter, 1.0, "L", 0.001);
    validate_reduce_value!(
        validate_reduce_value_pi,
        1.0,
        "[pi]",
        ::std::f64::consts::PI
    );
    validate_reduce_value!(
        validate_reduce_value_pi_factor,
        1.0,
        "10[pi]",
        ::std::f64::consts::PI * 10.0
    );
    validate_reduce_value!(validate_reduce_value_hectare, 1.0, "har", 10_000.0);
    validate_reduce_value!(validate_reduce_value_week, 1.0, "wk", 604_800.0);
    validate_reduce_value!(validate_reduce_value_kilogram, 1.0, "kg", 1000.0);
    validate_reduce_value!(
        validate_reduce_value_fahrenheit,
        1.0,
        "[degF]",
        255.927_777_777_777_8
    );

    // magnitude tests
    validate_calculate_magnitude!(validate_calculate_magnitude_meter, 1.0, "m", 1.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_kilometer, 1.0, "km", 1000.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_meter_minus1, 1.0, "m-1", 1.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_meter_factor, 1.0, "10m", 10.0);
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer_factor,
        1.0,
        "10km",
        10_000.0
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer_factor_exponent,
        1.0,
        "10km-1",
        0.000_1
    );
    validate_calculate_magnitude!(validate_calculate_magnitude_liter, 1.0, "L", 1.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_pi, 1.0, "[pi]", 1.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_pi_factor, 1.0, "10[pi]", 10.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_hectare, 1.0, "har", 100.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_week, 1.0, "wk", 1.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_kilogram, 1.0, "kg", 1000.0);
    validate_calculate_magnitude!(
        validate_calculate_magnitude_fahrenheit,
        1.0,
        "[degF]",
        1.000_000_000_000_056_8
    );
}
