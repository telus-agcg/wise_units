use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Milli;

impl Prefix for Milli {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1e-3, "1") }
    fn names(&self) -> Vec<String> { vec!["milli".to_string()] }
    fn primary_code(&self) -> String { "m".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("m".to_string()) }
    fn secondary_code(&self) -> String { "M".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
