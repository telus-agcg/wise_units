use super::Decomposable;
use parser::Term;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

type Exponent = i32;
type InnerCollection = BTreeMap<String, Exponent>;

pub struct Decomposer;

impl<'a> Decomposable<'a> for Decomposer {
    type Terms = &'a [Term];
    type Collection = InnerCollection;

    fn terms_to_collection(&self, terms: &[Term]) -> InnerCollection {
        build_set(terms)
    }

    fn numerator(&self, collection: &Self::Collection) -> Option<String> {
        let result = collection
            .iter()
            .filter_map(|(k, v)| extract_numerator(k, *v))
            .fold(None, |acc, num_string| super::build_string(acc, num_string));

        if result.is_none() {
            Some("1".to_string())
        } else {
            result
        }
    }

    fn denominator(&self, collection: &Self::Collection) -> Option<String> {
        collection
            .iter()
            .filter_map(|(k, v)| extract_denominator(k, *v))
            .fold(None, |acc, num_string| super::build_string(acc, num_string))
    }
}

fn build_set(terms: &[Term]) -> BTreeMap<String, Exponent> {
    let mut set: BTreeMap<String, Exponent> = BTreeMap::new();

    for term in terms {
        let mut key = String::new();

        if term.factor != 1 {
            key.push_str(&term.factor.to_string())
        };

        if let Some(prefix) = term.prefix {
            key.push_str(&prefix.to_string());
        }

        if let Some(atom) = term.atom {
            key.push_str(&atom.to_string());
        }

        if !key.is_empty() {
            match set.entry(key) {
                Entry::Vacant(entry) => {
                    entry.insert(term.exponent);
                }
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += term.exponent;
                }
            }
        }
    }

    set
}

fn extract_numerator(key: &str, value: i32) -> Option<String> {
    if !value.is_positive() {
        return None;
    }

    let new_string = if value == 1 {
        key.to_string()
    } else {
        format!("{}{}", key, value)
    };

    if new_string.is_empty() {
        None
    } else {
        Some(new_string)
    }
}

fn extract_denominator(key: &str, value: i32) -> Option<String> {
    if !value.is_negative() {
        return None;
    }

    let new_string = if value == -1 {
        key.to_string()
    } else {
        format!("{}{}", key, -value)
    };

    if new_string.is_empty() {
        None
    } else {
        Some(new_string)
    }
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
    validate_decompose!(validate_decompose_pri_m2_per_m, "m2/m", "m");
    validate_decompose!(validate_decompose_sec_m, "M", "m");
    validate_decompose!(validate_decompose_sec_km, "KM", "km");
    validate_decompose!(validate_decompose_pri_km_slash_pri_10m, "km/10m", "km/10m");
    validate_decompose!(validate_decompose_pri_km_slash_pri_s2, "km/s2", "km/s2");
    validate_decompose!(
        validate_decompose_pri_km_slash_pri_60s2,
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
        "[in_i]"
    );
}
