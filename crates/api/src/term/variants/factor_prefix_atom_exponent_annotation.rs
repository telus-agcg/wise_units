use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{Atom, Prefix};

use super::{
    Annotation, Exponent, Factor, FactorPrefixAtomAnnotation, InvOutput, PowOutput, SetExponent,
    Term, UnassignExponent,
};

// ╭────────────────────────────────────╮
// │ FactorPrefixAtomExponentAnnotation │
// ╰────────────────────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorPrefixAtomExponentAnnotation {
    pub(crate) factor: Factor,
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
    pub(crate) annotation: Annotation,
}

impl FactorPrefixAtomExponentAnnotation {
    #[must_use]
    pub const fn new(
        factor: Factor,
        prefix: Prefix,
        atom: Atom,
        exponent: Exponent,
        annotation: Annotation,
    ) -> Self {
        Self {
            factor,
            prefix,
            atom,
            exponent,
            annotation,
        }
    }
}

impl From<FactorPrefixAtomExponentAnnotation> for Term {
    fn from(value: FactorPrefixAtomExponentAnnotation) -> Self {
        Self::FactorPrefixAtomExponentAnnotation(value)
    }
}

impl fmt::Display for FactorPrefixAtomExponentAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.factor, self.exponent) {
            (1, 1) => {
                write!(
                    f,
                    "{prefix}{atom}{annotation}",
                    prefix = self.prefix,
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
            (1, exponent) => {
                write!(
                    f,
                    "{prefix}{atom}{exponent}{annotation}",
                    prefix = self.prefix,
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
            (factor, 1) => {
                write!(
                    f,
                    "{factor}{prefix}{atom}{annotation}",
                    prefix = self.prefix,
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
            (factor, exponent) => {
                write!(
                    f,
                    "{factor}{prefix}{atom}{exponent}{annotation}",
                    prefix = self.prefix,
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
        }
    }
}

impl SetExponent for FactorPrefixAtomExponentAnnotation {
    type Output = PowOutput<Annotation, FactorPrefixAtomAnnotation, ()>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(mem::take(&mut self.annotation)),
            1 => PowOutput::One(FactorPrefixAtomAnnotation {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                annotation: mem::take(&mut self.annotation),
            }),
            _ => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

impl UnassignExponent for FactorPrefixAtomExponentAnnotation {
    fn unassign_exponent(self) -> Term {
        Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation {
            factor: self.factor,
            prefix: self.prefix,
            atom: self.atom,
            annotation: self.annotation,
        })
    }
}

impl Pow<Exponent> for FactorPrefixAtomExponentAnnotation {
    type Output = PowOutput<Annotation, FactorPrefixAtomAnnotation, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(fpaa) => PowOutput::One(fpaa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorPrefixAtomExponentAnnotation {
    type Output =
        PowOutput<Annotation, FactorPrefixAtomAnnotation, FactorPrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self.clone();

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(fpaa) => PowOutput::One(fpaa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorPrefixAtomExponentAnnotation {
    type Output = PowOutput<Annotation, FactorPrefixAtomAnnotation, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(self.exponent * rhs)
    }
}

impl Inv for FactorPrefixAtomExponentAnnotation {
    type Output = InvOutput<FactorPrefixAtomAnnotation, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(fpaa) => InvOutput::One(fpaa),
            PowOutput::Rest(fpaea) => InvOutput::Rest(fpaea),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a FactorPrefixAtomExponentAnnotation {
    type Output = InvOutput<FactorPrefixAtomAnnotation, FactorPrefixAtomExponentAnnotation>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(fpaa) => InvOutput::One(fpaa),
            PowOutput::Rest(fpaea) => InvOutput::Rest(fpaea),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorPrefixAtomExponentAnnotation {
    type Output = InvOutput<FactorPrefixAtomAnnotation, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(fpaa) => InvOutput::One(fpaa),
            PowOutput::Rest(fpaea) => InvOutput::Rest(fpaea),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}
