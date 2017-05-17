use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Exa;

impl Prefix for Exa {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1e18, "1") }
    fn names(&self) -> Vec<String> { vec!["exa".to_string()] }
    fn primary_code(&self) -> String { "E".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("E".to_string()) }
    fn secondary_code(&self) -> String { "EX".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
