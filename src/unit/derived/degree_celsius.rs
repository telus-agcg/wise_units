use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct DegreeCelsius;

impl Unit for DegreeCelsius {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self) -> Definition { Definition::new(1.0, "cel(1.0 K)") }
    fn dim(&self) -> Dimension { Dimension::Temperature }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { true }
    fn is_special(&self) -> bool { true }
    fn names(&self) -> Vec<String> { vec!["degree Celsius".to_string()] }
    fn primary_code(&self) -> String { "Cel".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("°C".to_string()) }
    fn property(&self) -> Property { Property::Temperature }
    fn secondary_code(&self) -> String { "CEL".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }

    // Override the trait function to supply the function that's used for
    // converting from Kelvin to Celsius.
    fn calculate_scalar(&self, magnitude: f64) -> f64 { magnitude + 273.15 }

    // Override the trait function to supply the function that's used for
    // converting from Celsius to Kelvin.
    fn calculate_magnitude(&self, scalar: f64) -> f64 { scalar - 273.15 }
}
