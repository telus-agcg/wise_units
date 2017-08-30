use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct InchInternational;

impl Unit for InchInternational {
    fn classification(&self) -> Classification { Classification::Intcust }
    fn definition(&self) -> Definition { Definition::new(254e-2, "cm") }
    fn dim(&self) -> Dimension { Dimension::Length }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["inch".to_string()] }
    fn primary_code(&self) -> String { "[in_i]".to_string() }
    fn print_symbol(&self) -> Option<String> { None }
    fn property(&self) -> Property { Property::Length }
    fn secondary_code(&self) -> String { "[IN_I]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
