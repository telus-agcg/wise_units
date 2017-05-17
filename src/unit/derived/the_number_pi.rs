use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct TheNumberPi;

impl Unit for TheNumberPi {
    fn classification(&self) -> Classification { Classification::Dimless }
    fn definition(&self)     -> Definition {
        Definition::new(3.141_592_653_589_793_238_462_643_383_279_502_884_197_169_399_375_105_820_974_944_592_3, "1")
    }
    fn dim(&self)            -> Dimension { Dimension::None }
    fn is_arbitrary(&self)   -> bool { false }
    fn is_metric(&self)      -> bool { false }
    fn is_special(&self)     -> bool { false }
    fn names(&self)          -> Vec<String> { vec!["the number pi".to_string()] }
    fn primary_code(&self)   -> String { "[pi]".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("π".to_string()) }
    fn property(&self)       -> Property { Property::Number }
    fn secondary_code(&self) -> String { "[PI]".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Derived }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_building() {
        let subject = TheNumberPi;
        assert_eq!(subject.definition().value, 3.141_592_653_589_793_238_462_643_383_279_502_884_197_169_399_375_105_820_974_944_592_3);
    }
}
