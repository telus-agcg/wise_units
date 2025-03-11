use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Atom, Composable, IsCompatibleWith,
    UcumUnit,
};

use super::{
    Annotation, AtomAnnotation, AtomExponentAnnotation, Exponent, Factor,
    FactorAtomExponentAnnotation, PowOutput, SetAnnotation, SetExponent, Term,
};

// ╭──────────────────────╮
// │ FactorAtomAnnotation │
// ╰──────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

    #[must_use]
    pub const fn factor(&self) -> Factor {
        self.factor
    }

    #[must_use]
    pub const fn atom(&self) -> Atom {
        self.atom
    }

    #[must_use]
    pub const fn annotation(&self) -> &Annotation {
        &self.annotation
    }

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        f64::from(self.factor) * self.atom.scalar()
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

impl<'a> SetAnnotation for &'a mut FactorAtomAnnotation {
    type Output = ();

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        self.annotation = annotation.into();
    }
}

impl ComposablyEq<Term> for FactorAtomAnnotation {
    fn composably_eq(&self, rhs: &Term) -> Option<Exponent> {
        match rhs {
            Term::AtomAnnotation(inner) => self.composably_eq(inner),
            Term::AtomExponentAnnotation(inner) => self.composably_eq(inner),
            Term::FactorAtomAnnotation(inner) => self.composably_eq(inner),
            Term::FactorAtomExponentAnnotation(inner) => self.composably_eq(inner),
            _ => None,
        }
    }
}

impl ComposablyEq<AtomAnnotation> for FactorAtomAnnotation {
    fn composably_eq(&self, rhs: &AtomAnnotation) -> Option<Exponent> {
        if self.factor == 1 && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<AtomExponentAnnotation> for FactorAtomAnnotation {
    fn composably_eq(&self, rhs: &AtomExponentAnnotation) -> Option<Exponent> {
        if self.factor == 1 && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for FactorAtomAnnotation {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self == rhs {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAtomExponentAnnotation> for FactorAtomAnnotation {
    fn composably_eq(&self, rhs: &FactorAtomExponentAnnotation) -> Option<Exponent> {
        if self.factor == rhs.factor && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for FactorAtomAnnotation {
    fn composition(&self) -> crate::Composition {
        self.atom.composition()
    }
}

impl IsCompatibleWith<Term> for FactorAtomAnnotation {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition()
            && Some(self.annotation.as_str()) == rhs.annotation()
    }
}

impl TermReduce for FactorAtomAnnotation {
    fn build(&self, exponent: Exponent) -> Term {
        match (self.factor, exponent) {
            (1, 1) => Term::AtomAnnotation(AtomAnnotation {
                atom: self.atom,
                annotation: self.annotation.clone(),
            }),
            (1, _) => Term::AtomExponentAnnotation(AtomExponentAnnotation {
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
            (_, 1) => Term::FactorAtomAnnotation(Self {
                factor: self.factor,
                atom: self.atom,
                annotation: self.annotation.clone(),
            }),
            (_, _) => Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                factor: self.factor,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}
