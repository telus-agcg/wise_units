use std::fmt;

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Annotation, Atom, Composable,
    IsCompatibleWith, UcumUnit,
};

use super::{
    AtomExponent, Exponent, Factor, FactorAtomAnnotation, FactorAtomExponent, PowOutput,
    SetAnnotation, SetExponent, Term,
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

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        f64::from(self.factor) * self.atom.scalar()
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

impl ComposablyEq<Term> for FactorAtom {
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

impl ComposablyEq<Atom> for FactorAtom {
    fn composably_eq(&self, rhs: &Atom) -> Option<Exponent> {
        if self.factor == 1 && self.atom == *rhs {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<AtomExponent> for FactorAtom {
    fn composably_eq(&self, rhs: &AtomExponent) -> Option<Exponent> {
        if self.factor == 1 && self.atom == rhs.atom {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for FactorAtom {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self == rhs {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAtomExponent> for FactorAtom {
    fn composably_eq(&self, rhs: &FactorAtomExponent) -> Option<Exponent> {
        if self.factor == rhs.factor && self.atom == rhs.atom {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for FactorAtom {
    fn composition(&self) -> crate::Composition {
        self.atom.composition()
    }
}

impl IsCompatibleWith<Term> for FactorAtom {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition() && rhs.annotation().is_none()
    }
}

impl TermReduce for FactorAtom {
    fn build(&self, exponent: Exponent) -> Term {
        match (self.factor, exponent) {
            (1, 1) => Term::Atom(self.atom),
            (1, _) => Term::AtomExponent(AtomExponent {
                atom: self.atom,
                exponent,
            }),
            (_, 1) => Term::FactorAtom(Self {
                factor: self.factor,
                atom: self.atom,
            }),
            (_, _) => Term::FactorAtomExponent(FactorAtomExponent {
                factor: self.factor,
                atom: self.atom,
                exponent,
            }),
        }
    }
}
