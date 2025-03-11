use std::fmt;

use num_traits::{Inv, Pow};

use crate::{
    composable::ComposablyEq, term::term_reduce::TermReduce, Annotation, Atom, Composable,
    IsCompatibleWith, Prefix, UcumSymbol, UcumUnit,
};

use super::{
    AssignFactor, Exponent, Factor, FactorPrefixAtom, FactorPrefixAtomExponent, InvOutput,
    PowOutput, PrefixAtom, PrefixAtomExponentAnnotation, SetAnnotation, SetExponent, Term,
};

// ╭────────────────────╮
// │ PrefixAtomExponent │
// ╰────────────────────╯
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    #[must_use]
    pub fn to_scalar(&self) -> f64 {
        self.prefix.definition_value() * self.atom.scalar().powi(self.exponent)
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

impl ComposablyEq<Term> for PrefixAtomExponent {
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

impl ComposablyEq<PrefixAtom> for PrefixAtomExponent {
    fn composably_eq(&self, rhs: &PrefixAtom) -> Option<Exponent> {
        if self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<Self> for PrefixAtomExponent {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorPrefixAtom> for PrefixAtomExponent {
    fn composably_eq(&self, rhs: &FactorPrefixAtom) -> Option<Exponent> {
        if rhs.factor == 1 && self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(self.exponent + 1)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorPrefixAtomExponent> for PrefixAtomExponent {
    fn composably_eq(&self, rhs: &FactorPrefixAtomExponent) -> Option<Exponent> {
        if rhs.factor == 1 && self.prefix == rhs.prefix && self.atom == rhs.atom {
            Some(self.exponent + rhs.exponent)
        } else {
            None
        }
    }
}

impl Composable for PrefixAtomExponent {
    fn composition(&self) -> crate::Composition {
        self.atom.composition() * self.exponent
    }
}

impl IsCompatibleWith<Term> for PrefixAtomExponent {
    fn is_compatible_with(&self, rhs: &Term) -> bool {
        self.composition() == rhs.composition() && rhs.annotation().is_none()
    }
}

impl TermReduce for PrefixAtomExponent {
    fn build(&self, exponent: Exponent) -> Term {
        if exponent == 1 {
            Term::PrefixAtom(PrefixAtom {
                prefix: self.prefix,
                atom: self.atom,
            })
        } else {
            Term::PrefixAtomExponent(Self {
                prefix: self.prefix,
                atom: self.atom,
                exponent,
            })
        }
    }
}
