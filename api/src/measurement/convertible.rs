use convertible::Convertible;
use field_eq::FieldEq;
use measurement::Measurement;
use parser::{Composable, Error};
use std::str::FromStr;
use unit::Unit;

/// This implementation of `Convertible` lets you pass in a `&str` for the
/// `Unit`, which will parse the chars and convert accordingly. If `expression`
/// is invalid, you'll get an `Error`. If `self`'s `Unit` and `other_unit` are
/// incompatible, you'll get an `Error`.
///
impl<'a> Convertible<&'a str> for Measurement {
    type Output = Self;
    type ConversionError = Error;

    fn convert_to(&self, expression: &'a str) -> Result<Self, Self::ConversionError> {
        let other_unit = Unit::from_str(expression)?;

        convert_measurement(self, &other_unit)
    }
}

/// This implementation of `Convertible` skips any string parsing and gets
/// right to converting to `other_unit`. If `self`'s `Unit` and `other_unit`
/// are incompatible, you'll get an `Error`.
///
impl<'a> Convertible<&'a Unit> for Measurement {
    type Output = Self;
    type ConversionError = Error;

    fn convert_to(&self, other_unit: &'a Unit) -> Result<Self, Self::ConversionError> {
        convert_measurement(self, other_unit)
    }
}

fn convert_measurement(lhs: &Measurement, dest_unit: &Unit) -> Result<Measurement, Error> {
    // Short-circuit if `dest_unit` is the same as the Measurement's Unit.
    if lhs.unit.field_eq(dest_unit) {
        return Ok(lhs.clone());
    }

    let source_unit = &lhs.unit;

    if !source_unit.is_compatible_with(&dest_unit) {
        let e = Error::IncompatibleUnitTypes {
            lhs: source_unit.expression(),
            rhs: dest_unit.expression(),
        };
        return Err(e);
    }

    let new_measurement = Measurement {
        value: lhs.converted_scalar(dest_unit),
        unit: dest_unit.clone(),
    };

    Ok(new_measurement)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_convert_to_meter_to_meter() {
        let meter = Measurement::new(1.0, "m").unwrap();
        let other = meter.convert_to("m").unwrap();
        assert_eq!(other, meter);
        assert_eq!(other.value, 1.0);

        let other = meter.convert_to(&meter.unit).unwrap();
        assert_eq!(other, meter);
        assert_eq!(other.value, 1.0);
    }

    #[test]
    fn validate_convert_to_meter_to_2meter() {
        let meter = Measurement::new(1.0, "m").unwrap();
        let mut other = meter.convert_to("m").unwrap();
        other.value = 2.0;
        assert_ne!(other, meter);
    }

    #[test]
    fn validate_convert_to_meter_to_km() {
        let meter = Measurement::new(1000.0, "m").unwrap();
        let other = meter.convert_to("km").unwrap();
        assert_eq!(other, meter);
        assert_eq!(other.value, 1.0);

        let other = meter.convert_to(&other.unit).unwrap();
        assert_eq!(other, meter);
        assert_eq!(other.value, 1.0);
    }

    #[test]
    fn validate_convert_to_meter_to_2km() {
        let meter = Measurement::new(1.0, "m").unwrap();
        let mut other = meter.convert_to("km").unwrap();
        other.value = 2.0;
        assert_ne!(other, meter);
    }
}
