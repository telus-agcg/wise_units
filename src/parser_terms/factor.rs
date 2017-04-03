use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Factor(pub u32);

impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
