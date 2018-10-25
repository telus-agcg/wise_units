use crate::parser::IsCompatibleWith;
use crate::ucum_unit::UcumUnit;
use crate::unit::Unit;

//-----------------------------------------------------------------------------
// impl PartialOrd
//-----------------------------------------------------------------------------
/// This allows for comparing `Units`s based on their reduced scalar values.
///
/// ```rust
/// use std::cmp::Ordering;
/// use std::str::FromStr;
/// use wise_units::Unit;
///
/// let unit = Unit::from_str("m").unwrap();
/// let other = Unit::from_str("m").unwrap();
/// assert_eq!(unit.partial_cmp(&other).unwrap(), Ordering::Equal);
///
/// let unit = Unit::from_str("m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert_eq!(unit.partial_cmp(&other).unwrap(), Ordering::Less);
/// assert!(unit < other);
///
/// let unit = Unit::from_str("km").unwrap();
/// let other = Unit::from_str("m").unwrap();
/// assert_eq!(unit.partial_cmp(&other).unwrap(), Ordering::Greater);
/// assert!(unit > other);
///
/// let unit = Unit::from_str("1000m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert_eq!(unit.partial_cmp(&other).unwrap(), Ordering::Equal);
///
/// let unit = Unit::from_str("999m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert_eq!(unit.partial_cmp(&other).unwrap(), Ordering::Less);
/// assert!(unit < other);
///
/// let unit = Unit::from_str("1001m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert_eq!(unit.partial_cmp(&other).unwrap(), Ordering::Greater);
/// assert!(unit > other);
/// ```
///
impl PartialOrd for Unit {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        if !self.is_compatible_with(other) {
            return None;
        }

        let other_scalar = other.scalar();
        let my_scalar = self.scalar();

        my_scalar.partial_cmp(&other_scalar)
    }
}
