use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Composable, IsCompatibleWith,
};

use super::{
    Annotation, Exponent, Factor, FactorExponentAnnotation, PowOutput, SetAnnotation, SetExponent,
    Term,
};

// ╭──────────────────╮
// │ FactorAnnotation │
// ╰──────────────────╯
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FactorAnnotation {
    pub(crate) factor: Factor,
    pub(crate) annotation: Annotation,
}

impl FactorAnnotation {
    #[must_use]
    pub const fn new(factor: Factor, annotation: Annotation) -> Self {
        Self { factor, annotation }
    }

    #[must_use]
    pub const fn factor(&self) -> Factor {
        self.factor
    }

    #[must_use]
    pub const fn annotation(&self) -> &Annotation {
        &self.annotation
    }

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        f64::from(self.factor)
    }
}

impl From<FactorAnnotation> for Term {
    fn from(value: FactorAnnotation) -> Self {
        Self::FactorAnnotation(value)
    }
}

impl fmt::Display for FactorAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.factor == 1 {
            write!(f, "{}", self.annotation)
        } else {
            write!(f, "{}{}", self.factor, self.annotation)
        }
    }
}

impl SetExponent for FactorAnnotation {
    type Output = PowOutput<Annotation, (), FactorExponentAnnotation>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(mem::take(&mut self.annotation)),
            1 => PowOutput::One(()),
            _ => PowOutput::Rest(FactorExponentAnnotation {
                factor: self.factor,
                exponent,
                annotation: mem::take(&mut self.annotation),
            }),
        }
    }
}

impl Pow<Exponent> for FactorAnnotation {
    type Output = PowOutput<Annotation, Self, FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(fea) => PowOutput::Rest(fea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorAnnotation {
    type Output = PowOutput<Annotation, FactorAnnotation, FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self.clone();

        match s.set_exponent(rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(fea) => PowOutput::Rest(fea),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorAnnotation {
    type Output = PowOutput<Annotation, (), FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(rhs)
    }
}

impl Inv for FactorAnnotation {
    type Output = FactorExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a FactorAnnotation {
    type Output = FactorExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut FactorAnnotation {
    type Output = FactorExponentAnnotation;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> SetAnnotation for &'a mut FactorAnnotation {
    type Output = ();

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        self.annotation = annotation.into();
    }
}

impl ComposablyEq<Term> for FactorAnnotation {
    fn composably_eq(&self, rhs: &Term) -> Option<Exponent> {
        match rhs {
            Term::Annotation(annotation) => self.composably_eq(annotation),
            Term::FactorAnnotation(inner) => self.composably_eq(inner),
            Term::FactorExponentAnnotation(inner) => self.composably_eq(inner),
            _ => None,
        }
    }
}

impl ComposablyEq<Annotation> for FactorAnnotation {
    fn composably_eq(&self, rhs: &Annotation) -> Option<Exponent> {
        if self.factor == 1 && &self.annotation == rhs {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for FactorAnnotation {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self == rhs {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorExponentAnnotation> for FactorAnnotation {
    fn composably_eq(&self, rhs: &FactorExponentAnnotation) -> Option<Exponent> {
        if self.factor == rhs.factor && self.annotation == rhs.annotation {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for FactorAnnotation {
    fn composition(&self) -> crate::Composition {
        crate::Composition::new_dimless()
    }
}

impl IsCompatibleWith<Term> for FactorAnnotation {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition()
            && Some(self.annotation.as_str()) == rhs.annotation()
    }
}

impl TermReduce for FactorAnnotation {
    fn build(&self, exponent: Exponent) -> Term {
        match (self.factor, exponent) {
            (1, 1) => Term::Annotation(self.annotation.clone()),
            (_, 1) => Term::FactorAnnotation(Self {
                factor: self.factor,
                annotation: self.annotation.clone(),
            }),
            (_, _) => Term::FactorExponentAnnotation(FactorExponentAnnotation {
                factor: self.factor,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}
