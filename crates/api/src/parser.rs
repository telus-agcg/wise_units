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
mod annotation_composition;
mod error;
pub(crate) mod term;

pub use self::{annotation_composition::AnnotationComposition, error::Error, term::Term};
