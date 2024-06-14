use std::fmt;

use num_traits::{Inv, Pow};

use crate::Atom;

use super::{
    Exponent, Factor, FactorAtom, InvOutput, PowOutput, SetExponent, Term, UnassignExponent,
};

// ╭────────────────────╮
// │ FactorAtomExponent │
// ╰────────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactorAtomExponent {
    pub(crate) factor: Factor,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
}

impl FactorAtomExponent {
    #[must_use]
    pub const fn new(factor: Factor, atom: Atom, exponent: Exponent) -> Self {
        Self {
            factor,
            atom,
            exponent,
        }
    }
}

impl From<FactorAtomExponent> for Term {
    fn from(value: FactorAtomExponent) -> Self {
        Self::FactorAtomExponent(value)
    }
}

impl fmt::Display for FactorAtomExponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.factor, self.exponent) {
            (1, 1) => {
                write!(f, "{}", self.atom)
            }
            (1, exponent) => {
                write!(f, "{atom}{exponent}", atom = self.atom)
            }
            (factor, 1) => {
                write!(f, "{factor}{atom}", atom = self.atom)
            }
            (factor, exponent) => {
                write!(f, "{factor}{atom}{exponent}", atom = self.atom)
            }
        }
    }
}

impl SetExponent for FactorAtomExponent {
    type Output = PowOutput<Factor, FactorAtom, ()>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(FactorAtom {
                factor: self.factor,
                atom: self.atom,
            }),
            _ => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

impl UnassignExponent for FactorAtomExponent {
    fn unassign_exponent(self) -> Term {
        Term::FactorAtom(FactorAtom {
            factor: self.factor,
            atom: self.atom,
        })
    }
}

impl Pow<Exponent> for FactorAtomExponent {
    type Output = PowOutput<Factor, FactorAtom, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(factor) => PowOutput::Zero(factor),
            PowOutput::One(fa) => PowOutput::One(fa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorAtomExponent {
    type Output = PowOutput<Factor, FactorAtom, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(self.exponent * rhs)
    }
}

impl Inv for FactorAtomExponent {
    type Output = InvOutput<FactorAtom, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_atom) => InvOutput::One(factor_atom),
            PowOutput::Rest(factor_atom_exponent) => InvOutput::Rest(factor_atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorAtomExponent {
    type Output = InvOutput<FactorAtom, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(factor_atom) => InvOutput::One(factor_atom),
            PowOutput::Rest(factor_atom_exponent) => InvOutput::Rest(factor_atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}
