use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Giga;

impl Prefix for Giga {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["giga".to_string()] }
    fn primary_code(&self) -> String { "G".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("G".to_string()) }
    fn scalar(&self) -> f64 { 1e9 }
    fn secondary_code(&self) -> String { "GA".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
