use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Pico;

impl Prefix for Pico {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1e-12, "1") }
    fn names(&self) -> Vec<String> { vec!["pico".to_string()] }
    fn primary_code(&self) -> String { "p".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("p".to_string()) }
    fn secondary_code(&self) -> String { "P".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
