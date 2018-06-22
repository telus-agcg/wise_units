use parser::{Error, FunctionSet, Term};

/// A `Definition` is a slimmed-down version of a `Measurement` that is used to
/// define `Atom`s in terms of other `Atom`s (ex. an "[in_i]" has a
/// `Definition` of 2.54 cm).
///
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

        Ok(Self {
            value,
            terms,
            function_set,
        })
    }

    pub fn calculate_scalar(&self, other_value: f64) -> f64 {
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

impl Default for Definition {
    fn default() -> Self {
        Self {
            value: 1.0,
            terms: vec![term!()],
            function_set: None,
        }
    }
}
