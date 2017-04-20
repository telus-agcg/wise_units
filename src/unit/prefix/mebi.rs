use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Mebi;

impl Prefix for Mebi {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["mebi".to_string()] }
    fn primary_code(&self) -> String { "Mi".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("Mi".to_string()) }
    fn scalar(&self) -> f64 { 1048576.0 }
    fn secondary_code(&self) -> String { "MIB".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
