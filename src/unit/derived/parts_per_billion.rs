use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct PartsPerBillion;

impl Unit for PartsPerBillion {
    fn classification(&self) -> Classification { Classification::Dimless }
    fn definition(&self) -> Definition { Definition::new(1.0, "10*-9") }
    fn dim(&self) -> Dimension { Dimension::None }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["parts per billion".to_string()] }
    fn primary_code(&self) -> String { "[ppb]".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("ppb".to_string()) }
    fn property(&self) -> Property { Property::Fraction }
    fn secondary_code(&self) -> String { "[PPB]".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
