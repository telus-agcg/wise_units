use std::fmt;

/// UCUM depends on dimensional analysis to reflect the meaning of units. It
/// uses dimensions to determine if two units are commensurable. Units that
/// end up just being some sort of scalar value (pi, ppm, mole, etc) simply do
/// not have a `Dimension`.
/// 
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Dimension {
    ElectricCharge,    // Q
    Length,            // L
    LuminousIntensity, // F
    Mass,              // M
    PlaneAngle,        // A
    Temperature,       // C
    Time,              // T
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Dimension::ElectricCharge => write!(f, "Q"),
            Dimension::Length => write!(f, "L"),
            Dimension::LuminousIntensity => write!(f, "F"),
            Dimension::Mass => write!(f, "M"),
            Dimension::PlaneAngle => write!(f, "A"),
            Dimension::Temperature => write!(f, "C"),
            Dimension::Time => write!(f, "T"),
        }
    }
}
