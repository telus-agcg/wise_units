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

pub(crate) trait TermReduce: crate::composable::ComposablyEq<Term> {
    fn term_reduce(&self, rhs: &Term) -> ReducedTerm {
        match self.composably_eq(rhs) {
            Some(0) => ReducedTerm::ReducedAway,
            Some(exponent) => ReducedTerm::ReducedToTerm(self.build(exponent)),
            None => ReducedTerm::NotReducible,
        }
    }
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
