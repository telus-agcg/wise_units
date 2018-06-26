use super::Decomposable;
use parser::Term;

pub struct Decomposer<'a>(&'a [Term]);

impl<'a> Decomposer<'a> {
    pub fn new(terms: &'a [Term]) -> Self {
        Decomposer(terms)
    }
}

impl<'a> Decomposable for Decomposer<'a> {
    fn numerator(&self) -> String {
        let result = self.0
            .iter()
            .filter_map(|term| extract_numerator(term))
            .fold(String::new(), |acc, term_string| {
                super::build_string(acc, term_string)
            });

        if result.is_empty() {
            "1".to_string()
        } else {
            result
        }
    }

    fn denominator(&self) -> String {
        self.0
            .iter()
            .filter_map(|term| extract_denominator(term))
            .fold(String::new(), |acc, term_string| {
                super::build_string(acc, term_string)
            })
    }
}

fn extract_numerator(term: &Term) -> Option<String> {
    if !term.exponent.is_positive() {
        return None;
    }

    let s = term.to_string();

    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn extract_denominator(term: &Term) -> Option<String> {
    if !term.exponent.is_negative() {
        return None;
    }

    let mut term_string = String::new();

    if term.factor != 1 {
        term_string.push_str(&term.factor.to_string())
    };

    if let Some(prefix) = term.prefix {
        term_string.push_str(&prefix.to_string());
    }

    if let Some(atom) = term.atom {
        if term.exponent.abs() == 1 {
            term_string.push_str(&atom.to_string());
        } else {
            let v = -term.exponent;
            term_string.push_str(&format!("{}{}", atom, v));
        }
    }

    if term_string.is_empty() {
        None
    } else {
        Some(term_string)
    }
}

#[cfg(test)]
mod tests {
    macro_rules! validate_expression {
        ($test_name:ident, $input_string:expr, $expected_expression:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($input_string).unwrap();
                let decomposer = Decomposer::new(&unit.terms);
                assert_eq!(decomposer.expression(), $expected_expression);
            }
        };
    }

    use super::super::Decomposable;
    use super::Decomposer;
    use std::str::FromStr;
    use unit::Unit;

    validate_expression!(validate_expression_pri_m, "m", "m");
    validate_expression!(validate_expression_pri_m2_per_m, "m2/m", "m2/m");
    validate_expression!(validate_expression_sec_m, "M", "m");
    validate_expression!(validate_expression_pri_km, "km", "km");
    validate_expression!(validate_expression_sec_km, "KM", "km");
    validate_expression!(validate_expression_pri_km_slash_pri_s2, "km/s2", "km/s2");
    validate_expression!(
        validate_expression_pri_im_slash_pri_60s2,
        "km/60s2",
        "km/60s2"
    );
    validate_expression!(
        validate_expression_sec_100km_slash_pri_60s,
        "100KM/60s2",
        "100km/60s2"
    );
    validate_expression!(
        validate_expression_pri_acr_us_sec_in_i_slash_pri_acr_us,
        "[acr_us].[IN_I]/[acr_us]",
        "[acr_us].[in_i]/[acr_us]"
    );
    validate_expression!(
        validate_expression_pri_acr_us_sec_in_i_slash_pri_acr_us2,
        "[acr_us].[IN_I]/[acr_us]2",
        "[acr_us].[in_i]/[acr_us]2"
    );
}