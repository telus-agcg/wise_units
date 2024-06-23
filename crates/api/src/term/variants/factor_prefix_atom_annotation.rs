use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{Atom, Prefix};

use super::{
    Annotation, Exponent, Factor, FactorPrefixAtomExponentAnnotation, PowOutput, SetAnnotation,
    SetExponent, Term,
};

// ╭────────────────────────────╮
// │ FactorPrefixAtomAnnotation │
// ╰────────────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorPrefixAtomAnnotation {
    pub(crate) factor: Factor,
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) annotation: Annotation,
}

impl FactorPrefixAtomAnnotation {
    #[must_use]
    pub const fn new(factor: Factor, prefix: Prefix, atom: Atom, annotation: Annotation) -> Self {
        Self {
            factor,
            prefix,
            atom,
            annotation,
        }
    }
}

impl From<FactorPrefixAtomAnnotation> for Term {
    fn from(value: FactorPrefixAtomAnnotation) -> Self {
        Self::FactorPrefixAtomAnnotation(value)
    }
}

impl fmt::Display for FactorPrefixAtomAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.factor == 1 {
            write!(
                f,
                "{prefix}{atom}{annotation}",
                prefix = self.prefix,
                atom = self.atom,
                annotation = self.annotation
            )
        } else {
            write!(
                f,
                "{factor}{prefix}{atom}{annotation}",
                factor = self.factor,
                prefix = self.prefix,
                atom = self.atom,
                annotation = self.annotation
            )
        }
    }
}

impl SetExponent for FactorPrefixAtomAnnotation {
    type Output = PowOutput<Annotation, (), FactorPrefixAtomExponentAnnotation>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(mem::take(&mut self.annotation)),
            1 => PowOutput::One(()),
            _ => PowOutput::Rest(FactorPrefixAtomExponentAnnotation {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: mem::take(&mut self.annotation),
            }),
        }
    }
}

impl Pow<Exponent> for FactorPrefixAtomAnnotation {
    type Output = PowOutput<Annotation, Self, FactorPrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(fpaea) => PowOutput::Rest(fpaea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorPrefixAtomAnnotation {
    type Output =
        PowOutput<Annotation, FactorPrefixAtomAnnotation, FactorPrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self.clone();

        match s.set_exponent(rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(fpaea) => PowOutput::Rest(fpaea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorPrefixAtomAnnotation {
    type Output = PowOutput<Annotation, (), FactorPrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(rhs)
    }
}

impl Inv for FactorPrefixAtomAnnotation {
    type Output = FactorPrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a FactorPrefixAtomAnnotation {
    type Output = FactorPrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut FactorPrefixAtomAnnotation {
    type Output = FactorPrefixAtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> SetAnnotation for &'a mut FactorPrefixAtomAnnotation {
    type Output = ();

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        self.annotation = annotation.into();
    }
}
