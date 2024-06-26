use std::fmt;

use num_traits::{Inv, Pow};

use crate::{Annotation, Atom};

use super::{
    Exponent, Factor, FactorAtomAnnotation, FactorAtomExponent, PowOutput, SetAnnotation,
    SetExponent, Term,
};

// ╭────────────╮
// │ FactorAtom │
// ╰────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactorAtom {
    pub(crate) factor: Factor,
    pub(crate) atom: Atom,
}

impl FactorAtom {
    #[must_use]
    pub const fn new(factor: Factor, atom: Atom) -> Self {
        Self { factor, atom }
    }

    #[must_use]
    pub const fn factor(&self) -> Factor {
        self.factor
    }

    #[must_use]
    pub const fn atom(&self) -> Atom {
        self.atom
    }
}

impl From<FactorAtom> for Term {
    fn from(value: FactorAtom) -> Self {
        Self::FactorAtom(value)
    }
}

impl fmt::Display for FactorAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.factor == 1 {
            write!(f, "{}", self.atom)
        } else {
            write!(f, "{}{}", self.factor, self.atom)
        }
    }
}

impl SetExponent for FactorAtom {
    type Output = PowOutput<Factor, (), FactorAtomExponent>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(()),
            _ => PowOutput::Rest(FactorAtomExponent {
                factor: self.factor,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

impl Pow<Exponent> for FactorAtom {
    type Output = PowOutput<Factor, Self, FactorAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(rhs) {
            PowOutput::Zero(factor) => PowOutput::Zero(factor),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(fae) => PowOutput::Rest(fae),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorAtom {
    type Output = PowOutput<Factor, (), FactorAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(rhs)
    }
}

impl Inv for FactorAtom {
    type Output = FactorAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut FactorAtom {
    type Output = FactorAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl SetAnnotation for FactorAtom {
    type Output = FactorAtomAnnotation;

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        FactorAtomAnnotation::new(self.factor, self.atom, Annotation::from(annotation))
    }
}
