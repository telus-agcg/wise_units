use classification::Classification;
use unit::{Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Yocto;

impl Prefix for Yocto {
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["yocto".to_string()] }
    fn primary_code(&self) -> String { "y".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("y".to_string()) }
    fn scalar(&self) -> f64 { 1e-24 }
    fn secondary_code(&self) -> String { "YO".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
