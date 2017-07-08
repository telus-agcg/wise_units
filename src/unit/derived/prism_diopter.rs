use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct PrismDiopter;

impl Unit for PrismDiopter {
    fn classification(&self) -> Classification { Classification::Clinical }
    // This is kinda fishy: 3/4 places in the UCUM read this, but one other place
    // reads "deg" instead of "rad".
    fn definition(&self) -> Definition { Definition::new(1.0, "100tan(1.0 rad)") }
    fn dim(&self) -> Dimension { Dimension::PlaneAngle }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { true }
    fn names(&self) -> Vec<String> { vec!["prism diopter".to_string()] }
    fn primary_code(&self) -> String { "[p'diop]".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("PD".to_string()) }
    fn property(&self) -> Property { Property::RefractionOfPrism }
    fn secondary_code(&self) -> String { "[P'DIOP]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }

    // Override the trait function to supply the function that's used for
    // converting from prism diopters to a plane angle.
    fn calculate_scalar(&self, magnitude: f64) -> f64 { magnitude.tan() * 100.0 }

    // Override the trait function to supply the function that's used for
    // converting from a plane angle to a prism diopter value.
    fn calculate_magnitude(&self, scalar: f64) -> f64 { (scalar / 100.0).atan() }
}
