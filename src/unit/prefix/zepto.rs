use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Zepto;

impl Prefix for Zepto {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["zepto".to_string()] }
    fn primary_code(&self) -> String { "z".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("z".to_string()) }
    fn scalar(&self) -> f64 { 1e-21 }
    fn secondary_code(&self) -> String { "ZO".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
