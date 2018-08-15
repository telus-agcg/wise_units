use parser::ucum_symbol::UcumSymbol;
use parser::{Atom, Composable, Composition, Prefix};
use reducible::Reducible;
use std::fmt;
use ucum_unit::UcumUnit;

/// A Term makes up an Atom (at its core) along with any Atom modifiers
/// (anything that can change its scalar). It is, however, possible to have an
/// Atom-less Term, which would simple be a Factor (with or without an
/// annotation) (ex. the 10 in "10" or "10/m" would be an Atom-less Term).
///
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Term {
    pub atom: Option<Atom>,
    pub prefix: Option<Prefix>,
    pub factor: Option<u32>,
    pub exponent: Option<i32>,
    pub annotation: Option<String>,
}

impl Term {
    pub fn new(prefix: Option<Prefix>, atom: Option<Atom>) -> Self {
        Self {
            atom,
            prefix,
            factor: None,
            exponent: None,
            annotation: None,
        }
    }

    /// A `Term` is a unity `Term` if represents "1", which technically means
    /// here:
    ///
    /// * its `factor` is 1
    /// * its `exponent` is 1
    /// * it has no `Atom`
    /// * it has no `Prefix`
    ///
    pub fn is_unity(&self) -> bool {
        self.factor.is_none()
            && self.exponent.is_none()
            && self.atom.is_none()
            && self.prefix.is_none()
    }

    /// If `self` has an `exponent`, it negates that value; if not, it sets it to `-1` (since
    /// `None` is analogous to an exponent of 1).
    ///
    pub fn invert_exponent(&mut self) {
        self.exponent = match self.exponent {
            Some(exponent) => Some(-exponent),
            // None is analogous to an exponent of 1.
            None => Some(-1),
        };
    }

    /// If `self` has an `exponent`, it checks if its value is positive; if not, it returns `true`
    /// (since `None` is analogous to an exponent of 1).
    ///
    pub fn exponent_is_positive(&self) -> bool {
        match self.exponent {
            Some(e) => e.is_positive(),
            // None is analogous to an exponent of 1.
            None => true,
        }
    }

    /// If `self` has an `exponent`, it checks if its value is negative; if not, it returns `false`
    /// (since `None` is analogous to an exponent of 1).
    ///
    pub fn exponent_is_negative(&self) -> bool {
        match self.exponent {
            Some(e) => e.is_negative(),
            // None is analogous to an exponent of 1.
            None => false,
        }
    }

    pub fn factor_and_is_not_one<F: FnOnce(u32)>(&self, f: F) {
        if let Some(factor) = self.factor {
            if factor != 1 {
                f(factor)
            }
        }
    }

    pub fn factor_as_u32(&self) -> u32 {
        match self.factor {
            Some(f) => f,
            None => 1,
        }
    }
}

impl UcumUnit for Term {
    fn is_special(&self) -> bool {
        match self.atom {
            Some(ref a) => a.is_special(),
            None => false,
        }
    }

    /// The UCUM defines "arbitrary units" using three points. First:
    ///
    /// > units whose meaning entirely depends on the measurement procedure
    /// (assay). These units > have no general meaning in relation with any
    /// other unit in the SI.
    ///
    /// Second:
    ///
    /// > An arbitrary unit has no further definition in the semantic framework
    /// of The Unified Code > for Units of Measure.
    ///
    /// Third:
    ///
    /// > Arbitrary units are not “of any specific dimension” and are not
    /// “commensurable with” any > other unit.
    ///
    fn is_arbitrary(&self) -> bool {
        match self.atom {
            Some(ref a) => a.is_arbitrary(),
            None => false,
        }
    }

    /// A `Term` is metric if it has some `Atom` that is metric.
    ///
    fn is_metric(&self) -> bool {
        match self.atom {
            Some(ref a) => a.is_metric(),
            None => false,
        }
    }

    fn scalar(&self) -> f64 {
        self.reduce_value(1.0)
    }

    fn magnitude(&self) -> f64 {
        self.calculate_magnitude(self.scalar())
    }
}

//-----------------------------------------------------------------------------
// impl Reducible
//-----------------------------------------------------------------------------
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

//-----------------------------------------------------------------------------
// impl Composable
//-----------------------------------------------------------------------------
impl Composable for Term {
    /// Combines the `Composition` from the `Term`'s `Atom` with its own `exponent` to build a
    /// `Composition`. If the `Term` has no `Atom`, it has no dimension, thus will have an empty
    /// `Composition`.
    ///
    /// TODO: https://agrian.atlassian.net/browse/DEV-971
    ///
    fn composition(&self) -> Composition {
        match self.atom {
            Some(atom) => {
                let atom_composition = atom.composition();

                match self.exponent {
                    Some(term_exponent) => atom_composition * term_exponent,
                    None => atom_composition,
                }
            }
            // If there's no Atom in the Term, there's no dimension--even if there's an exponent on
            // the Term.
            None => Composition::default(),
        }
    }
}

impl<'a> Composable for &'a [Term] {
    fn composition(&self) -> Composition {
        self.iter()
            .fold(Composition::default(), |acc, term| acc * term.composition())
    }
}

impl ::std::default::Default for Term {
    fn default() -> Self {
        Self {
            prefix: None,
            atom: None,
            factor: None,
            exponent: None,
            annotation: None,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", extract_term_string(self))
    }
}

fn extract_term_string(term: &Term) -> String {
    let mut term_string = String::new();

    if let Some(factor) = term.factor {
        if factor != 1 {
            term_string.push_str(&factor.to_string())
        }
    };

    if let Some(atom) = term.atom {
        if let Some(prefix) = term.prefix {
            term_string.push_str(&prefix.to_string());
        }

        match term.exponent {
            Some(exponent) => {
                if exponent == 1 {
                    term_string.push_str(&atom.to_string());
                } else {
                    term_string.push_str(&format!("{}{}", atom, exponent));
                }
            }
            None => term_string.push_str(&atom.to_string()),
        }
    }

    term_string
}

#[cfg(test)]
mod tests {
    use super::super::{Atom, Composable, Composition, Dimension, Prefix, Term};
    use reducible::Reducible;

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
    validate_reduce_value!(validate_reduce_value_meter, term!(Meter), 1.0);
    validate_reduce_value!(validate_reduce_value_kilometer, term!(Kilo, Meter), 1000.0);
    validate_reduce_value!(
        validate_reduce_value_meter_eminus1,
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
        validate_calculate_magnitude_meter_eminus1,
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
        use parser::{Atom, Prefix};
        use serde_json;

        #[test]
        fn validate_serialization_empty_term() {
            let term = term!();

            let expected_json = r#"{
                                    "atom": null,
                                    "prefix": null,
                                    "factor": null,
                                    "exponent": null,
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
                            "factor": null,
                            "exponent": null,
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
