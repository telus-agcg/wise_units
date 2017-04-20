use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Yotta;

impl Prefix for Yotta {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["yotta".to_string()] }
    fn primary_code(&self) -> String { "Y".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("Y".to_string()) }
    fn scalar(&self) -> f64 { 1e24 }
    fn secondary_code(&self) -> String { "YA".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
