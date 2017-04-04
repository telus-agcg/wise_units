use atom::{Atom, AtomType};
use classification::Classification;
pub use dimension::Dimension;
use property::Property;

#[derive(Debug, Default)]
pub struct PartsPerMillion;

impl Atom for PartsPerMillion {
    fn atom_type(&self) -> AtomType { AtomType::Derived }
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
}
