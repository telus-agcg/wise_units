use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct Radian;

impl Unit for Radian {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1.0, "1") }
    fn dim(&self) -> Dimension { Dimension::PlaneAngle }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { true }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["radian".to_string()] }
    fn primary_code(&self) -> String { "rad".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("rad".to_string()) }
    fn property(&self) -> Property { Property::PlaneAngle }
    fn secondary_code(&self) -> String { "RAD".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Base }
}
