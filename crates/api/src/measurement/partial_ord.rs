use crate::{is_compatible_with::IsCompatibleWith, measurement::Measurement, ucum_unit::UcumUnit};
use std::cmp::Ordering;

//-----------------------------------------------------------------------------
// impl PartialOrd
//-----------------------------------------------------------------------------
/// This allows for comparing `Measurement`s based on their reduced scalar
/// values.
///
/// ```rust
/// use std::cmp::Ordering;
/// use wise_units::Measurement;
///
/// let measurement = Measurement::try_new(1.0, "m").unwrap();
/// let other = Measurement::try_new(1.0, "m").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Equal);
///
/// let measurement = Measurement::try_new(1.0, "m").unwrap();
/// let other = Measurement::try_new(2.0, "m").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Less);
/// assert!(measurement < other);
///
/// let measurement = Measurement::try_new(2.0, "m").unwrap();
/// let other = Measurement::try_new(1.0, "m").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Greater);
/// assert!(measurement > other);
///
/// let measurement = Measurement::try_new(1000.0, "m").unwrap();
/// let other = Measurement::try_new(1.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Equal);
///
/// let measurement = Measurement::try_new(1000.0, "m").unwrap();
/// let other = Measurement::try_new(2.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Less);
/// assert!(measurement < other);
///
/// let measurement = Measurement::try_new(1000.1, "m").unwrap();
/// let other = Measurement::try_new(1.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Greater);
/// assert!(measurement > other);
///
/// let measurement = Measurement::try_new(1.0, "1000m").unwrap();
/// let other = Measurement::try_new(1.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Equal);
///
/// let measurement = Measurement::try_new(1.0, "1000m").unwrap();
/// let other = Measurement::try_new(2.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Less);
/// assert!(measurement < other);
///
/// let measurement = Measurement::try_new(1.1, "1000m").unwrap();
/// let other = Measurement::try_new(1.0, "km").unwrap();
/// assert_eq!(measurement.partial_cmp(&other).unwrap(), Ordering::Greater);
/// assert!(measurement > other);
/// ```
///
impl PartialOrd for Measurement {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !self.unit.is_compatible_with(&other.unit) {
            return None;
        }

        let other_scalar = other.scalar();
        let my_scalar = self.scalar();

        my_scalar.partial_cmp(&other_scalar)
    }
}
