use crate::{
    composable::ComposablyEq,
    term::{
        variants::{AtomExponent, FactorAtom, FactorAtomExponent},
        Exponent,
    },
    Atom, Composable, Composition, Dimension, Term,
};

impl Composable for Atom {
    fn composition(&self) -> Composition {
        match self {
            Self::Candela => Composition::new(Dimension::LuminousIntensity, 1),
            Self::Coulomb => Composition::new(Dimension::ElectricCharge, 1),
            Self::Gram => Composition::new(Dimension::Mass, 1),
            Self::Kelvin => Composition::new(Dimension::Temperature, 1),
            Self::Meter => Composition::new(Dimension::Length, 1),
            Self::Radian => Composition::new(Dimension::PlaneAngle, 1),
            Self::Second => Composition::new(Dimension::Time, 1),
            _ => self.definition().terms().composition(),
        }
    }
}

impl ComposablyEq<Term> for Atom {
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

impl ComposablyEq<Self> for Atom {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        if self == rhs {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<AtomExponent> for Atom {
    fn composably_eq(&self, rhs: &AtomExponent) -> Option<Exponent> {
        if *self == rhs.atom {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAtom> for Atom {
    fn composably_eq(&self, rhs: &FactorAtom) -> Option<Exponent> {
        if rhs.factor == 1 && *self == rhs.atom {
            Some(2)
        } else {
            None
        }
    }
}

impl ComposablyEq<FactorAtomExponent> for Atom {
    fn composably_eq(&self, rhs: &FactorAtomExponent) -> Option<Exponent> {
        if rhs.factor == 1 && *self == rhs.atom {
            Some(1 + rhs.exponent)
        } else {
            None
        }
    }
}
