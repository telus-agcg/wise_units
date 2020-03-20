use crate::parser::Error as ParserError;

#[derive(Clone, Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "Units are not compatible: {}, {}", lhs, rhs)]
    IncompatibleUnitTypes { lhs: String, rhs: String },

    #[fail(display = "{}", _0)]
    ParsingFailed(#[fail(cause)] ParserError),

    #[fail(display = "Operation caused a divide by 0")]
    DivideByZero,
}

impl From<ParserError> for Error {
    fn from(other: ParserError) -> Self {
        Self::ParsingFailed(other)
    }
}
