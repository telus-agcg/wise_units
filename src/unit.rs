use composition::Composition;
use std::fmt;
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
                Some(ref atom) => atom.is_special(),
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

    pub fn calculate_scalar(&self, value: f64) -> f64 {
        self.terms.iter()
            .fold(1.0, |acc, ref term| {
                let term = *term;
                acc * term.calculate_scalar(value)
            })
    }

    pub fn calculate_magnitude(&self, value: f64) -> f64 {
        self.terms.iter()
            .fold(1.0, |acc, ref term| {
                let term = *term;
                acc * term.calculate_magnitude(value)
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

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

#[cfg(test)]
mod tests {
    use composition::Composition;
    use interpreter::Interpreter;
    use dimension::Dimension;

    #[test]
    fn validate_is_special() {
        let mut i = Interpreter;
        let unit = i.interpret("m");

        assert!(!unit.is_special());
    }

    #[test]
    fn validate_scalar() {
        let mut i = Interpreter;

        let unit = i.interpret("m");
        assert_eq!(unit.scalar(), 1.0);

        let unit = i.interpret("km");
        assert_eq!(unit.scalar(), 1000.0);

        let unit = i.interpret("km/10m");
        assert_eq!(unit.scalar(), 100.0);

        let unit = i.interpret("m-1");
        assert_eq!(unit.scalar(), 1.0);

        let unit = i.interpret("10m");
        assert_eq!(unit.scalar(), 10.0);

        let unit = i.interpret("10km");
        assert_eq!(unit.scalar(), 10_000.0);

        let unit = i.interpret("10km-1");
        assert_eq!(unit.scalar(), 0.0001);

        let unit = i.interpret("km-1/m2");
        assert_eq!(unit.scalar(), 0.001);

        let unit = i.interpret("km/m2.cm");
        assert_eq!(unit.scalar(), 100_000.0);

        let unit = i.interpret("km-1/m2.cm");
        assert_eq!(unit.scalar(), 0.1);

        let unit = i.interpret("m/s2");
        assert_eq!(unit.scalar(), 1.0);

        // TODO: Interpreter needs to parse using Rule::main_term, not Rule::term
        // let unit = i.interpret("/{tot}");
        // assert_eq!(unit.scalar(), 1.0);
    }

    #[test]
    fn validate_magnitude() {
        let mut i = Interpreter;

        let unit = i.interpret("m");
        assert_eq!(unit.magnitude(), 1.0);

        let unit = i.interpret("km");
        assert_eq!(unit.magnitude(), 1000.0);

        let unit = i.interpret("km/10m");
        assert_eq!(unit.magnitude(), 100.0);

        let unit = i.interpret("m-1");
        assert_eq!(unit.magnitude(), 1.0);

        let unit = i.interpret("10m");
        assert_eq!(unit.magnitude(), 10.0);

        let unit = i.interpret("10km");
        assert_eq!(unit.magnitude(), 10_000.0);

        let unit = i.interpret("10km-1");
        assert_eq!(unit.magnitude(), 0.0001);

        let unit = i.interpret("km-1/m2");
        assert_eq!(unit.magnitude(), 0.001);

        let unit = i.interpret("km/m2.cm");
        assert_eq!(unit.magnitude(), 100_000.0);

        let unit = i.interpret("km-1/m2.cm");
        assert_eq!(unit.magnitude(), 0.1);

        let unit = i.interpret("m/s2");
        assert_eq!(unit.magnitude(), 1.0);

        // TODO: Interpreter needs to parse using Rule::main_term, not Rule::term
        // let unit = i.interpret("/{tot}");
        // assert_eq!(unit.magnitude(), 1.0);
    }

    #[test]
    fn validate_composition() {
        let mut i = Interpreter;

        let unit = i.interpret("m");
        let composition = Composition::new_from_values(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = i.interpret("km");
        let composition = Composition::new_from_values(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = i.interpret("km/10m");
        let composition = Composition::new_from_values(Dimension::Length, 0);
        assert_eq!(unit.composition(), composition);

        let unit = i.interpret("m-1");
        let composition = Composition::new_from_values(Dimension::Length, -1);
        assert_eq!(unit.composition(), composition);

        let unit = i.interpret("10m");
        let composition = Composition::new_from_values(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = i.interpret("10km");
        let composition = Composition::new_from_values(Dimension::Length, 1);
        assert_eq!(unit.composition(), composition);

        let unit = i.interpret("10km-1");
        let composition = Composition::new_from_values(Dimension::Length, -1);
        assert_eq!(unit.composition(), composition);

        let unit = i.interpret("km-1/m2");
        let composition = Composition::new_from_values(Dimension::Length, -3);
        assert_eq!(unit.composition(), composition);

        let unit = i.interpret("km/m2.cm");
        let composition = Composition::new_from_values(Dimension::Length, -2);
        assert_eq!(unit.composition(), composition);

        let unit = i.interpret("km-1/m2.cm");
        let composition = Composition::new_from_values(Dimension::Length, -4);
        assert_eq!(unit.composition(), composition);

        let unit = i.interpret("m/s2");
        let mut composition = Composition::new_from_values(Dimension::Length, 1);
        composition.insert(Dimension::Time, -2);
        assert_eq!(unit.composition(), composition);

        // TODO: Interpreter needs to parse using Rule::main_term, not Rule::term
        // let unit = i.interpret("/{tot}");
        // assert_eq!(unit.magnitude(), 1.0);
    }

    #[test]
    fn validate_is_compatible_with() {
        let mut i = Interpreter;

        let meter = i.interpret("m");
        let km = i.interpret("km");
        assert!(meter.is_compatible_with(&km));

        let km_per_10m = i.interpret("km/10m");
        assert!(!meter.is_compatible_with(&km_per_10m));

        let per_meter = i.interpret("m-1");
        assert!(!meter.is_compatible_with(&per_meter));

        let ten_meter = i.interpret("10m");
        assert!(meter.is_compatible_with(&ten_meter));

        let ten_km = i.interpret("10km");
        assert!(meter.is_compatible_with(&ten_km));

        let per_ten_km = i.interpret("10km-1");
        assert!(!meter.is_compatible_with(&per_ten_km));

        let per_meter_cubed = i.interpret("km-1/m2");
        assert!(!meter.is_compatible_with(&per_meter_cubed));

        let km_per_length_cubed = i.interpret("km/m2.cm");
        assert!(!meter.is_compatible_with(&km_per_length_cubed));

        let km_per_length_fourthed = i.interpret("km-1/m2.cm");
        assert!(!meter.is_compatible_with(&km_per_length_fourthed));

        let meter_per_second_squared = i.interpret("m/s2");
        assert!(!meter.is_compatible_with(&meter_per_second_squared));

        let km_cubed_per_nanometer_squared = i.interpret("km3/nm2");
        assert!(meter.is_compatible_with(&km_cubed_per_nanometer_squared));
    }
}
