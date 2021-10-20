use std::{convert::AsRef, fmt};

/// UCUM depends on dimensional analysis to reflect the meaning of units. It
/// uses dimensions to determine if two units are commensurable. Units that
/// end up just being some sort of scalar value (pi, ppm, mole, etc) simply do
/// not have a `Dimension`.
///
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for Dimension {
    fn as_ref(&self) -> &str {
        match *self {
            Self::ElectricCharge => "Q",
            Self::Length => "L",
            Self::LuminousIntensity => "F",
            Self::Mass => "M",
            Self::PlaneAngle => "A",
            Self::Temperature => "C",
            Self::Time => "T",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Dimension;

    #[test]
    fn validate_display() {
        assert_eq!(&Dimension::Time.to_string(), "T");
    }
}
