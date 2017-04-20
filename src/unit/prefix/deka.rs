use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Deka;

impl Prefix for Deka {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["deka".to_string()] }
    fn primary_code(&self) -> String { "da".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("da".to_string()) }
    fn scalar(&self) -> f64 { 1e1 }
    fn secondary_code(&self) -> String { "DA".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
