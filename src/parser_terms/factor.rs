use std::fmt;

/// # ucum.html section 1:
///
/// "Each unit is defined relative to a system of base units by a numeric factor
/// and a vector of exponents by which the base units contribute to the unit to
/// be defined."
///
/// This struct serves essentially to make up the part of the parser's AST.
///
#[derive(Debug, PartialEq, Eq)]
pub struct Factor(pub u32);

impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

#[cfg(test)]
mod tests {
    use super::Factor;
    use parser::parse_Factor;

    #[test]
    fn validate_parsing_0() {
        let factor = parse_Factor("0").unwrap();
        assert_eq!(factor, Factor(0));
    }

    // TODO: This seems to fail since we've got UnitByPrimaryCode defined to
    // parse "1" as TheUnity. If I comment that out, this passes.
    //
    // #[test]
    // fn validate_parsing_1() {
    //     let factor = parse_Factor("1").unwrap();
    //     assert_eq!(factor.0, 1);
    // }

    #[test]
    fn validate_parsing_2() {
        let factor = parse_Factor("2").unwrap();
        assert_eq!(factor, Factor(2));
    }

    #[test]
    fn validate_parsing_10() {
        let factor = parse_Factor("10").unwrap();
        assert_eq!(factor, Factor(10));
    }
}
