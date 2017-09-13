use interpreter::Interpreter;
use measurable::Measurable;
use std::fmt;
use std::ops::{Add, Div, Mul};
use unit::Unit;

/// A Measurement is the prime interface for consumers of the library. It
/// consists of some scalar value and a `Unit`, where the Unit represents the
/// type of unit.
///
/// # Examples
///
/// ```
/// use wise_units::Measurement;
///
/// let one_km = Measurement::new(1.0, "km");
/// let in_meters = one_km.convert_to("m").unwrap();
///
/// // Since we can't assert float values, check that the difference is
/// // negligible.
/// let value_difference = (in_meters.value - 1_000.0).abs();
///
/// assert!(value_difference < 0.000_001);
/// ```
///
#[derive(Debug)]
pub struct Measurement {
    pub value: f64,
    pub unit: Unit,
}

/// Errors when trying to convert between types that aren't commensurable.
///
#[derive(Debug)]
pub enum ConversionError {
    IncompatibleUnitTypes,
}

impl Measurable for Measurement {
    fn get_unit(&self) -> &Unit { &self.unit }

    fn get_value(&self) -> f64 { self.value }

    /// This scalar is the Measurement's value combined with any scalars that
    /// are part of the Unit's designation.
    ///
    /// # Examples
    ///
    /// ```
    /// use wise_units::Measurement;
    /// use wise_units::Measurable;
    ///
    /// let five_meters = Measurement::new(5.0, "m");
    /// assert_eq!(five_meters.scalar(), 5.0);
    ///
    /// let five_meters_squared = Measurement::new(5.0, "m2");
    /// assert_eq!(five_meters_squared.scalar(), 5.0);
    ///
    /// let five_three_meters = Measurement::new(5.0, "[pi].m");
    /// assert_eq!(five_three_meters.scalar(), 15.707_963_267_948_966);
    ///
    /// let sixty_five_f = Measurement::new(65.0, "[degF]");
    /// assert!((sixty_five_f.scalar() - 291.483_333).abs() < 0.000_001);
    /// ```
    ///
    fn scalar(&self) -> f64 {
        if self.is_special() {
            self.unit.calculate_scalar(self.value)
        } else {
            self.value * self.unit.calculate_scalar(1.0)
        }
    }

    /// This magnitude is the Measurement's value combined with any magnitude
    /// that is part of the Unit's designation.
    ///
    /// # Examples
    ///
    /// ```
    /// use wise_units::Measurement;
    /// use wise_units::Measurable;
    ///
    /// let five_meters = Measurement::new(5.0, "m");
    /// assert_eq!(five_meters.magnitude(), 5.0);
    ///
    /// let five_meters_squared = Measurement::new(5.0, "m2");
    /// assert_eq!(five_meters_squared.magnitude(), 5.0);
    ///
    /// let five_three_meters = Measurement::new(5.0, "[pi].m");
    /// assert_eq!(five_three_meters.magnitude(), 5.0);
    ///
    /// let sixty_five_f = Measurement::new(65.0, "[degF]");
    /// assert!((sixty_five_f.magnitude() - 65.0).abs() < 0.000_001);
    /// ```
    ///
    fn magnitude(&self) -> f64 {
        if self.is_special() {
            let scalar = self.scalar();
            self.unit.calculate_magnitude(scalar)
        } else {
            self.value * self.unit.calculate_magnitude(1.0)
        }
    }
}

impl Measurement {
    pub fn new(value: f64, expression: &str) -> Self {
        // TODO: Decouple parser and interpreter
        let mut interpreter = Interpreter;
        let su = interpreter.interpret(expression);

        Measurement {
            value: value,
            unit: su,
        }
    }

    /// Converts the Measurement to another unit type. That type is specified
    /// using a str of characters that represents the other unit type: ex.
    /// `"m2/s"`.
    ///
    pub fn convert_to<'a>(&self, expression: &'a str) -> Result<Measurement, ConversionError> {
        let my_unit = &self.unit;

        let mut interpreter = Interpreter;
        let other_unit = interpreter.interpret(expression);

        if !my_unit.is_compatible_with(&other_unit) {
            return Err(ConversionError::IncompatibleUnitTypes);
        }

        let new_measurement = Measurement {
            value: self.converted_scalar(&other_unit),
            unit: other_unit,
        };

        Ok(new_measurement)
    }

    /// The Measurement's Unit as a String.
    ///
    /// # Example
    ///
    /// ```
    /// use wise_units::Measurement;
    /// let km = Measurement::new(1.0, "km");
    /// assert_eq!(km.unit_string(), "km".to_string());
    /// ```
    ///
    pub fn unit_string(&self) -> String { self.unit.to_string() }

    fn converted_scalar(&self, other_unit: &Unit) -> f64 {
        if self.is_special() && other_unit.is_special() {
            let ts = self.unit.calculate_scalar(self.value);
            other_unit.calculate_magnitude(ts)
        } else if self.is_special() {
            self.unit.calculate_scalar(self.value)
        } else if other_unit.is_special() {
            other_unit.calculate_magnitude(self.value)
        } else {
            self.scalar() / other_unit.calculate_scalar(1.0)
        }
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

impl PartialEq for Measurement {
    fn eq(&self, other: &Self) -> bool {
        let my_unit_string = self.unit_string();

        if let Ok(converted_other) = other.convert_to(&my_unit_string) {
            self.to_string() == converted_other.to_string()
        } else {
            false
        }
    }
}

impl Add for Measurement {
    type Output = Measurement;

    fn add(self, other: Measurement) -> Measurement {
        let unit = self.unit_string();
        let other_converted = other.convert_to(&unit).unwrap();
        let new_value = self.value + other_converted.value;

        Measurement {
            value: new_value,
            unit: self.unit,
        }
    }
}

impl<'pointer> Add for &'pointer Measurement {
    type Output = Measurement;

    fn add(self, other: &'pointer Measurement) -> Measurement {
        let unit = self.unit_string();
        let other_converted = other.convert_to(&unit).unwrap();
        let new_value = self.value + other_converted.value;

        Measurement::new(new_value, &unit)
    }
}

impl Div for Measurement {
    type Output = Measurement;

    fn div(self, other: Measurement) -> Measurement {
        let unit = self.unit_string();
        let other_converted = other.convert_to(&unit).unwrap();
        let new_value = self.value / other_converted.value;

        Measurement {
            value: new_value,
            unit: self.unit,
        }
    }
}

impl Mul for Measurement {
    type Output = Measurement;

    fn mul(self, other: Measurement) -> Measurement {
        let unit = self.unit_string();
        let other_converted = other.convert_to(&unit).unwrap();
        let new_value = self.value * other_converted.value;

        Measurement {
            value: new_value,
            unit: self.unit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atom::Atom;
    use term::Term;
    use unit::Unit;

    #[test]
    fn validate_new() {
        let m = Measurement::new(1.0, "m");

        let expected_unit = Unit {
            terms: vec![Term::new(Some(Atom::Meter), None)],
        };

        assert_eq!(m.value, 1.0);
        assert_eq!(m.unit, expected_unit);
    }

    #[test]
    fn validate_convert_to_meter_to_meter() {
        let meter = Measurement::new(1.0, "m");
        let other = meter.convert_to("m").unwrap();
        assert_eq!(other, meter);
    }

    #[test]
    fn validate_convert_to_meter_to_2meter() {
        let meter = Measurement::new(1.0, "m");
        let mut other = meter.convert_to("m").unwrap();
        other.value = 2.0;
        assert_ne!(other, meter);
    }

    #[test]
    fn validate_convert_to_meter_to_km() {
        let meter = Measurement::new(1.0, "m");
        let other = meter.convert_to("km").unwrap();
        assert_eq!(other, meter);
    }

    #[test]
    fn validate_convert_to_meter_to_2km() {
        let meter = Measurement::new(1.0, "m");
        let mut other = meter.convert_to("km").unwrap();
        other.value = 2.0;
        assert_ne!(other, meter);
    }

    #[test]
    fn validate_display() {
        assert_eq!(Measurement::new(1.0, "meter").to_string(), "1m".to_string());
        assert_eq!(Measurement::new(1.1, "m").to_string(), "1.1m".to_string());
        assert_eq!(Measurement::new(1.1, "m2").to_string(), "1.1m2".to_string());
        assert_eq!(
            Measurement::new(1.1, "km2").to_string(),
            "1.1km2".to_string()
        );
        assert_eq!(
            Measurement::new(1.1, "km2/s").to_string(),
            "1.1km2/s".to_string()
        );
        assert_eq!(
            Measurement::new(1.1, "km2/rad.s").to_string(),
            "1.1km2/rad.s".to_string()
        );
    }

    #[test]
    fn validate_eq_same_unit() {
        let m1 = Measurement::new(1.0, "m");
        let m2 = Measurement::new(1.0, "m");
        assert_eq!(&m1, &m2);

        let m2 = Measurement::new(1.1, "m");
        assert_ne!(m1, m2);
    }

    #[test]
    fn validate_eq_unit_with_prefix() {
        let m = Measurement::new(1000.0, "m");
        let km = Measurement::new(1.0, "km");
        assert_eq!(&m, &km);

        let km = Measurement::new(1.1, "km");
        assert_ne!(&m, &km);
    }

    #[test]
    fn validate_eq_different_unit() {
        let m = Measurement::new(1.0, "m");
        let s = Measurement::new(1.0, "s");
        assert_ne!(&m, &s);
    }
}
