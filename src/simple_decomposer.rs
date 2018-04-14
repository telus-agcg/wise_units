use decomposable::Decomposable;
use term::Term;

pub struct SimpleDecomposer<'a>(&'a [Term]);

impl<'a> SimpleDecomposer<'a> {
    pub fn new(terms: &'a [Term]) -> Self {
        SimpleDecomposer(terms)
    }
}

impl<'a> Decomposable for SimpleDecomposer<'a> {
    fn numerator(&self) -> String {
        let result = self.0
            .iter()
            .filter(|&term| term.exponent.is_positive())
            .map(|term| term.to_string())
            .filter(|term_string| !term_string.is_empty())
            .fold(String::new(), |mut acc, term_string| {
                let new_string = if acc.is_empty() {
                    format!("{}", term_string)
                } else {
                    format!(".{}", term_string)
                };
                acc.push_str(&new_string);

                acc
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
            .filter(|&term| term.exponent.is_negative())
            .map(|term| {
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

                term_string
            })
            .filter(|term_string| !term_string.is_empty())
            .fold(String::new(), |mut acc, term_string| {
                let new_string = if acc.is_empty() {
                    format!("{}", term_string)
                } else {
                    format!(".{}", term_string)
                };
                acc.push_str(&new_string);

                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleDecomposer;
    use decomposable::Decomposable;
    use std::str::FromStr;
    use unit::Unit;

    #[test]
    fn validate_expression() {
        let unit = Unit::from_str("m").unwrap();
        let decomposer = SimpleDecomposer::new(&unit.terms);
        assert_eq!(decomposer.expression(), "m");

        let unit = Unit::from_str("km").unwrap();
        let decomposer = SimpleDecomposer::new(&unit.terms);
        assert_eq!(decomposer.expression(), "km");

        let unit = Unit::from_str("km/s2").unwrap();
        let decomposer = SimpleDecomposer::new(&unit.terms);
        assert_eq!(decomposer.expression(), "km/s2");

        let unit = Unit::from_str("km/60s2").unwrap();
        let decomposer = SimpleDecomposer::new(&unit.terms);
        assert_eq!(decomposer.expression(), "km/60s2");

        let unit = Unit::from_str("100KM/60s2").unwrap();
        let decomposer = SimpleDecomposer::new(&unit.terms);
        assert_eq!(decomposer.expression(), "100km/60s2");

        let unit = Unit::from_str("[acr_us].[IN_I]/[acr_us]").unwrap();
        let decomposer = SimpleDecomposer::new(&unit.terms);
        assert_eq!(decomposer.expression(), "[acr_us].[in_i]/[acr_us]");

        let unit = Unit::from_str("[acr_us].[IN_I]/[acr_us]2").unwrap();
        let decomposer = SimpleDecomposer::new(&unit.terms);
        assert_eq!(decomposer.expression(), "[acr_us].[in_i]/[acr_us]2");
    }
}
