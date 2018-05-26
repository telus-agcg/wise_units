use error::Error;
use function_set::FunctionSet;
use term::Term;
use ucum_symbol::UcumSymbol;

#[derive(Debug)]
pub struct Definition {
    pub value: f64,
    pub terms: Vec<Term>,

    /// Conversion functions only required for special (non-ratio based) atoms.
    pub function_set: Option<FunctionSet>,
}

impl Definition {
    pub fn new(
        value: f64,
        expression: &str,
        function_set: Option<FunctionSet>,
    ) -> Result<Self, Error> {
        let terms = super::parse(expression)?;

        Ok(Definition {
            value,
            terms,
            function_set,
        })
    }

    pub fn is_special(&self) -> bool {
        self.terms.iter().any(|term| match term.atom {
            Some(ref atom) => atom.is_special(),
            None => false,
        })
    }

    pub fn scalar(&self) -> f64 {
        match self.function_set {
            None => self.value * self.calculate_scalar(1.0),
            Some(ref f) => {
                let result = (f.convert_to)(self.value);

                self.calculate_scalar(result)
            }
        }
    }

    pub fn magnitude(&self) -> f64 {
        match self.function_set {
            None => self.value * self.calculate_magnitude(1.0),
            Some(ref f) => {
                let result = (f.convert_from)(self.value);

                self.calculate_magnitude(result)
            }
        }
    }

    pub fn calculate_scalar(&self, other_value: f64) -> f64 {
        debug!("calculate_scalar()");

        match self.function_set {
            None => {
                if self.is_unity() {
                    self.value
                } else {
                    let terms_scalar = self.terms
                        .iter()
                        .fold(1.0, |acc, term| acc * term.calculate_scalar(other_value));

                    self.value * terms_scalar
                }
            }
            Some(ref f) => (f.convert_to)(other_value),
        }
    }

    pub fn calculate_magnitude(&self, other_value: f64) -> f64 {
        debug!("calculate_magnitude()");

        match self.function_set {
            None => {
                if self.is_unity() {
                    self.value
                } else {
                    let terms_magnitude = self.terms
                        .iter()
                        .fold(1.0, |acc, term| acc * term.calculate_magnitude(other_value));

                    self.value * terms_magnitude
                }
            }
            Some(ref f) => (f.convert_from)(other_value),
        }
    }

    pub fn is_unity(&self) -> bool {
        self.terms.len() == 1 && self.terms[0].is_unity()
    }
}
