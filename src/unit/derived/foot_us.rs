use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct FootUS;

impl Unit for FootUS {
    fn classification(&self) -> Classification { Classification::USLengths }
    fn definition(&self) -> Definition { Definition::new(1200.0, "m/3937") }
    fn dim(&self) -> Dimension { Dimension::Length }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["foot".to_string()] }
    fn primary_code(&self) -> String { "[ft_us]".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("ft (us)".to_string()) }
    fn property(&self) -> Property { Property::Length }
    fn secondary_code(&self) -> String { "[FT_US]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
