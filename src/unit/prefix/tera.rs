use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Tera;

impl Prefix for Tera {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["tera".to_string()] }
    fn primary_code(&self) -> String { "T".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("T".to_string()) }
    fn scalar(&self) -> f64 { 1e12 }
    fn secondary_code(&self) -> String { "TR".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
