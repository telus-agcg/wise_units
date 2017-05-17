use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Mebi;

impl Prefix for Mebi {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self)     -> Definition { Definition::new(1_048_576.0, "1") }
    fn names(&self)          -> Vec<String> { vec!["mebi".to_string()] }
    fn primary_code(&self)   -> String { "Mi".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("Mi".to_string()) }
    fn secondary_code(&self) -> String { "MIB".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Prefix }
}
