use std::fmt;

use super::{Factor, Term};

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", extract_term_string(self))
    }
}

fn extract_term_string(term: &Term) -> String {
    if term.is_unity() && term.annotation.is_none() {
        return String::from("1");
    };

    let mut term_string = String::new();
    extract_term_string_factor(&mut term_string, term.factor);
    extract_term_string_atom(&mut term_string, term);

    if let Some(ref annotation) = term.annotation {
        term_string.push_str(&format!("{{{annotation}}}"));
    }

    term_string
}

fn extract_term_string_factor(term_string: &mut String, term_factor: Option<Factor>) {
    if let Some(factor) = term_factor {
        if factor != 1 {
            term_string.push_str(&factor.to_string());
        }
    }
}

fn extract_term_string_atom(term_string: &mut String, term: &Term) {
    if let Some(atom) = term.atom {
        if let Some(prefix) = term.prefix {
            term_string.push_str(&prefix.to_string());
        }

        match term.exponent {
            Some(exponent) => {
                if exponent == 1 {
                    term_string.push_str(&atom.to_string());
                } else {
                    term_string.push_str(&format!("{atom}{exponent}"));
                }
            }
            None => term_string.push_str(&atom.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{term::UNITY, Term};

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

    validate_display!(validate_display_empty, "");
    validate_display!(
        validate_display_empty_annotation,
        {
            let mut t = Term::new(None, None);
            t.annotation = Some("seed".to_string());
            t
        },
        "{seed}"
    );
    validate_display!(validate_display_unity, UNITY, "1");
    validate_display!(
        validate_display_unity_annotation,
        {
            let mut t = UNITY;
            t.annotation = Some("seed".to_string());
            t
        },
        "{seed}"
    );
    validate_display!(validate_display_meter, term!(Meter), "m");
    validate_display!(
        validate_display_meter_exponent1,
        term!(Meter, exponent: -1),
        "m-1"
    );
    validate_display!(
        validate_display_meter_exponent1_annotation,
        term!(Meter, exponent: -1, annotation: "seed".to_string()),
        "m-1{seed}"
    );
    validate_display!(
        validate_display_meter_exponent_factor,
        term!(Meter, exponent: -1, factor: 5),
        "5m-1"
    );
    validate_display!(
        validate_display_meter_exponent_factor_annotation,
        term!(Meter, exponent: -1, factor: 5, annotation: "seed".to_string()),
        "5m-1{seed}"
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
    validate_display!(
        validate_display_kilometer_factor_exponent_annotation,
        term!(Kilo, Meter, factor: 10, exponent: -1, annotation: "seed".to_string()),
        "10km-1{seed}"
    );
}
