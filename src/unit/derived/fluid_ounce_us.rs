use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct FluidOunceUS;

impl Unit for FluidOunceUS {
    fn classification(&self) -> Classification { Classification::USVolumes }
    fn definition(&self) -> Definition { Definition::new(1.0, "[gil_us]/4") }
    fn dim(&self) -> Dimension { Dimension::Length }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["fluid ounce".to_string()] }
    fn primary_code(&self) -> String { "[foz_us]".to_string() }
    fn print_symbol(&self) -> Option<String> { Some("oz fl".to_string()) }
    fn property(&self) -> Property { Property::FluidVolume }
    fn secondary_code(&self) -> String { "[FOZ_US]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
