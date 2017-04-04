use atom::{Atom, AtomType};
use classification::Classification;
pub use dimension::Dimension;
use property::Property;

#[derive(Debug, Default)]
pub struct Second;

impl Atom for Second {
    fn atom_type(&self) -> AtomType { AtomType::Base }
    fn classification(&self) -> Classification { Classification::SI }
    fn dim(&self) -> Dimension { Dimension::Time }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { true }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["second".to_string()] }
    fn primary_code(&self) -> String { "s".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("s".to_string()) }
    fn property(&self) -> Property { Property::Time }
    fn scale(&self) -> f64 { 1.0 }
    fn secondary_code(&self) -> String { "S".to_string()}
}
