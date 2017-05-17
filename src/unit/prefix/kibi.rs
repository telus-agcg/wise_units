use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Kibi;

impl Prefix for Kibi {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1024.0, "1") }
    fn names(&self) -> Vec<String> { vec!["kibi".to_string()] }
    fn primary_code(&self) -> String { "Ki".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("Ki".to_string()) }
    fn secondary_code(&self) -> String { "KIB".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
