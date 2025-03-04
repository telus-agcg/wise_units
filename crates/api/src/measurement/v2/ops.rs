use std::cmp::Ordering;

use approx::ulps_eq;

use crate::{
    v2::{
        dim::IsCommensurableWith,
        ops::{CommensurableEq, CommensurableOrd},
    },
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

impl CommensurableOrd<Composition> for Measurement {
    fn commensurable_cmp(&self, other: &Self) -> Option<Ordering> {
        if !self.is_commensurable_with(other) {
            return None;
        }

        self.scalar().partial_cmp(&other.scalar())
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::const_units::{
        l1::{KILOMETER, METER},
        m1::GRAM,
    };

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

    #[test]
    fn commensurable_ord_test() {
        let lhs = Measurement::new(1000.0, METER);
        let rhs = Measurement::new(1.0, KILOMETER);
        assert_eq!(lhs.commensurable_cmp(&rhs), Some(Ordering::Equal));

        let rhs = Measurement::new(1.1, KILOMETER);
        assert_eq!(lhs.commensurable_cmp(&rhs), Some(Ordering::Less));

        let rhs = Measurement::new(0.9, KILOMETER);
        assert_eq!(lhs.commensurable_cmp(&rhs), Some(Ordering::Greater));

        let rhs = Measurement::new(1000.0, GRAM);
        assert_eq!(lhs.commensurable_cmp(&rhs), None);
    }
}
