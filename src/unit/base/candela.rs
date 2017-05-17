use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct Candela;

impl Unit for Candela {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1.0, "1") }
    fn dim(&self) -> Dimension { Dimension::LuminousIntensity }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { true }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["candela".to_string()] }
    fn primary_code(&self) -> String { "cd".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("cd".to_string()) }
    fn property(&self) -> Property { Property::LuminousIntensity }
    fn secondary_code(&self) -> String { "CD".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Base }
}
