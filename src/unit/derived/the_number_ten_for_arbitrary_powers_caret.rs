use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct TheNumberTenForArbitraryPowersCaret;

impl Unit for TheNumberTenForArbitraryPowersCaret {
    fn classification(&self) -> Classification { Classification::Dimless }
    fn definition(&self)     -> Definition { Definition::new(10.0, "1") }
    fn dim(&self)            -> Dimension { Dimension::None }
    fn is_arbitrary(&self)   -> bool { false }
    fn is_metric(&self)      -> bool { false }
    fn is_special(&self)     -> bool { false }
    fn names(&self)          -> Vec<String> { vec!["the number ten for arbitrary powers".to_string()] }
    fn primary_code(&self)   -> String { "10^".to_string()}
    fn print_symbol(&self)   -> Option<String> { Some("10".to_string()) }
    fn property(&self)       -> Property { Property::Number }
    fn secondary_code(&self) -> String { "10^".to_string()}
    fn unit_type(&self)      -> UnitType { UnitType::Derived }
}
