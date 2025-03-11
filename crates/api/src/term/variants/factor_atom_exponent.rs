use std::fmt;

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Annotation, Atom, Composable,
    IsCompatibleWith, UcumUnit,
};

use super::{
    AtomExponent, Exponent, Factor, FactorAtom, FactorAtomExponentAnnotation, InvOutput, PowOutput,
    SetAnnotation, SetExponent, Term,
};

// ╭────────────────────╮
// │ FactorAtomExponent │
// ╰────────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    #[must_use]
    pub const fn factor(&self) -> Factor {
        self.factor
    }

    #[must_use]
    pub const fn atom(&self) -> Atom {
        self.atom
    }

    #[must_use]
    pub const fn exponent(&self) -> Exponent {
        self.exponent
    }

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        f64::from(self.factor) * self.atom.scalar().powi(self.exponent)
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

impl SetAnnotation for FactorAtomExponent {
    type Output = FactorAtomExponentAnnotation;

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        FactorAtomExponentAnnotation::new(
            self.factor,
            self.atom,
            self.exponent,
            Annotation::from(annotation),
        )
    }
}

impl ComposablyEq<Term> for FactorAtomExponent {
    fn composably_eq(&self, rhs: &Term) -> Option<Exponent> {
        match rhs {
            Term::Atom(inner) => self.composably_eq(inner),
            Term::AtomExponent(inner) => self.composably_eq(inner),
            Term::FactorAtom(inner) => self.composably_eq(inner),
            Term::FactorAtomExponent(inner) => self.composably_eq(inner),
            _ => None,
        }
    }
}

impl ComposablyEq<Atom> for FactorAtomExponent {
    fn composably_eq(&self, rhs: &Atom) -> Option<Exponent> {
        if self.factor == 1 && self.atom == *rhs {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<AtomExponent> for FactorAtomExponent {
    fn composably_eq(&self, rhs: &AtomExponent) -> Option<Exponent> {
        if self.factor == 1 && self.atom == rhs.atom {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAtom> for FactorAtomExponent {
    fn composably_eq(&self, rhs: &FactorAtom) -> Option<Exponent> {
        if self.factor == rhs.factor && self.atom == rhs.atom {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for FactorAtomExponent {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self.factor == rhs.factor && self.atom == rhs.atom {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for FactorAtomExponent {
    fn composition(&self) -> crate::Composition {
        self.atom.composition() * self.exponent
    }
}

impl IsCompatibleWith<Term> for FactorAtomExponent {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition() && rhs.annotation().is_none()
    }
}

impl TermReduce for FactorAtomExponent {
    fn build(&self, exponent: Exponent) -> Term {
        match (self.factor, exponent) {
            (1, 1) => Term::Atom(self.atom),
            (1, _) => Term::AtomExponent(AtomExponent {
                atom: self.atom,
                exponent,
            }),
            (_, 1) => Term::FactorAtom(FactorAtom {
                factor: self.factor,
                atom: self.atom,
            }),
            (_, _) => Term::FactorAtomExponent(Self {
                factor: self.factor,
                atom: self.atom,
                exponent,
            }),
        }
    }
}
