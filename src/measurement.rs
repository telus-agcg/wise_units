use parser::parse_MainTerm;
use parser_terms::Term;

#[derive(Clone, Debug, PartialEq)]
pub struct Measurement<'a> {
    value: f64,
    term: Term<'a>,
}

#[derive(Debug)]
pub enum ConversionError {
    Incompatible
}

impl<'a> Measurement<'a> {
    // TODO: wrong lifetime for expression here
    pub fn new(value: f64, expression: &'a str) -> Self {
        // TODO: unwrap
        let main_term = parse_MainTerm(expression).unwrap();

        Measurement {
            value: value,
            term: main_term
        }
    }

    pub fn convert_to(&self, expression: &'a str) -> Result<Measurement<'a>, ConversionError> {
        let other_term = parse_MainTerm(expression).unwrap();
        let my_term = &self.term;

        if !my_term.is_compatible_with(&other_term) {
            return Err(ConversionError::Incompatible)
        }

        let new_measurement = Measurement {
            value: 1.0,
            term: other_term
        };

        Ok(new_measurement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atom::ATOMS;
    use parser_terms::{Annotatable, Component, SimpleUnit, Term};

    #[test]
    fn validate_new() {
        let m = Measurement::new(1.0, "m");
        assert_eq!(m.value, 1.0);
        assert_eq!(
            m.term,
            Term::Basic(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(ATOMS[0].clone())
                    )
                )
            )
        );
    }

    #[test]
    fn validate_convert_to() {
        let m = Measurement::new(1.0, "m");
        let other = m.convert_to("m").unwrap();
        assert_eq!(other, m);

        let mut other = m.convert_to("m").unwrap();
        other.value = 2.0;
        assert_ne!(other, m);
    }
}
