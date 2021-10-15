use super::Measurement;
use crate::{parser::Composable, UcumUnit};
use std::hash::{Hash, Hasher};

/// The implementation here has to match up with that of `PartialEq`, which a) returns false if
/// the two Units are of a different dimension, and b) checks equality of the two Units' `scalar()`
/// values. Since the Hash value must be equal if and only if the two objects are equal, this also
/// has to hash based on the dimension/composition _and_ scalar value of the Units. And again, this
/// is because `wise_units` treats Measurements and such as equal if both their dimensions and
/// scalar values are equal, regardless of their make-up.
///
impl Hash for Measurement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.composition().hash(state);
        self.scalar().to_string().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    fn make_hash(measurement: Measurement) -> u64 {
        let mut hasher = DefaultHasher::new();
        measurement.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_same_dim_same_scalar_same_unit() {
        let lhs_hash = make_hash(Measurement::try_new(1000.0, "m").unwrap());
        let rhs_hash = make_hash(Measurement::try_new(1000.0, "m").unwrap());

        assert_eq!(lhs_hash, rhs_hash);
    }

    #[test]
    fn test_same_dim_same_scalar_different_unit() {
        let lhs_hash = make_hash(Measurement::try_new(1000.0, "m").unwrap());
        let rhs_hash = make_hash(Measurement::try_new(1.0, "km").unwrap());

        assert_eq!(lhs_hash, rhs_hash);
    }

    #[test]
    fn test_same_dim_different_scalar_different_unit() {
        let lhs_hash = make_hash(Measurement::try_new(1000.0, "m").unwrap());
        let rhs_hash = make_hash(Measurement::try_new(1000.0, "km").unwrap());

        assert_ne!(lhs_hash, rhs_hash);
    }

    #[test]
    fn test_same_dim_different_scalar_same_unit() {
        let lhs_hash = make_hash(Measurement::try_new(1.0, "m").unwrap());
        let rhs_hash = make_hash(Measurement::try_new(2.0, "m").unwrap());

        assert_ne!(lhs_hash, rhs_hash);
    }

    #[test]
    fn test_different_dim_same_scalar() {
        let lhs_hash = make_hash(Measurement::try_new(1.0, "m").unwrap());
        let rhs_hash = make_hash(Measurement::try_new(1.0, "g").unwrap());

        assert_ne!(lhs_hash, rhs_hash);
    }

    #[test]
    fn test_different_dim_different_scalar() {
        let lhs_hash = make_hash(Measurement::try_new(1.0, "m").unwrap());
        let rhs_hash = make_hash(Measurement::try_new(2.0, "g").unwrap());

        assert_ne!(lhs_hash, rhs_hash);
    }
}
