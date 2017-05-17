use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Nano;

impl Prefix for Nano {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self)     -> Definition { Definition::new(1e-9, "1") }
    fn names(&self)          -> Vec<String> { vec!["nano".to_string()] }
    fn primary_code(&self)   -> String { "n".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("n".to_string()) }
    fn secondary_code(&self) -> String { "N".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Prefix }
}
