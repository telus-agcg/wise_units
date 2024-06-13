use std::borrow::Cow;

use crate::{Composable, Composition};

use super::{variants::*, Term};

impl Composable for Term {
    /// Combines the `Composition` from the `Term`'s `Atom` with its own `exponent` to build a
    /// `Composition`. If the `Term` has no `Atom`, it has no dimension, thus will have an empty
    /// `Composition`.
    ///
    // TODO: https://agrian.atlassian.net/browse/DEV-971
    //
    #[allow(clippy::redundant_closure)]
    fn composition(&self) -> Composition {
        match self {
            Self::Annotation(_)
            | Self::Factor(_)
            | Self::FactorAnnotation { .. }
            | Self::FactorExponent { .. }
            | Self::FactorExponentAnnotation { .. } => Composition::default(),
            Self::Atom(atom)
            | Self::AtomAnnotation(AtomAnnotation { atom, .. })
            | Self::PrefixAtom(PrefixAtom { atom, .. })
            | Self::PrefixAtomAnnotation(PrefixAtomAnnotation { atom, .. })
            | Self::FactorAtom(FactorAtom { atom, .. })
            | Self::FactorAtomAnnotation(FactorAtomAnnotation { atom, .. })
            | Self::FactorPrefixAtom(FactorPrefixAtom { atom, .. })
            | Self::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation { atom, .. }) => {
                atom.composition()
            }
            Self::AtomExponent(AtomExponent { atom, exponent })
            | Self::AtomExponentAnnotation(AtomExponentAnnotation { atom, exponent, .. })
            | Self::PrefixAtomExponent(PrefixAtomExponent { atom, exponent, .. })
            | Self::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
                atom,
                exponent,
                ..
            })
            | Self::FactorAtomExponent(FactorAtomExponent { atom, exponent, .. })
            | Self::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
                atom,
                exponent,
                ..
            })
            | Self::FactorPrefixAtomExponent(FactorPrefixAtomExponent { atom, exponent, .. })
            | Self::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
                atom,
                exponent,
                ..
            }) => atom.composition() * *exponent,
        }
    }
}

impl<'a> Composable for Cow<'a, [Term]> {
    fn composition(&self) -> Composition {
        self.iter()
            .fold(Composition::default(), |acc, term| acc * term.composition())
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
