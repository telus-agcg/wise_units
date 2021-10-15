use super::Unit;
use crate::{parser::Composable, UcumUnit};
use std::hash::{Hash, Hasher};

/// The implementation here has to match up with that of `PartialEq`, which a) returns false if
/// the two Units are of a different dimension, and b) checks equality of the two Units' `scalar()`
/// values. Since the Hash value must be equal if and only if the two objects are equal, this also
/// has to hash based on the dimension/composition _and_ scalar value of the Units. And again, this
/// is because `wise_units` treats Measurements and such as equal if both their dimensions and
/// scalar values are equal, regardless of their make-up.
///
impl Hash for Unit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.composition().hash(state);
        self.scalar().to_string().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::hash_map::DefaultHasher, str::FromStr};

    #[allow(clippy::needless_pass_by_value)]
    fn make_hash(unit: Unit) -> u64 {
        let mut hasher = DefaultHasher::new();
        unit.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_same_dim_same_scalar_same_unit() {
        let lhs_hash = make_hash(Unit::from_str("m").unwrap());
        let rhs_hash = make_hash(Unit::from_str("m").unwrap());

        assert_eq!(lhs_hash, rhs_hash);
    }

    #[test]
    fn test_same_dim_same_scalar_different_unit() {
        let lhs_hash = make_hash(Unit::from_str("1000m").unwrap());
        let rhs_hash = make_hash(Unit::from_str("km").unwrap());

        assert_eq!(lhs_hash, rhs_hash);
    }

    #[test]
    fn test_same_dim_different_scalar_different_unit() {
        let lhs_hash = make_hash(Unit::from_str("m").unwrap());
        let rhs_hash = make_hash(Unit::from_str("km").unwrap());

        assert_ne!(lhs_hash, rhs_hash);
    }

    #[test]
    fn test_same_dim_different_scalar_same_unit() {
        let lhs_hash = make_hash(Unit::from_str("1000m").unwrap());
        let rhs_hash = make_hash(Unit::from_str("m").unwrap());

        assert_ne!(lhs_hash, rhs_hash);
    }

    #[test]
    fn test_different_dim_same_scalar() {
        let lhs_hash = make_hash(Unit::from_str("m").unwrap());
        let rhs_hash = make_hash(Unit::from_str("g").unwrap());

        assert_ne!(lhs_hash, rhs_hash);
    }

    #[test]
    fn test_different_dim_different_scalar() {
        let lhs_hash = make_hash(Unit::from_str("m").unwrap());
        let rhs_hash = make_hash(Unit::from_str("1000g").unwrap());

        assert_ne!(lhs_hash, rhs_hash);
    }
}
