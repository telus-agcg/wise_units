use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::Atom;

use super::{
    Annotation, AssignFactor, AtomExponentAnnotation, Exponent, Factor, FactorAtomAnnotation,
    PowOutput, SetAnnotation, SetExponent, Term,
};

// ╭────────────────╮
// │ AtomAnnotation │
// ╰────────────────╯
/// Ex. "g{sucrose}"
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtomAnnotation {
    pub(crate) atom: Atom,
    pub(crate) annotation: Annotation,
}

impl AtomAnnotation {
    #[must_use]
    pub const fn new(atom: Atom, annotation: Annotation) -> Self {
        Self { atom, annotation }
    }

    #[must_use]
    pub const fn atom(&self) -> Atom {
        self.atom
    }

    #[must_use]
    pub const fn annotation(&self) -> &Annotation {
        &self.annotation
    }
}

impl From<AtomAnnotation> for Term {
    fn from(value: AtomAnnotation) -> Self {
        Self::AtomAnnotation(value)
    }
}

impl fmt::Display for AtomAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{atom}{annotation}",
            atom = self.atom,
            annotation = self.annotation
        )
    }
}

impl AssignFactor for AtomAnnotation {
    type Output = FactorAtomAnnotation;

    fn assign_factor(&mut self, factor: Factor) -> Self::Output {
        FactorAtomAnnotation {
            factor,
            atom: self.atom,
            annotation: mem::take(&mut self.annotation),
        }
    }
}

impl SetExponent for AtomAnnotation {
    type Output = PowOutput<Annotation, (), AtomExponentAnnotation>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(mem::take(&mut self.annotation)),
            1 => PowOutput::One(()),
            _ => PowOutput::Rest(AtomExponentAnnotation {
                atom: self.atom,
                exponent,
                annotation: mem::take(&mut self.annotation),
            }),
        }
    }
}

impl Pow<Exponent> for AtomAnnotation {
    type Output = PowOutput<Annotation, Self, AtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(aea) => PowOutput::Rest(aea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a AtomAnnotation {
    type Output = PowOutput<Annotation, AtomAnnotation, AtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self.clone();

        match s.set_exponent(rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(aea) => PowOutput::Rest(aea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut AtomAnnotation {
    type Output = PowOutput<Annotation, (), AtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(rhs)
    }
}

impl Inv for AtomAnnotation {
    type Output = AtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a AtomAnnotation {
    type Output = AtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut AtomAnnotation {
    type Output = AtomExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> SetAnnotation for &'a mut AtomAnnotation {
    type Output = ();

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        self.annotation = annotation.into();
    }
}
