mod atom_type;
pub mod base;

pub use atom::atom_type::AtomType;
pub use classification::Classification;
pub use dimension::Dimension;
pub use property::Property;

use std::cmp::PartialEq;
use std::fmt;

pub trait Atom {
    fn atom_type(&self) -> AtomType;
    fn classification(&self) -> Classification;
    fn dim(&self) -> Dimension;
    fn is_arbitrary(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_special(&self) -> bool;
    fn names(&self) -> Vec<String>;
    fn primary_code(&self) -> String;
    fn print_symbol(&self) -> Option<String>;
    fn property(&self) -> Property;
    fn scale(&self) -> f64;
    fn secondary_code(&self) -> String;
}

impl<'a> fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.primary_code())
    }
}

impl<'a> PartialEq for &'a Box<Atom> {
    fn eq(&self, other: &&'a Box<Atom>) -> bool {
        self.primary_code() == other.primary_code()
    }
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("debug self pc: {}", &self.primary_code());
        write!(f, "Atom ({})", &self.primary_code())
    }
}
