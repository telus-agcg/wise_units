use crate::parser::{
    symbols::symbol_parser::Rule as SymbolRule, terms::term_parser::Rule as TermRule,
};
use pest::error::Error as PestError;

/// Errors when trying to convert between types that aren't commensurable.
///
#[derive(Clone, thiserror::Error, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Error {
    #[error(transparent)]
    UnableToParseTerm(#[from] PestError<TermRule>),

    #[error(transparent)]
    UnableToParseSymbol(#[from] PestError<SymbolRule>),

    #[error(transparent)]
    UnableToParseInteger(#[from] std::num::ParseIntError),

    #[error("Unknown unit string fragment: {fragment} ({position})")]
    BadFragment { fragment: String, position: usize },

    /// Indicates the whole unit string/expression is bad.
    ///
    #[error("Unknown unit string: {0}")]
    UnknownUnitString(String),
}
