use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Tebi;

impl Prefix for Tebi {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1_099_511_627_776.0, "1") }
    fn names(&self) -> Vec<String> { vec!["tebi".to_string()] }
    fn primary_code(&self) -> String { "Ti".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("Ti".to_string()) }
    fn secondary_code(&self) -> String { "TIB".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
