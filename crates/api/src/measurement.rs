mod composable;
mod convert;
mod convertible;
mod display;
mod field_eq;
mod invert;
mod is_compatible_with;
mod num_traits;
mod ops;
mod partial_eq;
mod partial_ord;
mod reducible;
mod to_reduced;
mod ucum_unit;

use crate::{reducible::Reducible, ucum_unit::UcumUnit, unit::Unit};

use ::num_traits::One;

#[cfg(feature = "cffi")]
use ffi_common::derive::FFI;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A Measurement is the prime interface for consumers of the library. It
/// consists of some scalar value and a `Unit`, where the Unit represents the
/// type of unit.
///
/// # Examples
///
/// ```
/// use wise_units::{Convertible, Measurement};
///
/// let one_km = Measurement::try_new(1.0, "km").unwrap();
/// let in_meters = one_km.convert_to("m").unwrap();
///
/// assert_eq!(in_meters.value(), 1000.0);
/// ```
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "cffi", derive(FFI))]
#[derive(Clone, Debug)]
pub struct Measurement {
    value: f64,
    unit: Unit,
}

impl Measurement {
    /// Creates a new `Measurement` by converting `value` to an `f64` and `unit` to a `Unit`.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if `unit` can't be converted to a `Unit`.
    ///
    #[inline]
    pub fn try_new<V, U, E>(value: V, unit: U) -> Result<Self, E>
    where
        f64: From<V>,
        Unit: TryFrom<U, Error = E>,
    {
        let unit = Unit::try_from(unit)?;

        Ok(Self {
            value: f64::from(value),
            unit,
        })
    }

    /// Standard constructor.
    ///
    /// ```
    /// use wise_units::Measurement;
    ///
    /// let m1 = Measurement::try_new(10.0, "m").unwrap();
    /// let m2 = Measurement::new(10.0, m1.unit().clone());
    ///
    /// assert_eq!(m1, m2);
    /// ```
    ///
    #[must_use]
    pub const fn new(value: f64, unit: Unit) -> Self {
        Self { value, unit }
    }

    /// Accessor for the value, or magnitude, of the measurement.
    ///
    #[must_use]
    #[inline]
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Accessor for the `Unit` used for the measurement.
    ///
    #[must_use]
    #[inline]
    pub const fn unit(&self) -> &Unit {
        &self.unit
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
            self.scalar() / other_unit.reduce_value(One::one())
        }
    }
}

impl AsRef<Self> for Measurement {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::parser::{Atom, Term},
        *,
    };
    use crate::unit::Unit;
    use approx::{assert_relative_eq, assert_ulps_eq};
    use std::{convert::Infallible, str::FromStr};

    #[test]
    fn validate_new() {
        let m = Measurement::try_new(1.0, "m").unwrap();

        assert_relative_eq!(m.value, 1.0);
        assert_ulps_eq!(m.value, 1.0);
        assert_eq!(m.unit, Unit::new(vec![term!(Meter)]));

        let m = Measurement::try_new(1.0, Unit::from_str("m").unwrap()).unwrap();

        assert_relative_eq!(m.value, 1.0);
        assert_ulps_eq!(m.value, 1.0);
        assert_eq!(m.unit, Unit::new(vec![term!(Meter)]));

        {
            struct Meter;

            impl TryFrom<Meter> for Unit {
                type Error = Infallible;

                fn try_from(_value: Meter) -> Result<Self, Self::Error> {
                    Ok(Self::new(vec![Term::new(None, Some(Atom::Meter))]))
                }
            }

            let m = Measurement::try_new(1.0, Meter).unwrap();

            assert_relative_eq!(m.value, 1.0);
            assert_ulps_eq!(m.value, 1.0);
            assert_eq!(m.unit, Unit::new(vec![term!(Meter)]));
        }
    }

    #[test]
    fn validate_converted_scalar() {
        // No special units
        let m = Measurement::try_new(1.0, "m").unwrap();
        let unit = Unit::from_str("m").unwrap();
        assert_relative_eq!(m.converted_scalar(&unit), 1.0);
        assert_ulps_eq!(m.converted_scalar(&unit), 1.0);

        let m = Measurement::try_new(1.0, "m").unwrap();
        let unit = Unit::from_str("km").unwrap();
        assert_relative_eq!(m.converted_scalar(&unit), 0.001);
        assert_ulps_eq!(m.converted_scalar(&unit), 0.001);

        let m = Measurement::try_new(1000.0, "m").unwrap();
        let unit = Unit::from_str("km").unwrap();
        assert_relative_eq!(m.converted_scalar(&unit), 1.0);
        assert_ulps_eq!(m.converted_scalar(&unit), 1.0);

        // Measurement unit is not special, but other_unit is
        let m = Measurement::try_new(1.0, "K").unwrap();
        let unit = Unit::from_str("Cel").unwrap();
        assert_relative_eq!(m.converted_scalar(&unit), -272.15);
        assert_ulps_eq!(m.converted_scalar(&unit), -272.15);

        // Measurement unit is special, but other_unit is not
        let m = Measurement::try_new(1.0, "Cel").unwrap();
        let unit = Unit::from_str("K").unwrap();
        assert_relative_eq!(m.converted_scalar(&unit), 274.15);
        assert_ulps_eq!(m.converted_scalar(&unit), 274.15);

        // Measurement unit and other_unit are special
        let m = Measurement::try_new(1.0, "Cel").unwrap();
        let unit = Unit::from_str("[degF]").unwrap();
        assert_relative_eq!(m.converted_scalar(&unit), 33.799_999_999_999_955);
        assert_ulps_eq!(m.converted_scalar(&unit), 33.799_999_999_999_955);
    }

    #[cfg(feature = "serde")]
    mod serde {
        use crate::Measurement;

        fn expected_measurement() -> Measurement {
            Measurement::try_new(432.1, "100cm456{stuff}/g4").unwrap()
        }

        fn validate_measurement(expected_measurement: &Measurement, expected_json: &str) {
            let json = serde_json::to_string(&expected_measurement)
                .expect("Couldn't convert Measurement to JSON String");
            assert_eq!(expected_json, json);
        }

        fn validate_json(expected_json: &str, expected_measurement: &Measurement) {
            let measurement: Measurement = serde_json::from_str(expected_json).unwrap();
            assert_eq!(&measurement, expected_measurement);
        }

        #[test]
        fn validate_serde_json_full_unit() {
            let expected_measurement = expected_measurement();
            let expected_json = r#"{"value":432.1,"unit":"100cm456{stuff}/g4"}"#;
            validate_measurement(&expected_measurement, expected_json);
            validate_json(expected_json, &expected_measurement);
        }

        #[test]
        fn validate_serde_json_empty_unit_terms() {
            let expected_measurement = Measurement::try_new(2.0, "1").unwrap();
            let expected_json = r#"{"value":2.0,"unit":"1"}"#;
            validate_measurement(&expected_measurement, expected_json);
            validate_json(expected_json, &expected_measurement);
        }

        #[test]
        fn validate_deserialize_json_integer_value() {
            let expected_json = r#"{"value":2,"unit":"m"}"#;
            let expected_measurement = Measurement::try_new(2.0, "m").unwrap();
            validate_json(expected_json, &expected_measurement);
        }

        #[test]
        fn validate_deserialize_json_errors_bad_value_type() {
            let expected_json = r#"{"value":"adsf","unit":"m"}"#;
            let measurement: Result<Measurement, serde_json::Error> =
                serde_json::from_str(expected_json);
            assert!(measurement.is_err());
        }

        #[test]
        fn validate_deserialize_json_errors_empty_unit() {
            let expected_json = r#"{"value":2.0,"unit":""}"#;
            let measurement: Result<Measurement, serde_json::Error> =
                serde_json::from_str(expected_json);
            assert!(measurement.is_err());
        }

        #[allow(box_pointers)]
        #[test]
        fn validate_bincode_serde() {
            let expected_measurement = Measurement::try_new(123.4, "100cm456{stuff}/g4").unwrap();
            let encoded: Vec<u8> = bincode::serialize(&expected_measurement).unwrap();
            let decoded: Measurement = bincode::deserialize(&encoded).unwrap();

            assert_eq!(expected_measurement, decoded);
        }

        #[test]
        fn validate_message_pack_serde() {
            use rmp_serde::{Deserializer, Serializer};
            use serde::{Deserialize, Serialize};

            let expected_measurement = expected_measurement();
            let mut buf = Vec::new();
            expected_measurement
                .serialize(&mut Serializer::new(&mut buf))
                .unwrap();

            assert_eq!(buf.len(), 29);

            let mut de = Deserializer::new(&buf[..]);
            assert_eq!(
                expected_measurement,
                Deserialize::deserialize(&mut de).unwrap()
            );
        }
    }

    #[cfg(feature = "cffi")]
    mod cffi {
        use super::*;
        use ffi_common::core;

        #[test]
        fn test_derived_ffi() {
            unsafe {
                let scalar = 123.456;
                let expression = "kg/[lb_av]";
                let unit = crate::unit::custom_ffi::unit_init(core::ffi_string!(expression));
                let unit_for_measurement = crate::unit::custom_ffi::clone_unit(unit).cast_mut();
                let measurement =
                    measurement_ffi::measurement_rust_ffi_init(scalar, unit_for_measurement);
                let retrieved_value = measurement_ffi::get_measurement_value(measurement);
                let retrieved_unit = measurement_ffi::get_measurement_unit(measurement);

                approx::assert_relative_eq!(scalar, retrieved_value);
                assert_eq!(*unit, *retrieved_unit);

                measurement_ffi::measurement_rust_ffi_free(measurement);
                crate::unit::unit_ffi::unit_free(retrieved_unit);
                crate::unit::unit_ffi::unit_free(unit);
            }
        }
    }
}
