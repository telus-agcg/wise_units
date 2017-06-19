use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct PartsPerThousand;

impl Unit for PartsPerThousand {
    fn classification(&self) -> Classification { Classification::Dimless }
    fn definition(&self) -> Definition { Definition::new(1.0, "10*-3") }
    fn dim(&self) -> Dimension { Dimension::None }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["parts per thousand".to_string()] }
    fn primary_code(&self) -> String { "[ppth]".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("ppth".to_string()) }
    fn property(&self) -> Property { Property::Fraction }
    fn secondary_code(&self) -> String { "[PPTH]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
