use atom::{Atom, AtomType};
use classification::Classification;
pub use dimension::Dimension;
use property::Property;

#[derive(Debug, Default)]
pub struct Kelvin;

impl Atom for Kelvin {
    fn arbitrary(&self) -> bool { false }
    fn atom_type(&self) -> AtomType { AtomType::Base }
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["Kelvin".to_string()] }
    fn primary_code(&self) -> String { "K".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("K".to_string()) }
    fn property(&self) -> Property { Property::Temperature }
    fn scale(&self) -> f64 { 1.0 }
    fn secondary_code(&self) -> String { "K".to_string()}
    fn dim(&self) -> Dimension { Dimension::Temperature }
    fn metric(&self) -> bool { true }
    fn special(&self) -> bool { false }
}
