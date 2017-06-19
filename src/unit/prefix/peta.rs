use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Peta;

impl Prefix for Peta {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1e15, "1") }
    fn names(&self) -> Vec<String> { vec!["peta".to_string()] }
    fn primary_code(&self) -> String { "P".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("P".to_string()) }
    fn secondary_code(&self) -> String { "PT".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
