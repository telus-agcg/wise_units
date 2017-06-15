use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct Liter;

impl Unit for Liter {
    fn classification(&self) -> Classification { Classification::ISO1000 }
    fn definition(&self) -> Definition { Definition::new(1.0, "dm3") }
    fn dim(&self) -> Dimension { Dimension::Length }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { true }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["liter".to_string()] }
    fn primary_code(&self) -> String { "l".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("l".to_string()) }
    fn property(&self) -> Property { Property::Volume }
    fn secondary_code(&self) -> String { "L".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
