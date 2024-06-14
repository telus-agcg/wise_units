use std::{borrow::Cow, fmt};

use crate::{Term, Unit};

//-----------------------------------------------------------------------------
// impl Display
//-----------------------------------------------------------------------------
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (numerators, denominators) = decompose(&self.terms);

        write!(f, "{}", recompose(&numerators, &denominators))
    }
}

/// Turns `terms` into two groups of strings: one for numerator terms, one for denominator terms.
/// These just need to be properly jointed by dots and slashes.
///
fn decompose(terms: &[Term]) -> (Vec<Cow<'_, str>>, Vec<String>) {
    let mut numerators = Vec::new();
    let mut denominators = Vec::new();

    for term in terms {
        match term.exponent() {
            Some(exponent) if exponent.is_positive() => {
                numerators.push(term.as_cow_str());
            }
            Some(exponent) => {
                let mut positive_exponent_term = term.clone();

                denominators.push(
                    positive_exponent_term
                        .set_exponent(exponent.abs())
                        .to_string(),
                );
            }
            None => numerators.push(term.as_cow_str()),
        }
    }

    (numerators, denominators)
}

fn recompose<'a>(numerators: &[Cow<'a, str>], denominators: &[String]) -> Cow<'a, str> {
    match (numerators.len(), denominators.len()) {
        (0, 0) => Cow::Borrowed("1"),
        (0, _) => Cow::Owned(format!("/{}", denominators.join("."))),
        (1, 0) => numerators[0].clone(),
        (_, 0) => Cow::Owned(numerators.join(".")),
        (_, _) => Cow::Owned(format!(
            "{}/{}",
            numerators.join("."),
            denominators.join("."),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::unit::Unit;
    use std::str::FromStr;

    #[test]
    fn validate_display() {
        let unit = Unit::from_str("1").unwrap();
        assert_eq!(unit.to_string(), "1");

        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.to_string(), "m");

        let unit = Unit::from_str("M").unwrap();
        assert_eq!(unit.to_string(), "m");

        let unit = Unit::from_str("{stuff}").unwrap();
        assert_eq!(unit.to_string(), "{stuff}");

        let unit = Unit::from_str("/{stuff}").unwrap();
        assert_eq!(unit.to_string(), "/{stuff}");

        let unit = Unit::from_str("m{stuff}").unwrap();
        assert_eq!(unit.to_string(), "m{stuff}");

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.to_string(), "km/10m");

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.to_string(), "/m");

        let unit = Unit::from_str("m-1{stuff}").unwrap();
        assert_eq!(unit.to_string(), "/m{stuff}");

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.to_string(), "10m");

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.to_string(), "10km");

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.to_string(), "/10km");

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.to_string(), "/km.m2");

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.to_string(), "km/m2.cm");

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.to_string(), "/km.m2.cm");

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.to_string(), "m/s2");

        let unit = Unit::from_str("km3/nm2").unwrap();
        assert_eq!(unit.to_string(), "km3/nm2");

        let unit = Unit::from_str("km3{foo}/nm2{bar}").unwrap();
        assert_eq!(unit.to_string(), "km3{foo}/nm2{bar}");

        let unit = Unit::from_str("{foo}/{bar}").unwrap();
        assert_eq!(unit.to_string(), "{foo}/{bar}");
    }
}
