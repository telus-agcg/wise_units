use crate::unit::parser::Rule as UnitRule;
use pest::error::Error as PestError;

/// Errors when trying to convert between types that aren't commensurable.
///
#[derive(Clone, thiserror::Error, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Error {
    #[error(transparent)]
    #[cfg_attr(feature = "serde", serde(serialize_with = "stringify"))]
    UnableToParseUnit(Box<PestError<UnitRule>>),

    #[error(transparent)]
    #[cfg_attr(feature = "serde", serde(serialize_with = "stringify"))]
    UnableToParseInteger(#[from] std::num::ParseIntError),

    #[error("Unknown unit string fragment: {fragment} ({position})")]
    BadFragment { fragment: String, position: usize },

    /// Indicates the whole unit string/expression is bad.
    ///
    #[error("Unknown unit string: {0}")]
    UnknownUnitString(String),
}

impl From<PestError<UnitRule>> for Error {
    fn from(value: PestError<UnitRule>) -> Self {
        Self::UnableToParseUnit(Box::new(value))
    }
}

#[cfg(feature = "serde")]
fn stringify<E, S>(error: E, s: S) -> Result<S::Ok, S::Error>
where
    E: std::error::Error,
    S: serde::Serializer,
{
    s.serialize_str(&error.to_string())
}
