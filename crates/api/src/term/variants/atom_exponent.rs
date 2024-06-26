use std::fmt;

use num_traits::{Inv, Pow};

use crate::{Annotation, Atom};

use super::{
    AssignFactor, AtomExponentAnnotation, Exponent, Factor, FactorAtomExponent, InvOutput,
    PowOutput, SetAnnotation, SetExponent, Term,
};

// ╭──────────────╮
// │ AtomExponent │
// ╰──────────────╯
/// Ex. "m2"
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtomExponent {
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
}

impl AtomExponent {
    #[must_use]
    pub const fn new(atom: Atom, exponent: Exponent) -> Self {
        Self { atom, exponent }
    }

    #[must_use]
    pub const fn atom(&self) -> Atom {
        self.atom
    }

    #[must_use]
    pub const fn exponent(&self) -> i32 {
        self.exponent
    }
}

impl From<AtomExponent> for Term {
    fn from(value: AtomExponent) -> Self {
        Self::AtomExponent(value)
    }
}

impl fmt::Display for AtomExponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent == 1 {
            write!(f, "{}", self.atom)
        } else {
            write!(f, "{}{}", self.atom, self.exponent)
        }
    }
}

impl AssignFactor for AtomExponent {
    type Output = FactorAtomExponent;

    fn assign_factor(&mut self, factor: Factor) -> Self::Output {
        FactorAtomExponent {
            factor,
            atom: self.atom,
            exponent: self.exponent,
        }
    }
}

impl SetExponent for AtomExponent {
    type Output = PowOutput<Factor, Atom, ()>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(self.atom),
            _ => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

impl Pow<Exponent> for AtomExponent {
    type Output = PowOutput<Factor, Atom, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(factor) => PowOutput::Zero(factor),
            PowOutput::One(atom) => PowOutput::One(atom),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut AtomExponent {
    type Output = PowOutput<Factor, Atom, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(self.exponent * rhs)
    }
}

impl Inv for AtomExponent {
    type Output = InvOutput<Atom, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(atom) => InvOutput::One(atom),
            PowOutput::Rest(atom_exponent) => InvOutput::Rest(atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut AtomExponent {
    type Output = InvOutput<Atom, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(atom) => InvOutput::One(atom),
            PowOutput::Rest(atom_exponent) => InvOutput::Rest(atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl SetAnnotation for AtomExponent {
    type Output = AtomExponentAnnotation;

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        Self::Output::new(self.atom, self.exponent, Annotation::from(annotation))
    }
}
