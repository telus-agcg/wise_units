use crate::{field_eq::FieldEq, Term};

//-----------------------------------------------------------------------------
// impl FieldEq
//-----------------------------------------------------------------------------
/// This is for comparing `Term`s to see if they have the exact same fields.
///
/// ```rust
/// use wise_units::{
///     Atom, FieldEq, Prefix, Term,
///     term::variants::{FactorAtom, FactorPrefixAtom, PrefixAtom}
/// };
///
/// // Both of these have the exact same field values.
/// let lhs = Term::FactorPrefixAtom(FactorPrefixAtom::new(123, Prefix::Deci, Atom::Meter));
/// let rhs = lhs.clone();
/// assert!(lhs.field_eq(&rhs));
///
/// // Both of these have the same scalar value, but have have different field values.
/// let lhs = Term::FactorAtom(FactorAtom::new(1000, Atom::Meter));
/// let rhs = Term::PrefixAtom(PrefixAtom::new(Prefix::Kilo, Atom::Meter));
/// assert!(!lhs.field_eq(&rhs));
/// ```
///
impl<'a> FieldEq<'a> for Term {
    #[inline]
    fn field_eq(&self, other: &'a Self) -> bool {
        match (self, other) {
            (Self::Annotation(lhs), Self::Annotation(rhs)) => lhs == rhs,
            (Self::Atom(lhs), Self::Atom(rhs)) => lhs == rhs,
            (Self::AtomAnnotation(lhs), Self::AtomAnnotation(rhs)) => lhs == rhs,
            (Self::AtomExponent(lhs), Self::AtomExponent(rhs)) => lhs == rhs,
            (Self::AtomExponentAnnotation(lhs), Self::AtomExponentAnnotation(rhs)) => lhs == rhs,
            (Self::PrefixAtom(lhs), Self::PrefixAtom(rhs)) => lhs == rhs,
            (Self::PrefixAtomAnnotation(lhs), Self::PrefixAtomAnnotation(rhs)) => lhs == rhs,
            (Self::PrefixAtomExponent(lhs), Self::PrefixAtomExponent(rhs)) => lhs == rhs,
            (Self::PrefixAtomExponentAnnotation(lhs), Self::PrefixAtomExponentAnnotation(rhs)) => {
                lhs == rhs
            }
            (Self::Factor(lhs), Self::Factor(rhs)) => lhs == rhs,
            (Self::FactorAnnotation(lhs), Self::FactorAnnotation(rhs)) => lhs == rhs,
            (Self::FactorExponent(lhs), Self::FactorExponent(rhs)) => lhs == rhs,
            (Self::FactorExponentAnnotation(lhs), Self::FactorExponentAnnotation(rhs)) => {
                lhs == rhs
            }
            (Self::FactorAtom(lhs), Self::FactorAtom(rhs)) => lhs == rhs,
            (Self::FactorAtomAnnotation(lhs), Self::FactorAtomAnnotation(rhs)) => lhs == rhs,
            (Self::FactorAtomExponent(lhs), Self::FactorAtomExponent(rhs)) => lhs == rhs,
            (Self::FactorAtomExponentAnnotation(lhs), Self::FactorAtomExponentAnnotation(rhs)) => {
                lhs == rhs
            }
            (Self::FactorPrefixAtom(lhs), Self::FactorPrefixAtom(rhs)) => lhs == rhs,
            (Self::FactorPrefixAtomAnnotation(lhs), Self::FactorPrefixAtomAnnotation(rhs)) => {
                lhs == rhs
            }
            (Self::FactorPrefixAtomExponent(lhs), Self::FactorPrefixAtomExponent(rhs)) => {
                lhs == rhs
            }
            (
                Self::FactorPrefixAtomExponentAnnotation(lhs),
                Self::FactorPrefixAtomExponentAnnotation(rhs),
            ) => lhs == rhs,
            (
                Self::Annotation(_)
                | Self::Atom(_)
                | Self::AtomAnnotation(_)
                | Self::AtomExponent(_)
                | Self::AtomExponentAnnotation(_)
                | Self::PrefixAtom(_)
                | Self::PrefixAtomAnnotation(_)
                | Self::PrefixAtomExponent(_)
                | Self::PrefixAtomExponentAnnotation(_)
                | Self::Factor(_)
                | Self::FactorAnnotation(_)
                | Self::FactorExponent(_)
                | Self::FactorExponentAnnotation(_)
                | Self::FactorAtom(_)
                | Self::FactorAtomAnnotation(_)
                | Self::FactorAtomExponent(_)
                | Self::FactorAtomExponentAnnotation(_)
                | Self::FactorPrefixAtom(_)
                | Self::FactorPrefixAtomAnnotation(_)
                | Self::FactorPrefixAtomExponent(_)
                | Self::FactorPrefixAtomExponentAnnotation(_),
                _,
            ) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Atom, Prefix};

    #[test]
    fn validate_field_eq() {
        let term = Term::new(None, Some(Atom::Are));
        let other = Term::new(None, Some(Atom::Are));
        assert!(term.field_eq(&other));

        let term = Term::new(None, Some(Atom::Are));
        let other = Term::new(Some(Prefix::Hecto), Some(Atom::Are));
        assert!(!term.field_eq(&other));

        let term = term!(factor: 100, atom: Atom::Are);
        let other = Term::new(Some(Prefix::Hecto), Some(Atom::Are));
        assert!(!term.field_eq(&other));
    }
}
