use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Nano;

impl Prefix for Nano {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["nano".to_string()] }
    fn primary_code(&self) -> String { "n".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("n".to_string()) }
    fn scalar(&self) -> f64 { 1e-9 }
    fn secondary_code(&self) -> String { "N".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
