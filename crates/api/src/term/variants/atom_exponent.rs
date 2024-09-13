use std::fmt;

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Annotation, Atom, Composable,
    IsCompatibleWith, UcumUnit,
};

use super::{
    AssignFactor, AtomExponentAnnotation, Exponent, Factor, FactorAtom, FactorAtomExponent,
    InvOutput, PowOutput, SetAnnotation, SetExponent, Term,
};

// ╭──────────────╮
// │ AtomExponent │
// ╰──────────────╯
/// Ex. "m2"
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        self.atom.scalar().powi(self.exponent)
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

impl ComposablyEq<Term> for AtomExponent {
    fn composably_eq(&self, rhs: &Term) -> Option<Exponent> {
        match rhs {
            Term::Atom(atom) => self.composably_eq(atom),
            Term::AtomExponent(inner) => self.composably_eq(inner),
            Term::FactorAtom(inner) => self.composably_eq(inner),
            Term::FactorAtomExponent(inner) => self.composably_eq(inner),
            _ => None,
        }
    }
}

impl ComposablyEq<Atom> for AtomExponent {
    fn composably_eq(&self, rhs: &Atom) -> Option<Exponent> {
        if self.atom == *rhs {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for AtomExponent {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self.atom == rhs.atom {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAtom> for AtomExponent {
    fn composably_eq(&self, rhs: &FactorAtom) -> Option<Exponent> {
        if rhs.factor == 1 && self.atom == rhs.atom {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAtomExponent> for AtomExponent {
    fn composably_eq(&self, rhs: &FactorAtomExponent) -> Option<Exponent> {
        if rhs.factor == 1 && self.atom == rhs.atom {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for AtomExponent {
    fn composition(&self) -> crate::Composition {
        self.atom.composition() * self.exponent
    }
}

impl IsCompatibleWith<Term> for AtomExponent {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition() && rhs.annotation().is_none()
    }
}

impl TermReduce for AtomExponent {
    fn build(&self, exponent: Exponent) -> Term {
        if exponent == 1 {
            Term::Atom(self.atom)
        } else {
            Term::AtomExponent(Self {
                atom: self.atom,
                exponent,
            })
        }
    }
}
