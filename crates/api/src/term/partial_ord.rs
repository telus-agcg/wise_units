use crate::{is_compatible_with::IsCompatibleWith, ucum_unit::UcumUnit, Term};

/// `Term`s are `PartialOrd` if
///
/// a) they are compatible
/// b) their `scalar()` values are comparable
///
/// ```rust
/// use std::cmp::Ordering;
/// use wise_units::{Atom, Prefix, Term};
///
/// let lhs = Term {
///     factor: Some(1000),
///     prefix: None,
///     atom: Some(Atom::Meter),
///     exponent: None,
///     annotation: None
/// };
/// let rhs = Term {
///     factor: None,
///     prefix: Some(Prefix::Kilo),
///     atom: Some(Atom::Meter),
///     exponent: None,
///     annotation: None
/// };
/// assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Equal));
///
/// let lhs = Term {
///     factor: None,
///     prefix: None,
///     atom: Some(Atom::Meter),
///     exponent: None,
///     annotation: None
/// };
/// let rhs = Term {
///     factor: None,
///     prefix: Some(Prefix::Kilo),
///     atom: Some(Atom::Meter),
///     exponent: None,
///     annotation: None
/// };
/// assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Less));
///
/// let lhs = Term {
///     factor: None,
///     prefix: None,
///     atom: Some(Atom::Meter),
///     exponent: None,
///     annotation: None
/// };
/// let rhs = Term {
///     factor: None,
///     prefix: None,
///     atom: Some(Atom::Gram),
///     exponent: None,
///     annotation: None
/// };
/// assert_eq!(lhs.partial_cmp(&rhs), None);
/// ```
///
impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if !self.is_compatible_with(other) {
            return None;
        }

        self.scalar().partial_cmp(&other.scalar())
    }
}
