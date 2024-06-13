use std::fmt;

use super::Term;

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_cow_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        term::{FactorAnnotation, UNITY},
        Annotation, Term,
    };

    macro_rules! validate_display {
        ($test_name:ident, $term:expr, $output:expr) => {
            #[test]
            fn $test_name() {
                let term = $term;
                assert_eq!(term.to_string(), $output);
            }
        };

        ($test_name:ident, $output:expr) => {
            #[test]
            fn $test_name() {
                let term = term!();
                assert_eq!(term.to_string(), $output);
            }
        };
    }

    validate_display!(validate_display_default, "1");
    validate_display!(
        validate_display_empty_annotation,
        Term::Annotation(Annotation::from("seed")),
        "{seed}"
    );
    validate_display!(validate_display_unity, UNITY, "1");
    validate_display!(
        validate_display_unity_annotation,
        {
            Term::FactorAnnotation(FactorAnnotation {
                factor: 1,
                annotation: Annotation::from("seed"),
            })
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
