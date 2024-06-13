use std::str::FromStr;

use crate::{Error, Unit};

//-----------------------------------------------------------------------------
// impl FromStr
//-----------------------------------------------------------------------------
impl FromStr for Unit {
    type Err = Error;

    #[inline]
    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(super::parser::parse(expression)?))
    }
}

#[cfg(test)]
mod tests {
    use super::{FromStr, Unit};

    #[test]
    fn validate_from_str_error() {
        let unit = Unit::from_str("ZZZXXXXXXXXXXXXx");
        assert!(unit.is_err());
    }

    #[test]
    fn validate_annotation() {
        let unit = Unit::from_str("{foo}").unwrap();
        let term = unit.terms.first().unwrap();

        assert_eq!(term.annotation(), Some("foo"));
    }
}
