use crate::{field_eq::FieldEq, Term};

//-----------------------------------------------------------------------------
// impl FieldEq
//-----------------------------------------------------------------------------
/// This is for comparing `Term`s to see if they have the exact same fields.
///
/// ```rust
/// use wise_units::{Atom, FieldEq, Prefix, Term};
///
/// // Both of these have the exact same field values.
/// let lhs = Term {
///     factor: Some(123),
///     prefix: Some(Prefix::Deci),
///     atom: Some(Atom::Meter),
///     exponent: None,
///     annotation: None
/// };
/// let rhs = Term {
///     factor: Some(123),
///     prefix: Some(Prefix::Deci),
///     atom: Some(Atom::Meter),
///     exponent: None,
///     annotation: None
/// };
/// assert!(lhs.field_eq(&rhs));
///
/// // Both of these have the same scalar value, but have have different field values.
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
/// assert!(!lhs.field_eq(&rhs));
/// ```
///
impl<'a> FieldEq<'a> for Term {
    #[inline]
    fn field_eq(&self, other: &'a Self) -> bool {
        self.factor == other.factor
            && self.prefix == other.prefix
            && self.atom == other.atom
            && self.exponent == other.exponent
            && self.annotation == other.annotation
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
