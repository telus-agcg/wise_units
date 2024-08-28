use std::fmt;

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Annotation, Composable,
    IsCompatibleWith,
};

use super::{
    Exponent, Factor, FactorExponentAnnotation, InvOutput, PowOutput, SetAnnotation, SetExponent,
    Term,
};

// ╭────────────────╮
// │ FactorExponent │
// ╰────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FactorExponent {
    pub(crate) factor: Factor,
    pub(crate) exponent: Exponent,
}

impl FactorExponent {
    #[must_use]
    pub const fn new(factor: Factor, exponent: Exponent) -> Self {
        Self { factor, exponent }
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
    pub fn to_scalar(&self) -> f64 {
        f64::from(self.factor).powi(self.exponent)
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

impl ComposablyEq<Term> for FactorExponent {
    fn composably_eq(&self, rhs: &Term) -> Option<Exponent> {
        match rhs {
            Term::Factor(factor) => self.composably_eq(factor),
            Term::FactorExponent(inner) => self.composably_eq(inner),
            _ => None,
        }
    }
}

impl ComposablyEq<Factor> for FactorExponent {
    fn composably_eq(&self, rhs: &Factor) -> Option<Exponent> {
        if self.factor == *rhs {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for FactorExponent {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self.factor == rhs.factor {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for FactorExponent {
    fn composition(&self) -> crate::Composition {
        crate::Composition::new_dimless()
    }
}

impl IsCompatibleWith<Term> for FactorExponent {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition() && rhs.annotation().is_none()
    }
}

impl TermReduce for FactorExponent {
    fn build(&self, exponent: Exponent) -> Term {
        if exponent == 1 {
            Term::Factor(self.factor)
        } else {
            Term::FactorExponent(Self {
                factor: self.factor,
                exponent,
            })
        }
    }
}
