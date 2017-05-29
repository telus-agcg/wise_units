use parser::parse_MainTerm;
use parser_terms::Term;

/// Represents the UCUM "definition" of a Unit (since units are defined in terms
/// of some scalar and another, more basic unit). For example a "degree" is:
///
/// ```
/// use wu::unit::Definition;
///
/// Definition::new(2.0, "[pi].rad/360");
/// ```
///
/// In that case the `Degree` struct will have some other functions/data to
/// describe it, but this `definition` is how the unit is compared to other
/// units.
///
pub struct Definition<'a> {
    pub value: f64,
    pub term: Term<'a>
}

impl<'a> Definition<'a> {
    // TODO: wrong lifetime for expression here
    pub fn new(value: f64, expression: &'a str) -> Self {
        // TODO: unwrap
        let main_term = parse_MainTerm(expression).unwrap();

        Definition {
            value: value,
            term: main_term
        }
    }

    pub fn scalar(&self) -> f64 {
        // Don't call (possibly) recursively if the Term is TheUnity (since that
        // is the base of all units).
        if self.term.to_string() == "1".to_string() {
            self.value
        } else {
            self.value * self.term.scalar()
        }
    }

    pub fn magnitude(&self) -> f64 {
        // Don't call (possibly) recursively if the Term is TheUnity (since that
        // is the base of all units).
        if self.term.to_string() == "1".to_string() {
            self.value
        } else {
            self.value * self.term.magnitude()
        }
    }

    pub fn calculate_scalar(&self, magnitude: f64) -> f64 {
        if self.term.to_string() == "1".to_string() {
            self.value
        } else {
            self.term.calculate_scalar(magnitude)
        }
    }

    pub fn calculate_magnitude(&self, scalar: f64) -> f64 {
        if self.term.to_string() == "1".to_string() {
            self.value
        } else {
            self.term.calculate_magnitude(scalar)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_scalar() {
        let d = Definition::new(1.0, "10*-2");
        assert_eq!(d.scalar(), 0.01);
    }
}
