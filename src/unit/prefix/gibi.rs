use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Gibi;

impl Prefix for Gibi {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["gibi".to_string()] }
    fn primary_code(&self) -> String { "Gi".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("Gi".to_string()) }
    fn scalar(&self) -> f64 { 1_073_741_824.0 }
    fn secondary_code(&self) -> String { "GIB".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
