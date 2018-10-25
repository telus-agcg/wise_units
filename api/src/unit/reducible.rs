use crate::reducible::Reducible;
use crate::unit::Unit;

//-----------------------------------------------------------------------------
// impl Reducible
//-----------------------------------------------------------------------------
impl Reducible for Unit {
    fn reduce_value(&self, value: f64) -> f64 {
        self.terms.reduce_value(value)
    }

    /// Calculates `value` count of `self` in terms of `self`'s unit.
    ///
    fn calculate_magnitude(&self, value: f64) -> f64 {
        self.terms.calculate_magnitude(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::reducible::Reducible;
    use std::str::FromStr;
    use crate::unit::Unit;

    macro_rules! validate_reduce_value {
        ($test_name:ident, $unit_str:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($unit_str).unwrap();
                assert_relative_eq!(unit.reduce_value(1.0), $expected_value);
            }
        };
    }

    macro_rules! validate_calculate_magnitude {
        ($test_name:ident, $unit_str:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($unit_str).unwrap();
                let scalar = unit.reduce_value(1.0);
                assert_relative_eq!(unit.calculate_magnitude(scalar), $expected_value);
            }
        };
    }

    // reduce_value tests
    validate_reduce_value!(validate_reduce_value_m, "m", 1.0);
    validate_reduce_value!(validate_reduce_value_km, "km", 1000.0);
    validate_reduce_value!(validate_reduce_value_meter_minus1, "m-1", 1.0);
    validate_reduce_value!(validate_reduce_value_meter_factor, "10m", 10.0);
    validate_reduce_value!(validate_reduce_value_kilometer_factor, "10km", 10_000.0);
    validate_reduce_value!(
        validate_reduce_value_kilometer_factor_exponent,
        "10km-1",
        0.0001
    );
    validate_reduce_value!(validate_reduce_value_liter, "L", 0.001);
    validate_reduce_value!(validate_reduce_value_pi, "[pi]", ::std::f64::consts::PI);
    validate_reduce_value!(
        validate_reduce_value_pi_factor,
        "10[pi]",
        ::std::f64::consts::PI * 10.0
    );
    validate_reduce_value!(validate_reduce_value_hectare, "har", 10_000.0);
    validate_reduce_value!(validate_reduce_value_week, "wk", 604_800.0);
    validate_reduce_value!(validate_reduce_value_kilogram, "kg", 1000.0);
    validate_reduce_value!(
        validate_reduce_value_fahrenheit,
        "[degF]",
        255.927_777_777_777_8
    );

    // magnitude tests
    validate_calculate_magnitude!(validate_calculate_magnitude_meter, "m", 1.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_kilometer, "km", 1000.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_meter_minus1, "m-1", 1.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_meter_factor, "10m", 10.0);
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer_factor,
        "10km",
        10_000.0
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer_factor_exponent,
        "10km-1",
        0.000_1
    );
    validate_calculate_magnitude!(validate_calculate_magnitude_liter, "L", 1.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_pi, "[pi]", 1.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_pi_factor, "10[pi]", 10.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_hectare, "har", 100.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_week, "wk", 1.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_kilogram, "kg", 1000.0);
    validate_calculate_magnitude!(
        validate_calculate_magnitude_fahrenheit,
        "[degF]",
        1.000_000_000_000_056_8
    );
}
