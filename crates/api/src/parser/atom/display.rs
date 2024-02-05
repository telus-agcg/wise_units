use std::fmt;

use crate::{Atom, UcumSymbol};

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.primary_code())
    }
}
