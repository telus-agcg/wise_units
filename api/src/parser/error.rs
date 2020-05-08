use pest;

/// Errors when trying to convert between types that aren't commensurable.
///
#[derive(Clone, thiserror::Error, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Error {
    #[error("Unable to parse expression: {}", expression)]
    UnableToParse { expression: String },

    #[error("Unknown unit string: {0}")]
    UnknownUnitString(String),
}

impl<'i, R: ::pest::RuleType> ::std::convert::From<pest::Error<'i, R>> for Error {
    fn from(pest_error: pest::Error<'i, R>) -> Self {
        Self::UnableToParse {
            expression: pest_error.to_string(),
        }
    }
}

impl ::std::convert::From<::std::num::ParseIntError> for Error {
    fn from(error: ::std::num::ParseIntError) -> Self {
        Self::UnableToParse {
            expression: error.to_string(),
        }
    }
}
