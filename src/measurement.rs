use parser::parse_MainTerm;
use parser_terms::Term;

pub struct Measurement<'a> {
    pub value: f64,
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
            value: self.converted_value(&other_term),
            term: other_term
        };

        Ok(new_measurement)
    }

    pub fn is_special(&self) -> bool {
        let ref t = self.term;

        t.is_special()
    }

    pub fn scalar(&self, magnitude: f64) -> f64 {
        if self.is_special() {
            self.term.scalar(magnitude)
        } else {
            self.value * self.term.scalar_default()
        }
    }

    pub fn scalar_default(&self) -> f64 {
        self.scalar(self.value)
    }

    fn converted_value(&self, other_term: &Term) -> f64 {
        if other_term.is_special() {
            other_term.magnitude(self.scalar_default())
        } else {
            self.scalar_default() / other_term.scalar_default()
        }
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
