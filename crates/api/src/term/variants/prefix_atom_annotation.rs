use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Atom, Composable, IsCompatibleWith,
    Prefix, UcumSymbol, UcumUnit,
};

use super::{
    Annotation, AssignFactor, Exponent, Factor, FactorPrefixAtomAnnotation,
    FactorPrefixAtomExponentAnnotation, PowOutput, PrefixAtomExponentAnnotation, SetAnnotation,
    SetExponent, Term,
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

    #[must_use]
    pub const fn prefix(&self) -> Prefix {
        self.prefix
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
        self.prefix.definition_value() * self.atom.scalar()
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

impl<'a> SetAnnotation for &'a mut PrefixAtomAnnotation {
    type Output = ();

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        self.annotation = annotation.into();
    }
}

impl ComposablyEq<Term> for PrefixAtomAnnotation {
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

impl ComposablyEq<Self> for PrefixAtomAnnotation {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self == rhs {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<PrefixAtomExponentAnnotation> for PrefixAtomAnnotation {
    fn composably_eq(&self, rhs: &PrefixAtomExponentAnnotation) -> Option<Exponent> {
        if self.prefix == rhs.prefix && self.atom == rhs.atom && self.annotation == rhs.annotation {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorPrefixAtomAnnotation> for PrefixAtomAnnotation {
    fn composably_eq(&self, rhs: &FactorPrefixAtomAnnotation) -> Option<Exponent> {
        if rhs.factor == 1
            && self.prefix == rhs.prefix
            && self.atom == rhs.atom
            && self.annotation == rhs.annotation
        {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorPrefixAtomExponentAnnotation> for PrefixAtomAnnotation {
    fn composably_eq(&self, rhs: &FactorPrefixAtomExponentAnnotation) -> Option<Exponent> {
        if rhs.factor == 1
            && self.prefix == rhs.prefix
            && self.atom == rhs.atom
            && self.annotation == rhs.annotation
        {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for PrefixAtomAnnotation {
    fn composition(&self) -> crate::Composition {
        self.atom.composition()
    }
}

impl IsCompatibleWith<Term> for PrefixAtomAnnotation {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition()
            && Some(self.annotation.as_str()) == rhs.annotation()
    }
}

impl TermReduce for PrefixAtomAnnotation {
    fn build(&self, exponent: Exponent) -> Term {
        if exponent == 1 {
            Term::PrefixAtomAnnotation(Self {
                prefix: self.prefix,
                atom: self.atom,
                annotation: self.annotation.clone(),
            })
        } else {
            Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            })
        }
    }
}
