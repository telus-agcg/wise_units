use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{Atom, Prefix};

use super::{
    Annotation, AssignFactor, Exponent, Factor, FactorPrefixAtomAnnotation, PowOutput,
    PrefixAtomExponentAnnotation, SetExponent, Term,
};

// ╭──────────────────────╮
// │ PrefixAtomAnnotation │
// ╰──────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixAtomAnnotation {
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) annotation: Annotation,
}

impl PrefixAtomAnnotation {
    #[must_use]
    pub const fn new(prefix: Prefix, atom: Atom, annotation: Annotation) -> Self {
        Self {
            prefix,
            atom,
            annotation,
        }
    }
}

impl From<PrefixAtomAnnotation> for Term {
    fn from(value: PrefixAtomAnnotation) -> Self {
        Self::PrefixAtomAnnotation(value)
    }
}

impl<'a> From<&'a mut PrefixAtomExponentAnnotation> for PrefixAtomAnnotation {
    fn from(value: &'a mut PrefixAtomExponentAnnotation) -> Self {
        Self {
            prefix: value.prefix,
            atom: value.atom,
            annotation: mem::take(&mut value.annotation),
        }
    }
}

impl fmt::Display for PrefixAtomAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.prefix, self.atom, self.annotation)
    }
}

impl AssignFactor for PrefixAtomAnnotation {
    type Output = FactorPrefixAtomAnnotation;

    fn assign_factor(&mut self, factor: Factor) -> Self::Output {
        FactorPrefixAtomAnnotation {
            factor,
            prefix: self.prefix,
            atom: self.atom,
            annotation: mem::take(&mut self.annotation),
        }
    }
}

impl SetExponent for PrefixAtomAnnotation {
    type Output = PowOutput<Annotation, (), PrefixAtomExponentAnnotation>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(mem::take(&mut self.annotation)),
            1 => PowOutput::One(()),
            _ => PowOutput::Rest(PrefixAtomExponentAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: mem::take(&mut self.annotation),
            }),
        }
    }
}

impl Pow<Exponent> for PrefixAtomAnnotation {
    type Output = PowOutput<Annotation, Self, PrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(paea) => PowOutput::Rest(paea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a PrefixAtomAnnotation {
    type Output = PowOutput<Annotation, PrefixAtomAnnotation, PrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self.clone();

        match s.set_exponent(rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(paea) => PowOutput::Rest(paea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut PrefixAtomAnnotation {
    type Output = PowOutput<Annotation, (), PrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(rhs)
    }
}

impl Inv for PrefixAtomAnnotation {
    type Output = PrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a PrefixAtomAnnotation {
    type Output = PrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut PrefixAtomAnnotation {
    type Output = PrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}
