use crate::{field_eq::FieldEq, unit::Unit, Term, UcumUnit};
use std::cmp::Ordering;

//-----------------------------------------------------------------------------
// impl FieldEq
//-----------------------------------------------------------------------------
/// This is for comparing `Unit`s to see if they have the exact same `Term`s.
///
/// ```rust
/// use std::str::FromStr;
/// use wise_units::{FieldEq, Unit};
///
/// // Both of these are defined in the same terms.
/// let unit = Unit::from_str("m").unwrap();
/// let other = Unit::from_str("m").unwrap();
/// assert!(unit.field_eq(&other));
///
/// // These are never equal.
/// let unit = Unit::from_str("m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert!(!unit.field_eq(&other));
///
/// // These scalar values are equal, but since the units are in different
/// // terms, they are not `field_eq`.
/// let unit = Unit::from_str("1000m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert!(!unit.field_eq(&other));
/// ```
///
impl<'a> FieldEq<'a> for Unit {
    #[inline]
    fn field_eq(&self, other: &'a Self) -> bool {
        fn sort_em(terms: &[Term]) -> Vec<Term> {
            let mut t = terms.to_vec();

            t.sort_by(|a, b| match a.scalar().partial_cmp(&b.scalar()) {
                Some(ordering) => ordering,
                None => Ordering::Equal,
            });

            t
        }

        if self.terms.len() != other.terms.len() {
            return false;
        }

        let lhs = sort_em(&self.terms);
        let rhs = sort_em(&other.terms);

        for (l, r) in lhs.into_iter().zip(rhs.into_iter()) {
            if !l.field_eq(&r) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{field_eq::FieldEq, unit::Unit};
    use std::str::FromStr;

    #[test]
    fn validate_field_eq() {
        let unit = Unit::from_str("ar").unwrap();
        let other = Unit::from_str("ar").unwrap();
        assert!(unit.field_eq(&other));

        let unit = Unit::from_str("ar").unwrap();
        let other = Unit::from_str("har").unwrap();
        assert!(!unit.field_eq(&other));

        let unit = Unit::from_str("100ar").unwrap();
        let other = Unit::from_str("har").unwrap();
        assert!(!unit.field_eq(&other));

        let unit = Unit::from_str("[acr_us].[in_i]").unwrap();
        let other = Unit::from_str("[in_i].[acr_us]").unwrap();
        assert!(unit.field_eq(&other));
    }
}
