use std::{fmt, mem};

use num_traits::{Inv, Pow};

use super::{Annotation, Exponent, Factor, FactorExponentAnnotation, PowOutput, SetExponent, Term};

// ╭──────────────────╮
// │ FactorAnnotation │
// ╰──────────────────╯
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactorAnnotation {
    pub(crate) factor: Factor,
    pub(crate) annotation: Annotation,
}

impl FactorAnnotation {
    #[must_use]
    pub const fn new(factor: Factor, annotation: Annotation) -> Self {
        Self { factor, annotation }
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
