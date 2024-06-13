use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Atom, Composable, IsCompatibleWith,
    Prefix, UcumSymbol, UcumUnit,
};

use super::{
    Annotation, Exponent, Factor, FactorPrefixAtomAnnotation, InvOutput, PowOutput,
    PrefixAtomAnnotation, PrefixAtomExponentAnnotation, SetAnnotation, SetExponent, Term,
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
    pub const fn exponent(&self) -> Exponent {
        self.exponent
    }

    #[must_use]
    pub const fn annotation(&self) -> &Annotation {
        &self.annotation
    }

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        f64::from(self.factor)
            * self.prefix.definition_value()
            * self.atom.scalar().powi(self.exponent)
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

impl<'a> SetAnnotation for &'a mut FactorPrefixAtomExponentAnnotation {
    type Output = ();

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        self.annotation = annotation.into();
    }
}

impl ComposablyEq<Term> for FactorPrefixAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &Term) -> Option<Exponent> {
        match rhs {
            Term::FactorPrefixAtomAnnotation(inner) => self.composably_eq(inner),
            Term::FactorPrefixAtomExponentAnnotation(inner) => self.composably_eq(inner),
            Term::PrefixAtomAnnotation(inner) => self.composably_eq(inner),
            Term::PrefixAtomExponentAnnotation(inner) => self.composably_eq(inner),
            _ => None,
        }
    }
}

impl ComposablyEq<FactorPrefixAtomAnnotation> for FactorPrefixAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &FactorPrefixAtomAnnotation) -> Option<Exponent> {
        if self.factor == rhs.factor
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

impl ComposablyEq<Self> for FactorPrefixAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self.factor == rhs.factor
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

impl ComposablyEq<PrefixAtomAnnotation> for FactorPrefixAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &PrefixAtomAnnotation) -> Option<Exponent> {
        if self.factor == 1
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

impl ComposablyEq<PrefixAtomExponentAnnotation> for FactorPrefixAtomExponentAnnotation {
    fn composably_eq(&self, rhs: &PrefixAtomExponentAnnotation) -> Option<Exponent> {
        if self.factor == 1
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

impl Composable for FactorPrefixAtomExponentAnnotation {
    fn composition(&self) -> crate::Composition {
        self.atom.composition() * self.exponent
    }
}

impl IsCompatibleWith<Term> for FactorPrefixAtomExponentAnnotation {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition()
            && Some(self.annotation.as_str()) == rhs.annotation()
    }
}

impl TermReduce for FactorPrefixAtomExponentAnnotation {
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
            (_, 1) => Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                annotation: self.annotation.clone(),
            }),
            (_, _) => Term::FactorPrefixAtomExponentAnnotation(Self {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}
