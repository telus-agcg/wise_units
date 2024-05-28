use approx::ulps_eq;

use crate::{
    v2::{dim::IsCommensurableWith, ops::CommensurableEq},
    Composition, Measurement, UcumUnit,
};

impl CommensurableEq<Composition> for Measurement {
    fn commensurable_eq(&self, other: &Self) -> Option<bool> {
        if !self.is_commensurable_with(other) {
            return None;
        }

        Some(ulps_eq!(self.scalar(), other.scalar()))
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::const_units::{GRAM, KILOMETER, METER};

    use super::*;

    #[test]
    fn commensurable_eq_test() {
        let lhs = Measurement::new(1000.0, METER);
        assert_eq!(lhs.commensurable_eq(&lhs), Some(true));

        let rhs = Measurement::new(1.0, KILOMETER);
        assert_eq!(lhs.commensurable_eq(&rhs), Some(true));

        let rhs = Measurement::new(1.1, KILOMETER);
        assert_eq!(lhs.commensurable_eq(&rhs), Some(false));

        let rhs = Measurement::new(1000.0, GRAM);
        assert_eq!(lhs.commensurable_eq(&rhs), None);
    }
}
