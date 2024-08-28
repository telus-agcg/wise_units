use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Atom, Composable, IsCompatibleWith,
    Prefix, UcumSymbol, UcumUnit,
};

use super::{
    Annotation, AssignFactor, Exponent, Factor, FactorPrefixAtomAnnotation,
    FactorPrefixAtomExponentAnnotation, InvOutput, PowOutput, PrefixAtomAnnotation, SetAnnotation,
    SetExponent, Term,
};

// ╭──────────────────────────────╮
// │ PrefixAtomExponentAnnotation │
// ╰──────────────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrefixAtomExponentAnnotation {
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
    pub(crate) annotation: Annotation,
}

impl PrefixAtomExponentAnnotation {
    #[must_use]
    pub const fn new(
        prefix: Prefix,
        atom: Atom,
        exponent: Exponent,
        annotation: Annotation,
    ) -> Self {
        Self {
            prefix,
            atom,
            exponent,
            annotation,
        }
    }

    #[must_use]
    pub const fn prefix(&self) -> Prefix {
        self.prefix
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
        self.prefix.definition_value() * self.atom.scalar().powi(self.exponent)
    }
}

impl AssignFactor for PrefixAtomExponentAnnotation {
    type Output = FactorPrefixAtomExponentAnnotation;

    fn assign_factor(&mut self, factor: Factor) -> Self::Output {
        FactorPrefixAtomExponentAnnotation {
            factor,
            prefix: self.prefix,
            atom: self.atom,
            exponent: self.exponent,
            annotation: mem::take(&mut self.annotation),
        }
    }
}

impl From<PrefixAtomExponentAnnotation> for Term {
    fn from(value: PrefixAtomExponentAnnotation) -> Self {
        Self::PrefixAtomExponentAnnotation(value)
    }
}

impl fmt::Display for PrefixAtomExponentAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent == 1 {
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
                "{prefix}{atom}{exponent}{annotation}",
                prefix = self.prefix,
                atom = self.atom,
                exponent = self.exponent,
                annotation = self.annotation
            )
        }
    }
}

impl SetExponent for PrefixAtomExponentAnnotation {
    type Output = PowOutput<Annotation, PrefixAtomAnnotation, ()>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(mem::take(&mut self.annotation)),
            1 => PowOutput::One(PrefixAtomAnnotation::from(self)),
            _ => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

impl Pow<Exponent> for PrefixAtomExponentAnnotation {
    type Output = PowOutput<Annotation, PrefixAtomAnnotation, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(paa) => PowOutput::One(paa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a PrefixAtomExponentAnnotation {
    type Output = PowOutput<Annotation, PrefixAtomAnnotation, PrefixAtomExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self.clone();

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(paa) => PowOutput::One(paa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut PrefixAtomExponentAnnotation {
    type Output = PowOutput<Annotation, PrefixAtomAnnotation, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(self.exponent * rhs)
    }
}

impl Inv for PrefixAtomExponentAnnotation {
    type Output = InvOutput<PrefixAtomAnnotation, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(prefix_atom_annotation) => InvOutput::One(prefix_atom_annotation),
            PowOutput::Rest(prefix_atom_exponent_annotation) => {
                InvOutput::Rest(prefix_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a PrefixAtomExponentAnnotation {
    type Output = InvOutput<PrefixAtomAnnotation, PrefixAtomExponentAnnotation>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(prefix_atom_annotation) => InvOutput::One(prefix_atom_annotation),
            PowOutput::Rest(prefix_atom_exponent_annotation) => {
                InvOutput::Rest(prefix_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut PrefixAtomExponentAnnotation {
    type Output = InvOutput<PrefixAtomAnnotation, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(prefix_atom_annotation) => InvOutput::One(prefix_atom_annotation),
            PowOutput::Rest(prefix_atom_exponent_annotation) => {
                InvOutput::Rest(prefix_atom_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> SetAnnotation for &'a mut PrefixAtomExponentAnnotation {
    type Output = ();

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        self.annotation = annotation.into();
    }
}

impl ComposablyEq<Term> for PrefixAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &Term) -> Option<Exponent> {
        match rhs {
            Term::PrefixAtomAnnotation(inner) => self.composably_eq(inner),
            Term::PrefixAtomExponentAnnotation(inner) => self.composably_eq(inner),
            Term::FactorPrefixAtomAnnotation(inner) => self.composably_eq(inner),
            Term::FactorPrefixAtomExponentAnnotation(inner) => self.composably_eq(inner),
            _ => None,
        }
    }
}

impl ComposablyEq<PrefixAtomAnnotation> for PrefixAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &PrefixAtomAnnotation) -> Option<Exponent> {
        if self.prefix == rhs.prefix && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for PrefixAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self.prefix == rhs.prefix && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorPrefixAtomAnnotation> for PrefixAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &FactorPrefixAtomAnnotation) -> Option<Exponent> {
        if rhs.factor == 1
            && self.prefix == rhs.prefix
            && self.atom == rhs.atom
            && self.annotation == rhs.annotation
        {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorPrefixAtomExponentAnnotation> for PrefixAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &FactorPrefixAtomExponentAnnotation) -> Option<Exponent> {
        if rhs.factor == 1
            && self.prefix == rhs.prefix
            && self.atom == rhs.atom
            && self.annotation == rhs.annotation
        {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for PrefixAtomExponentAnnotation {
    fn composition(&self) -> crate::Composition {
        self.atom.composition() * self.exponent
    }
}

impl IsCompatibleWith<Term> for PrefixAtomExponentAnnotation {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition()
            && Some(self.annotation.as_str()) == rhs.annotation()
    }
}

impl TermReduce for PrefixAtomExponentAnnotation {
    fn build(&self, exponent: Exponent) -> Term {
        if exponent == 1 {
            Term::PrefixAtomAnnotation(PrefixAtomAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                annotation: self.annotation.clone(),
            })
        } else {
            Term::PrefixAtomExponentAnnotation(Self {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            })
        }
    }
}
