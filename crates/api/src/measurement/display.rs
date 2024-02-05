use super::Measurement;
use std::fmt;

//-----------------------------------------------------------------------------
// impl Display
//-----------------------------------------------------------------------------
impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_display() {
        assert_eq!(
            &format!("{}", Measurement::try_new(1.1, "m").unwrap()),
            "1.1 m"
        );
        assert_eq!(
            &format!("{}", Measurement::try_new(1.1, "m2").unwrap()),
            "1.1 m2"
        );
        assert_eq!(
            &format!("{}", Measurement::try_new(1.1, "km2").unwrap()),
            "1.1 km2"
        );
        assert_eq!(
            &format!("{}", Measurement::try_new(1.1, "km2/rad.s").unwrap()),
            "1.1 km2/rad.s"
        );
        assert_eq!(
            &format!("{}", Measurement::try_new(1.1, "100km2/rad.s").unwrap()),
            "1.1 100km2/rad.s"
        );
    }
}
