use parser::IsCompatibleWith;
use ucum_unit::UcumUnit;
use unit::Unit;

//-----------------------------------------------------------------------------
// impl PartialEq
//-----------------------------------------------------------------------------
/// `Unit`s are `PartialEq` if
///
/// a) they are compatible
/// b) their `scalar()` values are equal
///
/// ```rust
/// use std::str::FromStr;
/// use wise_units::Unit;
///
/// let unit = Unit::from_str("m").unwrap();
/// let other = Unit::from_str("m").unwrap();
/// assert!(unit == other);
///
/// let unit = Unit::from_str("m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert!(unit != other);
///
/// let unit = Unit::from_str("1000m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert!(unit == other);
/// ```
///
impl PartialEq for Unit {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if !self.is_compatible_with(other) {
            return false;
        }

        self.scalar() == other.scalar()
    }
}
