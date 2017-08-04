use parser_terms::{FunctionSymbol, Term};
use std::collections::BTreeMap;
use std::cmp::PartialEq;
use std::fmt;
use unit::Dimension;

/// A SpecialUnit node in the AST represents a Unit whose definition is a
/// function of other non-special units.
///
#[derive(Debug)]
pub struct SpecialUnit(pub FunctionSymbol, pub f64, pub Box<Term>);

impl SpecialUnit {
    pub fn composition(&self) -> BTreeMap<Dimension, i32> {
        let ref term = self.2;
        term.composition()
    }

    // TODO: Not sure if returning 1.0 makes sense here...
    pub fn scalar(&self) -> f64 { 1.0 }

    // TODO: Not sure if returning 1.0 makes sense here...
    pub fn magnitude(&self) -> f64 { 1.0 }

    pub fn is_special(&self) -> bool { true }

    pub fn calculate_scalar(&self, magnitude: f64) -> f64 { self.calculate_magnitude(magnitude) }

    pub fn calculate_magnitude(&self, scalar: f64) -> f64 { self.2.calculate_magnitude(scalar) }
}

impl fmt::Display for SpecialUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({} {})", self.0, self.1, self.2)
    }
}

impl PartialEq for SpecialUnit {
    fn eq(&self, other: &SpecialUnit) -> bool { self.composition() == other.composition() }
}

impl Eq for SpecialUnit {}

#[cfg(test)]
mod tests {
    use super::SpecialUnit;
    use parser::parse_SpecialUnit;
    use parser_terms::{Annotatable, Component, FunctionSymbol, SimpleUnit, Term};
    use unit::base::Kelvin;

    #[test]
    fn validate_parsing_special_unit() {
        assert_eq!(
            parse_SpecialUnit("cel(1.0 K)").unwrap(),
            SpecialUnit(
                FunctionSymbol::Cel,
                1.0,
                Box::new(Term::Basic(Component::Annotatable(
                    Annotatable::Unit(SimpleUnit::Atom(Box::new(Kelvin))),
                ))),
            )
        );
    }

    #[test]
    fn validate_display() {
        let cel = parse_SpecialUnit("cel(1.0 K)").unwrap();
        let c = format!("{}", cel);
        assert_eq!(c, "cel(1 K)".to_string());
    }
}
