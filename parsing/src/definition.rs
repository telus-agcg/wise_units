use error::Error;
use term::Term;
use ucum_symbol::UcumSymbol;

#[derive(Debug)]
pub struct Definition {
    pub value: f64,
    pub terms: Vec<Term>,
}

impl Definition {
    pub fn new(value: f64, expression: &str) -> Result<Self, Error> {
        let terms = super::parse(expression)?;

        Ok(Definition { value, terms })
    }

    pub fn is_special(&self) -> bool {
        self.terms.iter().any(|term| match term.atom {
            Some(ref atom) => atom.is_special(),
            None => false,
        })
    }

    pub fn scalar(&self) -> f64 {
        if self.is_special() {
            let magnitude = self.value;
            self.calculate_scalar(magnitude)
        } else {
            self.value * self.calculate_scalar(1.0)
        }
    }

    pub fn magnitude(&self) -> f64 {
        if self.is_special() {
            let scalar = self.scalar();
            self.calculate_magnitude(scalar)
        } else {
            self.value * self.calculate_magnitude(1.0)
        }
    }

    pub fn calculate_scalar(&self, other_value: f64) -> f64 {
        if self.is_unity() {
            self.value
        } else {
            let terms_scalar = self.terms
                .iter()
                .fold(1.0, |acc, term| acc * term.calculate_scalar(other_value));

            self.value * terms_scalar
        }
    }

    pub fn calculate_magnitude(&self, other_value: f64) -> f64 {
        if self.is_unity() {
            self.value
        } else {
            let terms_magnitude = self.terms
                .iter()
                .fold(1.0, |acc, term| acc * term.calculate_magnitude(other_value));

            self.value * terms_magnitude
        }
    }

    pub fn is_unity(&self) -> bool {
        self.terms.len() == 1 && self.terms[0].is_unity()
    }
}
