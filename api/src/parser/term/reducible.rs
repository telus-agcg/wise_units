use reducible::Reducible;
use super::Term;
use parser::ucum_symbol::UcumSymbol;

impl Reducible for Term {
    fn reduce_value(&self, value: f64) -> f64 {
        let atom_scalar = self.atom.map_or(1.0, |a| a.reduce_value(value));
        let prefix_scalar = self.prefix.map_or(1.0, |p| p.definition_value());

        combine_term_values(atom_scalar, prefix_scalar, self.factor, self.exponent)
    }

    fn calculate_magnitude(&self, value: f64) -> f64 {
        let atom_magnitude = self.atom.map_or(1.0, |a| a.calculate_magnitude(value));
        let prefix_magnitude = self.prefix.map_or(1.0, |p| p.definition_value());

        combine_term_values(atom_magnitude, prefix_magnitude, self.factor, self.exponent)
    }
}

impl Reducible for Vec<Term> {
    fn reduce_value(&self, value: f64) -> f64 {
        self.iter()
            .fold(1.0, |acc, term| acc * term.reduce_value(value))
    }

    fn calculate_magnitude(&self, value: f64) -> f64 {
        self.iter()
            .fold(1.0, |acc, term| acc * term.calculate_magnitude(value))
    }
}

fn combine_term_values(
    calculated_atom: f64,
    calculated_prefix: f64,
    factor: Option<u32>,
    exponent: Option<i32>,
) -> f64 {
    let a_p_product = calculated_atom * calculated_prefix;

    match factor {
        Some(f) => {
            let product = a_p_product * f64::from(f);

            match exponent {
                Some(e) => product.powi(e),
                None => product,
            }
        }
        None => match exponent {
            Some(e) => a_p_product.powi(e),
            None => a_p_product,
        },
    }
}

#[cfg(test)]
mod tests {
    use parser::{Atom, Prefix, Term};
    use reducible::Reducible;

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
