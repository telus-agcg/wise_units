use classification::Classification;
pub use dimension::Dimension;
use property::Property;
use unit::{Definition, Unit, UnitType};

#[derive(Debug, Default)]
pub struct AcreUS;

impl Unit for AcreUS {
    fn classification(&self) -> Classification { Classification::USLengths }
    fn definition(&self) -> Definition { Definition::new(160.0, "[rd_us]2") }
    fn dim(&self) -> Dimension { Dimension::Length }
    fn is_arbitrary(&self) -> bool { false }
    fn is_metric(&self) -> bool { false }
    fn is_special(&self) -> bool { false }
    fn names(&self) -> Vec<String> { vec!["acre".to_string()] }
    fn primary_code(&self) -> String { "[acr_us]".to_string() }
    fn print_symbol(&self) -> Option<String> { None }
    fn property(&self) -> Property { Property::Area }
    fn secondary_code(&self) -> String { "[ACR_US]".to_string() }
    fn unit_type(&self) -> UnitType { UnitType::Derived }
}

#[cfg(test)]
mod tests {
    use measurement::Measurement;
    use parser::parse_MainTerm;
    use parser_terms::Term;
    use lalrpop_util::ParseError;

    #[test]
    fn validate_parsing() {
        // match parse_MainTerm("acre") {
        //     Term(term) => println!("got term: {:?}", term),
        //     ParseError::InvalidToken(location) => println!("location: {:?}", location),
        //     ParseError::UnrecognizedToken(token, expected) => {
        //         println!("token: {:?}", token);
        //         println!("expected: {:?}", expected);
        //     },
        //     ParseError::ExtraToken(token) => println!("token: {:?}", token),
        //     ParseError::User(error) => println!("error: {:?}", error),
        // }
        let name = parse_MainTerm("acre");
        let primary_code = parse_MainTerm("[acr_us]");
        let secondary_code = parse_MainTerm("[ACR_US]");
        assert_eq!(name, primary_code);
        assert_eq!(primary_code, secondary_code);
    }

    #[test]
    fn validate_measurement_with_name() {
        let name = Measurement::new(5.0, "acre");
        let primary_code = Measurement::new(5.0, "[acr_us]");
        assert_eq!(name, primary_code);
        assert_eq!(name.value, primary_code.value);
        assert_eq!(name.term, primary_code.term);
    }

    #[test]
    fn validate_measurement_with_primary_code() {
        let primary_code = Measurement::new(5.0, "[acr_us]");
    }

    #[test]
    fn validate_measurement_with_secondary_code() {
        let secondary_code = Measurement::new(5.0, "[ACR_US]");
    }
}
