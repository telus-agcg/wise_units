use std::fmt;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Dimension {
    ElectricCharge,     // Q
    Length,             // L
    LuminousIntensity,  // F
    Mass,               // M
    PlaneAngle,         // A
    Temperature,        // C
    Time,               // T
    None,               // For dimless units
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Dimension::ElectricCharge => { write!(f, "Q") },
            Dimension::Length => { write!(f, "L") },
            Dimension::LuminousIntensity => { write!(f, "F") },
            Dimension::Mass => { write!(f, "M") },
            Dimension::PlaneAngle => { write!(f, "A") },
            Dimension::Temperature => { write!(f, "C") },
            Dimension::Time => { write!(f, "T") },
            Dimension::None => { Ok(()) },
        }
    }
}

