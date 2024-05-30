use crate::unit::ParserError;

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[allow(variant_size_differences)]
#[allow(clippy::large_enum_variant)]
#[allow(clippy::result_large_err)]
pub enum Error {
    #[error("Units are not compatible: {lhs:?}, {rhs:?}")]
    IncompatibleUnitTypes { lhs: String, rhs: String },

    #[error(transparent)]
    ParsingFailed(#[from] ParserError),

    #[error("Operation caused a divide by 0")]
    DivideByZero,
}
