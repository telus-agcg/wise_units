use std::fmt;

#[derive(Debug, PartialEq)]
pub enum FunctionSymbol {
    Cel,
}

impl FunctionSymbol {
    pub fn calculate_scalar(&self, scalar: f64) -> f64 {
        match *self {
            FunctionSymbol::Cel      => { scalar + 273.15 }
        }
    }
}

impl fmt::Display for FunctionSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FunctionSymbol::Cel => write!(f, "cel")
        }
    }
}
