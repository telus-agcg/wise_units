use std::borrow::Cow;

use crate::{composable::ComposablyEq, term::variants::FactorExponent, Composable, Composition};

use super::{Exponent, Term};

impl Composable for Term {
    /// Combines the `Composition` from the `Term`'s `Atom` with its own `exponent` to build a
    /// `Composition`. If the `Term` has no `Atom`, it has no dimension, thus will have an empty
    /// `Composition`.
    ///
    // TODO: https://agrian.atlassian.net/browse/DEV-971
    //
    fn composition(&self) -> Composition {
        match self {
            Self::Annotation(inner) => inner.composition(),
            Self::Atom(inner) => inner.composition(),
            Self::AtomAnnotation(inner) => inner.composition(),
            Self::AtomExponent(inner) => inner.composition(),
            Self::AtomExponentAnnotation(inner) => inner.composition(),
            Self::PrefixAtom(inner) => inner.composition(),
            Self::PrefixAtomAnnotation(inner) => inner.composition(),
            Self::PrefixAtomExponent(inner) => inner.composition(),
            Self::PrefixAtomExponentAnnotation(inner) => inner.composition(),
            Self::Factor(_) => Composition::new_dimless(),
            Self::FactorAnnotation(inner) => inner.composition(),
            Self::FactorExponent(inner) => inner.composition(),
            Self::FactorExponentAnnotation(inner) => inner.composition(),
            Self::FactorAtom(inner) => inner.composition(),
            Self::FactorAtomAnnotation(inner) => inner.composition(),
            Self::FactorAtomExponent(inner) => inner.composition(),
            Self::FactorAtomExponentAnnotation(inner) => inner.composition(),
            Self::FactorPrefixAtom(inner) => inner.composition(),
            Self::FactorPrefixAtomAnnotation(inner) => inner.composition(),
            Self::FactorPrefixAtomExponent(inner) => inner.composition(),
            Self::FactorPrefixAtomExponentAnnotation(inner) => inner.composition(),
        }
    }
}

impl<'a> Composable for Cow<'a, [Term]> {
    fn composition(&self) -> Composition {
        self.iter()
            .fold(Composition::default(), |acc, term| acc * term.composition())
    }
}

impl ComposablyEq<Self> for Term {
    fn composably_eq(&self, rhs: &Self) -> Option<Exponent> {
        match self {
            Self::Annotation(lhs_annotation) => lhs_annotation.composably_eq(rhs),
            Self::Atom(lhs_atom) => lhs_atom.composably_eq(rhs),
            Self::AtomAnnotation(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::AtomExponent(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::AtomExponentAnnotation(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::PrefixAtom(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::PrefixAtomAnnotation(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::PrefixAtomExponent(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::PrefixAtomExponentAnnotation(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::Factor(lhs_factor) => match rhs {
                Self::Factor(factor) => {
                    if lhs_factor == factor {
                        Some(2)
                    } else {
                        None
                    }
                }
                Self::FactorExponent(FactorExponent { factor, exponent }) => {
                    if lhs_factor == factor {
                        Some(exponent + 1)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Self::FactorAnnotation(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::FactorExponent(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::FactorExponentAnnotation(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::FactorAtom(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::FactorAtomAnnotation(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::FactorAtomExponent(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::FactorAtomExponentAnnotation(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::FactorPrefixAtom(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::FactorPrefixAtomAnnotation(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::FactorPrefixAtomExponent(lhs_inner) => lhs_inner.composably_eq(rhs),
            Self::FactorPrefixAtomExponentAnnotation(lhs_inner) => lhs_inner.composably_eq(rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Dimension;

    use super::*;

    macro_rules! validate_composition {
        ($test_name:ident, $term:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let term = $term;
                assert_eq!(term.composition(), $expected_value);
            }
        };

        ($test_name:ident, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let term = term!();
                assert_eq!(term.composition(), $expected_value);
            }
        };
    }

    // Composition tests
    validate_composition!(validate_composition_blank, Composition::default());
    validate_composition!(
        validate_composition_meter,
        term!(Meter),
        Composition::new(Dimension::Length, 1)
    );
    validate_composition!(
        validate_composition_kilometer,
        term!(Kilo, Meter),
        Composition::new(Dimension::Length, 1)
    );
    validate_composition!(
        validate_composition_meter_positive_non1_exponent,
        term!(Meter, exponent: 2),
        Composition::new(Dimension::Length, 2)
    );
    validate_composition!(
        validate_composition_meter_negative_exponent,
        term!(Meter, exponent: -1),
        Composition::new(Dimension::Length, -1)
    );
    validate_composition!(
        validate_composition_meter_negative_exponent2,
        term!(Meter, exponent: -2),
        Composition::new(Dimension::Length, -2)
    );
    validate_composition!(
        validate_composition_meter_factor,
        term!(Meter, factor: 10),
        Composition::new(Dimension::Length, 1)
    );
    validate_composition!(
        validate_composition_kilometer_factor,
        term!(Kilo, Meter, factor: 10),
        Composition::new(Dimension::Length, 1)
    );
    validate_composition!(
        validate_composition_kilometer_factor_negative_exponent,
        term!(Kilo, Meter, factor: 10, exponent: -1),
        Composition::new(Dimension::Length, -1)
    );
}
