use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Hecto;

impl Prefix for Hecto {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self)     -> Definition { Definition::new(1e2, "1") }
    fn names(&self)          -> Vec<String> { vec!["hecto".to_string()] }
    fn primary_code(&self)   -> String { "h".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("h".to_string()) }
    fn secondary_code(&self) -> String { "H".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Prefix }
}
