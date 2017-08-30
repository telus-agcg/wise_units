use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct GillUS;

impl Unit for GillUS {
    fn classification(&self) -> Classification { Classification::USVolumes }
    fn definition(&self) -> Definition { Definition::new(1.0, "[pt_us]/4") }
    fn dim(&self) -> Dimension { Dimension::Length }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["gill".to_string()] }
    fn primary_code(&self) -> String { "[gil_us]".to_string() }
    fn print_symbol(&self) -> Option<String> { None }
    fn property(&self) -> Property { Property::FluidVolume }
    fn secondary_code(&self) -> String { "[GIL_US]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
