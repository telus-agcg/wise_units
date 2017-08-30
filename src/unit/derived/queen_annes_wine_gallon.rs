use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct QueenAnnesWineGallon;

impl Unit for QueenAnnesWineGallon {
    fn classification(&self) -> Classification { Classification::USVolumes }
    fn definition(&self) -> Definition { Definition::new(231.0, "[in_i]3") }
    fn dim(&self) -> Dimension { Dimension::Length }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["Queen Anne's wine gallon".to_string()] }
    fn primary_code(&self) -> String { "[gal_us]".to_string() }
    fn print_symbol(&self) -> Option<String> { None }
    fn property(&self) -> Property { Property::FluidVolume }
    fn secondary_code(&self) -> String { "[GAL_US]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}
