use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Atom, Composable, IsCompatibleWith,
    UcumUnit,
};

use super::{
    Annotation, AtomAnnotation, AtomExponentAnnotation, Exponent, Factor, FactorAtomAnnotation,
    InvOutput, PowOutput, SetAnnotation, SetExponent, Term,
};

// ╭──────────────────────────────╮
// │ FactorAtomExponentAnnotation │
// ╰──────────────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorAtomExponentAnnotation {
    pub(crate) factor: Factor,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
    pub(crate) annotation: Annotation,
}

impl FactorAtomExponentAnnotation {
    #[must_use]
    pub const fn new(
        factor: Factor,
        atom: Atom,
        exponent: Exponent,
        annotation: Annotation,
    ) -> Self {
        Self {
            factor,
            atom,
            exponent,
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
    pub const fn exponent(&self) -> Exponent {
        self.exponent
    }

    #[must_use]
    pub const fn annotation(&self) -> &Annotation {
        &self.annotation
    }

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        f64::from(self.factor) * self.atom.scalar().powi(self.exponent)
    }
}

impl From<FactorAtomExponentAnnotation> for Term {
    fn from(value: FactorAtomExponentAnnotation) -> Self {
        Self::FactorAtomExponentAnnotation(value)
    }
}

impl fmt::Display for FactorAtomExponentAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.factor, self.exponent) {
            (1, 1) => {
                write!(f, "{}{}", self.atom, self.annotation)
            }
            (1, exponent) => {
                write!(
                    f,
                    "{atom}{exponent}{annotation}",
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
            (factor, 1) => {
                write!(
                    f,
                    "{factor}{atom}{annotation}",
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
            (factor, exponent) => {
                write!(
                    f,
                    "{factor}{atom}{exponent}{annotation}",
                    atom = self.atom,
                    annotation = self.annotation
                )
            }
        }
    }
}

impl SetExponent for FactorAtomExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAtomAnnotation, ()>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(mem::take(&mut self.annotation)),
            1 => PowOutput::One(FactorAtomAnnotation {
                factor: self.factor,
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

impl Pow<Exponent> for FactorAtomExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAtomAnnotation, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(faa) => PowOutput::One(faa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorAtomExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAtomAnnotation, FactorAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self.clone();

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(faa) => PowOutput::One(faa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorAtomExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAtomAnnotation, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(self.exponent * rhs)
    }
}

impl Inv for FactorAtomExponentAnnotation {
    type Output = InvOutput<FactorAtomAnnotation, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_atom_annotation) => InvOutput::One(factor_atom_annotation),
            PowOutput::Rest(factor_atom_exponent_annotation) => {
                InvOutput::Rest(factor_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a FactorAtomExponentAnnotation {
    type Output = InvOutput<FactorAtomAnnotation, FactorAtomExponentAnnotation>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_atom_annotation) => InvOutput::One(factor_atom_annotation),
            PowOutput::Rest(factor_atom_exponent_annotation) => {
                InvOutput::Rest(factor_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorAtomExponentAnnotation {
    type Output = InvOutput<FactorAtomAnnotation, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_atom_annotation) => InvOutput::One(factor_atom_annotation),
            PowOutput::Rest(factor_atom_exponent_annotation) => {
                InvOutput::Rest(factor_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> SetAnnotation for &'a mut FactorAtomExponentAnnotation {
    type Output = ();

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        self.annotation = annotation.into();
    }
}

impl ComposablyEq<Term> for FactorAtomExponentAnnotation {
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

impl ComposablyEq<AtomAnnotation> for FactorAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &AtomAnnotation) -> Option<Exponent> {
        if self.factor == 1 && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<AtomExponentAnnotation> for FactorAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &AtomExponentAnnotation) -> Option<Exponent> {
        if self.factor == 1 && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAtomAnnotation> for FactorAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &FactorAtomAnnotation) -> Option<Exponent> {
        if self.factor == rhs.factor && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for FactorAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self.factor == rhs.factor && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for FactorAtomExponentAnnotation {
    fn composition(&self) -> crate::Composition {
        self.atom.composition() * self.exponent
    }
}

impl IsCompatibleWith<Term> for FactorAtomExponentAnnotation {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition()
            && Some(self.annotation.as_str()) == rhs.annotation()
    }
}

impl TermReduce for FactorAtomExponentAnnotation {
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
            (_, 1) => Term::FactorAtomAnnotation(FactorAtomAnnotation {
                factor: self.factor,
                atom: self.atom,
                annotation: self.annotation.clone(),
            }),
            (_, _) => Term::FactorAtomExponentAnnotation(Self {
                factor: self.factor,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}
