use std::{fmt, mem};

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Composable, IsCompatibleWith,
};

use super::{
    Annotation, Exponent, Factor, FactorAnnotation, InvOutput, PowOutput, SetAnnotation,
    SetExponent, Term,
};

// ╭──────────────────────────╮
// │ FactorExponentAnnotation │
// ╰──────────────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorExponentAnnotation {
    pub(crate) factor: Factor,
    pub(crate) exponent: Exponent,
    pub(crate) annotation: Annotation,
}

impl FactorExponentAnnotation {
    #[must_use]
    pub const fn new(factor: Factor, exponent: Exponent, annotation: Annotation) -> Self {
        Self {
            factor,
            exponent,
            annotation,
        }
    }

    #[must_use]
    pub const fn factor(&self) -> Factor {
        self.factor
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
        f64::from(self.factor).powi(self.exponent)
    }
}

impl From<FactorExponentAnnotation> for Term {
    fn from(value: FactorExponentAnnotation) -> Self {
        Self::FactorExponentAnnotation(value)
    }
}

impl fmt::Display for FactorExponentAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.factor, self.exponent) {
            (0, 0) | (1, 1) => self.annotation.fmt(f),
            (factor, exponent) if exponent.is_negative() => {
                write!(
                    f,
                    "{factor}{exponent}{annotation}",
                    exponent = self.exponent,
                    annotation = self.annotation
                )
            }
            (factor, exponent) => {
                write!(
                    f,
                    "{factor}+{exponent}{annotation}",
                    annotation = self.annotation
                )
            }
        }
    }
}

impl SetExponent for FactorExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAnnotation, ()>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(mem::take(&mut self.annotation)),
            1 => PowOutput::One(FactorAnnotation {
                factor: self.factor,
                annotation: mem::take(&mut self.annotation),
            }),
            _ => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

impl Pow<Exponent> for FactorExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAnnotation, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(fa) => PowOutput::One(fa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a FactorExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAnnotation, FactorExponentAnnotation>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self.clone();

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(annotation) => PowOutput::Zero(annotation),
            PowOutput::One(fa) => PowOutput::One(fa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorExponentAnnotation {
    type Output = PowOutput<Annotation, FactorAnnotation, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(self.exponent * rhs)
    }
}

impl Inv for FactorExponentAnnotation {
    type Output = InvOutput<FactorAnnotation, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_annotation) => InvOutput::One(factor_annotation),
            PowOutput::Rest(factor_exponent_annotation) => {
                InvOutput::Rest(factor_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a FactorExponentAnnotation {
    type Output = InvOutput<FactorAnnotation, FactorExponentAnnotation>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_annotation) => InvOutput::One(factor_annotation),
            PowOutput::Rest(factor_exponent_annotation) => {
                InvOutput::Rest(factor_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorExponentAnnotation {
    type Output = InvOutput<FactorAnnotation, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_annotation) => InvOutput::One(factor_annotation),
            PowOutput::Rest(factor_exponent_annotation) => {
                InvOutput::Rest(factor_exponent_annotation)
            }
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> SetAnnotation for &'a mut FactorExponentAnnotation {
    type Output = ();

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        self.annotation = annotation.into();
    }
}

impl ComposablyEq<Term> for FactorExponentAnnotation {
    fn composably_eq(&self, rhs: &Term) -> Option<Exponent> {
        match rhs {
            Term::Annotation(inner) => self.composably_eq(inner),
            Term::FactorAnnotation(inner) => self.composably_eq(inner),
            Term::FactorExponentAnnotation(inner) => self.composably_eq(inner),
            _ => None,
        }
    }
}

impl ComposablyEq<Annotation> for FactorExponentAnnotation {
    fn composably_eq(&self, rhs: &Annotation) -> Option<Exponent> {
        if self.factor == 1 && &self.annotation == rhs {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAnnotation> for FactorExponentAnnotation {
    fn composably_eq(&self, rhs: &FactorAnnotation) -> Option<Exponent> {
        if self.factor == rhs.factor && self.annotation == rhs.annotation {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for FactorExponentAnnotation {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self.factor == rhs.factor && self.annotation == rhs.annotation {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for FactorExponentAnnotation {
    fn composition(&self) -> crate::Composition {
        crate::Composition::new_dimless()
    }
}

impl IsCompatibleWith<Term> for FactorExponentAnnotation {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition()
            && Some(self.annotation.as_str()) == rhs.annotation()
    }
}

impl TermReduce for FactorExponentAnnotation {
    fn build(&self, exponent: Exponent) -> Term {
        match (self.factor, exponent) {
            (1, 1) => Term::Annotation(self.annotation.clone()),
            (_, 1) => Term::FactorAnnotation(FactorAnnotation {
                factor: self.factor,
                annotation: self.annotation.clone(),
            }),
            (_, _) => Term::FactorExponentAnnotation(Self {
                factor: self.factor,
                exponent,
                annotation: self.annotation.clone(),
            }),
        }
    }
}
