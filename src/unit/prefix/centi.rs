use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Centi;

impl Prefix for Centi {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["centi".to_string()] }
    fn primary_code(&self) -> String { "c".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("c".to_string()) }
    fn scalar(&self) -> f64 { 1e-2 }
    fn secondary_code(&self) -> String { "C".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
