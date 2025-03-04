use std::cmp::Ordering;

use approx::ulps_eq;

use crate::{
    v2::{
        dim::IsCommensurableWith,
        ops::{CommensurableEq, CommensurableOrd},
    },
    Composition, Term, UcumUnit,
};

impl CommensurableEq<Composition> for Term {
    fn commensurable_eq(&self, other: &Self) -> Option<bool> {
        if !self.is_commensurable_with(other) {
            return None;
        }

        Some(ulps_eq!(self.scalar(), other.scalar()))
    }
}

impl CommensurableOrd<Composition> for Term {
    fn commensurable_cmp(&self, other: &Self) -> Option<Ordering> {
        if !self.is_commensurable_with(other) {
            return None;
        }

        self.scalar().partial_cmp(&other.scalar())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commensurable_eq_test() {
        let lhs = term!(Meter, factor: 1000);
        let rhs = term!(Kilo, Meter);
        assert_eq!(lhs.commensurable_eq(&rhs), Some(true));

        let lhs = term!(Meter, factor: 10);
        let rhs = term!(Kilo, Meter);
        assert_eq!(lhs.commensurable_eq(&rhs), Some(false));

        let lhs = term!(Meter);
        let rhs = term!(Gram);
        assert!(lhs.commensurable_eq(&rhs).is_none());
    }

    #[test]
    fn commensurable_ord_test() {
        let lhs = term!(Meter, factor: 1000);
        let rhs = term!(Kilo, Meter);
        assert_eq!(lhs.commensurable_cmp(&rhs), Some(Ordering::Equal));

        let lhs = term!(Meter, factor: 10);
        assert_eq!(lhs.commensurable_cmp(&rhs), Some(Ordering::Less));

        let lhs = term!(Meter, factor: 1_000_000);
        assert_eq!(lhs.commensurable_cmp(&rhs), Some(Ordering::Greater));

        let lhs = term!(Meter);
        let rhs = term!(Gram);
        assert_eq!(lhs.commensurable_cmp(&rhs), None);
    }
}
