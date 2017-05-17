use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Deka;

impl Prefix for Deka {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self)     -> Definition { Definition::new(1e1, "1") }
    fn names(&self)          -> Vec<String> { vec!["deka".to_string()] }
    fn primary_code(&self)   -> String { "da".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("da".to_string()) }
    fn secondary_code(&self) -> String { "DA".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Prefix }
}
