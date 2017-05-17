use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Giga;

impl Prefix for Giga {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self)     -> Definition { Definition::new(1e9, "1") }
    fn names(&self)          -> Vec<String> { vec!["giga".to_string()] }
    fn primary_code(&self)   -> String { "G".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("G".to_string()) }
    fn secondary_code(&self) -> String { "GA".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Prefix }
}
