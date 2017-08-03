mod yotta;
mod zetta;
mod exa;
mod peta;
mod tera;
mod giga;
mod mega;
mod kilo;
mod hecto;
mod deka;
mod deci;
mod centi;
mod milli;
mod micro;
mod nano;
mod pico;
mod femto;
mod atto;
mod zepto;
mod yocto;
mod mebi;
mod gibi;
mod tebi;

pub use self::atto::Atto;
pub use self::centi::Centi;
pub use self::deci::Deci;
pub use self::deka::Deka;
pub use self::exa::Exa;
pub use self::femto::Femto;
pub use self::gibi::Gibi;
pub use self::giga::Giga;
pub use self::hecto::Hecto;
pub use self::kilo::Kilo;
pub use self::mebi::Mebi;
pub use self::mega::Mega;
pub use self::micro::Micro;
pub use self::milli::Milli;
pub use self::nano::Nano;
pub use self::peta::Peta;
pub use self::pico::Pico;
pub use self::tebi::Tebi;
pub use self::tera::Tera;
pub use self::yocto::Yocto;
pub use self::yotta::Yotta;
pub use self::zepto::Zepto;
pub use self::zetta::Zetta;
use std::fmt;

use unit::{Classification, Definition, UnitType};

pub trait Prefix {
    fn classification(&self) -> Classification;
    fn definition(&self) -> Definition;
    fn names(&self) -> Vec<String>;
    fn primary_code(&self) -> String;
    fn print_symbol(&self) -> Option<String>;
    fn secondary_code(&self) -> String;
    fn unit_type(&self) -> UnitType;
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.primary_code()) }
}

impl<'a> PartialEq for &'a Box<Prefix> {
    fn eq(&self, other: &&'a Box<Prefix>) -> bool { self.primary_code() == other.primary_code() }
}

impl fmt::Debug for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Prefix ({})", &self.primary_code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::parse_PrefixSymbol;

    #[test]
    fn validate_parsing_by_primary_code() {
        let subject = parse_PrefixSymbol("Y").unwrap();
        let prefix = Box::new(Yotta) as Box<Prefix>;
        assert_eq!(&subject, &prefix);

        let subject = parse_PrefixSymbol("Z").unwrap();
        let prefix = Box::new(Zetta) as Box<Prefix>;
        assert_eq!(&subject, &prefix);
    }

    #[test]
    fn validate_parsing_by_secondary_code() {
        let subject = parse_PrefixSymbol("YA").unwrap();
        let prefix = Box::new(Yotta) as Box<Prefix>;
        assert_eq!(&subject, &prefix);

        let subject = parse_PrefixSymbol("ZA").unwrap();
        let prefix = Box::new(Zetta) as Box<Prefix>;
        assert_eq!(&subject, &prefix);
    }
}
