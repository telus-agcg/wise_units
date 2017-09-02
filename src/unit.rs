use composition::Composition;
use term::Term;

#[derive(Debug, PartialEq)]
pub struct Unit {
    pub expression: String,
    pub terms: Vec<Term>,
}

impl Unit {
    pub fn is_special(&self) -> bool {
        self.terms.iter().any(|ref term| {
            match term.atom {
                Some(ref unit_container) => unit_container.is_special(),
                None => false
            }
        })
    }

    pub fn scalar(&self) -> f64 {
        self.calculate_scalar(1.0)
    }

    pub fn magnitude(&self) -> f64 {
        self.calculate_magnitude(self.scalar())
    }

    pub fn calculate_scalar(&self, magnitude: f64) -> f64 {
        self.terms.iter()
            .fold(1.0, |acc, ref term| {
                let term = *term;
                acc * term.calculate_scalar(magnitude)
            })
    }

    pub fn calculate_magnitude(&self, scalar: f64) -> f64 {
        self.terms.iter()
            .fold(1.0, |acc, ref term| {
                let term = *term;
                acc * term.calculate_magnitude(scalar)
            })
    }

    pub fn composition(&self) -> Composition {
        let mut composition = Composition::new();

        for term in &self.terms {
            match term.composition() {
                Some(term_composition) => {
                    for (term_dim, term_exponent) in term_composition {
                        composition.insert(term_dim, term_exponent);
                    }
                },
                None => continue
            }
        }

        composition
    }

    pub fn is_compatible_with(&self, other_unit: &Unit) -> bool {
        let me = self.composition();
        let other_comp = other_unit.composition();

        me == other_comp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use interpreter::Interpreter;

    #[test]
    fn validate_is_special() {
        let mut i = Interpreter;
        let su = i.interpret("m");

        assert!(!su.is_special());
    }

    #[test]
    fn validate_scalar() {
        let mut i = Interpreter;
        let su = i.interpret("m");

        assert_eq!(su.scalar(), 1.0);
    }

    #[test]
    fn validate_magnitude() {
        let mut i = Interpreter;
        let su = i.interpret("m");

        assert_eq!(su.magnitude(), 1.0);
    }
}
