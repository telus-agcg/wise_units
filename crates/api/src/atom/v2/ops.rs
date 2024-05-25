use approx::ulps_eq;

use crate::{
    v2::{dim::IsCommensurableWith, ops::CommensurableEq},
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
}
