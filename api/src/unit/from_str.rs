use parser::Error;
use std::str::FromStr;
use unit::Unit;

//-----------------------------------------------------------------------------
// impl FromStr
//-----------------------------------------------------------------------------
impl FromStr for Unit {
    type Err = Error;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        let terms = ::parser::parse(expression)?;

        Ok(Self::from(terms))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use unit::Unit;

    #[test]
    fn validate_from_str_error() {
        let unit = Unit::from_str("ZZZXXXXXXXXXXXXx");
        assert!(unit.is_err());
    }
}
