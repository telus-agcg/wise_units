use atom::Atom;
use composition::Composition;
use decomposable::Decomposable;
use reduction_decomposer::ReductionDecomposer;
use simple_decomposer::SimpleDecomposer;
use std::fmt;
use term::Term;

#[derive(Debug, PartialEq)]
pub struct Unit {
    pub terms: Vec<Term>,
}

impl Unit {
    pub fn is_special(&self) -> bool {
        self.terms.iter().any(|term| match term.atom {
            Some(ref atom) => atom.is_special(),
            None => false,
        })
    }

    /// Checks if this unit is really just a wrapper around `Atom::TheUnity`.
    /// This is helpful for knowing, internally, when to stop recursively
    /// calling some functions.
    ///
    pub fn is_unity(&self) -> bool {
        self.terms.len() == 1 &&
            self.terms[0]
                .atom
                .map_or(false, |atom| atom == Atom::TheUnity)
    }

    /// Use this when calculating the scalar when *not* part of a Measurable.
    pub fn scalar(&self) -> f64 { self.calculate_scalar(1.0) }

    pub fn magnitude(&self) -> f64 { self.calculate_magnitude(self.scalar()) }

    /// Use this when calculating the scalar when it's part of a Measurable.
    pub fn calculate_scalar(&self, value: f64) -> f64 {
        self.terms
            .iter()
            .fold(1.0, |acc, term| acc * term.calculate_scalar(value))
    }

    pub fn calculate_magnitude(&self, value: f64) -> f64 {
        self.terms
            .iter()
            .fold(1.0, |acc, term| acc * term.calculate_magnitude(value))
    }

    pub fn composition(&self) -> Option<Composition> {
        let mut composition = Composition::default();

        for term in &self.terms {
            match term.composition() {
                Some(term_composition) => for (term_dimension, term_exponent) in term_composition {
                    composition.insert(term_dimension, term_exponent);
                },
                None => continue,
            }
        }

        if composition.is_empty() {
            None
        } else {
            Some(composition)
        }
    }

    pub fn is_compatible_with(&self, other_unit: &Unit) -> bool {
        let me = self.composition();
        let other_comp = other_unit.composition();

        me == other_comp
    }

    /// Turns the Unit's Terms into Strings and combines them accordingly.
    /// This always returns a String that is parsable back into the same Unit.
    ///
    /// Ex. terms that would normally render `[acr_us].[in_i]/[acr_us]` would
    /// render the same result.
    ///
    pub fn expression(&self) -> String { SimpleDecomposer::new(&self.terms).expression() }

    /// If the unit terms are a fraction and can be reduced, this returns those
    /// as a string. Ex. terms that would normally render
    /// `[acr_us].[in_i]/[acr_us]` would simply render `[in_i]`.
    /// This always returns a String that is parsable back into the same Unit.
    ///
    pub fn expression_reduced(&self) -> String {
        ReductionDecomposer::new(&self.terms).expression()
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.expression()) }
}

#[cfg(test)]
mod tests {
    use composition::Composition;
    use dimension::Dimension;
    use interpreter::Interpreter;

    #[test]
    fn validate_is_special() {
        let mut i = Interpreter;
        let unit = i.interpret("m");

        assert!(!unit.is_special());
    }

    #[test]
    fn validate_is_unity() {
        let mut i = Interpreter;

        // The unity
        let unit = i.interpret("1");
        assert!(unit.is_unity());

        // Dimless unit
        let unit = i.interpret("[ppth]");
        assert!(!unit.is_unity());

        // Regular unit
        let unit = i.interpret("m");
        assert!(!unit.is_unity());
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

        let unit = i.interpret("[pi]");
        assert_eq!(unit.composition(), None);

        let unit = i.interpret("[ppth]");
        assert_eq!(unit.composition(), None);

        let unit = i.interpret("m");
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = i.interpret("km");
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = i.interpret("km/10m");
        let composition = Composition::new(Dimension::Length, 0);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = i.interpret("m-1");
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = i.interpret("10m");
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = i.interpret("10km");
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = i.interpret("10km-1");
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = i.interpret("km-1/m2");
        let composition = Composition::new(Dimension::Length, -3);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = i.interpret("km/m2.cm");
        let composition = Composition::new(Dimension::Length, -2);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = i.interpret("km-1/m2.cm");
        let composition = Composition::new(Dimension::Length, -4);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = i.interpret("m/s2");
        let mut composition = Composition::new(Dimension::Length, 1);
        composition.insert(Dimension::Time, -2);
        assert_eq!(unit.composition().unwrap(), composition);

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

    #[test]
    fn validate_expression_reduced() {
        let mut i = Interpreter;

        let unit = i.interpret("m");
        assert_eq!(unit.expression_reduced().as_str(), "m");

        let unit = i.interpret("M");
        assert_eq!(unit.expression_reduced().as_str(), "m");

        let unit = i.interpret("km/10m");
        assert_eq!(unit.expression_reduced().as_str(), "km/10m");

        let unit = i.interpret("m-1");
        assert_eq!(unit.expression_reduced().as_str(), "1/m");

        let unit = i.interpret("10m");
        assert_eq!(unit.expression_reduced().as_str(), "10m");

        let unit = i.interpret("10km");
        assert_eq!(unit.expression_reduced().as_str(), "10km");

        let unit = i.interpret("10km-1");
        assert_eq!(unit.expression_reduced().as_str(), "1/10km");

        let unit = i.interpret("km-1/m2");
        assert_eq!(unit.expression_reduced().as_str(), "1/km.m2");

        let unit = i.interpret("km/m2.cm");
        assert_eq!(unit.expression_reduced().as_str(), "km/cm.m2");

        let unit = i.interpret("km-1/m2.cm");
        assert_eq!(unit.expression_reduced().as_str(), "1/cm.km.m2");

        let unit = i.interpret("m/s2");
        assert_eq!(unit.expression_reduced().as_str(), "m/s2");

        let unit = i.interpret("km3/nm2");
        assert_eq!(unit.expression_reduced().as_str(), "km3/nm2");
    }

    #[test]
    fn validate_display() {
        let mut i = Interpreter;

        let unit = i.interpret("m");
        assert_eq!(unit.to_string().as_str(), "m");

        let unit = i.interpret("M");
        assert_eq!(unit.to_string().as_str(), "m");

        let unit = i.interpret("km/10m");
        assert_eq!(unit.to_string().as_str(), "km/10m");

        let unit = i.interpret("m-1");
        assert_eq!(unit.to_string().as_str(), "1/m");

        let unit = i.interpret("10m");
        assert_eq!(unit.to_string().as_str(), "10m");

        let unit = i.interpret("10km");
        assert_eq!(unit.to_string().as_str(), "10km");

        let unit = i.interpret("10km-1");
        assert_eq!(unit.to_string().as_str(), "1/10km");

        let unit = i.interpret("km-1/m2");
        assert_eq!(unit.to_string().as_str(), "1/km.m2");

        let unit = i.interpret("km/m2.cm");
        assert_eq!(unit.to_string().as_str(), "km/m2.cm");

        let unit = i.interpret("km-1/m2.cm");
        assert_eq!(unit.to_string().as_str(), "1/km.m2.cm");

        let unit = i.interpret("m/s2");
        assert_eq!(unit.to_string().as_str(), "m/s2");

        let unit = i.interpret("km3/nm2");
        assert_eq!(unit.to_string().as_str(), "km3/nm2");
    }
}
