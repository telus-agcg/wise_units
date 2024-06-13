use crate::{is_compatible_with::IsCompatibleWith, ucum_unit::UcumUnit, Term};

/// `Term`s are `PartialOrd` if
///
/// a) they are compatible
/// b) their `scalar()` values are comparable
///
/// ```rust
/// use std::cmp::Ordering;
/// use wise_units::{Atom, Prefix, Term, term::variants::{FactorAtom, PrefixAtom}};
///
/// let lhs = Term::FactorAtom(FactorAtom::new(1000, Atom::Meter));
/// let rhs = Term::PrefixAtom(PrefixAtom::new(Prefix::Kilo, Atom::Meter));
/// assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Equal));
///
/// let lhs = Term::Atom(Atom::Meter);
/// let rhs = Term::PrefixAtom(PrefixAtom::new(Prefix::Kilo, Atom::Meter));
/// assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Less));
///
/// let lhs = Term::Atom(Atom::Meter);
/// let rhs = Term::Atom(Atom::Gram);
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
