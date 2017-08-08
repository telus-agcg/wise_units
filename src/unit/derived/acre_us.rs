use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct AcreUS;

impl Unit for AcreUS {
    fn classification(&self) -> Classification { Classification::USLengths }
    fn definition(&self) -> Definition { Definition::new(160.0, "[rd_us]2") }
    fn dim(&self) -> Dimension { Dimension::Length }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["acre".to_string()] }
    fn primary_code(&self) -> String { "[acr_us]".to_string() }
    fn print_symbol(&self) -> Option<String> { None }
    fn property(&self) -> Property { Property::Area }
    fn secondary_code(&self) -> String { "[ACR_US]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
