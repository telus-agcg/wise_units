use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Mega;

impl Prefix for Mega {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["mega".to_string()] }
    fn primary_code(&self) -> String { "M".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("M".to_string()) }
    fn scalar(&self) -> f64 { 1e6 }
    fn secondary_code(&self) -> String { "MA".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
