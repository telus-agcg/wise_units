use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Zetta;

impl Prefix for Zetta {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1e21, "1") }
    fn names(&self) -> Vec<String> { vec!["zetta".to_string()] }
    fn primary_code(&self) -> String { "Z".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("Z".to_string()) }
    fn secondary_code(&self) -> String { "ZA".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
