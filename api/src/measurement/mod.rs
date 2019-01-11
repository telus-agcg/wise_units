pub mod composable;
pub mod convertible;
pub mod field_eq;
pub mod to_reduced;
pub mod is_compatible_with;
pub mod ops;
pub mod partial_eq;
pub mod partial_ord;
pub mod reducible;
pub mod ucum_unit;

use crate::parser::Error;
use crate::reducible::Reducible;
use crate::ucum_unit::UcumUnit;
use crate::unit::Unit;
use std::str::FromStr;

/// A Measurement is the prime interface for consumers of the library. It
/// consists of some scalar value and a `Unit`, where the Unit represents the
/// type of unit.
///
/// # Examples
///
/// ```
/// use wise_units::{Convertible, Measurement};
///
/// let one_km = Measurement::new(1.0, "km").unwrap();
/// let in_meters = one_km.convert_to("m").unwrap();
///
/// assert!(in_meters.value == 1000.0);
/// ```
///
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Measurement {
    pub value: f64,
    pub unit: Unit,
}

impl Measurement {
    /// Creates a new `Measurement` by parsing `expression` into a `Unit`.
    ///
    #[inline]
    pub fn new(value: f64, expression: &str) -> Result<Self, Error> {
        let unit = Unit::from_str(expression)?;

        let m = Self { value, unit };

        Ok(m)
    }

    /// The value of the `Measurement` in terms of `other_unit`. Only used for
    /// converting, and does not check the compatibility of units.
    ///
    fn converted_scalar(&self, other_unit: &Unit) -> f64 {
        if self.is_special() && other_unit.is_special() {
            let ts = self.unit.reduce_value(self.value);
            other_unit.calculate_magnitude(ts)
        } else if self.is_special() {
            self.unit.reduce_value(self.value)
        } else if other_unit.is_special() {
            other_unit.calculate_magnitude(self.value)
        } else {
            self.scalar() / other_unit.reduce_value(1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::parser::{Atom, Term};
    use super::*;
    use crate::unit::Unit;
    use std::str::FromStr;

    #[test]
    fn validate_new() {
        let m = Measurement::new(1.0, "m").unwrap();

        assert_eq!(m.value, 1.0);
        assert_eq!(m.unit, vec![term!(Meter)].into());
    }

    #[test]
    fn validate_converted_scalar() {
        // No special units
        let m = Measurement::new(1.0, "m").unwrap();
        let unit = Unit::from_str("m").unwrap();
        assert_eq!(m.converted_scalar(&unit), 1.0);

        let m = Measurement::new(1.0, "m").unwrap();
        let unit = Unit::from_str("km").unwrap();
        assert_eq!(m.converted_scalar(&unit), 0.001);

        let m = Measurement::new(1000.0, "m").unwrap();
        let unit = Unit::from_str("km").unwrap();
        assert_eq!(m.converted_scalar(&unit), 1.0);

        // Measurement unit is not special, but other_unit is
        let m = Measurement::new(1.0, "K").unwrap();
        let unit = Unit::from_str("Cel").unwrap();
        assert_eq!(m.converted_scalar(&unit), -272.15);

        // Measurement unit is special, but other_unit is not
        let m = Measurement::new(1.0, "Cel").unwrap();
        let unit = Unit::from_str("K").unwrap();
        assert_eq!(m.converted_scalar(&unit), 274.15);

        // Measurement unit and other_unit are special
        let m = Measurement::new(1.0, "Cel").unwrap();
        let unit = Unit::from_str("[degF]").unwrap();
        assert_eq!(m.converted_scalar(&unit), 33.799_999_999_999_955);
    }

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Measurement;
        use crate::parser::{Atom, Prefix, Term};
        use serde_json;

        #[test]
        fn validate_serialization_empty_terms() {
            let measurement = Measurement { value: 123.4, unit: vec![].into() };
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
                        "factor": null,
                        "exponent": -4,
                        "annotation": null
                    }]
                }
            }"#
            .replace("\n", "")
            .replace(" ", "");

            let term1 =
                term!(Centi, Meter, factor: 100, exponent: 456, annotation: "stuff".to_string());
            let term2 = term!(Gram, exponent: -4);

            let unit = vec![term1, term2].into();
            let measurement = Measurement { value: 123.4, unit };

            let j =
                serde_json::to_string(&measurement).expect("Couldn't convert Unit to JSON String");

            assert_eq!(expected_json, j);
        }

        #[test]
        fn validate_deserialization_empty_terms() {
            let json = r#"{"value":1.0, "unit":{"terms": []}}"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Unit");

            let unit = vec![].into();
            let expected_measurement = Measurement { value: 1.0, unit };

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

            let term1 =
                term!(Centi, Meter, factor: 100, exponent: 456, annotation: "stuff".to_string());
            let term2 = term!(Gram, exponent: -4);

            let unit = vec![term1, term2].into();
            let expected_measurement = Measurement { value: 432.1, unit };

            assert_eq!(expected_measurement, k);
        }
    }
}
