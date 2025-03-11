use std::fmt;

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Annotation, Atom, Composable,
    IsCompatibleWith, Prefix, UcumSymbol, UcumUnit,
};

use super::{
    Exponent, Factor, FactorPrefixAtomAnnotation, FactorPrefixAtomExponent, PowOutput, PrefixAtom,
    PrefixAtomExponent, SetAnnotation, SetExponent, Term,
};

// ╭──────────────────╮
// │ FactorPrefixAtom │
// ╰──────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        f64::from(self.factor) * self.prefix.definition_value() * self.atom.scalar()
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

impl ComposablyEq<Term> for FactorPrefixAtom {
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

impl ComposablyEq<PrefixAtom> for FactorPrefixAtom {
    fn composably_eq(&self, rhs: &PrefixAtom) -> Option<Exponent> {
        if self.factor == 1 && self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<PrefixAtomExponent> for FactorPrefixAtom {
    fn composably_eq(&self, rhs: &PrefixAtomExponent) -> Option<Exponent> {
        if self.factor == 1 && self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for FactorPrefixAtom {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self == rhs {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorPrefixAtomExponent> for FactorPrefixAtom {
    fn composably_eq(&self, rhs: &FactorPrefixAtomExponent) -> Option<Exponent> {
        if self.factor == rhs.factor && self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for FactorPrefixAtom {
    fn composition(&self) -> crate::Composition {
        self.atom.composition()
    }
}

impl IsCompatibleWith<Term> for FactorPrefixAtom {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition() && rhs.annotation().is_none()
    }
}

impl TermReduce for FactorPrefixAtom {
    fn build(&self, exponent: Exponent) -> Term {
        match (self.factor, exponent) {
            (1, 1) => Term::PrefixAtom(PrefixAtom {
                prefix: self.prefix,
                atom: self.atom,
            }),
            (1, _) => Term::PrefixAtomExponent(PrefixAtomExponent {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            }),
            (_, 1) => Term::FactorPrefixAtom(Self {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
            }),
            (_, _) => Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            }),
        }
    }
}
