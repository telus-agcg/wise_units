use approx::ulps_eq;

use crate::{
    v2::{dim::IsCommensurableWith, ops::CommensurableEq},
    Composition, UcumUnit, Unit,
};

impl CommensurableEq<Composition> for Unit {
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
        let lhs = unit!(term!(Meter, factor: 1000));
        let rhs = unit!(term!(Kilo, Meter));
        assert_eq!(lhs.commensurable_eq(&rhs), Some(true));

        let lhs = unit!(term!(Meter, factor: 10));
        let rhs = unit!(term!(Kilo, Meter));
        assert_eq!(lhs.commensurable_eq(&rhs), Some(false));

        let lhs = unit!(term!(Meter));
        let rhs = unit!(term!(Gram));
        assert!(lhs.commensurable_eq(&rhs).is_none());
    }
}
