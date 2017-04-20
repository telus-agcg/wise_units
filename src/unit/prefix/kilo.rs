use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Kilo;

impl Prefix for Kilo {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["kilo".to_string()] }
    fn primary_code(&self) -> String { "k".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("k".to_string()) }
    fn scalar(&self) -> f64 { 1e3 }
    fn secondary_code(&self) -> String { "K".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
