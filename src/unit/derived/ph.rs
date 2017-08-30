use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct PH;

impl Unit for PH {
    fn classification(&self) -> Classification { Classification::Chemical }
    fn definition(&self) -> Definition { Definition::new(1.0, "pH(1.0 mol/l)") }
    fn dim(&self) -> Dimension { Dimension::None }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { true }
    fn names(&self) -> Vec<String> { vec!["pH".to_string()] }
    fn primary_code(&self) -> String { "[pH]".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("pH".to_string()) }
    fn property(&self) -> Property { Property::Acidity }
    fn secondary_code(&self) -> String { "[PH]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }

    fn calculate_scalar(&self, magnitude: f64) -> f64 { 10.0_f64.powf(-magnitude) }
    fn calculate_magnitude(&self, scalar: f64) -> f64 { -scalar.log10() }
}
