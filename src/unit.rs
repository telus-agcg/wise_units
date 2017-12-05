use atom::Atom;
use composition::Composition;
use decomposable::Decomposable;
use error::Error;
use interpreter::Interpreter;
use pest::Parser;
use reduction_decomposer::ReductionDecomposer;
use simple_decomposer::SimpleDecomposer;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Div, Mul};
use std::str::FromStr;
use term::Term;
use unit_parser::{Rule, UnitParser};

#[derive(Clone, Debug, PartialEq)]
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
        self.terms.len() == 1
            && self.terms[0]
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

    /// Allows for dividing a Unit by a factor; results in dividing this Unit's
    /// associated Terms' factors by `other_factor`.
    ///
    pub fn div_u32(&self, other_factor: u32) -> Unit {
        let mut new_terms = Vec::with_capacity(self.terms.len());

        for term in &self.terms {
            let mut new_term = term.clone();
            new_term.factor /= other_factor;
            new_terms.push(new_term);
        }

        Unit { terms: new_terms }
    }

    /// Allows for multiplying a Unit by a factor; results in multiplying this
    /// Unit's associated Terms' factors by `other_factor`.
    ///
    pub fn mul_u32(&self, other_factor: u32) -> Unit {
        let mut new_terms = Vec::with_capacity(self.terms.len());

        for term in &self.terms {
            let mut new_term = term.clone();
            new_term.factor *= other_factor;
            new_terms.push(new_term);
        }

        Unit { terms: new_terms }
    }

    pub fn is_valid(expression: &str) -> bool {
        Unit::from_str(expression).is_ok()
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.expression()) }
}

impl FromStr for Unit {
    type Err = Error;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        match UnitParser::parse_str(Rule::main_term, expression) {
            Ok(pairs) => {
                let mut interpreter = Interpreter;
                Ok(interpreter.interpret(pairs)?)
            },
            Err(_)=> Err(Error::UnknownUnitString(expression.to_string()))
        }
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(self, other: Unit) -> Self::Output {
        let mut new_terms = self.terms.clone();

        for other_term in &other.terms {
            let mut new_other_term = other_term.clone();
            new_other_term.exponent = -new_other_term.exponent;
            new_terms.push(new_other_term);
        }

        Unit { terms: new_terms }
    }
}

impl<'a> Div for &'a Unit {
    type Output = Unit;

    fn div(self, other: &'a Unit) -> Self::Output {
        let mut new_terms = self.terms.clone();

        for other_term in &other.terms {
            let mut new_other_term = other_term.clone();
            new_other_term.exponent = -new_other_term.exponent;
            new_terms.push(new_other_term);
        }

        Unit { terms: new_terms }
    }
}

impl<'a> Div for &'a mut Unit {
    type Output = Unit;

    fn div(self, other: &'a mut Unit) -> Self::Output {
        let mut new_terms = self.terms.clone();

        for other_term in &other.terms {
            let mut new_other_term = other_term.clone();
            new_other_term.exponent = -new_other_term.exponent;
            new_terms.push(new_other_term);
        }

        Unit { terms: new_terms }
    }
}

impl Mul for Unit {
    type Output = Unit;

    fn mul(self, other: Unit) -> Self::Output {
        let mut new_terms = self.terms.clone();
        new_terms.extend(other.terms.clone());

        Unit { terms: new_terms }
    }
}

impl<'a> Mul for &'a Unit {
    type Output = Unit;

    fn mul(self, other: &'a Unit) -> Self::Output {
        let mut new_terms = self.terms.clone();
        new_terms.extend(other.terms.clone());

        Unit { terms: new_terms }
    }
}

impl<'a> Mul for &'a mut Unit {
    type Output = Unit;

    fn mul(self, other: &'a mut Unit) -> Self::Output {
        let mut new_terms = self.terms.clone();
        new_terms.extend(other.terms.clone());

        Unit { terms: new_terms }
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Unit) -> Option<Ordering> {
        if self.is_compatible_with(other) {
            let other_scalar = other.scalar();
            let my_scalar = self.scalar();

            my_scalar.partial_cmp(&other_scalar)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use composition::Composition;
    use dimension::Dimension;

    #[test]
    fn validate_from_str_error() {
        let unit = Unit::from_str("ZZZXXXXXXXXXXXXx");
        assert!(unit.is_err());
    }

    #[test]
    fn validate_is_special() {
        let unit = Unit::from_str("m").unwrap();
        assert!(!unit.is_special());
    }

    #[test]
    fn validate_is_unity() {
        // The unity
        let unit = Unit::from_str("1").unwrap();
        assert!(unit.is_unity());

        // Dimless unit
        let unit = Unit::from_str("[ppth]").unwrap();
        assert!(!unit.is_unity());

        // Regular unit
        let unit = Unit::from_str("m").unwrap();
        assert!(!unit.is_unity());
    }

    #[test]
    fn validate_scalar() {
        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.scalar(), 1.0);

        let unit = Unit::from_str("km").unwrap();
        assert_eq!(unit.scalar(), 1000.0);

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.scalar(), 100.0);

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.scalar(), 1.0);

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.scalar(), 10.0);

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.scalar(), 10_000.0);

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.scalar(), 0.0001);

        let unit = Unit::from_str("km-1m2").unwrap();
        assert_eq!(unit.scalar(), 0.001);

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.scalar(), 100_000.0);

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.scalar(), 0.1);

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.scalar(), 1.0);

        let unit = Unit::from_str("/1").unwrap();
        assert_eq!(unit.scalar(), 1.0);

        let unit = Unit::from_str("/m").unwrap();
        assert_eq!(unit.scalar(), 1.0);

        let unit = Unit::from_str("/{tot}").unwrap();
        assert_eq!(unit.scalar(), 1.0);
    }

    #[test]
    fn validate_magnitude() {
        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.magnitude(), 1.0);

        let unit = Unit::from_str("km").unwrap();
        assert_eq!(unit.magnitude(), 1000.0);

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.magnitude(), 100.0);

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.magnitude(), 1.0);

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.magnitude(), 10.0);

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.magnitude(), 10_000.0);

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.magnitude(), 0.0001);

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.magnitude(), 0.001);

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.magnitude(), 100_000.0);

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.magnitude(), 0.1);

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.magnitude(), 1.0);

        let unit = Unit::from_str("/1").unwrap();
        assert_eq!(unit.magnitude(), 1.0);

        let unit = Unit::from_str("/m").unwrap();
        assert_eq!(unit.magnitude(), 1.0);

        let unit = Unit::from_str("/{tot}").unwrap();
        assert_eq!(unit.magnitude(), 1.0);
    }

    #[test]
    fn validate_composition() {
        let unit = Unit::from_str("[pi]").unwrap();
        assert_eq!(unit.composition(), None);

        let unit = Unit::from_str("[ppth]").unwrap();
        assert_eq!(unit.composition(), None);

        let unit = Unit::from_str("m").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("km").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("km/10m").unwrap();
        let composition = Composition::new(Dimension::Length, 0);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("m-1").unwrap();
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("10m").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("10km").unwrap();
        let composition = Composition::new(Dimension::Length, 1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("10km-1").unwrap();
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("km-1/m2").unwrap();
        let composition = Composition::new(Dimension::Length, -3);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("km/m2.cm").unwrap();
        let composition = Composition::new(Dimension::Length, -2);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        let composition = Composition::new(Dimension::Length, -4);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("m/s2").unwrap();
        let mut composition = Composition::new(Dimension::Length, 1);
        composition.insert(Dimension::Time, -2);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("/1").unwrap();
        assert_eq!(unit.composition(), None);

        let unit = Unit::from_str("/m").unwrap();
        let composition = Composition::new(Dimension::Length, -1);
        assert_eq!(unit.composition().unwrap(), composition);

        let unit = Unit::from_str("/{tot}").unwrap();
        assert_eq!(unit.composition(), None);
    }

    #[test]
    fn validate_is_compatible_with() {
        let meter = Unit::from_str("m").unwrap();
        let km = Unit::from_str("km").unwrap();
        assert!(meter.is_compatible_with(&km));

        let km_per_10m = Unit::from_str("km/10m").unwrap();
        assert!(!meter.is_compatible_with(&km_per_10m));

        let per_meter = Unit::from_str("m-1").unwrap();
        assert!(!meter.is_compatible_with(&per_meter));

        let ten_meter = Unit::from_str("10m").unwrap();
        assert!(meter.is_compatible_with(&ten_meter));

        let ten_km = Unit::from_str("10km").unwrap();
        assert!(meter.is_compatible_with(&ten_km));

        let per_ten_km = Unit::from_str("10km-1").unwrap();
        assert!(!meter.is_compatible_with(&per_ten_km));

        let per_meter_cubed = Unit::from_str("km-1/m2").unwrap();
        assert!(!meter.is_compatible_with(&per_meter_cubed));

        let km_per_length_cubed = Unit::from_str("km/m2.cm").unwrap();
        assert!(!meter.is_compatible_with(&km_per_length_cubed));

        let km_per_length_fourthed = Unit::from_str("km-1/m2.cm").unwrap();
        assert!(!meter.is_compatible_with(&km_per_length_fourthed));

        let meter_per_second_squared = Unit::from_str("m/s2").unwrap();
        assert!(!meter.is_compatible_with(&meter_per_second_squared));

        let km_cubed_per_nanometer_squared = Unit::from_str("km3/nm2").unwrap();
        assert!(meter.is_compatible_with(&km_cubed_per_nanometer_squared));
    }

    #[test]
    fn validate_expression_reduced() {
        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "m");

        let unit = Unit::from_str("M").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "m");

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km/10m");

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "1/m");

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "10m");

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "10km");

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "1/10km");

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "1/km.m2");

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km/cm.m2");

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "1/cm.km.m2");

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "m/s2");

        let unit = Unit::from_str("km3/nm2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km3/nm2");
    }

    #[test]
    fn validate_display() {
        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.to_string().as_str(), "m");

        let unit = Unit::from_str("M").unwrap();
        assert_eq!(unit.to_string().as_str(), "m");

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.to_string().as_str(), "km/10m");

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/m");

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.to_string().as_str(), "10m");

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.to_string().as_str(), "10km");

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/10km");

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/km.m2");

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.to_string().as_str(), "km/m2.cm");

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/km.m2.cm");

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.to_string().as_str(), "m/s2");

        let unit = Unit::from_str("km3/nm2").unwrap();
        assert_eq!(unit.to_string().as_str(), "km3/nm2");
    }

    #[test]
    fn validate_div_u32() {
        let unit = Unit::from_str("2m").unwrap();
        assert_eq!(unit.div_u32(2).to_string().as_str(), "m");

        let unit = Unit::from_str("2m").unwrap();
        assert_eq!(unit.div_u32(4).to_string().as_str(), "0m");
    }

    #[test]
    fn validate_div() {
        let unit = Unit::from_str("m").unwrap();
        let other = Unit::from_str("km").unwrap();
        assert_eq!(unit.div(&other).to_string().as_str(), "m/km");

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        assert_eq!(unit.div(&other).to_string().as_str(), "10m/20m");
    }

    #[test]
    fn validate_mul_u32() {
        let unit = Unit::from_str("2m").unwrap();
        assert_eq!(unit.mul_u32(2).to_string().as_str(), "4m");
    }

    #[test]
    fn validate_mul() {
        let unit = Unit::from_str("m").unwrap();
        let other = Unit::from_str("km").unwrap();
        assert_eq!(unit.mul(&other).to_string().as_str(), "m.km");

        let unit = Unit::from_str("10m").unwrap();
        let other = Unit::from_str("20m").unwrap();
        assert_eq!(unit.mul(&other).to_string().as_str(), "10m.20m");
    }
}
