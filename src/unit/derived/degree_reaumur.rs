use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct DegreeReaumur;

impl Unit for DegreeReaumur {
    fn classification(&self) -> Classification { Classification::Heat }
    fn definition(&self) -> Definition { Definition::new(1.0, "degre(5.0 K/4)") }
    fn dim(&self) -> Dimension { Dimension::Temperature }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { true }
    fn names(&self) -> Vec<String> { vec!["degree Réaumur".to_string()] }
    fn primary_code(&self) -> String { "[degRe]".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("°R".to_string()) }
    fn property(&self) -> Property { Property::Temperature }
    fn secondary_code(&self) -> String { "[degRe]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }

    // Override the trait function to supply the function that's used for
    // converting from Réaumur to Kelvin.
    fn calculate_scalar(&self, magnitude: f64) -> f64 { (magnitude / 0.8) + 273.15 }

    // Override the trait function to supply the function that's used for
    // converting from Kelvin to Réaumur.
    fn calculate_magnitude(&self, scalar: f64) -> f64 { (scalar - 273.15) * 0.8 }
}
