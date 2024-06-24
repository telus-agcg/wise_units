use std::borrow::Cow;

use num_traits::One;

use crate::{reducible::Reducible, Atom, UcumSymbol};

use super::{
    variants::{
        AtomAnnotation, AtomExponent, AtomExponentAnnotation, FactorAnnotation, FactorAtom,
        FactorAtomAnnotation, FactorAtomExponent, FactorAtomExponentAnnotation, FactorExponent,
        FactorExponentAnnotation, FactorPrefixAtom, FactorPrefixAtomAnnotation,
        FactorPrefixAtomExponent, FactorPrefixAtomExponentAnnotation, PrefixAtom,
        PrefixAtomAnnotation, PrefixAtomExponent, PrefixAtomExponentAnnotation,
    },
    Term,
};

impl Reducible<f64> for Term {
    fn reduce_value(&self, value: f64) -> f64 {
        calculate(self, value, Reducible::reduce_value)
    }

    fn calculate_magnitude(&self, value: f64) -> f64 {
        calculate(self, value, Atom::calculate_magnitude)
    }
}

fn calculate<F>(term: &Term, value: f64, atom_function: F) -> f64
where
    F: Fn(&Atom, f64) -> f64,
{
    match term {
        Term::Annotation(_) => One::one(),
        Term::Atom(atom) | Term::AtomAnnotation(AtomAnnotation { atom, .. }) => {
            atom_function(atom, value)
        }
        Term::AtomExponent(AtomExponent { atom, exponent })
        | Term::AtomExponentAnnotation(AtomExponentAnnotation { atom, exponent, .. }) => {
            atom_function(atom, value).powi(*exponent)
        }
        Term::PrefixAtom(PrefixAtom { prefix, atom })
        | Term::PrefixAtomAnnotation(PrefixAtomAnnotation { prefix, atom, .. }) => {
            prefix.definition_value() * atom_function(atom, value)
        }
        Term::PrefixAtomExponent(PrefixAtomExponent {
            prefix,
            atom,
            exponent,
        })
        | Term::PrefixAtomExponentAnnotation(PrefixAtomExponentAnnotation {
            prefix,
            atom,
            exponent,
            ..
        }) => (prefix.definition_value() * atom_function(atom, value)).powi(*exponent),
        Term::Factor(factor) | Term::FactorAnnotation(FactorAnnotation { factor, .. }) => {
            f64::from(*factor)
        }
        Term::FactorExponent(FactorExponent { factor, exponent })
        | Term::FactorExponentAnnotation(FactorExponentAnnotation {
            factor, exponent, ..
        }) => f64::from(*factor).powi(*exponent),
        Term::FactorAtom(FactorAtom { factor, atom })
        | Term::FactorAtomAnnotation(FactorAtomAnnotation { factor, atom, .. }) => {
            f64::from(*factor) * atom_function(atom, value)
        }
        Term::FactorAtomExponent(FactorAtomExponent {
            factor,
            atom,
            exponent,
        })
        | Term::FactorAtomExponentAnnotation(FactorAtomExponentAnnotation {
            factor,
            atom,
            exponent,
            ..
        }) => (f64::from(*factor) * atom_function(atom, value)).powi(*exponent),
        Term::FactorPrefixAtom(FactorPrefixAtom {
            factor,
            prefix,
            atom,
        })
        | Term::FactorPrefixAtomAnnotation(FactorPrefixAtomAnnotation {
            factor,
            prefix,
            atom,
            ..
        }) => f64::from(*factor) * prefix.definition_value() * atom_function(atom, value),
        Term::FactorPrefixAtomExponent(FactorPrefixAtomExponent {
            factor,
            prefix,
            atom,
            exponent,
        })
        | Term::FactorPrefixAtomExponentAnnotation(FactorPrefixAtomExponentAnnotation {
            factor,
            prefix,
            atom,
            exponent,
            ..
        }) => (f64::from(*factor) * prefix.definition_value() * atom_function(atom, value))
            .powi(*exponent),
    }
}

impl<'a> Reducible<f64> for Cow<'a, [Term]> {
    fn reduce_value(&self, value: f64) -> f64 {
        self.iter()
            .fold(One::one(), |acc, term| acc * term.reduce_value(value))
    }

    fn calculate_magnitude(&self, value: f64) -> f64 {
        self.iter().fold(One::one(), |acc, term| {
            acc * term.calculate_magnitude(value)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::reducible::Reducible;
    use approx::assert_relative_eq;

    macro_rules! validate_reduce_value {
        ($test_name:ident, $term:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let term = $term;
                assert_relative_eq!(term.reduce_value(1.0), $expected_value);
            }
        };
    }

    macro_rules! validate_calculate_magnitude {
        ($test_name:ident, $term:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let term = $term;
                let scalar = term.reduce_value(1.0);
                assert_relative_eq!(term.calculate_magnitude(scalar), $expected_value);
            }
        };
    }

    validate_reduce_value!(validate_reduce_value_meter, term!(Meter), 1.0);
    validate_reduce_value!(validate_reduce_value_kilometer, term!(Kilo, Meter), 1000.0);
    validate_reduce_value!(
        validate_reduce_value_meter_minus1,
        term!(Meter, exponent: -1),
        1.0
    );
    validate_reduce_value!(
        validate_reduce_value_meter_factor,
        term!(Meter, factor: 10),
        10.0
    );
    validate_reduce_value!(
        validate_reduce_value_kilometer_factor,
        term!(Kilo, Meter, factor: 10),
        10_000.0
    );
    validate_reduce_value!(
        validate_reduce_value_kilometer_factor_exponent,
        term!(Kilo, Meter, exponent: -1, factor: 10),
        0.0001
    );
    validate_reduce_value!(validate_reduce_value_liter, term!(Liter), 0.001);
    validate_reduce_value!(
        validate_reduce_value_pi,
        term!(TheNumberPi),
        ::std::f64::consts::PI
    );
    validate_reduce_value!(
        validate_reduce_value_pi_factor,
        term!(TheNumberPi, factor: 10),
        ::std::f64::consts::PI * 10.0
    );
    validate_reduce_value!(validate_reduce_value_hectare, term!(Hecto, Are), 10_000.0);
    validate_reduce_value!(validate_reduce_value_week, term!(Week), 604_800.0);
    validate_reduce_value!(validate_reduce_value_kilogram, term!(Kilo, Gram), 1000.0);
    validate_reduce_value!(
        validate_reduce_value_fahrenheit,
        term!(DegreeFahrenheit),
        255.927_777_777_777_8
    );

    // magnitude tests
    validate_calculate_magnitude!(validate_calculate_magnitude_meter, term!(Meter), 1.0);
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer,
        term!(Kilo, Meter),
        1000.0
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_meter_minus1,
        term!(Meter, exponent: -1),
        1.0
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_meter_factor,
        term!(Meter, factor: 10),
        10.0
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer_factor,
        term!(Kilo, Meter, factor: 10),
        10_000.0
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilometer_factor_exponent,
        term!(Kilo, Meter, exponent: -1, factor: 10),
        0.000_1
    );
    validate_calculate_magnitude!(validate_calculate_magnitude_liter, term!(Liter), 1.0);
    validate_calculate_magnitude!(validate_calculate_magnitude_pi, term!(TheNumberPi), 1.0);
    validate_calculate_magnitude!(
        validate_calculate_magnitude_pi_factor,
        term!(TheNumberPi, factor: 10),
        10.0
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_hectare,
        term!(Hecto, Are),
        100.0
    );
    validate_calculate_magnitude!(validate_calculate_magnitude_week, term!(Week), 1.0);
    validate_calculate_magnitude!(
        validate_calculate_magnitude_kilogram,
        term!(Kilo, Gram),
        1000.0
    );
    validate_calculate_magnitude!(
        validate_calculate_magnitude_fahrenheit,
        term!(DegreeFahrenheit),
        1.000_000_000_000_056_8
    );
}
