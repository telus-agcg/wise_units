use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Unit, UnitType};

#[derive(Debug, Default)]
pub struct PartsPerMillion;

impl Unit for PartsPerMillion {
    fn classification(&self) -> Classification { Classification::Dimless }
    fn dim(&self) -> Dimension { Dimension::None }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["parts per million".to_string()] }
    fn primary_code(&self) -> String { "[ppm]".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("ppm".to_string()) }
    fn property(&self) -> Property { Property::Fraction }
    fn scale(&self) -> f64 { 10.0e-6 }
    fn secondary_code(&self) -> String { "[PPM]".to_string()}
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
