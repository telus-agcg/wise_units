use atom::Atom;
use composable::Composable;
use composition::Composition;
use prefix::Prefix;
use std::fmt;
use ucum_symbol::UcumSymbol;

/// A Term makes up an Atom (at its core) along with any Atom modifiers
/// (anything that can change its scalar). It is, however, possible to have an
/// Atom-less Term, which would simple be a Factor (with or without an
/// annotation) (ex. the 10 in "10" or "10/m" would be an Atom-less Term).
///
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Term {
    pub atom: Option<Atom>,
    pub prefix: Option<Prefix>,
    pub factor: u32,
    pub exponent: i32,
    pub annotation: Option<String>,
}

impl Term {
    pub fn new(prefix: Option<Prefix>, atom: Option<Atom>) -> Self {
        Term {
            atom,
            prefix,
            factor: 1,
            exponent: 1,
            annotation: None,
        }
    }

    pub fn is_unity(&self) -> bool {
        self.factor == 1 && self.exponent == 1 && self.atom.is_none() && self.prefix.is_none()
    }

    pub fn scalar(&self) -> f64 {
        self.calculate_scalar(1.0)
    }

    pub fn magnitude(&self) -> f64 {
        self.calculate_magnitude(self.scalar())
    }

    pub fn calculate_scalar(&self, value: f64) -> f64 {
        debug!("calculate_scalar()");
        let e = self.exponent;

        debug!("calculate_scalar() calling atom.calculate_scalar");
        let atom_scalar = self.atom.map_or(1.0, |a| a.calculate_scalar(value));
        debug!("calculate_scalar() done with atom.calculate_scalar");

        // TODO: Interesting that this change causes tests to pass now.
        // let prefix_scalar = self.prefix.map_or(1.0, |p| p.magnitude());
        debug!("calculate_scalar() calling prefix.calculate_scalar");
        let prefix_scalar = self.prefix.map_or(1.0, |p| p.scalar());
        debug!("calculate_scalar() done with prefix.calculate_scalar");

        (atom_scalar * prefix_scalar * f64::from(self.factor)).powi(e)
    }

    pub fn calculate_magnitude(&self, value: f64) -> f64 {
        debug!("calculate_magnitude()");
        let e = self.exponent;

        let atom_magnitude = self.atom.map_or(1.0, |a| a.calculate_magnitude(value));
        let prefix_magnitude = self.prefix.map_or(1.0, |p| p.scalar());

        (atom_magnitude * prefix_magnitude * f64::from(self.factor)).powi(e)
    }
}

impl Composable for Term {
    fn composition(&self) -> Composition {
        match self.atom {
            Some(atom) => {
                let composition = atom.composition();

                if self.exponent == 1 {
                    return composition;
                }

                let mut new_composition = Composition::default();

                for (dim, exp) in composition {
                    let atom_exp = if exp == 1 { 0 } else { exp };
                    new_composition.insert(dim, atom_exp + self.exponent);
                }

                new_composition
            }
            None => Composition::default(),
        }
    }
}

impl Composable for Vec<Term> {
    fn composition(&self) -> Composition {
        let mut composition = Composition::default();

        for term in self {
            let term_composition = term.composition();

            for (term_dimension, term_exponent) in term_composition {
                composition.insert(term_dimension, term_exponent);
            }
        }

        composition
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", extract_term_string(self))
    }
}

fn extract_term_string(term: &Term) -> String {
    let mut term_string = String::new();

    if term.factor != 1 {
        term_string.push_str(&term.factor.to_string())
    };

    if let Some(prefix) = term.prefix {
        term_string.push_str(&prefix.to_string());
    }

    if let Some(atom) = term.atom {
        if term.exponent == 1 {
            term_string.push_str(&atom.to_string());
        } else {
            term_string.push_str(&format!("{}{}", atom, term.exponent));
        }
    }

    term_string
}

#[cfg(test)]
mod tests {
    use atom::Atom;
    use composable::Composable;
    use composition::Composition;
    use dimension::Dimension;
    use prefix::Prefix;
    use term::Term;

    macro_rules! validate_display {
        ($test_name:ident, $term:expr, $output:expr) => {
            #[test]
            fn $test_name() {
                let term = $term;
                assert_eq!(term.to_string().as_str(), $output);
            }
        };

        ($test_name:ident, $output:expr) => {
            #[test]
            fn $test_name() {
                let term = term!();
                assert_eq!(term.to_string().as_str(), $output);
            }
        };
    }

    macro_rules! validate_scalar {
        ($test_name:ident, $term:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let term = $term;
                assert_relative_eq!(term.scalar(), $expected_value);
            }
        };
    }

    macro_rules! validate_magnitude {
        ($test_name:ident, $term:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let term = $term;
                assert_relative_eq!(term.magnitude(), $expected_value);
            }
        };
    }

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

    // scalar tests
    validate_scalar!(validate_scalar_meter, term!(Meter), 1.0);
    validate_scalar!(validate_scalar_kilometer, term!(Kilo, Meter), 1000.0);
    validate_scalar!(
        validate_scalar_meter_eminus1,
        term!(Meter, exponent: -1),
        1.0
    );
    validate_scalar!(validate_scalar_meter_factor, term!(Meter, factor: 10), 10.0);
    validate_scalar!(
        validate_scalar_kilometer_factor,
        term!(Kilo, Meter, factor: 10),
        10_000.0
    );
    validate_scalar!(
        validate_scalar_kilometer_factor_exponent,
        term!(Kilo, Meter, exponent: -1, factor: 10),
        0.0001
    );
    validate_scalar!(validate_scalar_liter, term!(Liter), 0.001);
    validate_scalar!(
        validate_scalar_pi,
        term!(TheNumberPi),
        ::std::f64::consts::PI
    );
    validate_scalar!(
        validate_scalar_pi_factor,
        term!(TheNumberPi, factor: 10),
        ::std::f64::consts::PI * 10.0
    );
    validate_scalar!(validate_scalar_hectare, term!(Hecto, Are), 10_000.0);
    validate_scalar!(validate_scalar_week, term!(Week), 604_800.0);
    validate_scalar!(validate_scalar_kilogram, term!(Kilo, Gram), 1000.0);
    validate_scalar!(
        validate_scalar_fahrenheit,
        term!(DegreeFahrenheit),
        255.927_777_777_777_8
    );

    // magnitude tests
    validate_magnitude!(validate_magnitude_meter, term!(Meter), 1.0);
    validate_magnitude!(validate_magnitude_kilometer, term!(Kilo, Meter), 1000.0);
    validate_magnitude!(
        validate_magnitude_meter_eminus1,
        term!(Meter, exponent: -1),
        1.0
    );
    validate_magnitude!(
        validate_magnitude_meter_factor,
        term!(Meter, factor: 10),
        10.0
    );
    validate_magnitude!(
        validate_magnitude_kilometer_factor,
        term!(Kilo, Meter, factor: 10),
        10_000.0
    );
    validate_magnitude!(
        validate_magnitude_kilometer_factor_exponent,
        term!(Kilo, Meter, exponent: -1, factor: 10),
        0.000_1
    );
    validate_magnitude!(validate_magnitude_liter, term!(Liter), 1.0);
    validate_magnitude!(validate_magnitude_pi, term!(TheNumberPi), 1.0);
    validate_magnitude!(
        validate_magnitude_pi_factor,
        term!(TheNumberPi, factor: 10),
        10.0
    );
    validate_magnitude!(validate_magnitude_hectare, term!(Hecto, Are), 100.0);
    validate_magnitude!(validate_magnitude_week, term!(Week), 1.0);
    validate_magnitude!(validate_magnitude_kilogram, term!(Kilo, Gram), 1000.0);
    validate_magnitude!(
        validate_magnitude_fahrenheit,
        term!(DegreeFahrenheit),
        1.000_000_000_000_056_8
    );

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

    // Display tests
    validate_display!(validate_display_empty, "");
    validate_display!(validate_display_meter, term!(Meter), "m");
    validate_display!(
        validate_display_meter_exponent1,
        term!(Meter, exponent: -1),
        "m-1"
    );
    validate_display!(
        validate_display_meter_exponent_factor,
        term!(Meter, exponent: -1, factor: 5),
        "5m-1"
    );
    validate_display!(
        validate_display_meter_exponent2,
        term!(Meter, exponent: -2),
        "m-2"
    );
    validate_display!(
        validate_display_meter_factor,
        term!(Meter, factor: 10),
        "10m"
    );
    validate_display!(validate_display_kilometer, term!(Kilo, Meter), "km");
    validate_display!(
        validate_display_kilometer_factor,
        term!(Kilo, Meter, factor: 10),
        "10km"
    );
    validate_display!(
        validate_display_kilometer_factor_exponent,
        term!(Kilo, Meter, factor: 10, exponent: -1),
        "10km-1"
    );

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Term;
        use atom::Atom;
        use prefix::Prefix;
        use serde_json;

        #[test]
        fn validate_serialization_empty_term() {
            let term = term!();

            let expected_json = r#"{
                                    "atom": null,
                                    "prefix": null,
                                    "factor": 1,
                                    "exponent": 1,
                                    "annotation": null
                                   }"#.replace("\n", "")
                .replace(" ", "");

            let j = serde_json::to_string(&term).expect("Couldn't convert Term to JSON String");

            assert_eq!(expected_json.as_str(), j);
        }

        #[test]
        fn validate_serialization_full_term() {
            let mut term = term!(Kilo, Meter, factor: 123, exponent: -456);
            term.annotation = Some("stuff".to_string());

            let expected_json = r#"{
                                    "atom": "Meter",
                                    "prefix": "Kilo",
                                    "factor": 123,
                                    "exponent": -456,
                                    "annotation": "stuff"
                                   }"#.replace("\n", "")
                .replace(" ", "");

            let j = serde_json::to_string(&term).expect("Couldn't convert Term to JSON String");

            assert_eq!(expected_json.as_str(), j);
        }

        #[test]
        fn validate_deserialization_empty_term() {
            let json = r#"{
                            "atom": null,
                            "prefix": null,
                            "factor": 1,
                            "exponent": 1,
                            "annotation": null
                           }"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Term");

            let expected_term = term!();

            assert_eq!(expected_term, k);
        }

        #[test]
        fn validate_deserialization_full_term() {
            let json = r#"{
                            "atom": "Meter",
                            "prefix": "Kilo",
                            "factor": 123,
                            "exponent": -456,
                            "annotation": "stuff"
                           }"#;

            let k = serde_json::from_str(json).expect("Couldn't convert JSON String to Term");

            let mut expected_term = term!(Kilo, Meter, factor: 123, exponent: -456);
            expected_term.annotation = Some("stuff".to_string());

            assert_eq!(expected_term, k);
        }
    }
}
