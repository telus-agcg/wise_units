use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Atto;

impl Prefix for Atto {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self)     -> Definition { Definition::new(1e-18, "1") }
    fn names(&self)          -> Vec<String> { vec!["atto".to_string()] }
    fn primary_code(&self)   -> String { "a".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("a".to_string()) }
    fn secondary_code(&self) -> String { "A".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Prefix }
}
