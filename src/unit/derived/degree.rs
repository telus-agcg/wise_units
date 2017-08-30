use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct Degree;

impl Unit for Degree {
    fn classification(&self) -> Classification { Classification::ISO1000 }
    fn definition(&self) -> Definition { Definition::new(2.0, "[pi].rad/360") }
    fn dim(&self) -> Dimension { Dimension::PlaneAngle }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["degree".to_string()] }
    fn primary_code(&self) -> String { "deg".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("°".to_string()) }
    fn property(&self) -> Property { Property::PlaneAngle }
    fn secondary_code(&self) -> String { "DEG".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
