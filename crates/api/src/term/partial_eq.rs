use approx::ulps_eq;

use crate::{is_compatible_with::IsCompatibleWith, ucum_unit::UcumUnit, Term};

/// `Term`s are `PartialEq` if
///
/// a) they are compatible
/// b) their `scalar()` values are equal
///
/// ```rust
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
/// assert!(lhs == rhs);
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
/// assert!(lhs != rhs);
/// ```
///
impl PartialEq for Term {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if !self.is_compatible_with(other) {
            return false;
        }

        ulps_eq!(self.scalar(), other.scalar())
    }
}
