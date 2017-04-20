use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Milli;

impl Prefix for Milli {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["milli".to_string()] }
    fn primary_code(&self) -> String { "m".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("m".to_string()) }
    fn scalar(&self) -> f64 { 1e-3 }
    fn secondary_code(&self) -> String { "M".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
