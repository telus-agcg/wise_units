use std::fmt;

use num_traits::{Inv, Pow};

use crate::Annotation;

use super::{
    Exponent, Factor, FactorExponentAnnotation, InvOutput, PowOutput, SetAnnotation, SetExponent,
    Term,
};

// ╭────────────────╮
// │ FactorExponent │
// ╰────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactorExponent {
    pub(crate) factor: Factor,
    pub(crate) exponent: Exponent,
}

impl FactorExponent {
    #[must_use]
    pub const fn new(factor: Factor, exponent: Exponent) -> Self {
        Self { factor, exponent }
    }
}

impl From<FactorExponent> for Term {
    fn from(value: FactorExponent) -> Self {
        Self::FactorExponent(value)
    }
}

impl fmt::Display for FactorExponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent.is_negative() {
            write!(
                f,
                "{factor}-{exponent}",
                factor = self.factor,
                exponent = self.exponent
            )
        } else {
            write!(
                f,
                "{factor}+{exponent}",
                factor = self.factor,
                exponent = self.exponent
            )
        }
    }
}

impl SetExponent for FactorExponent {
    type Output = PowOutput<Factor, Factor, ()>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(self.factor),
            _ => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

impl Pow<Exponent> for FactorExponent {
    type Output = PowOutput<Factor, Factor, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(self.exponent * rhs) {
            PowOutput::Zero(factor) => PowOutput::Zero(factor),
            PowOutput::One(factor) => PowOutput::One(factor),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorExponent {
    type Output = PowOutput<Factor, Factor, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(self.exponent * rhs)
    }
}

impl Inv for FactorExponent {
    type Output = InvOutput<Factor, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor) => InvOutput::One(factor),
            PowOutput::Rest(factor_exponent) => InvOutput::Rest(factor_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorExponent {
    type Output = InvOutput<Factor, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor) => InvOutput::One(factor),
            PowOutput::Rest(factor_exponent) => InvOutput::Rest(factor_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl SetAnnotation for FactorExponent {
    type Output = FactorExponentAnnotation;

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        FactorExponentAnnotation::new(self.factor, self.exponent, Annotation::from(annotation))
    }
}
