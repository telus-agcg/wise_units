use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Gibi;

impl Prefix for Gibi {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self)     -> Definition { Definition::new(1_073_741_824.0, "1") }
    fn names(&self)          -> Vec<String> { vec!["gibi".to_string()] }
    fn primary_code(&self)   -> String { "Gi".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("Gi".to_string()) }
    fn secondary_code(&self) -> String { "GIB".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Prefix }
}
