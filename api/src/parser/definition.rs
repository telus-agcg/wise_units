#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]

mod reducible;

#[cfg(feature = "v2")]
mod v2;

use crate::parser::{function_set::FunctionSet, Error, Term};

/// A `Definition` is a slimmed-down version of a `Measurement` that is used to
/// define `Atom`s in terms of other `Atom`s (ex. an `"[in_i]"` has a
/// `Definition` of 2.54 cm).
///
#[derive(Debug)]
pub struct Definition {
    value: f64,
    terms: Vec<Term>,

    /// Conversion functions only required for special (non-ratio based) atoms.
    function_set: Option<FunctionSet>,
}

impl Definition {
    pub(crate) fn try_new(
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

    pub(crate) fn new<T>(value: f64, terms: T, function_set: Option<FunctionSet>) -> Self
    where
        Vec<Term>: From<T>,
    {
        let terms = {
            let t = Vec::from(terms);

            if t.is_empty() {
                vec![Term::new_unity()]
            } else {
                t
            }
        };
        Self {
            value,
            terms,
            function_set,
        }
    }

    pub(crate) fn new_non_dimensional(value: f64) -> Self {
        Self {
            value,
            terms: vec![Term::new_unity()],
            function_set: None,
        }
    }

    pub(crate) const fn value(&self) -> f64 {
        self.value
    }

    pub(crate) const fn terms(&self) -> &Vec<Term> {
        &self.terms
    }

    pub(crate) fn is_unity(&self) -> bool {
        self.terms.len() == 1 && self.terms[0].is_unity()
    }
}

impl Default for Definition {
    fn default() -> Self {
        Self {
            value: 1.0,
            terms: vec![Term::new_unity()],
            function_set: None,
        }
    }
}
