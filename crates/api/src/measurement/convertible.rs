#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]

use crate::{Convertible, Error, IsCompatibleWith, Measurement, Unit};
use std::str::FromStr;

/// This implementation of `Convertible` lets you pass in a `&str` for the
/// `Unit`, which will parse the chars and convert accordingly. If `expression`
/// is invalid, you'll get an `Error`. If `self`'s `Unit` and `other_unit` are
/// incompatible, you'll get an `Error`.
///
#[cfg_attr(feature = "cffi", ffi_common::derive::expose_impl)]
impl<'a> Convertible<&'a str> for Measurement {
    type Output = Self;
    type ConversionError = Error;

    #[inline]
    fn convert_to(&self, expression: &'a str) -> Result<Self, Self::ConversionError> {
        let dest_unit = Unit::from_str(expression)?;

        if self.is_compatible_with(&dest_unit) {
            Ok(Self {
                value: self.converted_scalar(&dest_unit),
                unit: dest_unit,
            })
        } else {
            Err(Error::IncompatibleUnitTypes {
                lhs: self.unit.expression(),
                rhs: expression.to_string(),
            })
        }
    }
}

/// This implementation of `Convertible` skips any string parsing and gets
/// right to converting to `other_unit`. If `self`'s `Unit` and `other_unit`
/// are incompatible, you'll get an `Error`.
///
impl<'a> Convertible<&'a Unit> for Measurement {
    type Output = Self;
    type ConversionError = Error;

    #[inline]
    fn convert_to(&self, rhs: &'a Unit) -> Result<Self, Self::ConversionError> {
        if self.is_compatible_with(rhs) {
            Ok(Self {
                value: self.converted_scalar(rhs),
                unit: rhs.clone(),
            })
        } else {
            Err(Error::IncompatibleUnitTypes {
                lhs: self.unit.expression(),
                rhs: rhs.expression(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::const_units::{KILOMETER, METER};

    use super::*;
    use approx::{assert_relative_eq, assert_ulps_eq};

    macro_rules! try_convert {
        ($subject:expr, $unit:expr, $expected_value:expr) => {
            let converted = $subject.convert_to($unit).unwrap();
            assert_eq!(converted, $subject);
            assert_relative_eq!(converted.value, $expected_value);
            assert_ulps_eq!(converted.value, $expected_value);
        };
    }

    #[test]
    fn validate_convert_to_meter_to_meter_str() {
        let meter = Measurement::try_new(1.0, "m").unwrap();
        try_convert!(meter, "m", 1.0);
    }

    #[test]
    fn validate_convert_to_meter_to_2meter_str() {
        let meter = Measurement::try_new(1.0, "m").unwrap();
        try_convert!(meter, "2m", 0.5);
    }

    #[test]
    fn validate_convert_to_meter_to_km_str() {
        let meter = Measurement::try_new(1000.0, "m").unwrap();
        try_convert!(meter, "km", 1.0);
    }

    #[test]
    fn validate_convert_to_meter_to_2km_str() {
        let meter = Measurement::try_new(1000.0, "m").unwrap();
        try_convert!(meter, "2km", 0.5);
    }

    #[test]
    fn validate_convert_to_meter_to_meter_unit() {
        let meter = Measurement::try_new(1.0, "m").unwrap();
        try_convert!(meter, &METER, 1.0);
    }

    #[test]
    fn validate_convert_to_meter_to_2meter_unit() {
        let meter = Measurement::try_new(1.0, "m").unwrap();
        let unit = unit!(term!(Meter, factor: 2));
        try_convert!(meter, &unit, 0.5);
    }

    #[test]
    fn validate_convert_to_meter_to_km_unit() {
        let meter = Measurement::try_new(1000.0, "m").unwrap();
        try_convert!(meter, &KILOMETER, 1.0);
    }

    #[test]
    fn validate_convert_to_meter_to_2km_unit() {
        let meter = Measurement::try_new(1000.0, "m").unwrap();
        let unit = unit!(term!(Kilo, Meter, factor: 2));
        try_convert!(meter, &unit, 0.5);
    }

    #[test]
    fn validate_arbitrary_den_conversions() {
        let m1 = Measurement::try_new(10.0, "[acr_us]/{tree}").unwrap();

        let u1 = Unit::from_str("m2/{tree}").unwrap();
        let u2 = Unit::from_str("m2/{vine}").unwrap();

        let converted = m1.convert_to(&u1).unwrap();

        assert_relative_eq!(converted.value, 40_468.726_098_742_52);
        assert_ulps_eq!(converted.value, 40_468.726_098_742_52);

        assert!(m1.convert_to(&u2).is_err());
    }

    #[test]
    fn validate_arbitrary_num_conversions() {
        let m1 = Measurement::try_new(10.0, "{tree}/[acr_us]").unwrap();

        let u1 = Unit::from_str("{tree}/m2").unwrap();
        let u2 = Unit::from_str("{vine}/m2").unwrap();

        let converted = m1.convert_to(&u1).unwrap();

        assert_relative_eq!(converted.value, 0.002_471_043_930_466_279);
        assert_ulps_eq!(converted.value, 0.002_471_043_930_466_279);

        assert!(m1.convert_to(&u2).is_err());
    }

    #[test]
    fn validate_arbitrary_conversions() {
        let m1 = Measurement::try_new(10.0, "{tree}").unwrap();

        let u1 = Unit::from_str("{tree}").unwrap();
        let u2 = Unit::from_str("{vine}").unwrap();

        let converted = m1.convert_to(&u1).unwrap();

        assert_relative_eq!(converted.value, 10.0);
        assert_ulps_eq!(converted.value, 10.0);

        assert!(m1.convert_to(&u2).is_err());
    }
}
