// use crate::{
//     v2::behavior_traits::{convert, ops},
//     Composable, IsCompatibleWith, Term,
// };

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
