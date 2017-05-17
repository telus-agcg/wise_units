use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Deci;

impl Prefix for Deci {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1e-1, "1") }
    fn names(&self) -> Vec<String> { vec!["deci".to_string()] }
    fn primary_code(&self) -> String { "d".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("d".to_string()) }
    fn secondary_code(&self) -> String { "D".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
