use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Atom, Composable, IsCompatibleWith,
    UcumUnit,
};

use super::{
    Annotation, AssignFactor, AtomAnnotation, Exponent, Factor, FactorAtomAnnotation,
    FactorAtomExponentAnnotation, InvOutput, PowOutput, SetAnnotation, SetExponent, Term,
};

// ╭────────────────────────╮
// │ AtomExponentAnnotation │
// ╰────────────────────────╯
/// Ex. "m2{peaches}"
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AtomExponentAnnotation {
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
    pub(crate) annotation: Annotation,
}

impl AtomExponentAnnotation {
    #[must_use]
    pub const fn new(atom: Atom, exponent: Exponent, annotation: Annotation) -> Self {
        Self {
            atom,
            exponent,
            annotation,
        }
    }

    #[must_use]
    pub const fn atom(&self) -> Atom {
        self.atom
    }

    #[must_use]
    pub const fn exponent(&self) -> i32 {
        self.exponent
    }

    #[must_use]
    pub const fn annotation(&self) -> &Annotation {
        &self.annotation
    }

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        self.atom.scalar().powi(self.exponent)
    }
}

impl From<AtomExponentAnnotation> for Term {
    fn from(value: AtomExponentAnnotation) -> Self {
        Self::AtomExponentAnnotation(value)
    }
}

impl fmt::Display for AtomExponentAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent == 1 {
            write!(f, "{}{}", self.atom, self.annotation)
        } else {
            write!(f, "{}{}{}", self.atom, self.exponent, self.annotation)
        }
    }
}

impl AssignFactor for AtomExponentAnnotation {
    type Output = FactorAtomExponentAnnotation;

    fn assign_factor(&mut self, factor: Factor) -> Self::Output {
        FactorAtomExponentAnnotation {
            factor,
            atom: self.atom,
            exponent: self.exponent,
            annotation: mem::take(&mut self.annotation),
        }
    }
}

impl SetExponent for AtomExponentAnnotation {
    type Output = PowOutput<Annotation, AtomAnnotation, ()>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(mem::take(&mut self.annotation)),
            1 => PowOutput::One(AtomAnnotation {
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

impl Pow<Exponent> for AtomExponentAnnotation {
    type Output = PowOutput<Annotation, AtomAnnotation, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(aa) => PowOutput::One(aa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a AtomExponentAnnotation {
    type Output = PowOutput<Annotation, AtomAnnotation, AtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self.clone();

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(aa) => PowOutput::One(aa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut AtomExponentAnnotation {
    type Output = PowOutput<Annotation, AtomAnnotation, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(self.exponent * rhs)
    }
}

impl Inv for AtomExponentAnnotation {
    type Output = InvOutput<AtomAnnotation, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(atom_annotation) => InvOutput::One(atom_annotation),
            PowOutput::Rest(atom_exponent_annotation) => InvOutput::Rest(atom_exponent_annotation),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a AtomExponentAnnotation {
    type Output = InvOutput<AtomAnnotation, AtomExponentAnnotation>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(atom_annotation) => InvOutput::One(atom_annotation),
            PowOutput::Rest(atom_exponent_annotation) => InvOutput::Rest(atom_exponent_annotation),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut AtomExponentAnnotation {
    type Output = InvOutput<AtomAnnotation, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(atom_annotation) => InvOutput::One(atom_annotation),
            PowOutput::Rest(atom_exponent_annotation) => InvOutput::Rest(atom_exponent_annotation),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> SetAnnotation for &'a mut AtomExponentAnnotation {
    type Output = ();

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        self.annotation = annotation.into();
    }
}

impl ComposablyEq<Term> for AtomExponentAnnotation {
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

impl ComposablyEq<AtomAnnotation> for AtomExponentAnnotation {
    fn composably_eq(&self, rhs: &AtomAnnotation) -> Option<Exponent> {
        if self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for AtomExponentAnnotation {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAtomAnnotation> for AtomExponentAnnotation {
    fn composably_eq(&self, rhs: &FactorAtomAnnotation) -> Option<Exponent> {
        if rhs.factor == 1 && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAtomExponentAnnotation> for AtomExponentAnnotation {
    fn composably_eq(&self, rhs: &FactorAtomExponentAnnotation) -> Option<Exponent> {
        if rhs.factor == 1 && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for AtomExponentAnnotation {
    fn composition(&self) -> crate::Composition {
        self.atom.composition() * self.exponent
    }
}

impl IsCompatibleWith<Term> for AtomExponentAnnotation {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition()
            && Some(self.annotation.as_str()) == rhs.annotation()
    }
}

impl TermReduce for AtomExponentAnnotation {
    fn build(&self, exponent: Exponent) -> Term {
        if exponent == 1 {
            Term::AtomAnnotation(AtomAnnotation {
                atom: self.atom,
                annotation: self.annotation.clone(),
            })
        } else {
            Term::AtomExponentAnnotation(Self {
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            })
        }
    }
}
