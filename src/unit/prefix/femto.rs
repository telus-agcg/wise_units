use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Femto;

impl Prefix for Femto {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self)     -> Definition { Definition::new(1e-15, "1") }
    fn names(&self)          -> Vec<String> { vec!["femto".to_string()] }
    fn primary_code(&self)   -> String { "f".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("f".to_string()) }
    fn secondary_code(&self) -> String { "F".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Prefix }
}
