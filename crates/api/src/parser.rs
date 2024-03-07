#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]

// Because long numbers are generated, there's no way (that I know of) to
// generate them using underscores (to make them pass the clippy lint).
#[cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::unreadable_literal,
        clippy::match_same_arms,
        clippy::too_many_lines,
        clippy::non_ascii_literal
    )
)]
pub mod composition;
#[allow(clippy::non_ascii_literal)]
pub mod property;

mod annotation_composition;
mod composable;
mod dimension;
mod error;
mod prefix;
pub(crate) mod term;
mod ucum_symbol;

pub use self::{
    annotation_composition::AnnotationComposition, composable::Composable,
    composition::Composition, dimension::Dimension, error::Error, prefix::Prefix,
    property::Property, term::Term, ucum_symbol::UcumSymbol,
};
