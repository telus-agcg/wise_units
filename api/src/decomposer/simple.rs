use super::Decomposable;
use parser::Term;

pub struct Decomposer;

impl<'a> Decomposable<'a> for Decomposer {
    type Terms = &'a [Term];
    type Collection = &'a [Term];

    fn terms_to_collection(&self, terms: Self::Terms) -> Self::Collection {
        terms
    }

    fn numerator(&self, terms: &Self::Collection) -> Option<String> {
        let result = string_from_collection(terms, extract_numerator);

        if result.is_none() {
            Some("1".to_string())
        } else {
            result
        }
    }

    fn denominator(&self, terms: &Self::Collection) -> Option<String> {
        string_from_collection(terms, extract_denominator)
    }
}

fn string_from_collection<F>(terms: &[Term], func: F) -> Option<String>
where
    F: Fn(&Term) -> Option<String>,
{
    terms
        .iter()
        .filter_map(|term| func(term))
        .fold(None, |acc, term_string| {
            super::build_string(acc, term_string)
        })
}

/// Specifically for use with `filter_map()`, this returns `None` if the `Term` is not positive.
///
fn extract_numerator(term: &Term) -> Option<String> {
    if !term.exponent_is_positive() {
        return None;
    }

    Some(term.to_string())
}

/// Specifically for use with `filter_map()`, this returns `None` if the `Term` is not negative.
///
fn extract_denominator(term: &Term) -> Option<String> {
    if !term.exponent_is_negative() {
        return None;
    }

    let mut term_string = String::new();

    term.factor_and_is_not_one(|factor| term_string.push_str(&factor.to_string()));

    if let Some(atom) = term.atom {
        if let Some(prefix) = term.prefix {
            term_string.push_str(&prefix.to_string());
        }

        if let Some(exponent) = term.exponent {
            let ex_abs = exponent.abs();

            if ex_abs == 1 {
                term_string.push_str(&atom.to_string());
            } else {
                term_string.push_str(&format!("{}{}", atom, ex_abs));
            }
        }
    }

    term_string.shrink_to_fit();

    Some(term_string)
}

#[cfg(test)]
mod tests {
    macro_rules! validate_decompose {
        ($test_name:ident, $input_string:expr, $expected_expression:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($input_string).unwrap();
                let decomposer = Decomposer;
                assert_eq!(decomposer.decompose(&unit.terms), $expected_expression);
            }
        };
    }

    use super::super::Decomposable;
    use super::Decomposer;
    use std::str::FromStr;
    use unit::Unit;

    validate_decompose!(validate_decompose_pri_m, "m", "m");
    validate_decompose!(validate_decompose_pri_m2_per_m, "m2/m", "m2/m");
    validate_decompose!(validate_decompose_sec_m, "M", "m");
    validate_decompose!(validate_decompose_pri_km, "km", "km");
    validate_decompose!(validate_decompose_sec_km, "KM", "km");
    validate_decompose!(validate_decompose_pri_km_slash_pri_s2, "km/s2", "km/s2");
    validate_decompose!(
        validate_decompose_pri_im_slash_pri_60s2,
        "km/60s2",
        "km/60s2"
    );
    validate_decompose!(
        validate_decompose_sec_100km_slash_pri_60s,
        "100KM/60s2",
        "100km/60s2"
    );
    validate_decompose!(
        validate_decompose_pri_acr_us_sec_in_i_slash_pri_acr_us,
        "[acr_us].[IN_I]/[acr_us]",
        "[acr_us].[in_i]/[acr_us]"
    );
    validate_decompose!(
        validate_decompose_pri_acr_us_sec_in_i_slash_pri_acr_us2,
        "[acr_us].[IN_I]/[acr_us]2",
        "[acr_us].[in_i]/[acr_us]2"
    );
}
