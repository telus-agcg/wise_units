pub mod base;
pub mod derived;
pub mod prefix;
mod unit_type;

pub use unit::unit_type::UnitType;
pub use unit::prefix::Prefix;
pub use classification::Classification;
pub use dimension::Dimension;
pub use property::Property;

use std::cmp::PartialEq;
use std::fmt;

pub trait Unit {
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
    fn unit_type(&self) -> UnitType;
}

impl<'a> fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.primary_code())
    }
}

impl<'a> PartialEq for &'a Box<Unit> {
    fn eq(&self, other: &&'a Box<Unit>) -> bool {
        self.primary_code() == other.primary_code()
    }
}

impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unit ({})", &self.primary_code())
    }
}
