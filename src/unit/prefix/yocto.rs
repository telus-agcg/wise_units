use classification::Classification;
use unit::{Definition, Prefix, UnitType};

#[derive(Debug, Default)]
pub struct Yocto;

impl Prefix for Yocto {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1e-24, "1") }
    fn names(&self) -> Vec<String> { vec!["yocto".to_string()] }
    fn primary_code(&self) -> String { "y".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("y".to_string()) }
    fn secondary_code(&self) -> String { "YO".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Prefix }
}
