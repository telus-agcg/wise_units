use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct RodUS;

impl Unit for RodUS {
    fn classification(&self) -> Classification { Classification::USLengths }
    fn definition(&self) -> Definition { Definition::new(16.6, "[ft_us]") }
    fn dim(&self) -> Dimension { Dimension::Length }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["rod".to_string()] }
    fn primary_code(&self) -> String { "[rd_us]".to_string() }
    fn print_symbol(&self) -> Option<String> { None }
    fn property(&self) -> Property { Property::Length }
    fn secondary_code(&self) -> String { "[RD_US]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
