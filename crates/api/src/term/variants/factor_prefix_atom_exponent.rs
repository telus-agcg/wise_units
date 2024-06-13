use std::fmt;

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Annotation, Atom, Composable,
    IsCompatibleWith, Prefix, UcumSymbol, UcumUnit,
};

use super::{
    Exponent, Factor, FactorPrefixAtom, FactorPrefixAtomExponentAnnotation, InvOutput, PowOutput,
    PrefixAtom, PrefixAtomExponent, SetAnnotation, SetExponent, Term,
};

// ╭──────────────────────────╮
// │ FactorPrefixAtomExponent │
// ╰──────────────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FactorPrefixAtomExponent {
    pub(crate) factor: Factor,
    pub(crate) prefix: Prefix,
    pub(crate) atom: Atom,
    pub(crate) exponent: Exponent,
}

impl FactorPrefixAtomExponent {
    #[must_use]
    pub const fn new(factor: Factor, prefix: Prefix, atom: Atom, exponent: Exponent) -> Self {
        Self {
            factor,
            prefix,
            atom,
            exponent,
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
    pub const fn exponent(&self) -> Exponent {
        self.exponent
    }

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        f64::from(self.factor)
            * self.prefix.definition_value()
            * self.atom.scalar().powi(self.exponent)
    }
}

impl From<FactorPrefixAtomExponent> for Term {
    fn from(value: FactorPrefixAtomExponent) -> Self {
        Self::FactorPrefixAtomExponent(value)
    }
}

impl fmt::Display for FactorPrefixAtomExponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.factor, self.exponent) {
            (1, 1) => {
                write!(f, "{prefix}{atom}", prefix = self.prefix, atom = self.atom)
            }
            (1, exponent) => {
                write!(
                    f,
                    "{prefix}{atom}{exponent}",
                    prefix = self.prefix,
                    atom = self.atom,
                )
            }
            (factor, 1) => {
                write!(
                    f,
                    "{factor}{prefix}{atom}",
                    prefix = self.prefix,
                    atom = self.atom,
                )
            }
            (factor, exponent) => {
                write!(
                    f,
                    "{factor}{prefix}{atom}{exponent}",
                    prefix = self.prefix,
                    atom = self.atom,
                )
            }
        }
    }
}

impl SetExponent for FactorPrefixAtomExponent {
    type Output = PowOutput<Factor, FactorPrefixAtom, ()>;

    fn set_exponent(&mut self, exponent: Exponent) -> Self::Output {
        match exponent {
            0 => PowOutput::Zero(1),
            1 => PowOutput::One(FactorPrefixAtom {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
            }),
            _ => {
                self.exponent = exponent;
                PowOutput::Rest(())
            }
        }
    }
}

impl Pow<Exponent> for FactorPrefixAtomExponent {
    type Output = PowOutput<Factor, FactorPrefixAtom, Self>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        let mut s = self;

        match s.set_exponent(s.exponent * rhs) {
            PowOutput::Zero(factor) => PowOutput::Zero(factor),
            PowOutput::One(fpa) => PowOutput::One(fpa),
            PowOutput::Rest(()) => PowOutput::Rest(s),
        }
    }
}

impl<'a> Pow<Exponent> for &'a mut FactorPrefixAtomExponent {
    type Output = PowOutput<Factor, FactorPrefixAtom, ()>;

    fn pow(self, rhs: Exponent) -> Self::Output {
        self.set_exponent(self.exponent * rhs)
    }
}

impl Inv for FactorPrefixAtomExponent {
    type Output = InvOutput<FactorPrefixAtom, Self>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(fpa) => InvOutput::One(fpa),
            PowOutput::Rest(fpae) => InvOutput::Rest(fpae),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl<'a> Inv for &'a mut FactorPrefixAtomExponent {
    type Output = InvOutput<FactorPrefixAtom, ()>;

    fn inv(self) -> Self::Output {
        match self.pow(-1) {
            PowOutput::One(fpa) => InvOutput::One(fpa),
            PowOutput::Rest(fpae) => InvOutput::Rest(fpae),
            PowOutput::Zero(_) => unreachable!(),
        }
    }
}

impl SetAnnotation for FactorPrefixAtomExponent {
    type Output = FactorPrefixAtomExponentAnnotation;

    fn set_annotation<T>(self, annotation: T) -> Self::Output
    where
        Annotation: From<T>,
    {
        Self::Output::new(
            self.factor,
            self.prefix,
            self.atom,
            self.exponent,
            Annotation::from(annotation),
        )
    }
}

impl ComposablyEq<Term> for FactorPrefixAtomExponent {
    fn composably_eq(&self, rhs: &Term) -> Option<Exponent> {
        match rhs {
            Term::FactorPrefixAtom(inner) => self.composably_eq(inner),
            Term::FactorPrefixAtomExponent(inner) => self.composably_eq(inner),
            Term::PrefixAtom(inner) => self.composably_eq(inner),
            Term::PrefixAtomExponent(inner) => self.composably_eq(inner),
            _ => None,
        }
    }
}

impl ComposablyEq<FactorPrefixAtom> for FactorPrefixAtomExponent {
    fn composably_eq(&self, rhs: &FactorPrefixAtom) -> Option<Exponent> {
        if self.factor == rhs.factor && self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for FactorPrefixAtomExponent {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self.factor == rhs.factor && self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<PrefixAtom> for FactorPrefixAtomExponent {
    fn composably_eq(&self, rhs: &PrefixAtom) -> Option<Exponent> {
        if self.factor == 1 && self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<PrefixAtomExponent> for FactorPrefixAtomExponent {
    fn composably_eq(&self, rhs: &PrefixAtomExponent) -> Option<Exponent> {
        if self.factor == 1 && self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for FactorPrefixAtomExponent {
    fn composition(&self) -> crate::Composition {
        self.atom.composition() * self.exponent
    }
}

impl IsCompatibleWith<Term> for FactorPrefixAtomExponent {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition() && rhs.annotation().is_none()
    }
}

impl TermReduce for FactorPrefixAtomExponent {
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
            (_, 1) => Term::FactorPrefixAtom(FactorPrefixAtom {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
            }),
            (_, _) => Term::FactorPrefixAtomExponent(Self {
                factor: self.factor,
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            }),
        }
    }
}
