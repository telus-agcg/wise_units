use parser_terms::{FunctionSymbol, Term};
use std::collections::BTreeMap;
use std::fmt;
use unit::Dimension;

#[derive(Debug, PartialEq)]
pub struct SpecialUnit<'a>(pub FunctionSymbol, pub f64, pub Box<Term<'a>>);

impl<'a> SpecialUnit<'a> {
    pub fn composition(&self) -> BTreeMap<Dimension, i32> {
        let ref term = self.2;
        let comp = term.composition();

        comp
    }

    // Not sure if returning 1.0 makes sense here...
    pub fn scalar(&self) -> f64 {
        1.0
    }

    pub fn is_special(&self) -> bool {
        true
    }

    pub fn calculate_scalar(&self, input: f64) -> f64 {
        self.0.calculate_scalar(input)
    }
}

impl<'a> fmt::Display for SpecialUnit<'a>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({} {})", self.0, self.1, self.2)
    }
}

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
                Box::new(
                    Term::Basic(
                        Component::Annotatable(
                            Annotatable::Unit(
                                SimpleUnit::Atom(Box::new(Kelvin))
                                )
                            )
                        )
                    )
                )
            );
    }
}
