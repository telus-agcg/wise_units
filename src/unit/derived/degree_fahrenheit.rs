use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct DegreeFahrenheit;

impl Unit for DegreeFahrenheit {
    fn classification(&self) -> Classification { Classification::SI }
    fn definition(&self)     -> Definition { Definition::new(1.0, "degf(5 K/9)") }
    fn dim(&self)            -> Dimension { Dimension::Temperature }
    fn is_arbitrary(&self)   -> bool { false }
    fn is_metric(&self)      -> bool { true }
    fn is_special(&self)     -> bool { true }
    fn names(&self)          -> Vec<String> { vec!["degree Fahrenheit".to_string()] }
    fn primary_code(&self)   -> String { "[degF]".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("°F".to_string()) }
    fn property(&self)       -> Property { Property::Temperature }
    fn secondary_code(&self) -> String { "[DEGF]".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Derived }

    fn calculate_scalar(&self, magnitude: f64) -> f64 {
        5.0 / 9.0 * (magnitude + 459.67)
    }

    fn calculate_magnitude(&self, scalar: f64) -> f64 {
        9.0 * scalar / 5.0 - 459.67
    }
}
