use std::fmt;

/// Exists just to represent part of the unit's AST when that unit is defined
/// by a function of other units.
///
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
