use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Exa;

impl Prefix for Exa {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["exa".to_string()] }
    fn primary_code(&self) -> String { "E".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("E".to_string()) }
    fn scalar(&self) -> f64 { 1e18 }
    fn secondary_code(&self) -> String { "EX".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
