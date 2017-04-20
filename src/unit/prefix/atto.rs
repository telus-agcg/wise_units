use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Atto;

impl Prefix for Atto {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["atto".to_string()] }
    fn primary_code(&self) -> String { "a".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("a".to_string()) }
    fn scalar(&self) -> f64 { 1e-18 }
    fn secondary_code(&self) -> String { "A".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
