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

pub use self::yotta::Yotta;
pub use self::zetta::Zetta;
pub use self::exa::Exa;
pub use self::peta::Peta;
pub use self::tera::Tera;
pub use self::giga::Giga;
pub use self::mega::Mega;
pub use self::kilo::Kilo;
pub use self::hecto::Hecto;
pub use self::deka::Deka;
pub use self::deci::Deci;
pub use self::centi::Centi;
pub use self::milli::Milli;
pub use self::micro::Micro;
pub use self::nano::Nano;
pub use self::pico::Pico;
pub use self::femto::Femto;
pub use self::atto::Atto;
pub use self::zepto::Zepto;
pub use self::yocto::Yocto;
pub use self::mebi::Mebi;
pub use self::gibi::Gibi;
pub use self::tebi::Tebi;

use unit::{Classification, UnitType};
use std::fmt;

pub trait Prefix {
    fn classification(&self) -> Classification;
    fn names(&self) -> Vec<String>;
    fn primary_code(&self) -> String;
    fn print_symbol(&self) -> Option<String>;
    fn scalar(&self) -> f64;
    fn secondary_code(&self) -> String;
    fn unit_type(&self) -> UnitType;
}

impl<'a> fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.primary_code())
    }
}

impl<'a> PartialEq for &'a Box<Prefix> {
    fn eq(&self, other: &&'a Box<Prefix>) -> bool {
        self.primary_code() == other.primary_code()
    }
}

impl fmt::Debug for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Prefix ({})", &self.primary_code())
    }
}
