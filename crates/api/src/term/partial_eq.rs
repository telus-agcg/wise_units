use approx::ulps_eq;

use crate::{is_compatible_with::IsCompatibleWith, ucum_unit::UcumUnit, Term};

/// `Term`s are `PartialEq` if
///
/// a) they are compatible
/// b) their `scalar()` values are equal
///
/// ```rust
/// use wise_units::{Atom, Prefix, Term, term::variants::{FactorAtom, PrefixAtom}};
///
/// let lhs = Term::FactorAtom(FactorAtom::new(1000, Atom::Meter));
/// let rhs = Term::PrefixAtom(PrefixAtom::new(Prefix::Kilo, Atom::Meter));
/// assert!(lhs == rhs);
///
/// let lhs = Term::Atom(Atom::Meter);
/// let rhs = Term::PrefixAtom(PrefixAtom::new(Prefix::Kilo, Atom::Meter));
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
