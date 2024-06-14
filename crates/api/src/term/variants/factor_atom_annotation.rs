use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::Atom;

use super::{
    Annotation, Exponent, Factor, FactorAtomExponentAnnotation, PowOutput, SetExponent, Term,
};

// ╭──────────────────────╮
// │ FactorAtomAnnotation │
// ╰──────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorAtomAnnotation {
    pub(crate) factor: Factor,
    pub(crate) atom: Atom,
    pub(crate) annotation: Annotation,
}

impl FactorAtomAnnotation {
    #[must_use]
    pub const fn new(factor: Factor, atom: Atom, annotation: Annotation) -> Self {
        Self {
            factor,
            atom,
            annotation,
        }
    }
}

impl From<FactorAtomAnnotation> for Term {
    fn from(value: FactorAtomAnnotation) -> Self {
        Self::FactorAtomAnnotation(value)
    }
}

impl fmt::Display for FactorAtomAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.factor == 1 {
            write!(f, "{}{}", self.atom, self.annotation)
        } else {
            write!(f, "{}{}{}", self.factor, self.atom, self.annotation)
        }
    }
}

impl SetExponent for FactorAtomAnnotation {
    type Output = PowOutput<Annotation, (), FactorAtomExponentAnnotation>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(mem::take(&mut self.annotation)),
            1 => PowOutput::One(()),
            _ => PowOutput::Rest(FactorAtomExponentAnnotation {
                factor: self.factor,
                atom: self.atom,
                exponent,
                annotation: mem::take(&mut self.annotation),
            }),
        }
    }
}

impl Pow<Exponent> for FactorAtomAnnotation {
    type Output = PowOutput<Annotation, Self, FactorAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(faea) => PowOutput::Rest(faea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorAtomAnnotation {
    type Output = PowOutput<Annotation, FactorAtomAnnotation, FactorAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self.clone();

        match s.set_exponent(rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(faea) => PowOutput::Rest(faea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorAtomAnnotation {
    type Output = PowOutput<Annotation, (), FactorAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(rhs)
    }
}

impl Inv for FactorAtomAnnotation {
    type Output = FactorAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a FactorAtomAnnotation {
    type Output = FactorAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut FactorAtomAnnotation {
    type Output = FactorAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}
