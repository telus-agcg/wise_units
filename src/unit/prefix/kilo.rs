use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Kilo;

impl Prefix for Kilo {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1e3, "1") }
    fn names(&self) -> Vec<String> { vec!["kilo".to_string()] }
    fn primary_code(&self) -> String { "k".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("k".to_string()) }
    fn secondary_code(&self) -> String { "K".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
