pub mod base;
pub mod derived;
pub mod prefix;
mod unit_type;
pub mod definition;

pub use classification::Classification;
pub use dimension::Dimension;
pub use measurement::Measurement;
pub use property::Property;

use std::cmp::PartialEq;
use std::fmt;
pub use unit::definition::Definition;
pub use unit::prefix::Prefix;
pub use unit::unit_type::UnitType;

pub trait Unit {
    fn classification(&self) -> Classification;
    fn definition(&self) -> Definition;
    fn dim(&self) -> Dimension;
    fn is_arbitrary(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_special(&self) -> bool;
    fn names(&self) -> Vec<String>;
    fn primary_code(&self) -> String;
    fn print_symbol(&self) -> Option<String>;
    fn property(&self) -> Property;
    fn secondary_code(&self) -> String;
    fn unit_type(&self) -> UnitType;

    fn scalar(&self) -> f64 { self.definition().scalar() }

    fn magnitude(&self) -> f64 { self.definition().magnitude() }

    fn calculate_scalar(&self, magnitude: f64) -> f64 {
        self.definition().calculate_scalar(magnitude)
    }

    fn calculate_magnitude(&self, scalar: f64) -> f64 {
        self.definition().calculate_magnitude(scalar)
    }
}

impl<'a> fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.primary_code()) }
}

impl<'a> PartialEq for &'a Box<Unit> {
    fn eq(&self, other: &&'a Box<Unit>) -> bool { self.primary_code() == other.primary_code() }
}

impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unit ({})", &self.primary_code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::base::*;
    use parser::parse_AtomSymbol;

    #[test]
    fn validate_parsing_by_primary_code() {
        let subject = parse_AtomSymbol("1").unwrap();
        let unit = Box::new(TheUnity) as Box<Unit>;
        assert_eq!(&subject, &unit);

        let subject = parse_AtomSymbol("m").unwrap();
        let unit = Box::new(Meter) as Box<Unit>;
        assert_eq!(&subject, &unit);
    }

    #[test]
    fn validate_parsing_by_secondary_code() {
        let subject = parse_AtomSymbol("M").unwrap();
        let unit = Box::new(Meter) as Box<Unit>;
        assert_eq!(&subject, &unit);
    }
}
