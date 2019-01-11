use crate::{Term, Unit};
use std::fmt;

//-----------------------------------------------------------------------------
// impl Display
//-----------------------------------------------------------------------------
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", decompose(self))
    }
}

/// Turns `terms` into a `String` for display.
///
fn decompose(unit: &Unit) -> String {
    let numerator = string_from_collection(&unit.terms, extract_numerator);
    let denominator = string_from_collection(&unit.terms, extract_denominator);

    format_output(numerator, denominator)
}

fn format_output(numerator: Option<String>, denominator: Option<String>) -> String {
    match (numerator, denominator) {
        (Some(n), Some(d)) => [n, d].join("/"),
        (Some(n), None) => n,
        (None, Some(d)) => format!("/{}", d),
        (None, None) => "1".to_string(),
    }
}

fn string_from_collection<F>(terms: &[Term], func: F) -> Option<String>
where
    F: Fn(&Term) -> Option<String>,
{
    terms
        .iter()
        .filter_map(|term| func(term))
        .fold(None, |acc, term_string| build_string(acc, term_string))
}

/// Specifically for use with `filter_map()`, this returns `None` if the `Term` is not positive.
///
fn extract_numerator(term: &Term) -> Option<String> {
    if !term.has_value() || !term.exponent_is_positive() {
        return None;
    }

    Some(term.to_string())
}

/// Specifically for use with `filter_map()`, this returns `None` if the `Term` is not negative.
///
fn extract_denominator(term: &Term) -> Option<String> {
    if !term.has_value() || !term.exponent_is_negative() {
        return None;
    }

    let mut term_string = String::new();

    term.factor_and_is_not_one(|factor| term_string.push_str(&factor.to_string()));

    extract_denominator_atom(term, &mut term_string);

    if let Some(ref annotation) = term.annotation {
        term_string.push_str(&format!("{{{}}}", annotation));
    }

    Some(term_string)
}

fn extract_denominator_atom(term: &Term, term_string: &mut String) {
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
        } else {
            term_string.push_str(&atom.to_string());
        }
    }
}

fn build_string(acc: Option<String>, term_string: String) -> Option<String> {
    match acc {
        Some(mut a) => {
            // let new_string = format!(".{}", term_string);

            // a.push_str(&new_string);
            a.push_str(".");
            a.push_str(&term_string);

            Some(a)
        }
        None => Some(term_string),
    }
}

#[cfg(test)]
mod tests {
    use crate::unit::Unit;
    use std::str::FromStr;

    #[test]
    fn validate_display() {
        let unit = Unit::from_str("1").unwrap();
        assert_eq!(unit.to_string().as_str(), "1");

        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.to_string().as_str(), "m");

        let unit = Unit::from_str("M").unwrap();
        assert_eq!(unit.to_string().as_str(), "m");

        let unit = Unit::from_str("{stuff}").unwrap();
        assert_eq!(unit.to_string().as_str(), "{stuff}");

        let unit = Unit::from_str("m{stuff}").unwrap();
        assert_eq!(unit.to_string().as_str(), "m{stuff}");

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.to_string().as_str(), "km/10m");

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.to_string().as_str(), "/m");

        let unit = Unit::from_str("m-1{stuff}").unwrap();
        assert_eq!(unit.to_string().as_str(), "/m{stuff}");

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.to_string().as_str(), "10m");

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.to_string().as_str(), "10km");

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.to_string().as_str(), "/10km");

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.to_string().as_str(), "/km.m2");

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.to_string().as_str(), "km/m2.cm");

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.to_string().as_str(), "/km.m2.cm");

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.to_string().as_str(), "m/s2");

        let unit = Unit::from_str("km3/nm2").unwrap();
        assert_eq!(unit.to_string().as_str(), "km3/nm2");

        let unit = Unit::from_str("km3{foo}/nm2{bar}").unwrap();
        assert_eq!(unit.to_string().as_str(), "km3{foo}/nm2{bar}");
    }
}
