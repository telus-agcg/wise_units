use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct TheUnity;

/// This isn't directly defined in the UCUM, yet is used all over the place.
impl Unit for TheUnity {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1.0, "1") }
    fn dim(&self) -> Dimension { Dimension::None }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { true }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["the unity".to_string()] }
    fn primary_code(&self) -> String { "1".to_string()}
    fn print_symbol(&self) -> Option<String> { None }
    fn property(&self) -> Property { Property::Unclassified }
    fn secondary_code(&self) -> String { "1".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Base }
}
