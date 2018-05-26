use measurable::Measurable;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use unit::Unit;
use wise_units_parser::{Composable, Error};

/// A Measurement is the prime interface for consumers of the library. It
/// consists of some scalar value and a `Unit`, where the Unit represents the
/// type of unit.
///
/// # Examples
///
/// ```
/// use wise_units::Measurement;
///
/// let one_km = Measurement::new(1.0, "km").unwrap();
/// let in_meters = one_km.convert_to("m").unwrap();
///
/// // Since we can't assert float values, check that the difference is
/// // negligible.
/// let value_difference = (in_meters.value - 1_000.0).abs();
///
/// assert!(value_difference < 0.000_001);
/// ```
///
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialOrd)]
pub struct Measurement {
    pub value: f64,
    pub unit: Unit,
}

#[cfg(all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"))]
js_serializable!(Measurement);

#[cfg(all(any(target_arch = "wasm32", target_os = "emscripten"), feature = "with_stdweb"))]
js_deserializable!(Measurement);

impl Measurable for Measurement {
    fn get_unit(&self) -> &Unit {
        &self.unit
    }

    fn get_value(&self) -> f64 {
        self.value
    }

    /// This scalar is the Measurement's value combined with any scalars that
    /// are part of the Unit's designation.
    ///
    /// # Examples
    ///
    /// ```
    /// use wise_units::Measurable;
    /// use wise_units::Measurement;
    ///
    /// let five_meters = Measurement::new(5.0, "m").unwrap();
    /// assert_eq!(five_meters.scalar(), 5.0);
    ///
    /// let five_meters_squared = Measurement::new(5.0, "m2").unwrap();
    /// assert_eq!(five_meters_squared.scalar(), 5.0);
    ///
    /// let five_three_meters = Measurement::new(5.0, "[pi].m").unwrap();
    /// assert_eq!(five_three_meters.scalar(), 15.707_963_267_948_966);
    ///
    /// let sixty_five_f = Measurement::new(65.0, "[degF]").unwrap();
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
    /// use wise_units::Measurable;
    /// use wise_units::Measurement;
    ///
    /// let five_meters = Measurement::new(5.0, "m").unwrap();
    /// assert_eq!(five_meters.magnitude(), 5.0);
    ///
    /// let five_meters_squared = Measurement::new(5.0, "m2").unwrap();
    /// assert_eq!(five_meters_squared.magnitude(), 5.0);
    ///
    /// let five_three_meters = Measurement::new(5.0, "[pi].m").unwrap();
    /// assert_eq!(five_three_meters.magnitude(), 5.0);
    ///
    /// let sixty_five_f = Measurement::new(65.0, "[degF]").unwrap();
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
    pub fn new(value: f64, expression: &str) -> Result<Self, Error> {
        let unit = Unit::from_str(expression)?;

        let m = Measurement { value, unit };

        Ok(m)
    }

    /// Converts the Measurement to another unit type. That type is specified
    /// using a str of characters that represents the other unit type: ex.
    /// `"m2/s"`.
    ///
    pub fn convert_to<'a>(&self, expression: &'a str) -> Result<Measurement, Error> {
        let other_terms = ::wise_units_parser::parse(expression)?;
        let other_unit = Unit { terms: other_terms };

        if self.unit == other_unit {
            return Ok(self.clone());
        }

        let my_unit = &self.unit;

        if !my_unit.is_compatible_with(&other_unit) {
            let e = Error::IncompatibleUnitTypes {
                lhs: self.unit.expression(),
                rhs: expression.to_owned(),
            };
            return Err(e);
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
    /// let km = Measurement::new(1.0, "km").unwrap();
    /// assert_eq!(km.unit_string(), "km".to_string());
    /// ```
    ///
    pub fn unit_string(&self) -> String {
        self.unit.to_string()
    }

    /// Really this is just to comply with Unitwise's API; not really sure how
    /// useful it is.
    ///
    pub fn to_f64(&self) -> f64 {
        self.value
    }

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

    /// Multiplies the `Measurement`'s scalar by `scalar` and returns a new
    /// `Measurement`.
    ///
    pub fn mul_scalar(&self, scalar: f64) -> Measurement {
        let new_value = self.value * scalar;

        Measurement {
            value: new_value,
            unit: self.unit.clone(),
        }
    }

    /// Divides the `Measurement`'s scalar by `scalar` and returns a new
    /// `Measurement`.
    ///
    pub fn div_scalar(&self, scalar: f64) -> Measurement {
        let new_value = self.value / scalar;

        Measurement {
            value: new_value,
            unit: self.unit.clone(),
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
    type Output = Result<Measurement, Error>;

    fn add(self, other: Measurement) -> Self::Output {
        let unit = self.unit_string();
        let other_converted = other.convert_to(&unit)?;
        let new_value = self.value + other_converted.value;

        Measurement::new(new_value, &unit)
    }
}

impl<'a> Add for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn add(self, other: &'a Measurement) -> Self::Output {
        let unit = self.unit_string();
        let other_converted = other.convert_to(&unit)?;
        let new_value = self.value + other_converted.value;

        Measurement::new(new_value, &unit)
    }
}

impl<'a> Add for &'a mut Measurement {
    type Output = Result<Measurement, Error>;

    fn add(self, other: &'a mut Measurement) -> Self::Output {
        let unit = self.unit_string();
        let other_converted = other.convert_to(&unit)?;
        let new_value = self.value + other_converted.value;

        Measurement::new(new_value, &unit)
    }
}

impl Sub for Measurement {
    type Output = Result<Measurement, Error>;

    fn sub(self, other: Measurement) -> Self::Output {
        let unit = self.unit_string();
        let other_converted = other.convert_to(&unit)?;
        let new_value = self.value - other_converted.value;

        Measurement::new(new_value, &unit)
    }
}

impl<'a> Sub for &'a Measurement {
    type Output = Result<Measurement, Error>;

    fn sub(self, other: &'a Measurement) -> Self::Output {
        let unit = self.unit_string();
        let other_converted = other.convert_to(&unit)?;
        let new_value = self.value - other_converted.value;

        Measurement::new(new_value, &unit)
    }
}

impl<'a> Sub for &'a mut Measurement {
    type Output = Result<Measurement, Error>;

    fn sub(self, other: &'a mut Measurement) -> Self::Output {
        let unit = self.unit_string();
        let other_converted = other.convert_to(&unit)?;
        let new_value = self.value + -other_converted.value;

        Measurement::new(new_value, &unit)
    }
}

impl Mul for Measurement {
    type Output = Measurement;

    fn mul(self, other: Measurement) -> Self::Output {
        let new_value = self.value * other.value;
        let new_unit = self.unit * other.unit;

        Measurement {
            value: new_value,
            unit: new_unit,
        }
    }
}

impl<'a> Mul for &'a Measurement {
    type Output = Measurement;

    fn mul(self, other: &'a Measurement) -> Self::Output {
        let new_value = self.value * other.value;
        let new_unit = &self.unit * &other.unit;

        Measurement {
            value: new_value,
            unit: new_unit,
        }
    }
}

impl<'a> Mul for &'a mut Measurement {
    type Output = Measurement;

    fn mul(self, other: &'a mut Measurement) -> Self::Output {
        let new_value = self.value * other.value;
        let new_unit = &self.unit * &other.unit;

        Measurement {
            value: new_value,
            unit: new_unit,
        }
    }
}

impl Div for Measurement {
    type Output = Measurement;

    fn div(self, other: Measurement) -> Self::Output {
        let new_value = self.value / other.value;
        let new_unit = self.unit / other.unit;

        Measurement {
            value: new_value,
            unit: new_unit,
        }
    }
}

impl<'a> Div for &'a Measurement {
    type Output = Measurement;

    fn div(self, other: &'a Measurement) -> Self::Output {
        let new_value = self.value / other.value;
        let new_unit = &self.unit / &other.unit;

        Measurement {
            value: new_value,
            unit: new_unit,
        }
    }
}

impl<'a> Div for &'a mut Measurement {
    type Output = Measurement;

    fn div(self, other: &'a mut Measurement) -> Self::Output {
        let new_value = self.value / other.value;
        let new_unit = &self.unit / &other.unit;

        Measurement {
            value: new_value,
            unit: new_unit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use unit::Unit;
    use wise_units_parser::{Atom, Term};

    #[test]
    fn validate_new() {
        let m = Measurement::new(1.0, "m").unwrap();

        let expected_unit = Unit {
            terms: vec![term!(Meter)],
        };

        assert_eq!(m.value, 1.0);
        assert_eq!(m.unit, expected_unit);
    }

    #[test]
    fn validate_convert_to_meter_to_meter() {
        let meter = Measurement::new(1.0, "m").unwrap();
        let other = meter.convert_to("m").unwrap();
        assert_eq!(other, meter);
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
        let meter = Measurement::new(1.0, "m").unwrap();
        let other = meter.convert_to("km").unwrap();
        assert_eq!(other, meter);
    }

    #[test]
    fn validate_convert_to_meter_to_2km() {
        let meter = Measurement::new(1.0, "m").unwrap();
        let mut other = meter.convert_to("km").unwrap();
        other.value = 2.0;
        assert_ne!(other, meter);
    }

    #[test]
    fn validate_display() {
        assert_eq!(
            Measurement::new(1.1, "m").unwrap().to_string(),
            "1.1m".to_string()
        );
        assert_eq!(
            Measurement::new(1.1, "m2").unwrap().to_string(),
            "1.1m2".to_string()
        );
        assert_eq!(
            Measurement::new(1.1, "km2").unwrap().to_string(),
            "1.1km2".to_string()
        );
        assert_eq!(
            Measurement::new(1.1, "km2/s").unwrap().to_string(),
            "1.1km2/s".to_string()
        );
        assert_eq!(
            Measurement::new(1.1, "km2/rad.s").unwrap().to_string(),
            "1.1km2/rad.s".to_string()
        );
    }

    #[test]
    fn validate_eq_same_unit() {
        let m1 = Measurement::new(1.0, "m").unwrap();
        let m2 = Measurement::new(1.0, "m").unwrap();
        assert_eq!(&m1, &m2);

        let m2 = Measurement::new(1.1, "m").unwrap();
        assert_ne!(m1, m2);
    }

    #[test]
    fn validate_eq_unit_with_prefix() {
        let m = Measurement::new(1000.0, "m").unwrap();
        let km = Measurement::new(1.0, "km").unwrap();
        assert_eq!(&m, &km);

        let km = Measurement::new(1.1, "km").unwrap();
        assert_ne!(&m, &km);
    }

    #[test]
    fn validate_eq_different_unit() {
        let m = Measurement::new(1.0, "m").unwrap();
        let s = Measurement::new(1.0, "s").unwrap();
        assert_ne!(&m, &s);
    }

    #[test]
    fn validate_add() {
        let m1 = Measurement::new(1.0, "m").unwrap();
        let m2 = Measurement::new(2.0, "m").unwrap();
        let m3 = Measurement::new(3.0, "m").unwrap();
        assert_eq!((&m1 + &m2).unwrap(), m3);
        assert_eq!((m1 + m2).unwrap(), m3);
    }

    #[test]
    fn validate_sub() {
        let m1 = Measurement::new(1.0, "m").unwrap();
        let m2 = Measurement::new(2.0, "m").unwrap();
        let m3 = Measurement::new(-1.0, "m").unwrap();
        assert_eq!((&m1 - &m2).unwrap(), m3);
        assert_eq!((m1 - m2).unwrap(), m3);
    }

    // TODO: This doesn't make sense to me, but is in parity with Unitwise.
    #[test]
    fn validate_mul() {
        let m1 = Measurement::new(2.0, "m").unwrap();
        let m2 = Measurement::new(3.0, "m").unwrap();
        let r = &m1 * &m2;

        let terms = r.unit.terms;
        assert_eq!(terms.len(), 2);

        let first_term = term!(Meter);
        assert_eq!(terms[0], first_term);
        assert_eq!(terms[1], first_term);
    }

    // TODO: This doesn't make sense to me, but is in parity with Unitwise.
    #[test]
    fn validate_div() {
        let m1 = Measurement::new(10.0, "m").unwrap();
        let m2 = Measurement::new(2.0, "m").unwrap();
        let r = &m1 / &m2;

        let terms = r.unit.terms;
        assert_eq!(terms.len(), 2);

        let first_term = term!(Meter);
        assert_eq!(terms[0], first_term);

        let last_term = term!(Meter, exponent: -1);
        assert_eq!(terms[1], last_term);
    }

    #[test]
    fn validate_mul_scalar() {
        let m1 = Measurement::new(10.0, "m").unwrap();
        let m2 = Measurement::new(200.0, "m").unwrap();
        assert_eq!(m1.mul_scalar(20.0), m2);
    }

    #[test]
    fn validate_div_scalar() {
        let m1 = Measurement::new(10.0, "m").unwrap();
        let m2 = Measurement::new(2.0, "m").unwrap();
        assert_eq!(m1.div_scalar(5.0), m2);
    }

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Measurement;
        use serde_json;
        use unit::Unit;
        use wise_units_parser::{Atom, Prefix, Term};

        #[test]
        fn validate_serialization_empty_terms() {
            let unit = Unit { terms: vec![] };
            let measurement = Measurement {
                value: 123.4,
                unit: unit,
            };
            let expected_json = r#"{"value":123.4,"unit":{"terms":[]}}"#;

            let j =
                serde_json::to_string(&measurement).expect("Couldn't convert Unit to JSON String");

            assert_eq!(expected_json, j);
        }

        #[test]
        fn validate_serialization_full_terms() {
            let expected_json = r#"{
                "value":123.4,
                "unit":{
                    "terms":[{
                        "atom": "Meter",
                        "prefix": "Centi",
                        "factor": 100,
                        "exponent": 456,
                        "annotation": "stuff"
                    }, {
                        "atom": "Gram",
                        "prefix": null,
                        "factor": 1,
                        "exponent": -4,
                        "annotation": null
                    }]
                }
            }"#.replace("\n", "")
                .replace(" ", "");

            let term1 = term!(Centi, Meter, factor: 100, exponent: 456, annotation: Some("stuff".to_string()));
            let term2 = term!(Gram, exponent: -4);

            let unit = Unit {
                terms: vec![term1, term2],
            };
            let measurement = Measurement {
                value: 123.4,
                unit: unit,
            };

            let j =
                serde_json::to_string(&measurement).expect("Couldn't convert Unit to JSON String");

            assert_eq!(expected_json, j);
        }

        #[test]
        fn validate_deserialization_empty_terms() {
            let json = r#"{"value":1.0, "unit":{"terms": []}}"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Unit");

            let unit = Unit { terms: vec![] };
            let expected_measurement = Measurement {
                value: 1.0,
                unit: unit,
            };

            assert_eq!(expected_measurement, k);
        }

        #[test]
        fn validate_deserialization_full_terms() {
            let json = r#"{
                "value":432.1,
                "unit":{
                    "terms":[{
                        "atom": "Meter",
                        "prefix": "Centi",
                        "factor": 100,
                        "exponent": 456,
                        "annotation": "stuff"
                    }, {
                        "atom": "Gram",
                        "prefix": null,
                        "factor": 1,
                        "exponent": -4,
                        "annotation": null
                    }]
                }
            }"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Unit");

            let term1 = term!(Centi, Meter, factor: 100, exponent: 456, annotation: Some("stuff".to_string()));
            let term2 = term!(Gram, exponent: 4);

            let unit = Unit {
                terms: vec![term1, term2],
            };
            let expected_measurement = Measurement {
                value: 432.1,
                unit: unit,
            };

            assert_eq!(expected_measurement, k);
        }
    }
}
