use std::fmt;

use num_traits::{Inv, Pow};

use crate::{Annotation, Atom, Prefix};

use super::{
    Exponent, Factor, FactorPrefixAtomAnnotation, FactorPrefixAtomExponent, PowOutput,
    SetAnnotation, SetExponent, Term,
};

// ╭──────────────────╮
// │ FactorPrefixAtom │
// ╰──────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactorPrefixAtom {
    pub(crate) factor: Factor,
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
}

impl FactorPrefixAtom {
    #[must_use]
    pub const fn new(factor: Factor, prefix: Prefix, atom: Atom) -> Self {
        Self {
            factor,
            prefix,
            atom,
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
}

impl From<FactorPrefixAtom> for Term {
    fn from(value: FactorPrefixAtom) -> Self {
        Self::FactorPrefixAtom(value)
    }
}

impl fmt::Display for FactorPrefixAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.factor == 1 {
            write!(f, "{}{}", self.prefix, self.atom,)
        } else {
            write!(
                f,
                "{factor}{prefix}{atom}",
                factor = self.factor,
                prefix = self.prefix,
                atom = self.atom,
            )
        }
    }
}

impl SetExponent for FactorPrefixAtom {
    type Output = PowOutput<Factor, (), FactorPrefixAtomExponent>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(()),
            _ => PowOutput::Rest(FactorPrefixAtomExponent {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

impl Pow<Exponent> for FactorPrefixAtom {
    type Output = PowOutput<Factor, Self, FactorPrefixAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(rhs) {
            PowOutput::Zero(factor) => PowOutput::Zero(factor),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(fpae) => PowOutput::Rest(fpae),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorPrefixAtom {
    type Output = PowOutput<Factor, (), FactorPrefixAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(rhs)
    }
}

impl Inv for FactorPrefixAtom {
    type Output = FactorPrefixAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut FactorPrefixAtom {
    type Output = FactorPrefixAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl SetAnnotation for FactorPrefixAtom {
    type Output = FactorPrefixAtomAnnotation;

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        Self::Output::new(
            self.factor,
            self.prefix,
            self.atom,
            Annotation::from(annotation),
        )
    }
}
