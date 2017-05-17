use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct Second;

impl Unit for Second {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self)     -> Definition { Definition::new(1.0, "1") }
    fn dim(&self)            -> Dimension { Dimension::Time }
    fn is_arbitrary(&self)   -> bool { false }
    fn is_metric(&self)      -> bool { true }
    fn is_special(&self)     -> bool { false }
    fn names(&self)          -> Vec<String> { vec!["second".to_string()] }
    fn primary_code(&self)   -> String { "s".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("s".to_string()) }
    fn property(&self)       -> Property { Property::Time }
    fn secondary_code(&self) -> String { "S".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Base }
}
