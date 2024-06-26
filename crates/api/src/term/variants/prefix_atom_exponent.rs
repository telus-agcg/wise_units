use std::fmt;

use num_traits::{Inv, Pow};

use crate::{Annotation, Atom, Prefix};

use super::{
    AssignFactor, Exponent, Factor, FactorPrefixAtomExponent, InvOutput, PowOutput, PrefixAtom,
    PrefixAtomExponentAnnotation, SetAnnotation, SetExponent, Term,
};

// ╭────────────────────╮
// │ PrefixAtomExponent │
// ╰────────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrefixAtomExponent {
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
}

impl PrefixAtomExponent {
    #[must_use]
    pub const fn new(prefix: Prefix, atom: Atom, exponent: Exponent) -> Self {
        Self {
            prefix,
            atom,
            exponent,
        }
    }

    #[must_use]
    pub const fn prefix(&self) -> Prefix {
        self.prefix
    }

    #[must_use]
    pub const fn atom(&self) -> Atom {
        self.atom
    }

    #[must_use]
    pub const fn exponent(&self) -> Exponent {
        self.exponent
    }
}

impl From<PrefixAtomExponent> for Term {
    fn from(value: PrefixAtomExponent) -> Self {
        Self::PrefixAtomExponent(value)
    }
}

impl fmt::Display for PrefixAtomExponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent == 1 {
            write!(f, "{}{}", self.prefix, self.atom)
        } else {
            write!(f, "{}{}{}", self.prefix, self.atom, self.exponent)
        }
    }
}

impl AssignFactor for PrefixAtomExponent {
    type Output = FactorPrefixAtomExponent;

    fn assign_factor(&mut self, factor: Factor) -> Self::Output {
        FactorPrefixAtomExponent {
            factor,
            prefix: self.prefix,
            atom: self.atom,
            exponent: self.exponent,
        }
    }
}

impl SetExponent for PrefixAtomExponent {
    type Output = PowOutput<Factor, PrefixAtom, ()>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(PrefixAtom::from(*self)),
            _ => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

impl Pow<Exponent> for PrefixAtomExponent {
    type Output = PowOutput<Factor, PrefixAtom, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(one) => PowOutput::Zero(one),
            PowOutput::One(pa) => PowOutput::One(pa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut PrefixAtomExponent {
    type Output = PowOutput<Factor, PrefixAtom, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(self.exponent * rhs)
    }
}

impl Inv for PrefixAtomExponent {
    type Output = InvOutput<PrefixAtom, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(prefix_atom) => InvOutput::One(prefix_atom),
            PowOutput::Rest(prefix_atom_exponent) => InvOutput::Rest(prefix_atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut PrefixAtomExponent {
    type Output = InvOutput<PrefixAtom, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(prefix_atom) => InvOutput::One(prefix_atom),
            PowOutput::Rest(prefix_atom_exponent) => InvOutput::Rest(prefix_atom_exponent),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl SetAnnotation for PrefixAtomExponent {
    type Output = PrefixAtomExponentAnnotation;

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        Self::Output::new(
            self.prefix,
            self.atom,
            self.exponent,
            Annotation::from(annotation),
        )
    }
}
