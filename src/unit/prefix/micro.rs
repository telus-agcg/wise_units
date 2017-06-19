use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Micro;

impl Prefix for Micro {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1e-6, "1") }
    fn names(&self) -> Vec<String> { vec!["micro".to_string()] }
    fn primary_code(&self) -> String { "u".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("μ".to_string()) }
    fn secondary_code(&self) -> String { "U".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
