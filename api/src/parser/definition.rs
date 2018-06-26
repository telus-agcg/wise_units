use parser::{function_set::FunctionSet, Error, Term};
use reducible::Reducible;

/// A `Definition` is a slimmed-down version of a `Measurement` that is used to
/// define `Atom`s in terms of other `Atom`s (ex. an `"[in_i]"` has a
/// `Definition` of 2.54 cm).
///
#[derive(Debug)]
pub(crate) struct Definition {
    value: f64,
    terms: Vec<Term>,

    /// Conversion functions only required for special (non-ratio based) atoms.
    function_set: Option<FunctionSet>,
}

impl Definition {
    pub(crate) fn new(
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

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn terms(&self) -> &[Term] {
        &self.terms
    }

    pub fn is_unity(&self) -> bool {
        self.terms.len() == 1 && self.terms[0].is_unity()
    }
}

impl Reducible for Definition {
    fn reduce_value(&self, other_value: f64) -> f64 {
        match self.function_set {
            None => {
                if self.is_unity() {
                    self.value
                } else {
                    let terms_scalar = self.terms
                        .iter()
                        .fold(1.0, |acc, term| acc * term.reduce_value(other_value));

                    self.value * terms_scalar
                }
            }
            Some(ref f) => (f.convert_to)(other_value),
        }
    }

    fn calculate_magnitude(&self, other_value: f64) -> f64 {
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
