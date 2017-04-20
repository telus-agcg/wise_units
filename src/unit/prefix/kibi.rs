use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Kibi;

impl Prefix for Kibi {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["kibi".to_string()] }
    fn primary_code(&self) -> String { "Ki".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("Ki".to_string()) }
    fn scalar(&self) -> f64 { 1024.0 }
    fn secondary_code(&self) -> String { "KIB".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
