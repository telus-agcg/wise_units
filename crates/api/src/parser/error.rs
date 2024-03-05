use crate::parser::{
    symbols::symbol_parser::Rule as SymbolRule, terms::term_parser::Rule as TermRule,
    tokenizer::Rule as UnitRule,
};
use pest::error::Error as PestError;

/// Errors when trying to convert between types that aren't commensurable.
///
#[derive(Clone, thiserror::Error, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Error {
    #[error(transparent)]
    #[cfg_attr(feature = "serde", serde(serialize_with = "stringify"))]
    UnableToParseTerm(#[from] PestError<TermRule>),

    #[error(transparent)]
    #[cfg_attr(feature = "serde", serde(serialize_with = "stringify"))]
    UnableToParseSymbol(#[from] PestError<SymbolRule>),

    #[error(transparent)]
    #[cfg_attr(feature = "serde", serde(serialize_with = "stringify"))]
    UnableToParseUnit(#[from] PestError<UnitRule>),

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

#[cfg(feature = "serde")]
fn stringify<E, S>(error: E, s: S) -> Result<S::Ok, S::Error>
where
    E: std::error::Error,
    S: serde::Serializer,
{
    s.serialize_str(&error.to_string())
}
