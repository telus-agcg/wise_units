use field_eq::FieldEq;
use measurement::Measurement;

/// This is for comparing `Measurement`s to see if they have both the same
/// `value` *and* the same underlying `Unit` defined in the exact same terms.
///
/// ```rust
/// use wise_units::{FieldEq, Measurement};
///
/// // Both of these have the same value and are defined in the same terms.
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(1.0, "m").unwrap();
/// assert!(measurement.field_eq(&other));
///
/// // These have the same magnitude, but otherwise are never equal.
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(!measurement.field_eq(&other));
///
/// // These scalar values are equal, but since the units are in different
/// // terms, they are not `field_eq`.
/// let measurement = Measurement::new(1.0, "1000m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(!measurement.field_eq(&other));
///
/// // Neither the values nor unit terms are equal, thus are not `field_eq` here.
/// let measurement = Measurement::new(1000.0, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(!measurement.field_eq(&other));
/// ```
///
impl<'a> FieldEq<'a> for Measurement {
    fn field_eq(&self, other: &'a Self) -> bool {
        self.value == other.value && self.unit.field_eq(&other.unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_field_eq() {
        let measurement = Measurement::new(1.0, "ar").unwrap();
        let other = Measurement::new(1.0, "ar").unwrap();
        assert!(measurement.field_eq(&other));

        let measurement = Measurement::new(1.0, "ar").unwrap();
        let other = Measurement::new(1.0, "har").unwrap();
        assert!(!measurement.field_eq(&other));

        let measurement = Measurement::new(1.0, "100ar").unwrap();
        let other = Measurement::new(1.0, "har").unwrap();
        assert!(!measurement.field_eq(&other));

        let measurement = Measurement::new(100.0, "ar").unwrap();
        let other = Measurement::new(1.0, "har").unwrap();
        assert!(!measurement.field_eq(&other));
    }
}
