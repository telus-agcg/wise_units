use measurement::Measurement;
use std::fmt;

//-----------------------------------------------------------------------------
// impl Display
//-----------------------------------------------------------------------------
impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_display() {
        assert_eq!(
            Measurement::new(1.1, "m").unwrap().to_string(),
            "1.1 m".to_string()
        );
        assert_eq!(
            Measurement::new(1.1, "m2").unwrap().to_string(),
            "1.1 m2".to_string()
        );
        assert_eq!(
            Measurement::new(1.1, "km2").unwrap().to_string(),
            "1.1 km2".to_string()
        );
        assert_eq!(
            Measurement::new(1.1, "km2/s").unwrap().to_string(),
            "1.1 km2/s".to_string()
        );
        assert_eq!(
            Measurement::new(1.1, "km2/rad.s").unwrap().to_string(),
            "1.1 km2/rad.s".to_string()
        );
        assert_eq!(
            Measurement::new(1.1, "100km2/rad.s").unwrap().to_string(),
            "1.1 100km2/rad.s".to_string()
        );
    }
}
