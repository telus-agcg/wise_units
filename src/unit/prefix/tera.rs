use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Tera;

impl Prefix for Tera {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1e12, "1") }
    fn names(&self) -> Vec<String> { vec!["tera".to_string()] }
    fn primary_code(&self) -> String { "T".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("T".to_string()) }
    fn secondary_code(&self) -> String { "TR".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
