use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Femto;

impl Prefix for Femto {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["femto".to_string()] }
    fn primary_code(&self) -> String { "f".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("f".to_string()) }
    fn scalar(&self) -> f64 { 1e-15 }
    fn secondary_code(&self) -> String { "F".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
