use std::fmt;

use num_traits::{Inv, Pow};

use crate::{Annotation, Atom, Prefix};

use super::{
    AssignFactor, Exponent, Factor, FactorPrefixAtom, PowOutput, PrefixAtomAnnotation,
    PrefixAtomExponent, SetAnnotation, SetExponent, Term,
};

// ╭────────────╮
// │ PrefixAtom │
// ╰────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrefixAtom {
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
}

impl PrefixAtom {
    #[must_use]
    pub const fn new(prefix: Prefix, atom: Atom) -> Self {
        Self { prefix, atom }
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

impl From<PrefixAtom> for Term {
    fn from(value: PrefixAtom) -> Self {
        Self::PrefixAtom(value)
    }
}

impl From<PrefixAtomExponent> for PrefixAtom {
    fn from(value: PrefixAtomExponent) -> Self {
        Self {
            prefix: value.prefix,
            atom: value.atom,
        }
    }
}

impl fmt::Display for PrefixAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.prefix, self.atom,)
    }
}

impl AssignFactor for PrefixAtom {
    type Output = FactorPrefixAtom;

    fn assign_factor(&mut self, factor: Factor) -> Self::Output {
        FactorPrefixAtom {
            factor,
            prefix: self.prefix,
            atom: self.atom,
        }
    }
}

impl SetExponent for PrefixAtom {
    type Output = PowOutput<Factor, (), PrefixAtomExponent>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(()),
            _ => PowOutput::Rest(PrefixAtomExponent {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            }),
        }
    }
}

impl Pow<Exponent> for PrefixAtom {
    type Output = PowOutput<Factor, Self, PrefixAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(rhs) {
            PowOutput::Zero(factor) => PowOutput::Zero(factor),
            PowOutput::One(()) => PowOutput::One(s),
            PowOutput::Rest(pae) => PowOutput::Rest(pae),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut PrefixAtom {
    type Output = PowOutput<Factor, (), PrefixAtomExponent>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(rhs)
    }
}

impl Inv for PrefixAtom {
    type Output = PrefixAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl<'a> Inv for &'a mut PrefixAtom {
    type Output = PrefixAtomExponent;

    fn inv(self) -> Self::Output {
        self.pow(-1).unwrap_rest()
    }
}

impl SetAnnotation for PrefixAtom {
    type Output = PrefixAtomAnnotation;

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        Self::Output::new(self.prefix, self.atom, Annotation::from(annotation))
    }
}
