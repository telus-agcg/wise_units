use pest;
use unit_parser::Rule;

/// Errors when trying to convert between types that aren't commensurable.
///
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Units are not compatible: {}, {}", lhs, rhs)]
    IncompatibleUnitTypes {
        lhs: String,
        rhs: String,
    },

    #[fail(display = "Unable to parse expression: {}", expression)]
    ParsingError {
        expression: String,
    },

    #[fail(display = "Unknown unit string: {}", _0)]
    UnknownUnitString(String)
}

impl<I> ::std::convert::From<pest::Error<Rule, I>> for Error
    where I: pest::inputs::Input
{
    fn from(pest_error: pest::Error<Rule, I>) -> Self {
        Error::ParsingError {
            expression: pest_error.to_string()
        }
    }
}

impl ::std::convert::From<::std::num::ParseIntError> for Error {
    fn from(error: ::std::num::ParseIntError) -> Self {
        Error::ParsingError {
            expression: error.to_string()
        }
    }
}
