use crate::{
    v2::behavior_traits::{convert, ops},
    IsCompatibleWith, Term,
};

impl<'a> ops::Comparable<'a, f64> for Term {
    /// Overriding because current implementation of `IsCompatibleWith` also checks if the
    /// annotations are equal, thus leaving, say `{tree}` and `{bush}` to be treated
    /// as separate units.
    ///
    fn is_compatible_with(&'a self, rhs: &'a Self) -> bool {
        IsCompatibleWith::is_compatible_with(self, rhs)
    }

    /// Overriding default to use `ulps_eq` for comparing `f64`
    ///
    fn is_commensurable_with(&'a self, rhs: &'a Self) -> bool {
        use convert::ToScalar;

        if !ops::Comparable::is_compatible_with(self, rhs) {
            return false;
        }

        approx::ulps_eq!(self.to_scalar(), rhs.to_scalar())
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;
    use crate::{
        parser::{Atom, Prefix},
        v2::behavior_traits::convert::ToScalar,
    };

    #[test]
    fn is_compatible_with_test() {
        let lhs = term!(Meter);
        let rhs = term!(Kilo, Meter);
        assert!(ops::Comparable::is_compatible_with(&lhs, &rhs));

        let m = term!(Meter, annotation: "stuff".to_string());
        let km_stuff = term!(Kilo, Meter, annotation: "stuff".to_string());
        assert!(ops::Comparable::is_compatible_with(&m, &km_stuff));
    }

    #[test]
    fn is_commensurable_with_test() {
        let lhs = term!(Meter, factor: 1000);
        let rhs = term!(Kilo, Meter);
        assert!(ops::Comparable::is_commensurable_with(&lhs, &rhs));

        let lhs = term!(Meter);
        assert!(!ops::Comparable::is_commensurable_with(&lhs, &rhs));

        let m = term!(Meter, factor: 1000, annotation: "stuff".to_string());
        let km_stuff = term!(Kilo, Meter, annotation: "stuff".to_string());
        assert!(ops::Comparable::is_commensurable_with(&m, &km_stuff));

        let m = term!(Meter, factor: 1000, annotation: "123".to_string());
        let km_stuff = term!(Kilo, Meter, annotation: "456".to_string());
        dbg!(m.to_scalar());
        dbg!(km_stuff.to_scalar());
        assert!(!ops::Comparable::is_commensurable_with(&m, &km_stuff));
    }

    #[test]
    fn commensurable_ord_test() {
        let lhs = term!(Meter, factor: 1000);
        let rhs = term!(Kilo, Meter);
        assert_eq!(
            ops::Comparable::commensurable_ord(&lhs, &rhs),
            Some(Ordering::Equal)
        );

        let lhs = term!(Meter);
        assert_eq!(
            ops::Comparable::commensurable_ord(&lhs, &rhs),
            Some(Ordering::Less)
        );

        let lhs = term!(Meter, factor: 1000, annotation: "stuff".to_string());
        let rhs = term!(Kilo, Meter, annotation: "stuff".to_string());
        assert_eq!(
            ops::Comparable::commensurable_ord(&lhs, &rhs),
            Some(Ordering::Equal)
        );

        let lhs = term!(Meter, factor: 1000, annotation: "123".to_string());
        let rhs = term!(Kilo, Meter, annotation: "456".to_string());
        // These should be treated as not comparable because their annotations make them such.
        // This is not consistent with the spec.
        assert_eq!(ops::Comparable::commensurable_ord(&lhs, &rhs), None);
    }
}
