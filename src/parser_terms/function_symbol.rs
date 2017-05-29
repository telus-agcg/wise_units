use std::fmt;

#[derive(Debug, PartialEq)]
pub enum FunctionSymbol {
    Cel,
    DegF,
}

impl fmt::Display for FunctionSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FunctionSymbol::Cel => write!(f, "cel"),
            FunctionSymbol::DegF => write!(f, "degf"),
        }
    }
}
