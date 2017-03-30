use atom::{Atom, AtomType};
use classification::Classification;
pub use dimension::Dimension;
use property::Property;

#[derive(Debug, Default)]
pub struct Coulomb;

impl Atom for Coulomb {
    fn arbitrary(&self) -> bool { false }
    fn atom_type(&self) -> AtomType { AtomType::Base }
    fn classification(&self) -> Classification { Classification::SI }
    fn names(&self) -> Vec<String> { vec!["Coulomb".to_string()] }
    fn primary_code(&self) -> String { "C".to_string()}
    fn print_symbol(&self) -> Option<String> { Some("C".to_string()) }
    fn property(&self) -> Property { Property::ElectricCharge }
    fn scale(&self) -> f64 { 1.0 }
    fn secondary_code(&self) -> String { "C".to_string()}
    fn dim(&self) -> Dimension { Dimension::ElectricCharge }
    fn metric(&self) -> bool { true }
    fn special(&self) -> bool { false }
}
