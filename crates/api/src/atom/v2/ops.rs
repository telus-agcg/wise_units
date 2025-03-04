use std::cmp::Ordering;

use approx::ulps_eq;

use crate::{
    v2::{
        dim::IsCommensurableWith,
        ops::{CommensurableEq, CommensurableOrd},
    },
    Atom, Composition, UcumUnit,
};

impl CommensurableEq<Composition> for Atom {
    fn commensurable_eq(&self, other: &Self) -> Option<bool> {
        if !self.is_commensurable_with(other) {
            return None;
        }

        Some(ulps_eq!(self.scalar(), other.scalar()))
    }
}

impl CommensurableOrd<Composition> for Atom {
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
        let lhs = Atom::Meter;

        assert_eq!(lhs.commensurable_eq(&lhs), Some(true));
        assert_eq!(lhs.commensurable_eq(&Atom::FootInternational), Some(false));
        assert!(lhs.commensurable_eq(&Atom::Gram).is_none());
    }

    #[test]
    fn commensurable_ord_test() {
        let lhs = Atom::Liter;
        let rhs = Atom::LiterSecondary;
        assert_eq!(lhs.commensurable_cmp(&rhs), Some(Ordering::Equal));

        let rhs = Atom::QueenAnnesWineGallonUS;
        assert_eq!(lhs.commensurable_cmp(&rhs), Some(Ordering::Less));
        assert_eq!(rhs.commensurable_cmp(&lhs), Some(Ordering::Greater));

        let lhs = Atom::Meter;
        let rhs = Atom::Gram;
        assert_eq!(lhs.commensurable_cmp(&rhs), None);
    }
}
