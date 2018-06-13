use pest;

/// Errors when trying to convert between types that aren't commensurable.
///
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Units are not compatible: {}, {}", lhs, rhs)]
    IncompatibleUnitTypes { lhs: String, rhs: String },

    #[fail(display = "Unable to parse expression: {}", expression)]
    UnableToParse { expression: String },

    #[fail(display = "Unknown unit string: {}", _0)]
    UnknownUnitString(String),
}

impl<'i, R: ::pest::RuleType> ::std::convert::From<pest::Error<'i, R>> for Error {
    fn from(pest_error: pest::Error<'i, R>) -> Self {
        Error::UnableToParse {
            expression: pest_error.to_string(),
        }
    }
}

impl ::std::convert::From<::std::num::ParseIntError> for Error {
    fn from(error: ::std::num::ParseIntError) -> Self {
        Error::UnableToParse {
            expression: error.to_string(),
        }
    }
}
