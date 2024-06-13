use std::fmt;

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Annotation, Atom, Composable,
    IsCompatibleWith, Prefix, UcumSymbol, UcumUnit,
};

use super::{
    AssignFactor, Exponent, Factor, FactorPrefixAtom, FactorPrefixAtomExponent, PowOutput,
    PrefixAtomAnnotation, PrefixAtomExponent, SetAnnotation, SetExponent, Term,
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

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        self.prefix.definition_value() * self.atom.scalar()
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

impl ComposablyEq<Term> for PrefixAtom {
    fn composably_eq(&self, rhs: &Term) -> Option<Exponent> {
        match rhs {
            Term::PrefixAtom(inner) => self.composably_eq(inner),
            Term::PrefixAtomExponent(inner) => self.composably_eq(inner),
            Term::FactorPrefixAtom(inner) => self.composably_eq(inner),
            Term::FactorPrefixAtomExponent(inner) => self.composably_eq(inner),
            _ => None,
        }
    }
}

impl ComposablyEq<Self> for PrefixAtom {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self == rhs {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<PrefixAtomExponent> for PrefixAtom {
    fn composably_eq(&self, rhs: &PrefixAtomExponent) -> Option<Exponent> {
        if self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorPrefixAtom> for PrefixAtom {
    fn composably_eq(&self, rhs: &FactorPrefixAtom) -> Option<Exponent> {
        if rhs.factor == 1 && self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorPrefixAtomExponent> for PrefixAtom {
    fn composably_eq(&self, rhs: &FactorPrefixAtomExponent) -> Option<Exponent> {
        if rhs.factor == 1 && self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for PrefixAtom {
    fn composition(&self) -> crate::Composition {
        self.atom.composition()
    }
}

impl IsCompatibleWith<Term> for PrefixAtom {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition() && rhs.annotation().is_none()
    }
}

impl TermReduce for PrefixAtom {
    fn build(&self, exponent: Exponent) -> Term {
        if exponent == 1 {
            Term::PrefixAtom(Self {
                prefix: self.prefix,
                atom: self.atom,
            })
        } else {
            Term::PrefixAtomExponent(PrefixAtomExponent {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            })
        }
    }
}
