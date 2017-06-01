use std::fmt;
use parser_terms::UnitSign;

#[derive(Debug, PartialEq)]
pub struct Exponent(pub UnitSign, pub u32);

impl Exponent {
    pub fn as_i32(&self) -> i32 {
        match self.0 {
            UnitSign::Positive => self.1 as i32,
            UnitSign::Negative => 0 - self.1 as i32,
        }
    }
}

impl fmt::Display for Exponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::Exponent;
    use parser::parse_Exponent;
    use parser_terms::UnitSign;

    #[test]
    fn validate_exponent() {
        assert_eq!(parse_Exponent("123").unwrap(),
                   Exponent(UnitSign::Positive, 123));
        assert_eq!(parse_Exponent("-123").unwrap(),
                   Exponent(UnitSign::Negative, 123));
    }
}
