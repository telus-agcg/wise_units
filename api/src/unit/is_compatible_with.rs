use crate::is_compatible_with::DefaultCompatibility;
use crate::unit::Unit;

impl<'a> DefaultCompatibility for &'a Unit {}

#[cfg(test)]
mod tests {
    use crate::IsCompatibleWith;
    use crate::measurement::Measurement;
    use crate::unit::Unit;
    use std::str::FromStr;

    #[test]
    fn validate_is_compatible_with_unit() {
        let meter = Unit::from_str("m").unwrap();
        let km = Unit::from_str("km").unwrap();
        assert!(meter.is_compatible_with(&km));

        let km_per_10m = Unit::from_str("km/10m").unwrap();
        assert!(!meter.is_compatible_with(&km_per_10m));

        let per_meter = Unit::from_str("m-1").unwrap();
        assert!(!meter.is_compatible_with(&per_meter));

        let ten_meter = Unit::from_str("10m").unwrap();
        assert!(meter.is_compatible_with(&ten_meter));

        let ten_km = Unit::from_str("10km").unwrap();
        assert!(meter.is_compatible_with(&ten_km));

        let per_ten_km = Unit::from_str("10km-1").unwrap();
        assert!(!meter.is_compatible_with(&per_ten_km));

        let per_meter_cubed = Unit::from_str("km-1/m2").unwrap();
        assert!(!meter.is_compatible_with(&per_meter_cubed));

        let km_per_length_cubed = Unit::from_str("km/m2.cm").unwrap();
        assert!(!meter.is_compatible_with(&km_per_length_cubed));

        let km_per_length_fourthed = Unit::from_str("km-1/m2.cm").unwrap();
        assert!(!meter.is_compatible_with(&km_per_length_fourthed));

        let meter_per_second_squared = Unit::from_str("m/s2").unwrap();
        assert!(!meter.is_compatible_with(&meter_per_second_squared));

        let km_cubed_per_nanometer_squared = Unit::from_str("km3/nm2").unwrap();
        assert!(meter.is_compatible_with(&km_cubed_per_nanometer_squared));
    }

    #[test]
    fn validate_is_compatible_with_measurement() {
        let meter = Unit::from_str("m").unwrap();
        let km = Measurement::new(1.0, "km").unwrap();
        assert!(meter.is_compatible_with(&km));

        let km_per_10m = Measurement::new(1.0, "km/10m").unwrap();
        assert!(!meter.is_compatible_with(&km_per_10m));
    }
}
