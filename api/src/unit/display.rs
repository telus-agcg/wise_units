use std::fmt;
use unit::Unit;

//-----------------------------------------------------------------------------
// impl Display
//-----------------------------------------------------------------------------
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expression())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use unit::Unit;

    #[test]
    fn validate_display() {
        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.to_string().as_str(), "m");

        let unit = Unit::from_str("M").unwrap();
        assert_eq!(unit.to_string().as_str(), "m");

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.to_string().as_str(), "km/10m");

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/m");

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.to_string().as_str(), "10m");

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.to_string().as_str(), "10km");

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/10km");

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/km.m2");

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.to_string().as_str(), "km/m2.cm");

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.to_string().as_str(), "1/km.m2.cm");

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.to_string().as_str(), "m/s2");

        let unit = Unit::from_str("km3/nm2").unwrap();
        assert_eq!(unit.to_string().as_str(), "km3/nm2");
    }
}