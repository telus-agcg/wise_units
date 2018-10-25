use crate::field_eq::FieldEq;
use crate::unit::Unit;

//-----------------------------------------------------------------------------
// impl FieldEq
//-----------------------------------------------------------------------------
/// This is for comparing `Unit`s to see if they have the exact same `Term`s.
///
/// ```rust
/// use std::str::FromStr;
/// use wise_units::{FieldEq, Unit};
///
/// // Both of these are defined in the same terms.
/// let unit = Unit::from_str("m").unwrap();
/// let other = Unit::from_str("m").unwrap();
/// assert!(unit.field_eq(&other));
///
/// // These are never equal.
/// let unit = Unit::from_str("m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert!(!unit.field_eq(&other));
///
/// // These scalar values are equal, but since the units are in different
/// // terms, they are not `field_eq`.
/// let unit = Unit::from_str("1000m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert!(!unit.field_eq(&other));
/// ```
///
impl<'a> FieldEq<'a> for Unit {
    #[inline]
    fn field_eq(&self, other: &'a Self) -> bool {
        self.terms == other.terms
    }
}

#[cfg(test)]
mod tests {
    use crate::field_eq::FieldEq;
    use std::str::FromStr;
    use crate::unit::Unit;

    #[test]
    fn validate_field_eq() {
        let unit = Unit::from_str("ar").unwrap();
        let other = Unit::from_str("ar").unwrap();
        assert!(unit.field_eq(&other));

        let unit = Unit::from_str("ar").unwrap();
        let other = Unit::from_str("har").unwrap();
        assert!(!unit.field_eq(&other));

        let unit = Unit::from_str("100ar").unwrap();
        let other = Unit::from_str("har").unwrap();
        assert!(!unit.field_eq(&other));
    }
}
