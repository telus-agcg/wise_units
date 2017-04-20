use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Pico;

impl Prefix for Pico {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["pico".to_string()] }
    fn primary_code(&self) -> String { "p".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("p".to_string()) }
    fn scalar(&self) -> f64 { 1e-12 }
    fn secondary_code(&self) -> String { "P".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
