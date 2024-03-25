use std::borrow::Cow;

use crate::{parser::Composition, Composable};

use super::Term;

impl Composable for Term {
    /// Combines the `Composition` from the `Term`'s `Atom` with its own `exponent` to build a
    /// `Composition`. If the `Term` has no `Atom`, it has no dimension, thus will have an empty
    /// `Composition`.
    ///
    // TODO: https://agrian.atlassian.net/browse/DEV-971
    //
    #[allow(clippy::redundant_closure)]
    fn composition(&self) -> Composition {
        self.atom.map_or_else(
            || Composition::default(),
            |atom| {
                let atom_composition = atom.composition();
                self.exponent.map_or(atom_composition, |term_exponent| {
                    atom_composition * term_exponent
                })
            },
        )
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
