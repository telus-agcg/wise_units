use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Atom, Composable, IsCompatibleWith,
    Prefix, UcumSymbol, UcumUnit,
};

use super::{
    Annotation, Exponent, Factor, FactorPrefixAtomExponentAnnotation, PowOutput,
    PrefixAtomAnnotation, PrefixAtomExponentAnnotation, SetAnnotation, SetExponent, Term,
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

    #[must_use]
    pub const fn factor(&self) -> Factor {
        self.factor
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
        f64::from(self.factor) * self.prefix.definition_value() * self.atom.scalar()
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

impl ComposablyEq<Term> for FactorPrefixAtomAnnotation {
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

impl ComposablyEq<PrefixAtomAnnotation> for FactorPrefixAtomAnnotation {
    fn composably_eq(&self, rhs: &PrefixAtomAnnotation) -> Option<Exponent> {
        if self.factor == 1
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

impl ComposablyEq<PrefixAtomExponentAnnotation> for FactorPrefixAtomAnnotation {
    fn composably_eq(&self, rhs: &PrefixAtomExponentAnnotation) -> Option<Exponent> {
        if self.factor == 1
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

impl ComposablyEq<Self> for FactorPrefixAtomAnnotation {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self.factor == rhs.factor
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

impl ComposablyEq<FactorPrefixAtomExponentAnnotation> for FactorPrefixAtomAnnotation {
    fn composably_eq(&self, rhs: &FactorPrefixAtomExponentAnnotation) -> Option<Exponent> {
        if self.factor == rhs.factor
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

impl Composable for FactorPrefixAtomAnnotation {
    fn composition(&self) -> crate::Composition {
        self.atom.composition()
    }
}

impl IsCompatibleWith<Term> for FactorPrefixAtomAnnotation {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition()
            && Some(self.annotation.as_str()) == rhs.annotation()
    }
}

impl TermReduce for FactorPrefixAtomAnnotation {
    fn build(&self, exponent: Exponent) -> Term {
        match (self.factor, exponent) {
            (1, 1) => Term::PrefixAtomAnnotation(PrefixAtomAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                annotation: self.annotation.clone(),
            }),
            (1, _) => Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
            (_, 1) => Term::FactorPrefixAtomAnnotation(Self {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                annotation: self.annotation.clone(),
            }),
            (_, _) => {
                Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                    factor: self.factor,
                    prefix: self.prefix,
                    atom: self.atom,
                    exponent,
                    annotation: self.annotation.clone(),
                })
            }
        }
    }
}
