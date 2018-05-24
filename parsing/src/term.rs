use atom::Atom;
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
    pub fn new(atom: Option<Atom>, prefix: Option<Prefix>) -> Self {
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
    // use composition::Composition;
    // use dimension::Dimension;
    use prefix::Prefix;
    use term::Term;

    macro_rules! term {
        ($atom:ident, $prefix:ident) => {
            Term::new(Some(Atom::$atom), Some(Prefix::$prefix))
        };

        ($atom:ident) => {
            Term::new(Some(Atom::$atom), None)
        };
    }

    macro_rules! validate_display {
        (
            $test_name:ident,
            $term:expr,
            $output:expr,factor:
            $factor:expr,exponent:
            $exponent:expr
        ) => {
            #[test]
            fn $test_name() {
                let mut term = $term;
                term.factor = $factor;
                term.exponent = $exponent;
                assert_eq!(term.to_string().as_str(), $output);
            }
        };

        ($test_name:ident, $term:expr, $output:expr,factor: $factor:expr) => {
            #[test]
            fn $test_name() {
                let mut term = $term;
                term.factor = $factor;
                assert_eq!(term.to_string().as_str(), $output);
            }
        };

        ($test_name:ident, $term:expr, $output:expr,exponent: $exponent:expr) => {
            #[test]
            fn $test_name() {
                let mut term = $term;
                term.exponent = $exponent;
                assert_eq!(term.to_string().as_str(), $output);
            }
        };

        ($test_name:ident, $term:expr, $output:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!($term.to_string().as_str(), $output);
            }
        };

        ($test_name:ident, $output:expr) => {
            #[test]
            fn $test_name() {
                let term = Term::new(None, None);
                assert_eq!(term.to_string().as_str(), $output);
            }
        };
    }

    macro_rules! validate_scalar {
        (
            $test_name:ident,
            $method:ident,
            $atom_variant:ident,
            $prefix_variant:ident,
            $expected_value:expr,exponent:
            $exponent:expr,factor:
            $factor:expr
        ) => {
            #[test]
            fn $test_name() {
                let mut term = Term::new(Some(Atom::$atom_variant), Some(Prefix::$prefix_variant));
                term.factor = $factor;
                term.exponent = $exponent;
                assert_relative_eq!(term.$method(), $expected_value);
            }
        };

        (
            $test_name:ident,
            $method:ident,
            $atom_variant:ident,
            $prefix_variant:ident,
            $expected_value:expr,factor:
            $factor:expr
        ) => {
            #[test]
            fn $test_name() {
                let mut term = Term::new(Some(Atom::$atom_variant), Some(Prefix::$prefix_variant));
                term.factor = $factor;
                assert_relative_eq!(term.$method(), $expected_value);
            }
        };

        (
            $test_name:ident,
            $method:ident,
            $atom_variant:ident,
            $expected_value:expr,factor:
            $factor:expr
        ) => {
            #[test]
            fn $test_name() {
                let mut term = Term::new(Some(Atom::$atom_variant), None);
                term.factor = $factor;
                assert_relative_eq!(term.$method(), $expected_value);
            }
        };

        (
            $test_name:ident,
            $method:ident,
            $atom_variant:ident,
            $expected_value:expr,exponent:
            $exponent:expr
        ) => {
            #[test]
            fn $test_name() {
                let mut term = Term::new(Some(Atom::$atom_variant), None);
                term.exponent = $exponent;
                assert_relative_eq!(term.$method(), $expected_value);
            }
        };

        (
            $test_name:ident,
            $method:ident,
            $atom_variant:ident,
            $prefix_variant:ident,
            $expected_value:expr
        ) => {
            #[test]
            fn $test_name() {
                let term = Term::new(Some(Atom::$atom_variant), Some(Prefix::$prefix_variant));
                assert_relative_eq!(term.$method(), $expected_value);
            }
        };

        ($test_name:ident, $method:ident, $atom_variant:ident, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let term = Term::new(Some(Atom::$atom_variant), None);
                assert_relative_eq!(term.$method(), $expected_value);
            }
        };
    }

    validate_scalar!(validate_scalar_meter, scalar, Meter, 1.0);
    validate_scalar!(validate_scalar_kilometer, scalar, Meter, Kilo, 1000.0);
    validate_scalar!(validate_scalar_meter_eminus1, scalar, Meter, 1.0, exponent: -1);
    validate_scalar!(validate_scalar_meter_factor, scalar, Meter, 10.0, factor: 10);
    validate_scalar!(validate_scalar_kilometer_factor, scalar, Meter, Kilo, 10_000.0, factor: 10);
    validate_scalar!(validate_scalar_kilometer_factor_exponent, scalar, Meter, Kilo, 0.0001, exponent: -1, factor: 10);
    validate_scalar!(validate_scalar_liter, scalar, Liter, 0.001);
    validate_scalar!(
        validate_scalar_pi,
        scalar,
        TheNumberPi,
        ::std::f64::consts::PI
    );
    validate_scalar!(validate_scalar_pi_factor, scalar, TheNumberPi, ::std::f64::consts::PI * 10.0, factor: 10);
    validate_scalar!(validate_scalar_hectare, scalar, Are, Hecto, 10_000.0);
    validate_scalar!(validate_scalar_week, scalar, Week, 604_800.0);
    validate_scalar!(validate_scalar_kilogram, scalar, Gram, Kilo, 1000.0);
    validate_scalar!(
        validate_scalar_fahrenheit,
        scalar,
        DegreeFahrenheit,
        255.927_777_777_777_8
    );

    validate_scalar!(validate_magnitude_meter, magnitude, Meter, 1.0);
    validate_scalar!(validate_magnitude_kilometer, magnitude, Meter, Kilo, 1000.0);
    validate_scalar!(validate_magnitude_meter_eminus1, magnitude, Meter, 1.0, exponent: -1);
    validate_scalar!(validate_magnitude_meter_factor, magnitude, Meter, 10.0, factor: 10);
    validate_scalar!(validate_magnitude_kilometer_factor, magnitude, Meter, Kilo, 10_000.0, factor: 10);
    validate_scalar!(validate_magnitude_kilometer_factor_exponent, magnitude, Meter, Kilo, 0.000_1, exponent: -1, factor: 10);
    validate_scalar!(validate_magnitude_liter, magnitude, Liter, 1.0);
    validate_scalar!(validate_magnitude_pi, magnitude, TheNumberPi, 1.0);
    validate_scalar!(validate_magnitude_pi_factor, magnitude, TheNumberPi, 10.0, factor: 10);
    validate_scalar!(validate_magnitude_hectare, magnitude, Are, Hecto, 100.0);
    validate_scalar!(validate_magnitude_week, magnitude, Week, 1.0);
    validate_scalar!(validate_magnitude_kilogram, magnitude, Gram, Kilo, 1000.0);
    validate_scalar!(
        validate_magnitude_fahrenheit,
        magnitude,
        DegreeFahrenheit,
        1.000_000_000_000_056_8
    );

    // #[test]
    // fn validate_composition() {
    //     let term = Term::new(None, None);
    //     assert_eq!(term.composition(), None);

    //     let term = Term::new(Some(Atom::Meter), None);
    //     let composition = Composition::new(Dimension::Length, 1);
    //     assert_eq!(term.composition().unwrap(), composition);

    //     let term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
    //     let composition = Composition::new(Dimension::Length, 1);
    //     assert_eq!(term.composition().unwrap(), composition);

    //     let mut term = Term::new(Some(Atom::Meter), None);
    //     term.exponent = -1;
    //     let composition = Composition::new(Dimension::Length, -1);
    //     assert_eq!(term.composition().unwrap(), composition);

    //     let mut term = Term::new(Some(Atom::Meter), None);
    //     term.exponent = -2;
    //     let composition = Composition::new(Dimension::Length, -2);
    //     assert_eq!(term.composition().unwrap(), composition);

    //     let mut term = Term::new(Some(Atom::Meter), None);
    //     term.factor = 10;
    //     let composition = Composition::new(Dimension::Length, 1);
    //     assert_eq!(term.composition().unwrap(), composition);

    //     let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
    //     term.factor = 10;
    //     let composition = Composition::new(Dimension::Length, 1);
    //     assert_eq!(term.composition().unwrap(), composition);

    //     let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
    //     term.factor = 10;
    //     term.exponent = -1;
    //     let composition = Composition::new(Dimension::Length, -1);
    //     assert_eq!(term.composition().unwrap(), composition);
    // }

    validate_display!(validate_display_empty, "");
    validate_display!(validate_display_meter, term!(Meter), "m");
    validate_display!(validate_display_meter_exponent1, term!(Meter), "m-1", exponent: -1);
    validate_display!(validate_display_meter_exponent2, term!(Meter), "m-2", exponent: -2);
    validate_display!(validate_display_meter_factor, term!(Meter), "10m", factor: 10);
    validate_display!(validate_display_kilometer, term!(Meter, Kilo), "km");
    validate_display!(validate_display_kilometer_factor, term!(Meter, Kilo), "10km", factor: 10);
    validate_display!(validate_display_kilometer_factor_exponent, term!(Meter, Kilo), "10km-1", factor: 10, exponent: -1);

    #[cfg(feature = "with_serde")]
    mod with_serde {
        use super::super::Term;
        use atom::Atom;
        use prefix::Prefix;
        use serde_json;

        #[test]
        fn validate_serialization_empty_term() {
            let term = Term::new(None, None);

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
            let mut term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
            term.factor = 123;
            term.exponent = -456;
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

            let expected_term = Term::new(None, None);

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

            let mut expected_term = Term::new(Some(Atom::Meter), Some(Prefix::Kilo));
            expected_term.factor = 123;
            expected_term.exponent = -456;
            expected_term.annotation = Some("stuff".to_string());

            assert_eq!(expected_term, k);
        }
    }
}
