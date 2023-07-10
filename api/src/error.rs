use crate::parser::Error as ParserError;

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

    // TODO: Extract this from this enum. The cases where this can happen typically don't involve
    // any other error types--just this one; callers shouldn't have to handle all the variants here
    // if the call site can only ever return a single error.
    //
    #[error("Operation caused a divide by 0")]
    DivideByZero,
}
