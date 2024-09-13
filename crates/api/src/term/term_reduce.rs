use crate::Term;

use super::{
    variants::{AtomExponent, FactorExponent},
    Exponent,
};

pub(crate) enum ReducedTerm {
    ReducedToTerm(Term),
    ReducedAway,
    NotReducible,
}

/// This trait helps for implementing `Term` reducing/canceling across `Term` and its variants.
///
pub(crate) trait TermReduce: crate::composable::ComposablyEq<Term> {
    ///  It relies on the `ComposablyEq` trait to determine if `self` and `rhs` are of the same
    ///  prefix+atom combination. `wise_units` is conservative in this algorithm in that it won't
    ///  reduce/cancel `Term`s that are the same dimension but different prefix+atom combos, as
    ///  that falls more into the "unit conversion" category.
    ///
    ///  1. If `self` and `rhs` are `ComposablyEq` and their resulting exponent is 0, then
    ///     `term_reduce` considers `self` and `rhs` to have canceled each other out.
    ///  2. If `self` and `rhs` are `ComposablyEq` and their resulting exponent is non-zero, then
    ///     the two `Term`s have been combined into a new `Term` with the resulting exponent.
    ///  3. If `self` and `rhs` are not `ComposablyEq`, they can't be reduced.
    ///
    fn term_reduce(&self, rhs: &Term) -> ReducedTerm {
        match self.composably_eq(rhs) {
            Some(0) => ReducedTerm::ReducedAway,
            Some(exponent) => ReducedTerm::ReducedToTerm(self.build(exponent)),
            None => ReducedTerm::NotReducible,
        }
    }

    /// This determines how to build a new `Term` in the case that `self` and `rhs` are
    /// `ComposablyEq` and their resulting exponent is non-zero. For example, it allows handling
    /// the case when `self` is a non-exponent type (ex. `Term::Atom`), but `exponent` is non-one;
    /// in this case, the resulting `Term` should be a `Term::AtomExponent`.
    ///
    fn build(&self, exponent: Exponent) -> Term;
}

impl TermReduce for Term {
    fn build(&self, exponent: Exponent) -> Term {
        match self {
            Self::Annotation(lhs) => lhs.build(exponent),
            Self::Atom(lhs) => lhs.build(exponent),
            Self::AtomAnnotation(lhs) => lhs.build(exponent),
            Self::AtomExponent(lhs) => lhs.build(exponent),
            Self::AtomExponentAnnotation(lhs) => lhs.build(exponent),
            Self::PrefixAtom(lhs) => lhs.build(exponent),
            Self::PrefixAtomAnnotation(lhs) => lhs.build(exponent),
            Self::PrefixAtomExponent(lhs) => lhs.build(exponent),
            Self::PrefixAtomExponentAnnotation(lhs) => lhs.build(exponent),
            Self::Factor(factor) => {
                let factor = *factor;
                if exponent == 1 {
                    Self::Factor(factor)
                } else {
                    Self::FactorExponent(FactorExponent { factor, exponent })
                }
            }
            Self::FactorAnnotation(lhs) => lhs.build(exponent),
            Self::FactorExponent(lhs) => lhs.build(exponent),
            Self::FactorExponentAnnotation(lhs) => lhs.build(exponent),
            Self::FactorAtom(lhs) => lhs.build(exponent),
            Self::FactorAtomAnnotation(lhs) => lhs.build(exponent),
            Self::FactorAtomExponent(lhs) => lhs.build(exponent),
            Self::FactorAtomExponentAnnotation(lhs) => lhs.build(exponent),
            Self::FactorPrefixAtom(lhs) => lhs.build(exponent),
            Self::FactorPrefixAtomAnnotation(lhs) => lhs.build(exponent),
            Self::FactorPrefixAtomExponent(lhs) => lhs.build(exponent),
            Self::FactorPrefixAtomExponentAnnotation(lhs) => lhs.build(exponent),
        }
    }
}

impl TermReduce for crate::Atom {
    fn build(&self, exponent: Exponent) -> Term {
        if exponent == 1 {
            Term::Atom(*self)
        } else {
            Term::AtomExponent(AtomExponent {
                atom: *self,
                exponent,
            })
        }
    }
}
